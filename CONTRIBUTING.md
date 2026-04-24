# Contributing to physa-db

physa-db is developed with AI coding agents (Claude Code, Codex, Cursor, …). Whether you are the solo maintainer or a brand-new external contributor, the workflow is the same: open an agent in the repo, run `/onboard`, then `/next`. The skills shipped in [`.claude/skills/`](./.claude/skills/) encode every rule from [`AGENTS.md`](./AGENTS.md) so you don't have to memorise them — invoking the right skill at the right moment is enough.

## Contents

- [Quickstart](#quickstart)
- [The dev loop, step by step](#the-dev-loop-step-by-step)
- [Skills by moment](#skills-by-moment)
- [Error recovery](#error-recovery)
- [Issue lifecycle](#issue-lifecycle)
- [Branch & commit conventions](#branch--commit-conventions)
- [Code of conduct](#code-of-conduct)
- [Licensing](#licensing)
- [Security](#security)

## Quickstart

**Prerequisites:** [`mise`](https://mise.jdx.dev/) · `git` · `gh` (GitHub CLI — `/next` needs it authenticated).

```bash
git clone https://github.com/mroche14/physa-db.git
cd physa-db
claude .            # or: codex, cursor, …
```

Then, inside your agent:

1. **`/onboard`** — installs the pinned toolchain (`mise install`), verifies `gh auth`, reads the rules and pillars, lists ready tasks. Run it once per fresh clone; subsequent sessions enter the loop directly at `/next`. (If an agent mid-session loses a specific rule, it should re-read the relevant `AGENTS.md` section from disk — not re-run the full onboarding.)
2. **`/next`** — claims the next `status:ready` GitHub Issue atomically, creates the `agent/<n>-<slug>` branch, and invokes `/plan-feature` if the issue has no plan yet.

After that, you're inside the dev loop.

Codex syntax: the same skills are invoked with `$onboard`, `$next`, `$plan-feature`, etc. — or by asking Codex to "use the `<name>` skill".

## The dev loop, step by step

```
/onboard → /next → /plan-feature → [code + tests] → /pre-commit-check → commit + push → gh pr create → /wait-ci
                                                                                                          ├─ ✅ green   → done
                                                                                                          ├─ ❌ fail    → fix & repush  OR  /abandon blocked
                                                                                                          └─ ⏳ timeout → surface & resume later
```

| # | Step | What happens |
|---|---|---|
| 1 | `/onboard` | Env bootstrap (`mise install` + `gh auth status`) · read AGENTS.md §§0–2, 11, 12, 15 · list `status:ready` + `agent:good-first-task` |
| 2 | `/next` | Atomically claim a ready issue (sets `status:in-progress` + `assignee`) · create `agent/<n>-<slug>` branch off up-to-date `main` · sync remote, prune stale branches |
| 3 | `/plan-feature` | Locate the FM row · identify workload anchor in `ai-agent-workloads.md` · first-principles derivation · lock acceptance criteria · decide whether an ADR is needed |
| 3b | `/write-adr` *(conditional)* | ADR template with FM refs and quantitative derivation — always starts as Proposed |
| 4 | Write code + tests | Follow AGENTS.md §§1, 4, 11, 12. No `unimplemented!()` stubs. Cite research inline. |
| 5 | `/run-stress` *(conditional)* | Mandatory evidence for any PR touching storage / MVCC / cluster (AGENTS.md §5) |
| 5b | `/run-bench` *(conditional)* | Mandatory evidence for any PR labelled `type:perf` (AGENTS.md §§1.2, 8) |
| 6 | `/pre-commit-check` | Local mirror of the CI gate: fmt, clippy, tests, doc-tests, link-check, private path check, secrets scan, conventional commit subject |
| 7 | `git commit && git push && gh pr create` | Conventional commit subject · PR template auto-fills summary + test plan · branch `agent/<n>-<slug>` carries the issue number |
| 8 | `/wait-ci` | Poll CI, walk the PR test-plan checklist against the predicate library, post the single verdict comment · returns `success` / `pending-human` / `fail` / `timeout` |

"Done" is **PR green + checklist resolved**, not "PR open". See AGENTS.md §6.1.

## Skills by moment

The skills catalog organised by *when to use*, not by tier. Every skill lives under `.claude/skills/<name>/SKILL.md` and is visible to Codex via the directory symlink `.agents/skills -> ../.claude/skills`.

| Moment | Skill | Enforces | Mandatory for |
|---|---|---|---|
| Start of session | [`/onboard`](.claude/skills/onboard/SKILL.md) | all rules | every new agent session |
| Pick up work | [`/next`](.claude/skills/next/SKILL.md) | §§3, 6.1 | every new task |
| Before code | [`/plan-feature`](.claude/skills/plan-feature/SKILL.md) | §§11, 15 | every feature / behavioural change |
| Design doc | [`/write-adr`](.claude/skills/write-adr/SKILL.md) | §§11, 15 | when `/plan-feature` flagged it |
| Competitor research | [`/research-competitor`](.claude/skills/research-competitor/SKILL.md) | §7, ADR-0006 | any competitor profile |
| Before commit | [`/pre-commit-check`](.claude/skills/pre-commit-check/SKILL.md) | §§1, 4, 5, 7, 8 | every commit |
| Storage / MVCC / cluster change | [`/run-stress`](.claude/skills/run-stress/SKILL.md) | §5 | every such PR |
| Perf claim | [`/run-bench`](.claude/skills/run-bench/SKILL.md) | §§1.2, 8 | every `type:perf` PR |
| After push | `/wait-ci` | §6.1 | every `gh pr create` |
| Reviewing a PR | [`/review-pr`](.claude/skills/review-pr/SKILL.md) | §§1, 5, 7, 8, 11, 12, 15 | every PR review |
| Track follow-up | [`/file-issue`](.claude/skills/file-issue/SKILL.md) | §6 | any task > 1 h of effort |
| Can't finish | [`/abandon`](.claude/skills/abandon/SKILL.md) | §§3, 6.1 | every unfinished task |
| Promote ADR to Accepted | [`/promote-adr`](.claude/skills/promote-adr/SKILL.md) | §15 | M2 exit |

**Rule (AGENTS.md §16):** if a skill exists for the action you are about to take, use it. A PR that could have been produced via `plan-feature` + `write-adr` + `pre-commit-check` but skipped them is evidence that the author missed a rule.

## Error recovery

| Situation | Action |
|---|---|
| `/next` finds no ready task | Check `status:blocked` for work waiting on you. If nothing fits, there genuinely is no ready work — surface it to the human. |
| `/plan-feature` can't locate the FM row for the issue | The issue is under-specified. Run `/abandon blocked "missing FM row"`, file a `needs-spec` follow-up issue, exit. |
| `/pre-commit-check` fails | Fix the reported issue (fmt / clippy / tests / secrets / private path). Re-run until green. Never `--no-verify`. |
| CI red after push (your change) | `/wait-ci` (or `gh pr checks <n>`) gives you the failing job + logs. Fix, commit, push, `/wait-ci` again. |
| CI red after push (pre-existing rot, not your diff) | File a follow-up issue via `/file-issue`, add a surgical exclusion in the config that owns it (e.g. `lychee.toml`), reference the follow-up issue in the exclusion comment. Keeps your PR mergeable without papering over the root cause. |
| You are stuck (context / dependency / environment) | `/abandon blocked "<reason>"` — releases the claim back to the pool. Don't walk away silently. |
| Session ended mid-task, claim still yours | New session: `/onboard` then `/next`. The skill sees your existing claim and resumes instead of claiming a new issue. |
| Specific rule forgotten mid-session | Re-read the relevant `AGENTS.md` section from disk. The full `/onboard` guided tour is for cold-start agents, not for re-hydration. |
| You left a dormant claim > 24 h | [`reap-stale-claims.yml`](.github/workflows/reap-stale-claims.yml) reverts it to `status:ready` automatically. No cleanup needed from you. |

## Issue lifecycle

```
          ┌──────────── /next ────────────┐
          │                               ▼
  status:ready ──────────────────→ status:in-progress + assignee
          ▲                               │
          │ reap-stale-claims             │ open PR
          │ (> 24 h dormant)              ▼
          │                          PR open (CI runs)
          │                               │
          │   /abandon ready              │ /wait-ci → green
          ├───────────────────────────────┤
          │                               ▼
          │                         PR merged → issue closed
          │
  status:blocked ← /abandon blocked (needs-human / waiting on dep)
```

A claim is always live — an agent holds it (assignee + `status:in-progress`), or releases it (`/abandon` → `ready` / `blocked`), or closes it via merged PR. No silent walk-aways.

## Branch & commit conventions

- **Branch names:** `agent/<issue-n>-<slug>` (AI agents, set by `/next`) · `human/<gh-handle>/<slug>` (human contributors) · `chore/<scope>-<slug>` (infra / DX / docs not tied to a feature issue).
- **Commit subjects:** [Conventional Commits](https://www.conventionalcommits.org/) — `type(scope): imperative subject`. Enforced by the `Conventional Commits` CI check.
- **PR size:** ≤ 600 lines non-generated code. If bigger, split (AGENTS.md §1.4).
- **PR body:** use the template. Fill the `## Test plan` section with imperative checklist items — `/wait-ci` pattern-matches them against its predicate library.

## Code of conduct

Be excellent. A detailed CoC will follow; in the meantime, the Rust community norms apply.

## Licensing

By contributing, you agree that your contribution is licensed under Apache-2.0, matching the project licence.

## Security

Do **not** open public issues for security vulnerabilities. Use GitHub's private vulnerability reporting: [report a security issue](https://github.com/mroche14/physa-db/security/advisories/new). Details in [`SECURITY.md`](./SECURITY.md).
