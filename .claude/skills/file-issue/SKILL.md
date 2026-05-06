---
name: file-issue
description: >
  Create a GitHub Issue for physa-db following the canonical template — context,
  acceptance criteria, links (public only), out-of-scope. Applies the correct
  labels (area:, type:, status:, priority:, agent:) and traces the issue back
  to its FM row. Issues are the system of record (AGENTS.md §6).
when_to_use: >
  "file an issue", "open an issue", "create issue", at the end of
  `plan-feature`, when a bug is surfaced by `run-stress` or `run-bench`, or
  any time a task needs to become tracked work.
argument-hint: "[issue title]"
user-invocable: true
allowed-tools:
  - Bash(gh *)
  - Bash(git *)
  - Read
  - Grep
---

# file-issue — canonical GitHub Issue template

**Draft title:** $ARGUMENTS

GitHub Issues are the single source of truth (`AGENTS.md` §6). No parallel
trackers. The dashboard SPA reads a snapshot of the issue graph, so a
well-structured issue also makes the dashboard useful.

## Step 1 — Is this really an issue?

Before filing, verify:

- The work is at least 1 hour of effort (smaller = just do it in the
  current PR).
- It is not already covered by an existing issue (search with
  `gh issue list --search '<keyword>'`).
- It has a clear owner-type (a crate, a subsystem) and a clear
  exit condition.

Bugs, features, perf investigations, stress-test failures, research
tasks, docs gaps — all are issues. Refactors that nobody asked for are
not.

## Step 2 — Choose labels and milestone

Combine one label from each applicable prefix (`AGENTS.md` §6):

- **`area:`** — exactly one of: `storage`, `query`, `cluster`, `server`,
  `client`, `docs`, `benchmark`, `infra`, `dx`, `ai-native`, `research`.
- **`type:`** — exactly one of: `feature`, `bug`, `perf`, `refactor`,
  `research`, `docs`, `stress`, `adr`.
- **`status:`** — start at `status:ready` (work can begin) or
  `status:blocked` (waiting on something else).
- **`priority:`** — `p0` (drop everything) … `p3` (nice to have).
- **`agent:`** — `agent:good-first-task` for newly-onboarded agents,
  `agent:needs-human` if human judgement is required, `agent:long-running`
  if the task is > 1 day.

**Milestone is required.** Every issue must be filed against a milestone
(`AGENTS.md` §6). The `/next` skill filters by milestone; an issue with
none falls outside discovery and gets stranded. List milestones with
`gh api repos/:owner/:repo/milestones --jq '.[].title'` and pick the one
that captures *when* this work belongs. If the answer is "I don't know
yet", that decision must be made before filing — do **not** default to
the current milestone "to unblock". The CI gate
(`.github/workflows/issue-milestone-gate.yml`) will refuse a milestone-less
issue and force `agent:needs-human` until a human decides.

## Step 3 — Fill the template

Paste the following and fill every section. Each section below is a
required field — if a section is "N/A", say so and say WHY.

```markdown
### Context

<Why does this matter?>

- **Pillar:** Commercial | AI-agent-native | Both
- **Workload anchor (if AI-native):** W-A | W-B | W-C | W-D | W-E | W-F
- **FM row:** FM-NNN — <row title> (link: docs/requirements/feature-matrix.md#fm-NNN)
- **Governing ADR (if any):** ADR-NNNN or `none — no architectural decision required`
- **Upstream motivation:** <one paragraph. For a bug, include reproduction
  steps. For a feature, paraphrase the workload paragraph. For perf,
  include the current numbers.>

### Acceptance criteria

<Bulleted, mechanically verifiable.>

- [ ] Property tests pass: <target name>
- [ ] Stress scenarios pass: <scenario name(s)>
- [ ] Benchmarks: <target>, <threshold>
- [ ] Docs updated: <path>
- [ ] …

### Test plan

<How will we know this works? Concrete commands.>

```bash
just test
just stress <scenario>
just bench-compare main
```

### Optimization research brief

<Exhaustive per `plan-feature §3b` — research window, every URL
consulted (including dead-ends and "not applicable" reads), search
queries with engines + date filters, candidate libs with current
`crates.io` versions at plan date, SOTA method with year + citation +
disposition (adopt / adapt / reject), dead-ends explored, picked /
rejected with reasons, hardware floor vs target. Sources ≤ 12 months
old unless justified. Paste the brief from the planning step verbatim.>

<If this issue is trivial plumbing (docs, CI, renames, asset bumps,
label-only changes): write **"no optimisation surface"** on a single
line and move on.>

### Out of scope

<What this issue is NOT doing. Protect the scope against creep.>

- Not changing <other subsystem>
- Not addressing <adjacent concern> — see issue #M for that

### Links

- FM row: `docs/requirements/feature-matrix.md#FM-NNN`
- Workload: `docs/requirements/ai-agent-workloads.md#W-X`
- ADR: `docs/architecture/adr/NNNN-slug.md`
- Prior art: <public paper/blog only — never link to `private/`>
- Related issues: #A, #B

### Checklist for the author

- [ ] Title is imperative, ≤ 72 chars.
- [ ] Labels: area, type, status, priority, agent.
- [ ] Milestone assigned (no exceptions — see AGENTS.md §6).
- [ ] FM row cited (or "not applicable — explain").
- [ ] Optimization research brief present and exhaustive (or explicit
      "no optimisation surface" for trivial plumbing).
- [ ] No competitor names.
- [ ] No reference to `private/` paths or files.
```

## Step 4 — File it

The skill **refuses** to call `gh issue create` without `--milestone`.
The flag is mandatory — not a default, not a fallback. If you cannot
name the milestone, go back to Step 2 and decide; do not paper over
the gap with a placeholder.

```bash
# Sanity-check: milestone exists and is open.
MILESTONE="<milestone title>"   # e.g. "M0 — Foundation"
gh api repos/:owner/:repo/milestones --jq '.[] | select(.state == "open") | .title' \
  | grep -Fxq "$MILESTONE" \
  || { echo "Refusing to file: milestone '$MILESTONE' not found among open milestones." >&2; exit 1; }

gh issue create \
  --title "<imperative title>" \
  --label "area:<x>,type:<y>,status:ready,priority:<p>,agent:<q>" \
  --milestone "$MILESTONE" \
  --body-file /tmp/issue-body.md
```

If you use the browser instead of the CLI, the milestone field on the
right-hand sidebar must be set **before** clicking "Submit new issue".
The CI gate (see `.github/workflows/issue-milestone-gate.yml`) will
otherwise label the issue `agent:needs-human` and force a human to
decide which milestone the work belongs to.

## Step 5 — Post-create

- If this is the feature branch kickoff, open a PR stub with
  `Closes #<N>`.
- If this blocks other issues, add `Depends on #X` lines and update the
  project board dependencies.

## What NOT to do

- Do not create an issue just to track a thought. If you wouldn't
  accept the issue from someone else, don't file it.
- Do not reference `private/` from a public issue — the issue body is
  public (§7).
- Do not skip the acceptance criteria. An issue without AC is a wish.
- Do not omit the `FM row` field. If the row doesn't exist yet, invoke
  `plan-feature` first to add it.
- Do not use `status:in-progress` at creation time. Start at
  `status:ready`; the agent picking up the work flips it.
- Do not file an issue without a milestone. The CI gate will catch it
  and label it `agent:needs-human` — that's a triage burden you've
  punted to a maintainer. Pick the milestone in Step 2.
- Do not include personal information in the issue body (email
  addresses, legal names, phone numbers, home directory paths,
  hostnames). The issue is public. Refer to yourself or other
  contributors only by their GitHub handle. See AGENTS.md §10.
