# Requirements

The **public**, attribution-free synthesis of what physa-db will ship.

## Reading order

Requirements form a strict causal chain (`AGENTS.md` §15):

```
positioning.md  →  ai-agent-workloads.md  →  feature-matrix.md  →  performance-targets.md
    (why)              (what users need)         (what we ship)           (how fast)
                                                  non-goals.md
                                                  (what we refuse)
```

Start at the top; every downstream file cites the upstream one.

## Files

- [`positioning.md`](./positioning.md) — the two load-bearing pillars: **commercial** (§1, end the incumbent's pricing era) and **technical** (§§2–7, AI-agent-native graph database).
- [`ai-agent-workloads.md`](./ai-agent-workloads.md) — **authoritative source of the six AI-agent workload families** (W-A agent memory, W-B RAG, W-C knowledge graphs, W-D multi-modal assets, W-E agent observability, W-F temporal reasoning) and the derived data types / operators / indices / execution / storage / protocol / security requirements. Every AI-native feature traces back here.
- [`feature-matrix.md`](./feature-matrix.md) — every capability physa-db will ship, with parity/novel/stretch tier, target milestone, ADR link, and — for AI-native rows — workload anchor.
- [`performance-targets.md`](./performance-targets.md) — numerical targets: latency percentiles, throughput, scale, cold-start, recovery, plus AI-native targets (ANN recall vs latency, hybrid-query p95, ingest rate). Every target is a testable assertion the benchmark suite verifies.
- [`non-goals.md`](./non-goals.md) — explicitly out-of-scope items. Non-goals are load-bearing: they define the shape of the product by what it refuses.

## Editorial rules

- **No competitor names.** "A popular Java-based graph DB" is fine. Named attributions are not.
- **No quoted user complaints.** Summarise without attribution.
- **Numbers where possible.** A requirement that says "fast" is useless. "p95 < 10ms on LDBC SNB IC-2, SF10, 16-core x86_64" is testable.
- **Every row links to an ADR or an issue.** A requirement without a traceable owner is a wish.
- **Every AI-native row cites a workload** in `ai-agent-workloads.md`. A row without a workload anchor is premature (see `AGENTS.md` §15).
