---
name: next
description: >
  Continue the project — pick up the next ready GitHub Issue, claim it
  atomically (label + assignee), create an agent branch, and either invoke
  plan-feature or resume prior work. The single entry point for an agent
  that wants to "just keep going". Implements the claim protocol defined
  in AGENTS.md §6.1 so concurrent agents never double-task.
when_to_use: >
  "continue", "continue the project", "what's next", "next task", "pick
  up work", "resume", "keep going", at the start of a new agent session
  when no specific task was assigned.
argument-hint: "[milestone-filter]  # e.g. 'm1' to restrict to M1 issues"
user-invocable: true
allowed-tools:
  - Bash(gh *)
  - Bash(git *)
  - Bash(date *)
  - Read
  - Grep
---

# next — claim and continue the next ready task

**Milestone filter:** $ARGUMENTS (empty = current milestone from the open GitHub milestone with the earliest due date).

This skill is the autonomous loop's entry point. A fresh agent session
with no other context can invoke `/next` and end up working on a
well-scoped, claimed, branched issue — with zero risk of double-tasking
another agent (or human) already on it.

The skill enforces the **claim protocol** (AGENTS.md §6.1): GitHub is
the lock manager. An issue is "claimed" when it carries the
`status:in-progress` label **and** has an assignee. Atomic update via
the GitHub API ensures two agents can never both believe they own the
same issue — the loser sees the winner's assignee on re-fetch and
abandons.

## Step 0 — Sync with the remote

Before anything else, refresh every remote-tracking ref. On a
multi-dev / multi-agent project the local view goes stale fast
(branches pushed by others, new `agent/*` branches, advanced `main`),
and every later check — existing claims, candidate branches, FF-pull
of main — must read fresh data. A `/next` run on a stale clone can
silently branch off an out-of-date `main` and ship a broken diff.

```bash
git fetch --all --prune --tags
```

If the current `HEAD` is a strict-subset `agent/*` branch — the
common "the previous session ended on a merged branch" state — step
back onto `main` first, so the broader prune that follows can delete
it. The tree must be clean to do this safely; a dirty tree means
there is uncommitted work that belongs to the current task and has
not yet been released, so we stop instead.

```bash
CURRENT="$(git branch --show-current)"
if [[ "$CURRENT" == agent/* ]]; then
  UNREACHABLE="$(git rev-list --count "$CURRENT" --not origin/main 2>/dev/null || echo 999)"
  if [[ "$UNREACHABLE" == "0" ]]; then
    if [[ -n "$(git status --porcelain)" ]]; then
      echo "HEAD on strict-subset branch '$CURRENT' but tree is dirty — aborting."
      exit 1
    fi
    git switch main
    git pull --ff-only
    echo "HEAD rescued: $CURRENT is a strict subset of origin/main, switched to main."
  fi
fi
```

Then prune **every** local `agent/*` branch whose commits are already
reachable from `origin/main`, regardless of upstream state. This
catches the three leftover-state families observed in practice:

1. `[gone]` upstream + absorbed by main — the classical case (PR
   merged, GitHub auto-deleted the remote).
2. `[gone]` upstream + some divergence — we keep it, user may have
   local work to rescue.
3. **No upstream at all** — branch was created by a `/next` that
   never pushed (abandoned claim, short exploration). The old
   `[gone]`-only filter missed this forever; the catch-all below
   handles it.

**Safety rule unchanged**: only delete branches that are **strict
subsets** of `origin/main` (`git rev-list --count <br> --not
origin/main` == 0). Branches with any divergence survive the sweep
and are surfaced for human / original-agent inspection.

```bash
PRUNED=()
KEPT=()
git branch --list 'agent/*' --format='%(refname:short)' \
  | while read -r br; do
      [[ "$br" == "$(git branch --show-current)" ]] && continue
      UNREACHABLE="$(git rev-list --count "$br" --not origin/main 2>/dev/null || echo 999)"
      if [[ "$UNREACHABLE" == "0" ]]; then
        git branch -D "$br" >/dev/null && echo "pruned $br (strict subset of origin/main)"
      else
        echo "kept   $br: $UNREACHABLE commit(s) not on origin/main — inspect manually"
      fi
    done
```

Nothing outside the `agent/*` namespace is touched. Branches the dev
owns for other workflows (`feature/*`, `wip-*`, personal experiments)
are invisible to this sweep.

Then sweep **remote** orphan `agent/*` branches. Local pruning above
only catches branches whose remote was already deleted. The opposite
case is just as common and more damaging: a PR opened from
`agent/N-*` is **closed without merge** (intentional abandonment after
the work was redone or invalidated), but the remote branch is left
alive. Over time these accumulate, fool `/next` into believing work is
in flight on issues that are long-resolved, and — worst — silently
re-introduce stale FM / AFM IDs or competitor codenames if a future
agent cherry-picks them. Two real-world regression triggers we have
hit: (1) a stale branch's `feature-matrix-aggregate.md` re-using
`AFM-017..019` after main reassigned those IDs, and (2) a branch's
`non-goals.md` additions duplicating concepts main had already merged
under different IDs. Both fixable, both costly to debug.

Safety rule mirrors local pruning: only delete remote branches that
are either **fully reachable from `origin/main`** (a leftover after a
squash-merge the human did not push-delete), or tied to a **PR closed
without merge** (the diff is preserved by GitHub regardless of branch
existence — audit trail survives). Anything else has unmerged work
worth keeping alive for human review.

```bash
git for-each-ref --format='%(refname:short)' 'refs/remotes/origin/agent/*' \
  | sed 's|^origin/||' \
  | while read -r br; do
      UNREACHABLE="$(git rev-list --count "origin/$br" --not origin/main 2>/dev/null || echo 999)"
      if [[ "$UNREACHABLE" == "0" ]]; then
        # Fully absorbed by main — safe delete.
        git push origin --delete "$br" \
          && echo "swept origin/$br (fully reachable from main)"
      else
        # Has divergent commits. Last PR closed without merge?
        PR_STATE="$(gh pr list --state all --head "$br" \
          --json state,mergedAt,createdAt \
          --jq 'sort_by(.createdAt) | reverse | .[0]
                | "\(.state):\(.mergedAt // "null")"' 2>/dev/null)"
        if [[ "$PR_STATE" == "CLOSED:null" ]]; then
          # Explicitly abandoned — safe delete (PR keeps the diff).
          git push origin --delete "$br" \
            && echo "swept origin/$br (PR closed-not-merged)"
        else
          # Open PR, no PR yet, or genuinely merged elsewhere.
          echo "keeping origin/$br: $UNREACHABLE divergent commit(s), PR state=${PR_STATE:-none}"
        fi
      fi
    done
```

What this does NOT delete: branches with divergent commits and no PR
(work in flight), branches with an OPEN PR (under review), branches
that merged via something other than squash and look divergent for
hash reasons (rare, kept conservatively). Those surface as `keeping
origin/...` lines for human inspection.

Then verify local `main` is not diverged from `origin/main`. If
local main carries commits that origin doesn't, something was
committed directly to main and not pushed — that's a human issue, not
ours to paper over:

```bash
LOCAL_AHEAD="$(git rev-list --count origin/main..main 2>/dev/null || echo 0)"
if [[ "$LOCAL_AHEAD" -gt 0 ]]; then
  echo "main has $LOCAL_AHEAD local-only commits not on origin — investigate before /next"
  exit 1
fi
```

If this fails, stop and report. Never claim an issue on top of a
divergent local `main` — the branch you create will carry commits
that do not belong to the issue.

Finally, surface any local stashes — non-destructively. Stashes
encode in-flight work the skill has no right to drop (the user
stashed for a reason), but an accumulating stash list is a symptom
of abandoned branches the earlier sweep may not have caught.

```bash
STASH_COUNT="$(git stash list | wc -l)"
if [[ "$STASH_COUNT" -gt 0 ]]; then
  echo "⚠️ $STASH_COUNT local stash(es) present — inspect with 'git stash list':"
  git stash list
fi
```

The invariant after Step 0 is **mechanical**: a clone that finishes
a `/next` cycle holds only `main` + optionally the branch of the
currently claimed issue. No dangling `agent/*` branches, no stale
`[gone]` entries. If a user runs `/next` twice back-to-back on a
clean clone, the second run's sweep is a no-op.

## Step 0.5 — Resolve the agent identity (non-PII)

Every later step that writes the agent's identity to a surface — a
GitHub comment, a PR body, a branch name, a log — must read from
**this** resolver, never from `git config --get user.email`. The git
`user.email` field is designed for commit authoring (often a real
address); reusing it as a public display handle leaks PII onto issues
and PR threads. AGENTS.md §10 forbids that leak.

Resolution order (first match wins):

```bash
# 1. Opt-in repo-scoped alias — lets a dev run several agents from one clone.
AGENT_ID="$(git config --local --get physa.agent-id 2>/dev/null || true)"

# 2. Zero-config default — the GitHub handle is already public (it's the
#    value GitHub itself puts on the assignee, labels, commits via noreply).
if [[ -z "$AGENT_ID" ]]; then
  AGENT_ID="$(gh api user --jq .login 2>/dev/null || true)"
fi

# 3. First-run bootstrap — interactive only; no silent fallback to user.email.
if [[ -z "$AGENT_ID" ]]; then
  read -rp "Agent display name (stored in .git/config as physa.agent-id): " AGENT_ID
  [[ -n "$AGENT_ID" ]] || { echo "no identity provided — abort"; exit 1; }
  git config --local physa.agent-id "$AGENT_ID"
fi
```

Do **not** read `git config --get user.email`, `whoami`, `hostname`,
or `$HOME` for any identity that will be written to a public surface.
Those strings are PII under AGENTS.md §10.

## Step 1 — Pre-flight sanity checks

```bash
# Tree must be clean (no uncommitted changes on another task).
git status --porcelain
# Must be logged into gh with repo scope.
gh auth status
# Must be on main or on an existing agent/ branch. If on some other
# branch, the user was probably in the middle of something else; abort.
git branch --show-current
```

If any check fails, report the failure and stop. Do not attempt to
claim an issue if the tree is dirty — you will contaminate the next
task's diff.

## Step 2 — Check for an existing claim by this agent

A crashed / compacted agent that restarts must *resume*, not claim a
second issue. `$AGENT_ID` was resolved safely in Step 0.5 — reuse it.

```bash
# Issues I already claimed:
gh issue list --assignee @me --label status:in-progress --state open
```

If the list is **non-empty**:

1. Print the existing claim(s).
2. For each claim, look for an `agent/<n>-*` branch locally or on
   origin. If one exists, offer to `git switch` to it and continue.
3. Do **not** claim a new issue until the existing one is resolved
   (PR opened → `status:needs-review`, or released via `/release`).

If the list is empty, proceed to Step 3.

## Step 3 — Discover the ready queue

Build the candidate list. Precedence: priority (p0 → p3), then
milestone due-date ascending, then issue number ascending.

```bash
# Current milestone (earliest open due-date) unless user filtered.
MILESTONE="${ARGUMENTS:-$(gh api "repos/:owner/:repo/milestones?state=open&sort=due_on&direction=asc" \
  --jq '.[0].title')}"

# Ready, not blocked, not needing human, not already claimed.
gh issue list \
  --label status:ready \
  --milestone "$MILESTONE" \
  --state open \
  --limit 50 \
  --json number,title,labels,assignees,milestone,url \
  --jq '[.[] | select(
          (.assignees | length == 0) and
          ([.labels[].name] | contains(["status:in-progress"]) | not) and
          ([.labels[].name] | contains(["status:blocked"]) | not) and
          ([.labels[].name] | contains(["agent:needs-human"]) | not)
        )]'
```

Filter further by agent capability (optional but recommended):

- A newly-onboarded agent should prefer `agent:good-first-task`.
- A perf-specialised agent should prefer `type:perf`.
- A benchmark-runner agent should prefer `type:bench` or `area:benchmark`.

Sort by priority label (`p0`/`p1`/`p2`/`p3`) — pick the highest.

If the **first pass** is empty, **retry without the milestone filter**
before falling back any further. A ready, unassigned issue with no
milestone is almost always a triage gap (the maintainer filed a task
and forgot to milestone it), not a deliberate choice — and an external
contributor running `/next` should never have to debug that. The
widened query is identical except `--milestone "$MILESTONE"` is
dropped:

```bash
gh issue list \
  --label status:ready \
  --state open \
  --limit 50 \
  --json number,title,labels,assignees,milestone,url \
  --jq '[.[] | select(
          (.assignees | length == 0) and
          ([.labels[].name] | contains(["status:in-progress"]) | not) and
          ([.labels[].name] | contains(["status:blocked"]) | not) and
          ([.labels[].name] | contains(["agent:needs-human"]) | not)
        )]'
```

If a candidate surfaces here, claim it normally (Step 4) and add a
one-line note when reporting the claim:

> *Note: issue #N had no milestone — picked up via the widened fallback.*

The maintainer can re-tag at leisure; the contributor is unblocked.

If the **widened pass is also empty**:

- Check if any issues are `status:blocked` — unblocking one is
  higher-value than nothing.
- Otherwise, invoke `/plan-feature` on the next unstarted FM row (see
  `docs/requirements/feature-matrix.md` — find rows whose milestone
  matches `$MILESTONE` with no existing issue linked).
- Report to the user and stop.

## Step 4 — Atomic claim (optimistic + double-check)

Pick the top candidate. Let `N` be its issue number.

```bash
N=<top issue number>

# (a) Flip labels + assign in one API call each. The assignee is the
#     authoritative lock owner; no free-text claim comment is needed.
gh issue edit "$N" \
  --add-label status:in-progress \
  --remove-label status:ready \
  --add-assignee "@me"

# (b) DOUBLE-CHECK: re-fetch and confirm we are the sole assignee
#     and the label is set. Compare against the resolver-derived login,
#     never against $AGENT_ID directly (which may be an opt-in alias).
sleep 2
MY_LOGIN="$(gh api user --jq .login)"
VERDICT="$(gh issue view "$N" --json assignees,labels \
  --jq --arg me "$MY_LOGIN" '
        (.assignees | length == 1) and
        (.assignees[0].login == $me) and
        ([.labels[].name] | contains(["status:in-progress"]))')"

if [[ "$VERDICT" != "true" ]]; then
  # Another agent beat us or the update failed. Back off.
  gh issue edit "$N" --remove-assignee "@me" 2>/dev/null || true
  echo "Lost race on #$N — retrying with next candidate"
  # Go back to Step 3, exclude #N, retry.
fi
```

**Why no claim-marker comment.** The (label, assignee) pair is the
lock; a GitHub issue can hold both atomically. A redundant comment
adds a public surface that leaks whatever identity string the author
put in it — exactly the leak §10 forbids. The assignee event in the
issue timeline already records **who** claimed, and **when**, in a
structured, auditable way.

**Why double-check**: GitHub's label + assignee are eventually
consistent across replicas. In the 1–2 s window between our write and
a parallel agent's write, both writes may succeed. The double-check
catches the loser and lets them retry on the next issue rather than
silently double-tasking.

**Max 3 retries.** If we lose 3 races in a row, the queue is hot — back
off 60 s and re-enter Step 3.

## Step 5 — Create the agent branch

```bash
SLUG="$(gh issue view "$N" --json title --jq '.title' \
  | tr '[:upper:] ' '[:lower:]-' | tr -c 'a-z0-9-' '-' \
  | sed 's/--*/-/g; s/^-//; s/-$//' | cut -c1-40)"
git switch main
git pull --ff-only
git switch -c "agent/${N}-${SLUG}"
```

Branch naming `agent/<n>-<slug>` is load-bearing: the reaper
(`.github/workflows/reap-stale-claims.yml`) uses this convention to
detect stale claims via commit activity.

## Step 6 — Check for a plan, or invoke plan-feature

Read the issue body.

```bash
gh issue view "$N" --json body --jq '.body' > /tmp/issue-${N}.md
```

Look for:

- A filled-out "Acceptance criteria" section — if present, a
  plan-feature invocation already happened. Proceed to Step 7.
- FM row reference (e.g. `FM-103`) — if present but no AC, invoke
  `/plan-feature` with the FM context.
- Empty / thin body — the issue is under-spec'd. Either invoke
  `/plan-feature` to flesh it out, or `/abandon blocked
  "under-specified, needs human input"` and add the
  `agent:needs-human` label.

## Step 7 — Start work

Before writing code, confirm the issue body carries an
**Optimization research brief** (the section produced by
`/plan-feature` step 3b). If it is missing — common on older issues
or on tasks filed before that step existed — produce one inline
*before* touching code. One hour of web/rustdoc/SOTA scanning now is
the cheapest insurance against a compound-interest refactor later. If
the issue is trivial plumbing (docs, CI, renames), a one-line "no
optimisation surface" note is enough.

The inline brief must meet the same bar as `plan-feature §3b`:
≤ 12 months recency on sources, crate versions at their current
`crates.io` values, **every URL consulted logged exhaustively** (not
only those cited — dead-ends and "not applicable" reads too), search
queries + engines + date filters recorded, SOTA method named with
year + citation + disposition (adopt / adapt / reject), hardware
floor vs target. A brief that is three bullets is not a brief —
produce the full exhaustive form or `/abandon blocked
"under-specified: requires plan-feature with exhaustive research brief"`.

Then, and only then, begin the actual implementation. Use:

- `/run-stress <scenario>` for any storage / MVCC / cluster change.
- `/run-bench <mode>` if the issue is labelled `type:perf`.
- `/pre-commit-check` before each commit.
- `/review-pr` once the PR is open.

**Every commit** must reference the issue: `Refs #N` in the body, or
`Closes #N` on the final commit that the PR will merge. This feeds the
reaper's freshness check and the dashboard activity stream.

## Step 8 — Open the PR

When the work is ready for review:

```bash
gh pr create --fill --base main
PR_NUM="$(gh pr view --json number --jq '.number')"
```

Do **not** flip the issue label yet. The claim still belongs to this
agent until Step 9 confirms the PR is green — otherwise a red PR
becomes a stale `needs-review` that the reaper will not touch and no
other agent will pick up.

## Step 9 — Hand off to `/wait-ci`

A PR is not "done" at `gh pr create`. It is done when every required
check is green AND the PR body's test-plan checklist has been walked.
Both are the job of [`/wait-ci`](../wait-ci/SKILL.md); `/next` just
dispatches on the verdict.

```bash
/wait-ci "$PR_NUM"
CI_RC=$?
```

`/wait-ci` returns one of four exit codes. Dispatch:

### 9a — `success` (exit 0): hand off cleanly

All required checks green, every test-plan item auto-verified by the
predicate library. Flip the label and release the claim.

```bash
if [[ "$CI_RC" -eq 0 ]]; then
  gh issue edit "$N" \
    --add-label status:needs-review \
    --remove-label status:in-progress
  echo "PR #$PR_NUM green + checklist resolved — handed off for review"
  exit 0
fi
```

### 9b — `pending-human` (exit 5): hand off with a surfaced list

CI green, but one or more test-plan items fell into the manual-pending
or deferred-follow-up buckets. Technically mergeable, but the human
should see the list first.

```bash
if [[ "$CI_RC" -eq 5 ]]; then
  gh issue edit "$N" \
    --add-label status:needs-review \
    --remove-label status:in-progress
  echo "PR #$PR_NUM green — items pending human verification surfaced in \`/wait-ci\`'s comment on the PR"
  echo "Review the ⚠️ and 🔁 lists before merging."
  exit 0
fi
```

### 9c — `fail` (exit 4): diagnose, fix, re-push; max 3 iterations

`/wait-ci` already posted a structured failure comment with the last 50
lines of the failing job. Read it, reason about the root cause, apply
the fix (run `/pre-commit-check` before committing), push, re-invoke
`/wait-ci`.

Budget: **3 fix iterations**. Past that, the failure is almost
certainly not a surface bug — escalate per 9d.

```bash
ITER=0
while [[ "$CI_RC" -eq 4 && "$ITER" -lt 3 ]]; do
  ITER=$((ITER + 1))
  echo "=== CI fix iteration $ITER/3 ==="

  # The structured diagnosis is already on the PR — scroll `gh pr view --comments`.
  gh pr view "$PR_NUM" --comments --json comments \
    --jq '[.comments[] | select(.body | test("<!-- wait-ci:fail -->"))] | last | .body' \
    | head -80

  # Fix the code, commit (conventional prefix `fix(ci): …`), push.
  # Then re-run /wait-ci for the next round:
  /wait-ci "$PR_NUM"
  CI_RC=$?
done
```

### 9d — Still `fail` after 3 iterations: hand to a human

The failure is not mechanical. Don't burn more cycles.

```bash
if [[ "$CI_RC" -eq 4 ]]; then
  gh pr comment "$PR_NUM" --body "CI failing after 3 automated fix \
attempts. Handing to human — see the \`/wait-ci\` failure comments \
above for the log tails. Likely root cause: <one-sentence summary>."
  gh issue edit "$N" \
    --add-label agent:needs-human \
    --remove-label status:in-progress
fi
```

Do **not** mark the issue `status:needs-review` while CI is red — a
red PR masquerading as ready-to-review poisons the review queue and
demoralises the human reviewer who opens it expecting a clean diff.

### 9e — `timeout` (exit 3): surface and resume later

`/wait-ci`'s 20 min cap fired (GitHub Actions queue pressure, runner
outage, …). The PR isn't green yet, but there's no failing check to
fix either. Leave the claim in place, surface to the user, optionally
re-invoke.

```bash
if [[ "$CI_RC" -eq 3 ]]; then
  echo "PR #$PR_NUM CI still pending after 20 min — \`/wait-ci\` left a"
  echo "marker comment. Re-run \`/wait-ci $PR_NUM\` later, or \`gh run rerun\`"
  echo "the stuck workflow if it's been > 30 min with no state change."
  exit 6
fi
```

## What NOT to do

- Do not claim a second issue while you already hold one. One agent,
  one in-flight claim. Finish or release first.
- Do not skip the double-check in Step 4. The race window is small but
  non-zero, and a double-tasked issue wastes agent-hours.
- Do not use `git push --force` on an `agent/` branch after you have
  pushed it — other tooling (reaper, dashboard) reads commit
  timestamps from these branches. Force-push rewrites timestamps and
  may confuse staleness detection.
- Do not claim an issue labelled `agent:needs-human` — it explicitly
  wants a human in the loop. If an automated `/next` filters wrongly
  and surfaces one, `/abandon blocked "mis-filed: needs-human label"`.
- Do not repeatedly retry the same top issue if you lose the race.
  Move to the next candidate; the winner will finish or release.
- Do not claim issues across repositories. `/next` operates in the
  current repo only.
- Do not mark an issue `status:needs-review` while its PR has a red
  check. The label is a promise to the human reviewer; a red PR
  breaks it. Step 9 exists so this never happens.
- Do not walk away from a PR you opened. Own it through the first CI
  round; if you cannot stay, call `/abandon` with a clear hand-off
  comment on the PR so the next agent (or human) knows the state.
- Do not skip Step 0 when resuming a session. It is how the repo
  self-heals between cycles; skipping it leaves stale `agent/*`
  branches that accumulate across every run and pollute `git branch`
  until a human cleans up manually. The sweep is cheap (one
  `git fetch` + a few `rev-list` per branch) and idempotent.
- Do not read `git config --get user.email` as an identity source,
  and do not echo `$HOME`, `whoami`, or `hostname` into any GitHub
  comment, PR body, or issue body. Those strings are PII under
  AGENTS.md §10. Use the resolver from Step 0.5 instead — it returns
  the already-public GitHub login by default.
