---
name: onboard
description: >
  Mandatory first read for any agent joining physa-db. Surfaces the two-pillar
  positioning, the rules (§§7, 11, 12, 15), the causal chain
  (positioning → workloads → features → ADRs → code), and the reading order.
  Invoke when a new agent session starts, or when re-hydrating context after
  compaction, or when the user asks "where should I start?".
when_to_use: >
  "onboard", "how does this project work", "where do I start", "what are the
  rules", new agent session on physa-db, re-orientation after compaction.
argument-hint: ""
user-invocable: true
---

# physa-db — agent onboarding

You are about to work on **physa-db**, a next-generation open-source graph
database written in pure Rust. Before touching anything, internalise what
follows.

## 1. The two pillars (both load-bearing)

| Pillar | Source of truth | The one-liner |
|--------|-----------------|---------------|
| **Commercial** — why the project exists | [`initial-vision.md`](../../../initial-vision.md) (immutable) | End the Neo4j pricing era. Apache-2.0, full Cypher/GQL compat, native multi-tenancy, SaaS-grade, no enterprise-gated features. |
| **Technical** — why a user would pick us | [`docs/requirements/positioning.md`](../../../docs/requirements/positioning.md) + [`docs/requirements/ai-agent-workloads.md`](../../../docs/requirements/ai-agent-workloads.md) | **AI-agent-native graph database.** First-class support for the six agent workload families (W-A..W-F): agent memory, RAG, knowledge graphs, multi-modal assets, agent observability, temporal reasoning. |

Neither pillar alone is enough. AI-native features win *new* workloads;
Apache-2.0 + GQL/Cypher + Bolt-compat wins *migrations from* the incumbent.

## 2. The causal chain (inviolable — `AGENTS.md` §15)

```
positioning.md  →  ai-agent-workloads.md  →  feature-matrix.md  →  ADRs  →  code
    (why)              (what users need)        (what we ship)       (how)    (the build)
```

Read left-to-right to orient. Read right-to-left to verify: every line of code
traces back through an ADR → an `FM-NNN` row → a workload → a pillar. If a link
is missing, the artifact is premature and should not ship.

## 3. The three engineering rules (inviolable)

- **§15 Features before architecture.** No architectural ADR is *Accepted*
  before the `FM-NNN` rows it addresses are locked. ADRs 0002-0005 currently
  sit in *Proposed* pending M1 feature lock.
- **§11 First-principles thinking.** Derive designs from irreducible
  physical/informational constraints (NVMe latency, cache lines, CAP, entropy
  of IDs) and the theoretical optimum. Every non-trivial ADR has a
  "first-principles derivation" section.
- **§12 No shortcuts, unlimited engineering budget.** If a higher-complexity
  approach measurably gets us closer to the prime directive, take it. Never
  "simpler for now, optimise later" on core subsystems.

## 4. The research privacy rule (§7, ADR-0006)

- `private/` is **gitignored end-to-end**. Holds raw competitor profiles and
  pain-point mining. NEVER commit.
- Competitors are referenced by **codename** (ALPHA, BRAVO, …) inside
  `private/`. The codename ↔ real-name map lives in
  `private/research/codenames.md`, local only.
- Public output (`docs/requirements/`) is **attribution-free**: no competitor
  names, no quoted complaints.
- Any PR that references a competitor by real name in a public file is a §10
  violation.

## 5. Reading order for a new agent

1. [`AGENTS.md`](../../../AGENTS.md) — full agent contract (read §§0, 7, 10, 11, 12, 15 at minimum).
2. [`initial-vision.md`](../../../initial-vision.md) — immutable founder vision.
3. [`docs/requirements/positioning.md`](../../../docs/requirements/positioning.md) — the technical pillar.
4. [`docs/requirements/ai-agent-workloads.md`](../../../docs/requirements/ai-agent-workloads.md) — the six workload families.
5. [`docs/requirements/feature-matrix.md`](../../../docs/requirements/feature-matrix.md) — what ships (parity + AI-native rows).
6. [`docs/requirements/non-goals.md`](../../../docs/requirements/non-goals.md) — what we refuse.
7. [`ROADMAP.md`](../../../ROADMAP.md) — M0..M8 milestones. M1 = feature lock, M2 = architecture lock.
8. [`docs/architecture/adr/`](../../../docs/architecture/adr/) — ADR-0001 + ADR-0006 *Accepted*; ADRs 0002-0005 *Proposed*.

## 6. The other skills you should know

| Skill | When to use |
|-------|-------------|
| `plan-feature` | Before writing any feature — forces §15 compliance |
| `write-adr` | Before writing any non-trivial ADR — enforces §11 derivation + FM refs |
| `research-competitor` | Before adding a competitor profile — enforces §7 privacy |
| `file-issue` | Before opening a GitHub Issue — enforces §6 structure |
| `run-stress` | Before claiming a concurrency / storage change is done |
| `run-bench` | Before claiming any performance win |
| `pre-commit-check` | Before every commit |
| `review-pr` | When reviewing a peer agent's PR |
| `promote-adr` | Only at M2 exit, to move Proposed ADRs to Accepted |

## 7. What to do next

State in one short sentence what you intend to do, then invoke the most
relevant skill above. If your task does not match any skill, say so and
proceed according to `AGENTS.md` — stopping to file an issue if that's the
right move under §3.

**Do not start writing code or ADRs until you have read at least files 1–3
above.** Cite specific file paths (e.g. `AGENTS.md §15`) when explaining your
reasoning — it proves you did the reading.
