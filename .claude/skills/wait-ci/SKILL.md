---
name: wait-ci
description: >
  Poll CI on a pull request until all required checks settle, then walk the
  PR body's test-plan checklist and resolve every unchecked item against a
  deterministic predicate library. Runs inside the autonomous loop — no stdin
  required. End-of-task gate: a task is "done" when `/wait-ci` returns
  `success` (or `pending-human` with the list surfaced).
when_to_use: >
  Right after `gh pr create`, any time a PR's checks have been red and
  been re-pushed, before marking an issue `status:needs-review`, and as
  Step 9 of `/next` — the loop's mandatory post-push gate.
argument-hint: "[pr-number]  # defaults to the PR attached to the current branch"
user-invocable: true
allowed-tools:
  - Bash(gh *)
  - Bash(git *)
  - Bash(awk *)
  - Bash(grep *)
  - Bash(rg *)
  - Bash(sed *)
  - Bash(jq *)
  - Bash(cat *)
  - Bash(sleep *)
  - Bash(date *)
  - Read
---

# wait-ci — post-push gate: CI green + test-plan checklist resolved

`PR number:` $ARGUMENTS (empty = PR attached to the current branch).

A PR is not "done" at `gh pr create`. It is done when:

1. every required check is green,
2. the PR body's **Test plan** checklist has been walked — every `- [ ]`
   item ends in one of three explicit states (verified / manual-pending /
   deferred-follow-up), and
3. the structured verdict is recorded as a single comment on the PR.

This skill enforces all three. It runs inside the autonomous loop, so
every decision is deterministic from the predicate library — **never
prompts the user**.

## Step 1 — Resolve the PR number

```bash
N="${ARGUMENTS:-}"
if [[ -z "$N" ]]; then
  N="$(gh pr view --json number --jq '.number' 2>/dev/null || true)"
fi
[[ -n "$N" ]] || { echo "no PR number supplied and none attached to current branch — abort"; exit 2; }
```

## Step 2 — Poll CI

Cap at 20 min (80 × 15 s). `gh pr checks --watch --fail-fast --interval 15`
blocks the right way:

- returns 0 when every required check is green,
- returns non-zero the moment a required check fails (fail-fast),
- stays blocked on pending until it flips one way or the other.

Wrap it in `timeout` so we surface a clear timeout instead of an open loop.

```bash
CI_START_SHA="$(gh pr view "$N" --json headRefOid --jq .headRefOid)"
echo "wait-ci: polling PR #$N at $CI_START_SHA (cap = 20 min)"
if timeout 1200 gh pr checks "$N" --watch --fail-fast --interval 15; then
  CI_VERDICT="green"
else
  RC=$?
  # timeout returns 124; anything else is a hard CI failure
  if [[ $RC -eq 124 ]]; then
    CI_VERDICT="timeout"
  else
    CI_VERDICT="fail"
  fi
fi
echo "wait-ci: CI_VERDICT=$CI_VERDICT"
```

## Step 3 — Handle the three CI outcomes

### 3a — `timeout`

Post a neutral marker comment so the next agent knows state, then return.
Do **not** flip the issue label; the PR is still in-flight.

```bash
if [[ "$CI_VERDICT" == "timeout" ]]; then
  gh pr comment "$N" --body "⏳ \`/wait-ci\` report at \`$CI_START_SHA\`

CI still pending after 20 min. Re-run \`/wait-ci $N\` to resume
watching — timeouts are usually GitHub-Actions queue pressure, not
a code problem.

<!-- wait-ci:timeout -->"
  exit 3
fi
```

Exit code 3 = `timeout` (agent may re-invoke later).

### 3b — `fail`

Extract the failing job, grab the last 50 lines of its log, post a
structured diagnosis comment. Do **not** walk the checklist — a red PR is
not "done" and the review label must not be set.

```bash
if [[ "$CI_VERDICT" == "fail" ]]; then
  FAILED="$(gh pr checks "$N" --json name,state,link \
    --jq '[.[] | select(.state == "FAILURE" or .state == "CANCELLED")]')"
  RUN_ID="$(echo "$FAILED" | jq -r '.[0].link // empty' \
    | grep -oE '/runs/[0-9]+' | grep -oE '[0-9]+')"
  LOG_TAIL=""
  if [[ -n "$RUN_ID" ]]; then
    LOG_TAIL="$(gh run view "$RUN_ID" --log-failed 2>/dev/null | tail -50)"
  fi

  {
    echo "❌ \`/wait-ci\` report at \`$CI_START_SHA\`"
    echo ""
    echo "Failing checks:"
    echo "$FAILED" | jq -r '.[] | "- \(.name): \(.state)"'
    echo ""
    echo "Last 50 lines of the failing log:"
    echo ""
    echo '```'
    [[ -n "$LOG_TAIL" ]] && echo "$LOG_TAIL" || echo "(log fetch failed — check the job link above)"
    echo '```'
    echo ""
    echo "Next: fix, commit, push, and re-run \`/wait-ci $N\`."
    echo ""
    echo "<!-- wait-ci:fail -->"
  } | gh pr comment "$N" --body-file -
  exit 4
fi
```

Exit code 4 = `fail` (caller should iterate per `/next` Step 9c).

### 3c — `green`: proceed to checklist walk

Continue to Step 4.

## Step 4 — Walk the test-plan checklist

Fetch the PR body and scan for `- [ ]` / `- [x]` task-list lines. A PR
body without any task-list syntax skips this layer cleanly — the final
summary reflects only the CI result.

```bash
BODY="$(gh pr view "$N" --json body --jq '.body')"

# Extract task-list lines with 1-based line numbers (against the body).
TASK_LINES="$(printf '%s\n' "$BODY" | awk '
  /^[[:space:]]*-[[:space:]]*\[[ xX]\][[:space:]]+/ { print NR "\t" $0 }')"
```

If `$TASK_LINES` is empty, jump to Step 6 with the zero-item summary.

### 4a — Apply predicates, line by line

Read [`predicates.md`](./predicates.md) as the single source of truth. The
library has three buckets, scanned in this order:

1. **auto-verify** — the item is covered by a CI check or by this
   skill's own self-tests. Flip `[ ]` → `[x]` and append
   ` — verified by /wait-ci @ <sha>`.
2. **auto-skip** — the item is manual by design (smoke-test, visual
   render). Leave `[ ]` unchanged, record it in the summary as
   "needs human verification".
3. **auto-defer** — the item describes work outside the PR's scope.
   Leave `[ ]` unchanged, record it as "deferred to follow-up" with a
   suggested `/file-issue` title.

**Fallthrough** (no bucket matches): treat as manual-pending, same as
auto-skip. Conservative default; never auto-verify without explicit match.

For each checked-already `[x]` line, pass through unchanged.

Bash driver (simplified — the real loop reads `predicates.md` into three
regex arrays and scans each `[ ]` line):

```bash
VERIFIED=()
PENDING=()
DEFERRED=()

NEW_BODY="$BODY"
while IFS=$'\t' read -r lineno line; do
  [[ -z "$line" ]] && continue

  # Strip leading "- [ ] " / "- [x] " to get the predicate text.
  TEXT="$(echo "$line" | sed -E 's/^[[:space:]]*-[[:space:]]*\[[ xX]\][[:space:]]+//')"

  if [[ "$line" =~ \[[xX]\] ]]; then
    # Already checked — leave alone.
    continue
  fi

  BUCKET="$(classify "$TEXT")"   # returns: verify | skip | defer | fallthrough
  case "$BUCKET" in
    verify)
      VERIFIED+=("$TEXT")
      NEW_LINE="$(echo "$line" | sed -E 's/\[ \]/[x]/')"
      NEW_LINE="${NEW_LINE} — verified by /wait-ci @ ${CI_START_SHA:0:7}"
      NEW_BODY="${NEW_BODY//$line/$NEW_LINE}"
      ;;
    skip|fallthrough)
      PENDING+=("$TEXT")
      ;;
    defer)
      DEFERRED+=("$TEXT")
      ;;
  esac
done <<< "$TASK_LINES"
```

`classify()` is a bash function that sources the regex arrays from
`predicates.md`. See the predicates file for the actual patterns and the
one-example-per-pattern rule when adding new ones.

### 4b — Patch the PR body

Only if `VERIFIED` is non-empty. Use `gh pr edit --body "$NEW_BODY"`; this
is PR-body-only metadata, no commit is touched.

```bash
if [[ ${#VERIFIED[@]} -gt 0 ]]; then
  printf '%s' "$NEW_BODY" | gh pr edit "$N" --body-file -
fi
```

## Step 5 — Delete any previous wait-ci marker comment

`/wait-ci` is idempotent. Re-running it should replace the previous
report, not append a new one.

```bash
PREV_IDS="$(gh api "repos/:owner/:repo/issues/${N}/comments" --paginate \
  --jq '.[] | select(.body | test("<!-- wait-ci:(success|pending-human|fail|timeout) -->")) | .id')"
for id in $PREV_IDS; do
  gh api "repos/:owner/:repo/issues/comments/${id}" --method DELETE >/dev/null || true
done
```

## Step 6 — Post the single structured summary comment

```bash
{
  if [[ ${#PENDING[@]} -eq 0 && ${#DEFERRED[@]} -eq 0 ]]; then
    VERDICT="success"
  else
    VERDICT="pending-human"
  fi

  echo "✅ \`/wait-ci\` report at \`${CI_START_SHA:0:7}\`"
  echo ""
  echo "**CI:** all required checks green."
  echo ""
  echo "**Test-plan checklist:**"
  if [[ ${#VERIFIED[@]} -gt 0 ]]; then
    echo "- ✅ auto-verified (${#VERIFIED[@]}):"
    printf '  - %s\n' "${VERIFIED[@]}"
  fi
  if [[ ${#PENDING[@]} -gt 0 ]]; then
    echo "- ⚠️ needs human verification (${#PENDING[@]}):"
    printf '  - %s\n' "${PENDING[@]}"
  fi
  if [[ ${#DEFERRED[@]} -gt 0 ]]; then
    echo "- 🔁 deferred to follow-up (${#DEFERRED[@]}):"
    printf '  - %s\n' "${DEFERRED[@]}"
  fi
  if [[ ${#VERIFIED[@]} -eq 0 && ${#PENDING[@]} -eq 0 && ${#DEFERRED[@]} -eq 0 ]]; then
    echo "- (PR body has no task-list syntax — nothing to walk.)"
  fi
  echo ""
  echo "**Verdict:** $VERDICT."
  if [[ "$VERDICT" == "pending-human" ]]; then
    echo ""
    echo "Merge is technically unblocked (no required review), but a human"
    echo "should verify the ⚠️ items and action the 🔁 items before closing."
  fi
  echo ""
  echo "<!-- wait-ci:${VERDICT} -->"
} | gh pr comment "$N" --body-file -
```

## Step 7 — Return verdict

- `success` — exit 0. Caller (typically `/next` Step 9b) flips the issue
  label to `status:needs-review` and hands the PR off cleanly.
- `pending-human` — exit 5. Caller may still flip the label, but must
  surface the `⚠️`/`🔁` lists to the user before releasing the claim.
- `fail` — exit 4. Caller iterates per `/next` Step 9c (max 3).
- `timeout` — exit 3. Caller may re-invoke later or escalate per
  `/next` Step 9e.

```bash
case "$VERDICT" in
  success)         exit 0 ;;
  pending-human)   exit 5 ;;
esac
```

## What NOT to do

- Do not run `/wait-ci` before `gh pr create`. It requires a PR number,
  not a commit SHA. For local pre-push checks use `/pre-commit-check` +
  `just ci`.
- Do not force-push during a `/wait-ci` run. The skill reads
  `headRefOid` at Step 2 and compares in subsequent comments; force-push
  invalidates that reference.
- Do not edit the PR body while `/wait-ci` is walking the checklist. The
  skill patches the body at Step 4b; concurrent edits would be lost.
- Do not flip the issue to `status:needs-review` when the verdict is
  `fail` or `timeout`. A red PR masquerading as ready-to-review poisons
  the review queue (AGENTS.md §6.1, `/next` Step 9c/9d).
- Do not hand-walk the checklist in a PR-author comment when `/wait-ci`
  is about to run. The skill's marker comment will overwrite yours. If
  you need to leave a manual note, post **after** `/wait-ci` ran.
- Do not bypass the skill by deleting its marker comment. The marker
  is the audit trail; if you disagree with a verdict, amend the
  predicates library (`predicates.md`) and re-run.
