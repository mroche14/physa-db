# AGENTS.md — Instructions for AI agents working on physa-db

> **If you are an AI agent (Claude Code, Codex, Cursor, Devin, Aider, or any future equivalent), read this file in full before touching anything.**

This project is designed from day one to be developed, reviewed, tested, and optimized primarily by AI agents — with human review at key gates. Treat this document as authoritative.

---

## 0. Prime directive

Build **the most efficient graph database the planet has ever seen**, in pure Rust, open source, with **two intertwined positioning pillars**:

1. **Commercial pillar** — end the incumbent's pricing era (summarised publicly in [`docs/requirements/positioning.md`](./docs/requirements/positioning.md) §1; founder's immutable long-form held privately). Full feature parity with existing graph DBs, both dominant query languages — **GQL (ISO/IEC 39075:2024) AND openCypher** — supported natively, no compromise, Apache-2.0 end-to-end, no enterprise-gated features.
2. **Technical pillar** — be **AI-agent-native** (captured in [`docs/requirements/positioning.md`](./docs/requirements/positioning.md)). The workloads driving the feature set are those produced by agentic AI systems: vector + graph hybrid retrieval, long-term agent memory, knowledge graphs with provenance and confidence, multi-modal asset storage, agent-trace observability, temporal reasoning. Specifics live in [`docs/requirements/ai-agent-workloads.md`](./docs/requirements/ai-agent-workloads.md).

The commercial pillar says *why* the project exists. The technical pillar says *why a user would pick us*. They compound: AI-agent-native features win workloads that never had a good graph DB answer, and Apache-2.0 + GQL/Cypher compatibility win migrations from the incumbent.

Every decision, PR, and commit is judged against one question:
**"Does this move us closer to that goal without compromising correctness?"**

Three mental tools govern how we answer:
1. **Features before architecture** — see §15.
2. **First-principles thinking** — see §11.
3. **No shortcuts, unlimited engineering budget** — see §12.

If the answer is unclear, write your reasoning in a new ADR (`docs/architecture/adr/NNNN-title.md`) and open a discussion issue before coding.

---

## 1. Ground rules

1. **Never fake work.** Do not stub out functions with `unimplemented!()` and mark the task done. If a task is too large, split it and only claim what you shipped.
2. **Benchmark-driven.** No performance claim without a reproducible benchmark in `benchmarks/`. "Faster" is meaningless without numbers.
3. **Correctness > speed.** A fast database that returns wrong results is useless. Every storage-layer change must pass the property-based test suite and the fuzz suite.
4. **Small PRs.** One logical change per PR. If a PR touches more than ~600 lines of non-generated code, split it.
5. **Cite research.** When implementing an algorithm or data structure, link to the paper/blog you drew from, in the PR description. If we built something novel, say "first-principles derivation" and show the math.
6. **English for code and public docs.** French is fine in conversations with Marvin, but code, identifiers, public docs, and commit messages are English.
7. **No secret in the repo.** If you find one, open a security issue immediately and do not include it in your diff.
8. **Never publish private research.** The `private/` directory is gitignored and must NEVER be committed, referenced by URL in public files, or summarised publicly with attribution. See §7 and ADR-0006.

---

## 2. Repository map

```
physa-db/
├── AGENTS.md              ← you are here
├── README.md              ← human-facing pitch
├── initial-vision.md      ← immutable founder's vision
├── ROADMAP.md             ← milestones & feature matrix
├── CONTRIBUTING.md        ← human contributor guide
├── SECURITY.md            ← responsible disclosure policy
├── LICENSE                ← Apache-2.0
├── justfile               ← unified dev commands (fmt, test, bench, stress, …)
├── .mise.toml             ← tool versions pinned per project (Rust, just, etc.)
├── rust-toolchain.toml
├── docs/
│   ├── architecture/
│   │   ├── adr/           ← numbered Architecture Decision Records
│   │   └── ...            ← subsystem design docs
│   ├── requirements/      ← PUBLIC synthesised feature set, targets, non-goals
│   ├── benchmarks/        ← PUBLIC methodology + historical results
│   └── dev-setup.md       ← one-command onboarding guide
├── private/               ← GITIGNORED; never commit contents
│   └── research/          ← competitor profiles, pain-point mining (sensitive)
├── crates/                ← Cargo workspace
│   ├── physa-core/        ← storage engine, graph data structures
│   ├── physa-query/       ← GQL + openCypher parsers, planner, executor
│   ├── physa-cluster/     ← Raft, sharding, replication, multi-tenancy
│   ├── physa-server/      ← Bolt v5 protocol, daemon, network
│   ├── physa-client/      ← native Rust driver
│   └── physa-cli/         ← `physa` command-line tool
├── xtask/                 ← workspace dev-task runner (heavy automation)
├── benchmarks/            ← criterion + LDBC SNB + SNAP harness
├── tests/
│   ├── integration/       ← cross-crate integration tests
│   └── stress/            ← chaos, soak, and stress test scenarios
├── dashboard/             ← GitHub Pages SPA (reads state.json)
├── .github/
│   ├── workflows/         ← ci.yml, snapshot-dashboard.yml, bench-regression.yml, release-plz.yml
│   ├── ISSUE_TEMPLATE/
│   └── agent-prompts/     ← reusable prompts for specific agent tasks
├── release-plz.toml       ← automated version bumping & changelog config
└── Cargo.toml             ← workspace root
```

---

## 3. How to pick up work

**TL;DR — invoke the `next` skill and it does all of this for you.** In Claude Code this is `/next`; in Codex use `$next` or ask "use the next skill". The skill encodes the protocol below and handles race conditions.

1. **Claim via `next`** — selects the top-priority `status:ready` issue in the current milestone, atomically flips it to `status:in-progress`, assigns you, and creates an `agent/<issue-number>-<short-slug>` branch. See §6.1 for the claim protocol.
2. **Follow the acceptance criteria** on the issue — they are contractual.
3. **Every commit references the issue** with `Refs #N` in the body (or `Closes #N` on the final commit). Feeds the dashboard and the reaper's freshness check.
4. **Open a PR** (`gh pr create --fill`) linking the issue. Include benchmark deltas if `type:perf`. The PR flips the issue to `status:needs-review`.
5. **If you cannot finish**, invoke `abandon <ready|blocked> <reason>` — never just walk away. A silent `status:in-progress` blocks the whole fleet.

If no issue matches your task, **stop and open one first** via `file-issue`. Issues are the system of record (§6).

---

## 4. Coding standards

- **Rust edition:** 2024 (or latest stable).
- **Toolchain:** pinned by `rust-toolchain.toml`. Reproducible via `mise install`.
- **Formatter:** `just fmt` (wraps `cargo fmt --all`).
- **Linter:** `just lint` (wraps `cargo clippy --workspace --all-targets -- -D warnings`).
- **Unsafe:** forbidden by default (`#![forbid(unsafe_code)]` per crate). Exception requires an entry in `docs/architecture/unsafe-allowlist.md` with justification and a property-tested safety wrapper.
- **`unwrap`/`expect`:** allowed only in tests, in `xtask/`, and in `main`. In library code, use `Result` + `thiserror`.
- **Async runtime:** `tokio` (multi-threaded). No second runtime permitted.
- **Error types:** per-crate `Error` enum via `thiserror`; `anyhow` only in binaries.
- **Naming:** follow Rust API guidelines. `physa_*` crate prefix for every workspace member.

---

## 5. Testing & verification standards

- **Unit tests:** in-file with `#[cfg(test)]`.
- **Integration tests:** under `tests/integration/`.
- **Property tests:** `proptest` for storage codecs, serialization, query planner equivalence.
- **Fuzzing:** `cargo fuzz` targets for parser (GQL + Cypher), wire protocol, storage codec. Until those targets land, `just fuzz-smoke` is a truthful scaffold that reports the gap without pretending coverage exists.
- **Concurrency:** `loom` for lock-free / atomic code. No concurrency primitive ships without a `loom` test.
- **Deterministic simulation:** `turmoil` (or equivalent) for cluster/network-partition scenarios.
- **Benchmarks:** `criterion` for wall-time micro-benches, `iai-callgrind` for instruction-stable benches, and a macro harness behind `just bench-macro`. During M0/M1 the macro command is a truthful placeholder scaffold until the real harness lands.
- **Stress tests:** `tests/stress/` defines the target scenario matrix and `just stress` is the entrypoint. During M0/M1 the command is a truthful placeholder scaffold until the real harness lands.
- **Coverage:** `cargo llvm-cov` is pinned for later use. Coverage reporting lands once the first non-trivial product-test surface exists. Coverage is a diagnostic, not a target.

Every **core** CI gate is a local `just` command — **if it passes locally, it must pass in CI**. Specialized workflows (for example `bench-regression`) may add extra checks on top.

---

## 6. Project tracking (the system of record)

**Single source of truth: GitHub Issues + GitHub Projects v2.** See ADR-0001.

- Every task, bug, feature, research item is a GitHub Issue.
- The GitHub Pages dashboard (`dashboard/`) reads a pre-generated `dashboard/data/state.json`, rebuilt by `snapshot-dashboard.yml` when that workflow is enabled.
- **Do not** create parallel tracking systems (TODO files, taskwarrior, etc.). If you need transient state inside a task, put it in a PR description or issue comment.

### Labels (canonical set)

| Prefix | Canonical labels | Meaning |
|--------|----------|---------|
| `area:` | `area:storage`, `area:query`, `area:cluster`, `area:server`, `area:client`, `area:docs`, `area:benchmark`, `area:infra`, `area:dx`, `area:research`, `area:ai-native` | Subsystem |
| `type:` | `type:feature`, `type:bug`, `type:perf`, `type:refactor`, `type:research`, `type:docs`, `type:stress`, `type:adr` | Nature of work |
| `status:` | `status:ready`, `status:in-progress`, `status:blocked`, `status:needs-review`, `status:done` | Lifecycle |
| `priority:` | `p0`, `p1`, `p2`, `p3` | Urgency |
| `agent:` | `agent:good-first-task`, `agent:needs-human`, `agent:long-running` | Agent-specific hints |

The source of truth for label metadata is [`.github/labels.yml`](./.github/labels.yml).
The `sync-labels` workflow creates missing labels and updates declared labels
on `main`; it does not delete stray labels automatically.

### Issue template
Every issue must have:
1. **Context** — why does this matter?
2. **Acceptance criteria** — concrete, testable bullets.
3. **Links** — prior art (public only — never link to `private/`), related issue.
4. **Out of scope** — what this issue is NOT about.

### 6.1 Claim protocol (autonomous agent coordination)

When multiple agents work in parallel, GitHub acts as the lock manager. The protocol below prevents double-tasking without a central coordinator; the `next` and `abandon` skills encode it mechanically.

- **Claim state.** An issue is claimed iff it carries `status:in-progress` **and** has at least one assignee. Both conditions together — a lone label or lone assignee is an invalid state.
- **Claim (via `next`).** The skill picks the top-priority `status:ready` issue in the active milestone, flips the label, adds the assignee, and posts a `<!-- claim-marker -->` comment. It then **re-fetches** the issue after ~2 s and aborts if a competing assignee appears — GitHub's label writes are eventually consistent across replicas, so the double-check is load-bearing.
- **One agent, one claim.** An agent that already holds a claim must finish it (→ PR → `status:needs-review`) or `abandon` it before claiming another.
- **Branch discipline.** Work happens on `agent/<issue-number>-<slug>`. Every commit includes `Refs #N`. The reaper uses commit timestamps on this branch as a liveness signal.
- **Clean exit.** Two sanctioned exits: a PR (flip to `status:needs-review`), or `abandon <ready|blocked> <reason>` (flip back, unassign). Never leave a silent `status:in-progress`.
- **Reaper.** `.github/workflows/reap-stale-claims.yml` runs every 6 h. Any `status:in-progress` issue with no issue activity **and** no commits on its `agent/` branch for 24 h is reverted to `status:ready` automatically. A compacted/crashed agent that restarts and re-claims via `next` will take over normally.

If you are extending the protocol (e.g. adding a claim TTL override label), update this section, the `next` and `abandon` skills, and the reaper workflow in the same PR.

### 6.2 Clean-repo invariant

After every `/next` run, the developer's clone holds **only** `main` + optionally the branch of the currently claimed issue. No dangling `agent/*` branches (whether from abandoned claims or merged PRs whose remote was auto-deleted), no stale `[gone]` upstreams, no HEAD parked on a merged branch. The Step 0 sweep in `/next` is the enforcer; invoking `/next` twice back-to-back on a clean clone is a no-op. Stashes are surfaced non-destructively but never auto-dropped — they may encode in-flight work no skill has the right to discard.

---

## 7. Research protocol (public output, private input)

Competitive research has **two halves**:

- **Input (sensitive, private):** raw competitor analysis, pain-point mining from Reddit/X/HN/forums, license dissection. Lives under `private/research/`. **Never commit. Never link publicly. Never reference competitor names with attribution in public artifacts.**
- **Output (public, attribution-free):** the synthesised feature set, non-goals, and performance targets. Lives under `docs/requirements/`. These describe what physa-db WILL do, without naming "because Neo4j does X" or "because users complain about Y on Reddit".

Rule: **a public PR may cite `docs/requirements/` but never `private/`.** If a reader can infer competitor attribution from a public file, rewrite the file.

If you are an agent running competitor research:
1. Open a `type:research` issue with a **codename** for the competitor (e.g. "Competitor ALPHA"). The codename↔real-name mapping is a local note in the issue thread, not committed.
2. Produce the profile under `private/research/competitors/<codename>.md`.
3. Extract ONLY the general feature/performance conclusions into a PR against `docs/requirements/`.

See ADR-0006 for the full rationale.

---

## 8. Commit & PR conventions

- **Commit style:** Conventional Commits (`feat(storage): ...`, `fix(query): ...`, `perf(cluster): ...`, `docs: ...`, `chore: ...`, `test: ...`, `bench: ...`).
- **Automated versioning:** `release-plz` reads conventional commits and proposes release PRs. Agents should not bump `Cargo.toml` versions manually.
- **Co-author tag:** when an AI agent authors, include `Co-Authored-By: <Agent-Name> <agent@example.com>` so attribution is transparent.
- **PR title:** imperative, ≤72 chars.
- **PR body:**
  - `## What` — one paragraph.
  - `## Why` — link to issue, explain motivation.
  - `## How` — key design choices. Note where first-principles reasoning was applied.
  - `## Benchmarks` (required for `type:perf`) — before/after numbers, hardware, command.
  - `## Stress` (required for `type:stress` and any concurrency change) — scenario, duration, outcome.
  - `## Checklist` — tests added, docs updated, ADR if architectural.
- **Required checks before merge:** `just ci` (the shared core gate) plus any applicable specialized workflows such as `bench-regression`.

---

## 9. Escalation

If an agent is stuck, lacks authority, or detects a cross-cutting concern:
1. Comment on the issue with the blocker.
2. Apply `status:blocked` and `agent:needs-human`.
3. Tag `@marvin` (project owner).

Do not silently drop work. Do not force a workaround that compromises §0.

---

## 10. Prohibited actions (without explicit human approval)

- Merging to `main`.
- Force-pushing to any shared branch.
- Deleting branches other than `agent/*` that you created.
- Publishing to crates.io (release-plz handles this once enabled).
- Creating new GitHub repositories.
- Adding a top-level dependency with a restrictive licence (GPL, AGPL, SSPL, BSL, commercial).
- Any git operation with `--no-verify`.
- **Committing anything from `private/` or referencing competitors by name in public files.**
- **Committing, pushing, or otherwise persisting any credential, API key, token, password, or secret of any kind.** This includes:
  - Scanning staged diffs for env-like patterns (`*_KEY=`, `*_TOKEN=`, `*_SECRET=`, `Bearer `, UUID-format API keys) before every commit.
  - Never running commands that would print a cred to the terminal (`echo $SECRET`, `curl -v` with auth headers, `cat .env`, partial-prefix displays via `head -c`). To verify a cred exists, use existence checks only: `[ -n "$KEY" ] && echo OK`.
  - If a cred is accidentally printed in the transcript, flag it to the human immediately — don't hide it.
  - Pre-commit hooks (§13) MUST include a secret-scan gate; treat its failure as a hard stop, never `--no-verify`.
- **Writing personal information (PII) to any public surface.** Public surfaces include: GitHub issue bodies, issue comments, PR titles, PR bodies, PR comments, review comments, release notes, CHANGELOG entries, commit messages that will be pushed, committed files, and any `gh api --method POST/PATCH` payload. PII includes:
  - Real-address email (anything not `@users.noreply.github.com`).
  - Full legal names (first + last). The GitHub handle alone is allowed — that's already public.
  - Phone numbers, postal addresses, physical locations more precise than country / region.
  - Absolute filesystem paths containing `/home/<name>/`, `/Users/<name>/`, or hostnames.
  - `whoami`, `hostname`, MAC addresses, IPs of dev / home machines.

  Identity resolver rule: any skill that needs to write "who did this"
  on a public surface uses this chain, in order, and never falls
  through to `git config --get user.email` or `whoami`:

  1. `git config --local --get physa.agent-id` — opt-in repo-scoped
     alias; lets a dev label multiple concurrent agents.
  2. `gh api user --jq .login` — the GitHub login; already public.
  3. Interactive prompt on first run, stored back to
     `git config --local physa.agent-id`.

  `/next` implements this in Step 0.5; other skills reuse the same
  chain. Reading `user.email` is forbidden for any path that writes
  to a public surface — the field is designed for commit authoring
  (often a real email) and is not a display handle.

  Mechanical enforcement: `/pre-commit-check` Gate 6 fails on any
  staged email literal except `*@users.noreply.github.com`, and on
  any staged skill file containing `git config --get user.email`.
  Treat that gate's failure the same as a secret leak: fix, re-stage,
  never `--no-verify`.

- Introducing a "shortcut" — a deliberately-suboptimal approach chosen because the better one is harder. See §12.

When in doubt, **ask**.

---

## 11. First-principles thinking

Our default cognitive mode is first-principles, not analogy.

**Analogy thinking (avoid as default):** "Other graph DBs use RocksDB, so we use RocksDB." "Neo4j uses Raft per shard, so we use Raft per shard."

**First-principles thinking (preferred):** ask, for each problem,
1. **What are the irreducible physical / informational constraints?** (Latency of an NVMe read, size of an L1 cacheline, entropy of a graph ID, CAP constraints of the consensus needed.)
2. **What is the theoretical optimum under those constraints?** (Minimum bytes moved, minimum round-trips, minimum synchronisation.)
3. **What is the smallest structure that realises that optimum?** Build from there.
4. **What prior art has already solved a sub-problem correctly?** Incorporate that piece, not the whole system.

Every non-trivial ADR must include a "first-principles derivation" section explaining the irreducible constraints and why the chosen design approaches the theoretical optimum.

**Concrete example.** For `adjacency iteration`: the irreducible constraint is that following an edge in a graph of `N` nodes takes at minimum one cache-line-sized read if the out-neighbour list of the source node is colocated with the source. A 4KB page-oriented B-tree that stores edges globally defeats that. So we don't mimic Postgres/InnoDB; we design a graph-native layout that colocates neighbourhoods. Any alternative must justify why it beats the theoretical optimum.

This rule is the reason we are willing to build a custom storage engine rather than wrap RocksDB. See ADR-0003.

---

## 12. No shortcuts — unlimited engineering budget

The founder has committed to an unbounded engineering budget in terms of time and complexity. Consequences:

- **If a higher-complexity approach measurably gets us closer to §0, take it.** Do not propose "simpler for now, optimise later" unless the simpler approach also approaches the theoretical optimum.
- **No performance trade-off is accepted "for convenience".** Convenience belongs to the API layer, not the storage/query/cluster layer.
- **No feature is skipped "because it's hard".** Missing features that competitors ship (graph algorithms, full-text search, vector search, temporal queries, GraphQL endpoint, subgraph snapshots, online backups, …) must be on the roadmap — or there is an explicit ADR documenting the non-goal.
- **"Rewrite" is not a dirty word.** If a subsystem reaches a performance ceiling, propose a rewrite via ADR. Do not accumulate sediment.

This rule intentionally inverts normal startup economics. It's a deliberate choice: the moat is engineering depth.

Counterbalance: correctness is still a hard constraint (§1.3). "No shortcut" does not mean "no discipline". Every complex design must come with the test/bench/stress matrix that proves it.

---

## 13. Skills, tools, and dev workflow

Any human or agent joining the project should be productive within one command:

```bash
mise install   # installs pinned Rust toolchain and helper tools
just dev       # starts the local dev loop (fmt-on-save, watch-test, benches on demand)
```

The **canonical set of dev skills** (codified as `justfile` recipes and `xtask` subcommands):

| Recipe | Purpose |
|--------|---------|
| `just fmt` | format |
| `just lint` | clippy with `-D warnings` |
| `just test` | `cargo test --workspace` |
| `just test-prop` | proptest regression suite |
| `just fuzz <target>` | run a `cargo fuzz` target |
| `just bench` | run micro-benches (criterion) |
| `just bench-compare <baseline>` | compare current vs stored baseline |
| `just bench-macro` | run LDBC SNB / SNAP workloads |
| `just stress <scenario>` | run a stress scenario from `tests/stress/` |
| `just ci` | full gate used by CI — must pass locally before PR |
| `just research-prompt <codename>` | emit the agent prompt for a competitor profile |

The `xtask/` crate holds heavier dev tasks (benchmark plotting, competitor-data prep, release orchestration helpers).

Every agent contribution that adds a dev workflow must land a matching `just` recipe. No "undocumented incantation" — if you typed it, codify it.

---

## 14. Versioning & release automation

Versions are bumped by **`release-plz`** (config: `release-plz.toml`, workflow: `.github/workflows/release-plz.yml`) based on conventional commits. Agents:
- write conventional commits;
- never edit `Cargo.toml` versions by hand;
- never tag or publish — the automation does it;
- may respond to a release PR with benchmark/changelog refinements.

---

## 15. Features-first, architecture-second

**Features precede architecture. Always.** Every architectural decision serves a workload; no workload, no decision.

### The causal chain

```
positioning.md   →   ai-agent-workloads.md   →   feature-matrix.md   →   ADRs   →   code
  (why)                  (what users need)          (what we ship)       (how)     (the build)
```

Reading the chain right-to-left: any line of code traces back to an ADR; every ADR cites one or more `FM-NNN` rows in [`docs/requirements/feature-matrix.md`](./docs/requirements/feature-matrix.md); every FM row traces back to a workload in [`docs/requirements/ai-agent-workloads.md`](./docs/requirements/ai-agent-workloads.md) or to the commercial positioning in `initial-vision.md`. If a link is missing, the artifact is premature.

### Rules

1. **No architectural ADR may be Accepted before the features it addresses are locked.** Until M1 feature lock completes, architectural ADRs stay in *Proposed* status (direction only). This is why ADRs 0002, 0003, 0004, 0005 sit in *Proposed*.
2. **Every non-trivial ADR header lists the `FM-NNN` rows it addresses.** If you cannot fill that list, stop and lock the features first.
3. **Every `FM-NNN` row traces back to a workload** in `ai-agent-workloads.md` or to a commercial requirement derived from `initial-vision.md`. Rows without a workload anchor must be justified or deleted.
4. **When a workload surfaces a new sub-requirement, add an `FM-NNN` row first. ADRs follow.** Do not write an ADR that introduces a new capability without first encoding it as a feature row.
5. **Premature architecture is a §10 violation.** Choosing a storage format, index type, or wire protocol before the workload that motivates it has been captured is prohibited.

### Why this rule is load-bearing

On 2026-04-20 the project had five *Accepted* architectural ADRs and an empty AI-native feature section. The author had jumped to technical choices before characterising the agent workloads that justify them. Consequence: some of those choices (e.g. the tiered node layout in ADR-0003) did not yet account for dense vectors, large blobs, or bi-temporal history — workloads that shape storage non-trivially. The fix was to downgrade the ADRs, add positioning + workload documents, and codify this rule. Future agents should read this §15 before opening any ADR-shaped PR.

### What counts as "locked"

A feature row is **locked** when:
- it has an ID (`FM-NNN`), area tag, and tier (Parity / Novel / Stretch);
- it references a workload in `ai-agent-workloads.md` OR the commercial pillar;
- it has a numerical or behavioural acceptance criterion (even if "TBD with research" — the TBD is itself a tracked issue).

M1 (see `ROADMAP.md`) is the milestone where all primary feature rows move from draft to locked; M2 is where the corresponding ADRs move from Proposed to Accepted.

---

## 16. Skills — mechanical rule enforcement

The repo ships a catalog of skills under [`.claude/skills/`](./.claude/skills/). Each skill is a playbook that enforces one or more of the rules above so they are checked mechanically, not recalled from memory. Codex discovers the same canonical skill files through a single directory-level symlink (`.agents/skills -> ../.claude/skills`); a skill added, renamed, or removed under `.claude/skills/` is immediately visible to Codex — no bridge-refresh step.

Invocation syntax differs by agent. Claude Code exposes these as slash commands such as `/next`. Codex skills are not CLI slash commands; invoke them with `$next`, `$plan-feature`, etc., or by asking Codex to use the named skill.

**Tier 0 (autonomy loop).** The entry and exit points for an agent that wants to keep working without a human prompt. Implements the claim protocol (§6.1).

| Skill | Rule encoded | When to call |
|-------|--------------|--------------|
| `next [milestone]` | §§3, 6.1 | Start of every new agent session, or after finishing a PR — claim the next ready issue |
| `abandon <ready\|blocked> <reason>` | §§3, 6.1 | When you cannot finish — unassign and flip label; never walk away silently |

**Tier 1 (rule-enforcers).** Invoke at the right moment and the rule is respected automatically.

| Skill | Rule encoded | When to call |
|-------|--------------|--------------|
| `onboard` | §§0, 7, 11, 12, 15 | First run on a fresh clone (a guided tour of the rules — not a re-hydration command; an agent mid-session that lost context should re-read the relevant AGENTS.md section directly) |
| `plan-feature <desc>` | §§11, 15 | Before any code — locks the FM row, workload anchor, derivation, AC |
| `write-adr <NNNN> <title>` | §§11, 15 | When `plan-feature` concluded an ADR is needed |
| `research-competitor <CODENAME> <real-name>` | §7, ADR-0006 | Any competitor profile — forces codename + private input + public delta |
| `pre-commit-check` | §§1, 4, 5, 7, 8 | Before every commit — local mirror of CI |
| `review-pr <num>` | §§1, 5, 7, 8, 11, 12, 15 | Reviewing any PR |
| `promote-adr <NNNN>` | §15 | Only at M2 exit — gates Proposed → Accepted |

**Tier 2 (workflow automators).** Make the expensive / error-prone operations reproducible.

| Skill | When to call |
|-------|--------------|
| `run-stress <scenario>` | After any storage / MVCC / cluster change. Mandatory evidence for those PRs |
| `run-bench <mode> <baseline>` | Before claiming any perf win. Mandatory for `type:perf` PRs |
| `file-issue <title>` | End of `plan-feature`; any tracked task > 1 h of effort |
| `wait-ci [pr]` | After every `gh pr create` and after every re-push on a red PR. Polls CI, walks the test-plan checklist, posts the verdict. Mandatory gate before flipping an issue to `status:needs-review` |

Read [`.claude/skills/README.md`](./.claude/skills/README.md) for the full catalog, conventions, and the Tier 3 backlog (proptest / loom / fuzz scaffolds, regression diff tool, subagent delegations).

**Rule.** If a skill exists for the action you are about to take, use it. A PR that could have been produced via `plan-feature` + `write-adr` + `pre-commit-check` but skipped them is evidence that the author missed a rule — and is grounds for a review-request change.

---

_This file evolves. Propose changes via PR just like any other file. Last reviewed: 2026-04-20._
