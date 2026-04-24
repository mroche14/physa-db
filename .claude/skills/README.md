# physa-db agent skills

> Project-level Claude Code skills. Committed to the repo so every agent that
> clones this project inherits the same playbooks.

## How skills work here

Each subdirectory is one skill with a `SKILL.md` file. Claude Code exposes
project skills as slash commands (`/plan-feature`, `/run-stress chaos`, etc.),
or auto-invokes a skill when its description matches the user's request. Codex
reads the same canonical skill directories through a single directory-level
symlink (`.agents/skills -> ../.claude/skills`) — adding or removing a skill
here makes it visible to Codex automatically, no bridge-refresh step. Codex
skills are invoked with `$plan-feature`, `$run-stress`, etc., or by asking
Codex to use the named skill.

Skills exist to make **rules mechanical**. Rather than trusting every agent
to remember `AGENTS.md` §§7, 11, 12, 15 from memory, skills bake the relevant
rule-check into the workflow itself. An agent that uses `plan-feature` cannot
skip §15 by accident.

## Catalog

### Autonomy loop (Tier 0 — the "just keep going" primitives)

| Skill | Purpose | Enforces |
|-------|---------|----------|
| [`next`](./next/SKILL.md) | Claim the top-priority ready issue, create `agent/<n>-<slug>` branch, resume or invoke plan-feature — the entry point of the autonomous loop | §3, §6.1 |
| [`abandon`](./abandon/SKILL.md) | Hand back a claim (→ `status:ready` or `status:blocked`), always unassign. Clean exit for any unfinished task | §3, §6.1 |

Companion infra: [`.github/workflows/reap-stale-claims.yml`](../../.github/workflows/reap-stale-claims.yml) reverts dormant claims (> 24 h) to `ready` automatically.

### Rule-enforcers (Tier 1 — invoke these first)

| Skill | Purpose | Enforces |
|-------|---------|----------|
| [`onboard`](./onboard/SKILL.md) | Mandatory first read for any new agent — pillars, causal chain, rules, reading order | All |
| [`plan-feature`](./plan-feature/SKILL.md) | Before any code: locate FM row, workload anchor, first-principles derivation, acceptance criteria | §11, §15 |
| [`write-adr`](./write-adr/SKILL.md) | ADR template with FM refs and quantitative derivation; always starts as Proposed | §11, §15 |
| [`research-competitor`](./research-competitor/SKILL.md) | Codename-only private profile + attribution-free public delta | §7, ADR-0006 |
| [`pre-commit-check`](./pre-commit-check/SKILL.md) | CI-core gate plus pre-commit-only checks: fmt, clippy, tests, privacy check, secrets scan, conventional-commit | §§1, 4, 5, 7, 8 |
| [`review-pr`](./review-pr/SKILL.md) | Full reviewer checklist cross-referencing every applicable rule | §§1, 5, 7, 8, 11, 12, 15 |
| [`promote-adr`](./promote-adr/SKILL.md) | Proposed → Accepted promotion, gated on feature lock + quantitative derivation | §15 |

### Workflow automators (Tier 2)

| Skill | Purpose | Mandatory for |
|-------|---------|---------------|
| [`run-stress`](./run-stress/SKILL.md) | Run a stress scenario (smoke / chaos / soak / disk-full / oom / partition / clock-skew / poison / supernode) and check invariants | Any PR touching storage / MVCC / cluster |
| [`run-bench`](./run-bench/SKILL.md) | Reproducible criterion + iai-callgrind + macro bench with hardware context and baseline comparison | Any PR labelled `type:perf` |
| [`file-issue`](./file-issue/SKILL.md) | Canonical GitHub Issue template with FM row, acceptance criteria, labels | Every task > 1 h of effort |
| [`wait-ci`](./wait-ci/SKILL.md) | Poll CI after a push, walk the PR test-plan checklist against the predicate library, post a single verdict comment | Every `gh pr create` (Step 9 of `/next`) |

### Planned (Tier 3 — not yet authored)

These are on the backlog; open an issue if you need one before it ships.

- `write-proptest` — proptest scaffold for storage codecs and query equivalence.
- `write-loom-test` — loom harness for lock-free / atomic code.
- `write-fuzz-target` — `cargo fuzz` target scaffold for parser, wire
  protocol, storage codec.
- `run-fuzz` — `cargo fuzz` smoke / soak orchestration.
- `bench-regression` — standalone regression gate (current criterion vs stored
  baseline) — currently folded into `run-bench`.
- `design-subsystem` — delegate to the Plan subagent with a pre-filled brief
  (positioning + workloads + constraints).
- `explore-codebase` — delegate to the Explore subagent for a targeted survey.

## Conventions

- **Scope.** Every canonical skill lives under `.claude/skills/<name>/SKILL.md`.
  Project-level. No personal skills (`~/.claude/skills/`) — they would be
  invisible to agents running on another machine. Codex compatibility is handled
  automatically by the `.agents/skills -> ../.claude/skills` directory symlink —
  add a skill here, it shows up for Codex on its own.
- **Frontmatter.** `name` (optional; defaults to directory name), `description`
  (used by Claude to decide auto-invocation — front-load keywords),
  `when_to_use` (trigger phrases), `argument-hint`, `user-invocable`,
  `disable-model-invocation` (for risky ops), `allowed-tools` (pre-approved).
- **Length.** Skills stay ≤ 500 lines. Longer reference material lives in
  supporting files next to `SKILL.md`.
- **Tone.** Imperative. Skills are playbooks, not documentation — tell the
  agent what to do, in order.
- **Cross-references.** Link to `AGENTS.md`, `docs/requirements/`, ADRs by
  relative path from the `SKILL.md` location (i.e. prefix with `../../../`
  to reach repo root).
- **What NOT to do.** Every skill ends with a "What NOT to do" section. The
  section exists because the failure modes are more predictable than the
  success paths — so we document them explicitly.

## Authoring a new skill

1. Pick a name: lowercase, hyphens, ≤ 64 chars. Describe the action, not the
   noun (`run-stress`, not `stress-tests`).
2. Create `.claude/skills/<name>/SKILL.md` with frontmatter and body (≤ 500
   lines).
3. If the skill runs shell commands, declare `allowed-tools` for those
   commands to avoid permission prompts.
4. If the skill is destructive or governance-critical (like `promote-adr`),
   add `disable-model-invocation: true` so only a human can trigger it.
5. Add an entry to the catalog table above.
6. Add a cross-reference in [`AGENTS.md`](../../AGENTS.md) §16 (skills).
7. Open a `type:dx area:dx` PR with the new skill.

## Disabling and overriding

- To temporarily disable a skill: rename the directory with a `_` prefix
  (`_plan-feature`). Claude Code won't load it.
- To override a project skill with a personal one: place a skill of the
  same name under `~/.claude/skills/<name>/SKILL.md`. Personal skills take
  precedence over project skills per Claude Code's cascade rules.

## Related

- [`../../AGENTS.md`](../../AGENTS.md) — the top-level agent contract.
- [`../../docs/requirements/`](../../docs/requirements/) — what the skills
  are protecting (positioning, workloads, features, targets, non-goals).
- [`../../justfile`](../../justfile) — the recipes that most skills invoke.
