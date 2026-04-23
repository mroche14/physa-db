# Contributing to physa-db

Welcome. This project is designed so both humans and AI agents can contribute with a minimum of friction.

## If you are a human contributor

1. Read [`README.md`](./README.md) and [`ROADMAP.md`](./ROADMAP.md).
2. Skim [`AGENTS.md`](./AGENTS.md) — it describes how the project is driven day-to-day. The rules apply to you too, not just to AI agents.
3. Pick an open issue labelled `status:ready`. Comment that you're picking it up.
4. Branch from `main` as `human/<gh-handle>/<slug>` (reserved `agent/*` prefix for AI agents).
5. Open a PR using the template. Expect review within 48h.

## If you are an AI agent

The entire workflow is encoded as project skills versioned under [`.claude/skills/`](./.claude/skills/) (Claude Code) and mirrored via symlinks under [`.agents/skills/`](./.agents/skills/) (Codex). A fresh clone already has every skill you need — no separate install.

**First session (one-time setup):**

```bash
git clone https://github.com/mroche14/physa-db.git
cd physa-db
mise install         # pinned toolchain: Rust, just, nextest, criterion, …
gh auth login        # the /next skill claims GitHub Issues atomically
```

Then open your agent in the repo (Claude Code, Codex, Cursor, …) and run:

1. **`/onboard`** — mandatory first read: two pillars, rules (§§7, 11, 12, 15), reading order.
2. **`/next`** — claims the next `status:ready` issue atomically, creates `agent/<n>-<slug>` branch, and either invokes `/plan-feature` or resumes prior work.

**While working:**

| Skill | When |
|---|---|
| `/plan-feature <desc>` | Before any code. Locks FM row + workload anchor + derivation + acceptance criteria |
| `/write-adr <NNNN> <title>` | When `/plan-feature` concluded an ADR is needed |
| `/research-competitor <CODENAME>` | Any competitor profile — codename-only, private input, public delta |
| `/pre-commit-check` | Before every commit — local mirror of the CI gate |
| `/run-stress <scenario>` | After any storage / MVCC / cluster change (mandatory evidence) |
| `/run-bench <mode> <baseline>` | Before claiming any perf win (mandatory for `type:perf` PRs) |
| `/review-pr <num>` | Reviewing any PR |
| `/file-issue <title>` | Any tracked task > 1 h of effort |
| `/abandon ready <reason>` | When you can't finish — always releases the claim, never walk away silently |

Codex users: invoke the same skills with `$onboard`, `$next`, `$plan-feature`, etc.

**Full contract:** read [`AGENTS.md`](./AGENTS.md) §§0–16 in full before touching anything. It is authoritative.

## Code of conduct

Be excellent. Detailed CoC to be added; in the meantime, the Rust community norms apply.

## Licensing

By contributing, you agree that your contribution is licensed under Apache-2.0, matching the project licence.

## Security

Please do NOT open public issues for security vulnerabilities. Contact the maintainer at `marvin@dynovant.com` (temporary; a security policy will be set up with M0).
