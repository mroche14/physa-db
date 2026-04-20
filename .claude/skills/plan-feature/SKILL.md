---
name: plan-feature
description: >
  Structured playbook to plan a feature on physa-db before any code is written.
  Enforces AGENTS.md §15 (features before architecture) by requiring the FM row,
  workload anchor, first-principles derivation, and acceptance criteria to be
  locked BEFORE implementation starts. Use this any time a user asks "implement
  X", "add Y", "let's build Z".
when_to_use: >
  "implement", "add feature", "build X", "start working on Y", whenever a new
  feature is being scoped.
argument-hint: "[short feature description]"
user-invocable: true
---

# plan-feature — features-first planning playbook

**Prime directive (`AGENTS.md` §15):** no line of code before the feature is
locked. No ADR before the feature row exists. Planning IS the work at this
stage.

The user's feature description: **$ARGUMENTS**

Work through the steps below, in order. Do not skip.

## Step 1 — Locate or draft the FM row

Search [`docs/requirements/feature-matrix.md`](../../../docs/requirements/feature-matrix.md)
for an existing row that covers this feature.

- **If an `FM-NNN` row exists:** cite its ID, area tag, tier, and any workload
  anchor (W-A..W-F) already present. You are *augmenting*, not inventing.
- **If no row exists:** STOP. You cannot plan implementation before the row
  exists. Either:
  1. Propose a new row (ID = next available in the right range — `FM-041+`
     for parity, `FM-131+` for AI-native) with area, kind (Parity / Novel /
     Stretch), target milestone, and workload anchor. Post this proposal for
     human approval first.
  2. Or argue that the feature is covered by an existing row the user did not
     reference.

Never write code tied to a phantom FM row.

## Step 2 — Anchor to a workload or commercial requirement

If the feature is AI-native (FM-100+), cite at least one workload family from
[`docs/requirements/ai-agent-workloads.md`](../../../docs/requirements/ai-agent-workloads.md):

- **W-A** Agent memory — episodic + semantic facts, TTL, reinforcement.
- **W-B** RAG — hybrid ANN → graph expansion → rerank.
- **W-C** Knowledge graphs — entity resolution, provenance, confidence.
- **W-D** Multi-modal assets — blob + chunk hierarchy + embeddings.
- **W-E** Agent observability — high-throughput trace ingest.
- **W-F** Temporal reasoning — bi-temporal, `AS OF` queries.

Quote the relevant paragraph. If no workload fits, the feature may be a
non-goal ([`docs/requirements/non-goals.md`](../../../docs/requirements/non-goals.md)
check).

If the feature is commercial-pillar (FM-001..099), cite the relevant promise
in [`initial-vision.md`](../../../initial-vision.md) or an incumbent-compat
need.

## Step 3 — First-principles derivation (`AGENTS.md` §11)

In 5–15 lines, answer:

1. **What are the irreducible physical / informational constraints?** (NVMe
   read latency, cache-line size, entropy of the ID, CAP, consensus
   round-trips, network RTT, vector dimension × f32 byte cost, etc.)
2. **What is the theoretical optimum under those constraints?** (Minimum
   bytes moved, minimum round-trips, minimum synchronisation.)
3. **What is the smallest structure that realises that optimum?**
4. **What prior art has already solved a sub-problem correctly?** (Cite
   papers/blogs. We steal pieces, not whole systems.)

If you find yourself writing "because Neo4j does it" or "because Postgres
uses X", STOP. That's analogy thinking (`AGENTS.md` §11). Rewrite from the
constraint.

## Step 4 — Acceptance criteria

List testable criteria as bullets. Every bullet must be mechanically
verifiable: a benchmark delta, a property-test assertion, a stress scenario
outcome, a conformance row passed, a latency p95 under a threshold.

Include:

- **Correctness gates:** which property tests / fuzz targets / stress
  scenarios the feature must pass before merge.
- **Performance gates:** if the feature is on a perf-sensitive path, state
  the numerical target (p50/p95/p99 / throughput / memory), with the
  hardware and dataset.
- **Non-regression gates:** which existing benchmarks must not regress by
  >2% (`AGENTS.md` §5).

## Step 5 — Decide: do we need an ADR?

An ADR is required when the feature:

- introduces a new on-disk format, wire protocol, or data type;
- changes the concurrency model, consistency guarantees, or durability
  semantics;
- adds a new subsystem or refactors an existing one;
- commits to an external dependency that affects licence/performance/ceiling;
- is a Novel or Stretch feature (by definition those need first-principles).

If yes, invoke the `write-adr` skill instead of continuing here.

If no (small parity feature, well-scoped), proceed to the issue.

## Step 6 — File the issue (invoke `file-issue`)

Use the `file-issue` skill to turn this plan into a GitHub Issue with the
right labels, acceptance criteria, and links. The issue becomes the
system-of-record (`AGENTS.md` §6).

## Step 7 — Only now, start coding

Create the branch `agent/<issue-number>-<short-slug>` and begin work.
Reference the `FM-NNN` row and the issue in every commit message.

## Output format

When you finish this skill, produce a short report with the following
sections, then await human confirmation before moving to step 6+:

```
## Feature plan for: <user's description>

### FM row
FM-NNN (existing | NEW: proposed)

### Pillar / workload anchor
Commercial | W-X (+ brief citation)

### First-principles derivation
[5–15 lines]

### Acceptance criteria
- [ ] ...
- [ ] ...

### ADR required?
yes (invoke `write-adr`) | no (proceed to `file-issue`)
```
