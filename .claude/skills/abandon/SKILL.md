---
name: abandon
description: >
  Hand back a claimed GitHub Issue — unassign and flip the status back
  to ready (default) or blocked (if waiting on an external dependency).
  Always unassigns; no agent holds a claim while waiting. The clean
  counterpart to /next: every started-but-unfinished task ends with
  /abandon or with a PR, never by walking away.
when_to_use: >
  "I'm blocked", "abandon this", "hand off", "cannot continue", "give
  up on this task", when an agent hits an obstacle it cannot resolve
  (missing context, needs-human decision, dependency not ready,
  environment broken, under-specified issue).
argument-hint: "[ready|blocked] <reason>   # mode defaults to 'ready'"
user-invocable: true
allowed-tools:
  - Bash(gh *)
  - Bash(git *)
  - Bash(date *)
  - Bash(jq *)
---

# abandon — hand back a claimed task cleanly

**Arguments:** $ARGUMENTS

`/abandon` is the only sanctioned exit from a claim that is not going
to become a PR. It always unassigns. The agent never holds a claim
while idle — a dormant `status:in-progress` with a silent assignee is
the exact failure mode the claim protocol exists to prevent.

## Two destination states

| Mode | End label | Use when |
|------|-----------|----------|
| `ready` (default) | `status:ready` | The work is picked up again immediately by the next agent. Partial code on the branch may be cherry-picked. |
| `blocked` | `status:blocked` | External dependency not ready (another PR must merge, an ADR must be decided, a tool must exist, a human must answer). Another agent cannot usefully pick this up yet. |

**Do NOT use `/abandon` when the task is done.** A finished task exits
via the PR flow (`gh pr create` → reviewer flips to `status:done` on
merge). `/abandon` is for unfinished work only.

## Step 1 — Identify the claim

```bash
CLAIM="$(gh issue list --assignee @me --label status:in-progress \
  --state open --json number,title --jq '.[0]')"
N="$(echo "$CLAIM" | jq -r '.number')"
TITLE="$(echo "$CLAIM" | jq -r '.title')"
```

If I hold **zero** claims, `/abandon` has nothing to do — stop.

If I hold **more than one** (violates the one-claim-per-agent rule in
AGENTS.md §6.1), report as a protocol bug and ask which to abandon.

## Step 2 — Make the reason actionable

Before writing the comment, collect:

- What was attempted — `git log --oneline main..HEAD | head -10`.
- The concrete obstacle — exact error, missing interface, ambiguous
  spec, environment gap.
- What the next agent (or human) needs — the unblocker.

Good reason: `"FM-103 IVF-PQ codec needs decision on F16 vs F32
payload layout before I can write the storage format — see open
question in ADR-0009 §Alternatives"`.

Bad reason: `"stuck"`, `"too hard"`, `"waiting"`.

## Step 3 — Push partial work (if any)

If commits exist on the agent branch, push so the next agent can
cherry-pick:

```bash
BRANCH="$(git branch --show-current)"
if [[ "$BRANCH" == agent/${N}-* ]] && [[ -n "$(git log main..HEAD 2>/dev/null)" ]]; then
  git push -u origin "$BRANCH"
fi
```

The branch survives the abandon — the reaper does not delete
branches, and the dashboard's activity stream reads from them.

## Step 4 — Flip labels and unassign

```bash
TS="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
MODE="${1:-ready}"   # first positional arg; defaults to 'ready'
REASON="${*:2}"      # rest of args = reason text

if [[ "$MODE" == "blocked" ]]; then
  END_LABEL="status:blocked"
else
  END_LABEL="status:ready"
fi

gh issue edit "$N" \
  --add-label "$END_LABEL" \
  --remove-label status:in-progress \
  --remove-assignee "@me"

gh issue comment "$N" --body "**Abandoned** at $TS → \`$END_LABEL\`.

**Reason:** $REASON

**Branch with partial work:** \`agent/${N}-…\` (pushed to origin if any commits).

**What the next agent / human needs:** <concrete unblocker>.

<!-- abandon-marker:$END_LABEL -->"
```

## Step 5 — Return to main

```bash
git switch main
```

Leave the `agent/<n>-*` branch alone — it is the record of the
attempt. The next `/next` invocation will start with a clean tree.

## Step 6 — What next

You hold zero claims now. You are free to invoke `/next` again on a
different ready issue, or stop the session.

If you abandoned as `blocked`, the issue will not reappear in `/next`
until a human (or another skill) flips it back to `status:ready`
after the blocker is resolved. If you abandoned as `ready`, it is
immediately claimable.

## What NOT to do

- Do not `/abandon` when the work is ready to review — that belongs
  in a PR, not a label flip.
- Do not `/abandon` without a comment explaining *why*. A bare label
  flip is invisible to the next agent and defeats the point of the
  protocol.
- Do not delete the `agent/<n>-*` branch. It is the audit trail for
  what was tried.
- Do not `/abandon` as `blocked` when the block is actually "I do not
  know how". That is a research task — invoke `/plan-feature` on a
  sub-issue, do not park the parent.
- Do not `/abandon` and immediately `/next` without the `git switch
  main` step. A dirty tree on a stale branch will contaminate the
  next task.
- Do not `/abandon` and leave yourself as assignee. The whole point
  is to release the claim; keeping the assignee defeats the protocol.
- Do not put personal information (email, legal name, home path,
  hostname) into the `$REASON` or the follow-up comment. The comment
  is public. AGENTS.md §10 forbids PII on public surfaces; refer to
  yourself only by the GitHub handle the assignee already exposes.
