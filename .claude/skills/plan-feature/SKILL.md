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
in [`docs/requirements/positioning.md`](../../../docs/requirements/positioning.md)
§1 or an incumbent-compat need.

## Step 3 — First-principles derivation + optimization hunt (`AGENTS.md` §11)

### 3a. Constraints and theoretical optimum

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

### 3b. Hunt for the maximal-optimization path

Before locking the implementation, actively scan for a better approach
than the first one that came to mind. At project start the cost of
picking a wrong primitive compounds faster than any other decision: an
hour of research now is worth weeks of refactor later. This is not
optional for features on hot paths or with durability/consistency
implications.

**Recency window (non-negotiable).** Sources must be ≤ 12 months old
unless you can justify an older one inline (e.g. a seminal paper whose
successors have not improved on it). Use date-filtered searches (arXiv
`submittedDate` ≥ today − 12 months; Google `tbs=qdr:y`; GitHub
`pushed:>YYYY-MM-DD`). For every library/crate, record the **latest
stable version on `crates.io` at the date of this plan** — not the
version pinned in our `Cargo.toml` six months ago. Stale inputs
silently cap the design.

**Exhaustive documentation (non-negotiable).** The research is only
valuable if it is reproducible. The brief below records, without
abbreviation:

- every URL visited, including dead-ends and "not applicable" reads —
  closing the search perimeter is how the next agent avoids re-doing
  your work;
- every search query + the engine/tool it ran on + the date filter
  applied;
- for every candidate library / crate / API, the exact version string
  (`crate A.B.C` as of `crates.io` YYYY-MM-DD, Rust edition, compiler
  channel, nightly tracking-issue URL) at the consult date;
- for the SOTA method of the problem class, the method name +
  publication year + citation URL, and physa-db's disposition
  (**adopt / adapt / reject**) with a one-line reason.

Silent sweeps are not sweeps. If you cannot list what you read, the
work is not done — reviewers will reject a brief that hides its search.

Scan, in this order:

- **Papers & SOTA benchmarks.** Web-search the problem class (e.g.
  "lock-free hash map rust 2026", "MVCC GC amortisation", "HNSW vs
  DiskANN recall/QPS"). Prefer recent arXiv preprints, conference
  talks (VLDB, SIGMOD, NSDI, ATC, OSDI, EuroSys), and engineering
  blogs with real numbers. Name the SOTA method explicitly — not "a
  hash map", but "Abseil SwissTable (2018) + follow-ups through 2025".
- **Rust ecosystem sweep.** For every candidate primitive, check
  `docs.rs` and `lib.rs` (the index-of-crates sites): existing
  production-grade crates, their benchmark claims, **current latest
  version on `crates.io`**, last-release date, licence (Apache-2.0 /
  MIT compatible — no GPL-family), maintenance status. A neglected
  crate with theoretical wins is a trap; a well-maintained crate with
  a 95 %-perfect fit saves months.
- **Rust stdlib & nightly.** Check whether the feature is better
  handled by a soon-stabilising API (`core::simd` portable SIMD,
  `allocator_api`, `strict_provenance`, `BufRead::read_until`). Cite
  the tracking issue; choose based on the stabilisation ETA versus
  our milestone.
- **Hardware floor.** If this is a hot path, compute the theoretical
  lower bound on the target hardware (NVMe seq read ~7 GB/s, DRAM
  random ~100 ns, L1 ~4 cycles, AVX-512 width, etc.). A design that
  leaves 10× on the table is not an optimisation — it's a ceiling.

Produce a **research brief** (include it verbatim in the issue body):

```
### Optimization research brief

**Research window:** YYYY-MM-DD – YYYY-MM-DD (≤ 12 months unless justified).

**Sources consulted (exhaustive — every URL visited, including dead-ends and "not applicable" reads):**
  - <URL> — <title> — <date read> — <finding / "not applicable">
  - <URL> — ...

**Search queries used:**
  - "<query>" on <engine/site> (date filter: last 12 months)
  - ...

**Candidates evaluated:**
  1. <crate/lib name> <version X.Y.Z> (crates.io, YYYY-MM-DD) — <claim / cite> — picked / rejected: <reason>
  2. ...

**SOTA method for this problem class:** <method name>, <year>, <citation URL>. physa-db disposition: **adopt / adapt / reject** — <reason>.

**Picked:** <N> because <reason grounded in §3a optimum>.

**Rejected:** <others with 1-line reason each>.

**Dead-ends explored:** <paths that looked promising but weren't> — why.

**Uncovered surprise:** <finding the naïve plan missed, or "none">.

**Hardware floor estimate:** <lower bound> — our target: <value> (gap: <ratio>).
```

The brief is the receipts: future contributors must be able to see
why the non-obvious choice won **and retrace the full search**. If
this step produces nothing surprising, say so explicitly — "obvious
pick confirmed after exhaustive sweep" is a valid outcome; skipping
the sweep is not, and an abbreviated sweep is not a sweep.

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

### Optimization research brief
[candidates evaluated, pick, rejections with reasons, uncovered
surprise or "none", hardware floor vs target]

### Acceptance criteria
- [ ] ...
- [ ] ...

### ADR required?
yes (invoke `write-adr`) | no (proceed to `file-issue`)
```
