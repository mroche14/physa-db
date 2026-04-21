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

Then prune local branches whose upstream was deleted AND whose commits
are already reachable from `origin/main`. `--prune` cleans
remote-tracking refs but leaves local branches behind; left alone they
accumulate across every `/next` cycle, clutter `git branch`, and can
confuse the Step 1 sanity check if one of them matches `agent/*`.

**Safety rule:** a `[gone]` upstream alone is not enough — another
agent (or a previous self) may have local-only commits that were
never pushed, or were pushed and then the remote branch was deleted
for a reason other than merge. Deleting such a branch with `-D`
silently drops unmerged work. So check reachability first:

```bash
git branch --list --format='%(refname:short) %(upstream:track)' \
  | awk '$2 == "[gone]" {print $1}' \
  | while read -r br; do
      # Skip if this branch has ANY commit not reachable from origin/main.
      # That means real work is at risk — leave the branch alone and
      # surface it for the human / original agent.
      UNREACHABLE="$(git rev-list --count "$br" --not origin/main 2>/dev/null || echo 999)"
      if [[ "$UNREACHABLE" == "0" ]]; then
        git branch -D "$br"
      else
        echo "skipping $br: $UNREACHABLE commit(s) not on origin/main — inspect manually"
      fi
    done
```

Only branches that are **strict subsets** of `origin/main` get deleted
— those are provably safe (the squash-merge already absorbed their
content). Branches with any divergence survive the sweep.

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
second issue.

```bash
AGENT_ID="$(git config --get user.email || echo "$(whoami)@$(hostname)")"
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

If the list is **empty** after all filters:

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
TS="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
AGENT="$(git config --get user.email || echo "$(whoami)@$(hostname)")"

# (a) Flip labels + assign in one API call each.
gh issue edit "$N" \
  --add-label status:in-progress \
  --remove-label status:ready \
  --add-assignee "@me"

# (b) Post the claim marker comment.
gh issue comment "$N" --body "Claimed by \`$AGENT\` at $TS via \`/next\`.

<!-- claim-marker:$AGENT -->"

# (c) DOUBLE-CHECK: re-fetch and confirm we are the sole assignee
# and the label is set.
sleep 2
VERDICT="$(gh issue view "$N" --json assignees,labels \
  --jq '(.assignees | length == 1) and
        (.assignees[0].login == "<my-gh-login>") and
        ([.labels[].name] | contains(["status:in-progress"]))')"

if [[ "$VERDICT" != "true" ]]; then
  # Another agent beat us or the update failed. Back off.
  gh issue edit "$N" --remove-assignee "@me" 2>/dev/null || true
  echo "Lost race on #$N — retrying with next candidate"
  # Go back to Step 3, exclude #N, retry.
fi
```

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

## Step 9 — Watch the PR through CI

A PR is not "done" when pushed — it is done when every required check
passes. An agent that opens a PR and walks away ships the red to the
human reviewer. The agent owns its PR until CI is green.

### 9a — Block on the first round

```bash
gh pr checks "$PR_NUM" --watch --fail-fast --interval 10
CHECK_RC=$?
```

`--watch` blocks while any check is pending. `--fail-fast` returns
non-zero the moment one fails (no need to wait for the rest). The
`--interval 10` polls every 10 s — costs nothing on a quiet repo, and
is polite on GitHub's API.

### 9b — If all green

```bash
if [[ "$CHECK_RC" -eq 0 ]]; then
  gh issue edit "$N" \
    --add-label status:needs-review \
    --remove-label status:in-progress
  echo "PR #$PR_NUM green — handed off for review"
  exit 0
fi
```

### 9c — If any red: diagnose, fix, re-push

Budget: **3 fix iterations**. Past that, the failure is almost
certainly not a surface bug — it needs a human.

```bash
ITER=0
while [[ "$CHECK_RC" -ne 0 && "$ITER" -lt 3 ]]; do
  ITER=$((ITER + 1))
  echo "=== CI fix iteration $ITER/3 ==="

  # Which checks failed, and why.
  FAILED_JOBS="$(gh pr checks "$PR_NUM" --json name,state,link \
    --jq '[.[] | select(.state == "FAILURE" or .state == "CANCELLED")]')"
  echo "$FAILED_JOBS"

  # Fetch logs for each failed job. The `--log-failed` flag returns only
  # failing steps, which fits agent context windows better than full logs.
  RUN_ID="$(gh pr checks "$PR_NUM" --json link \
    --jq '.[] | select(.state == "FAILURE") | .link' \
    | head -1 | grep -oE '/runs/[0-9]+' | grep -oE '[0-9]+')"
  gh run view "$RUN_ID" --log-failed > /tmp/failed-${PR_NUM}-${ITER}.log

  # Read the log, reason about the root cause, apply the fix,
  # run `/pre-commit-check`, commit with a `fix(ci): …` prefix, push.
  # THEN re-enter the watch loop:
  gh pr checks "$PR_NUM" --watch --fail-fast --interval 10
  CHECK_RC=$?
done
```

### 9d — If still red after 3 iterations

The failure is not mechanical. Hand the PR back to a human instead of
wasting more cycles:

```bash
gh pr comment "$PR_NUM" --body "CI failing after 3 automated fix \
attempts. Handing to human — see \`/tmp/failed-${PR_NUM}-*.log\` \
snapshots in the last commits. Likely root cause: <one-sentence \
summary from the logs>."
gh issue edit "$N" \
  --add-label agent:needs-human \
  --remove-label status:in-progress
```

Do **not** mark the issue `status:needs-review` while CI is red — a
red PR masquerading as ready-to-review poisons the review queue and
demoralises the human reviewer who opens it expecting a clean diff.

### 9e — If the tree is clean but checks are stuck pending

GitHub Actions sometimes wedges (queue stall, runner outage). After 30
minutes of pending with no state transition:

```bash
gh pr comment "$PR_NUM" --body "CI stuck in pending for 30+ min — \
re-running via \`gh pr rerun\`."
gh run rerun "$RUN_ID" --failed
```

If the rerun also stalls, treat as 9d and escalate.

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
