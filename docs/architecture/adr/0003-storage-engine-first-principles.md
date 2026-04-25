# ADR-0003: Segment-class storage substrate under one transactional fabric

- **Status:** Accepted
- **Date:** 2026-04-25
- **Features addressed:** FM-008, FM-009, FM-015, FM-017, FM-041, FM-042, FM-100, FM-102, FM-110, FM-114, FM-115, FM-118, FM-123, FM-124, FM-126
- **Workloads addressed:** W-A, W-B, W-C, W-D, W-E, W-F
- **Context issue:** _(to be filed as `type:feature area:storage priority:p0`)_

## Context

physa-db cannot meet the locked workload set with one universal physical layout. Agent memory, hybrid retrieval, knowledge graphs, blobs, observability, and bi-temporal history each force different hot paths, different write shapes, and different retention behavior. Campaign M1-Lock therefore replaced the earlier "two-tier graph layout" draft with an explicit segment-class substrate. The storage engine remains custom and graph-native, but its physical contract is now broader and more precise.

## Decision

physa-db adopts a segment-class storage substrate. A write-ahead log (WAL) is the durable journal that records every change before the storage engine applies it. A blob is a value too large to inline next to its key - typically at least several KiB - kept in a separate value store so the hot index remains small and cache-resident.

1. **Topology segments.** Store vertex headers, inline mini-adjacency for low-degree neighborhoods, and external adjacency chunks for high-degree neighborhoods. The topology WAL records edge and vertex mutations as ordered logical records before chunk or header mutation becomes visible.
2. **Value/blob segments.** Store property columns, manifest metadata, and three blob tiers: `<= 16 KiB` inline, `> 16 KiB && <= 1 MiB` in local append-only blob logs, and `> 1 MiB` by external object reference. The architecture permits per-tenant threshold overrides, but shipped defaults are the values above and any override remains sentinel-gated until benchmark evidence justifies exposure. The value/blob WAL is metadata-first: manifest and dedupe intent record durably before blob-log append or external object commit flips the steady-state pointer.
3. **Embedding/ANN segments.** Store contiguous vector payloads and segment-local ANN overlays keyed by tenant and model version. Their WAL appends vector-version mutations and ANN delta overlays; full ANN rebuilds are background maintenance, never commit-path work.
4. **Temporal delta segments.** Store version fragments plus per-segment interval lists and partition zone maps for `tx_time` and `valid_time`. Their WAL appends the version fragment and the temporal side-index delta atomically so temporal pushdown is crash-safe.
5. **Event-append segments.** Store append-only, time-partitioned observability runs and causal edge references. Their WAL is sequential and ingest-optimized: event batches commit by durable append, while secondary summaries and compaction metadata lag behind but remain replayable.
6. Segment classes compact, scan, and cold-move independently. No universal page format is allowed to force all workload families through the same locality tradeoff.

## First-principles derivation

### 1. Irreducible constraints

1. A cold 4 KiB NVMe read is roughly two orders of magnitude slower than an L3 hit and roughly five orders of magnitude slower than an L1 hit. Hot graph topology and cold blob payloads therefore cannot share the same locality target.
2. A 64-byte cache line can hold a compact vertex header plus a small adjacency sketch, but it cannot also hold a `1 MiB` asset body or a long temporal history.
3. A `1536`-dimension `F16` embedding is about `3 KiB`; a top-K ANN probe wants contiguous vectors, not graph-pointer indirection.
4. W-E targets `100k events/s` per tenant. At `1 KiB` of payload per event before indexes, that is about `100 MiB/s` of raw append traffic; random in-place mutation is the wrong physical shape.
5. W-F requires time pruning before ANN, text, or graph expansion. Version chains alone are not a sufficient access path.

### 2. Theoretical optimum

The theoretical optimum is not one universal page layout. It is one transactional fabric with multiple physical classes, each approaching the lower bound of its access path:

- topology: one local read for the common low-degree neighborhood and bounded chunk reads for supernodes;
- blobs: keep only the range where local microsecond access changes latency, and externalize payloads once bytes dominate over seeks;
- ANN: contiguous vector scans and bounded graph-walk overlays per segment;
- temporal: coarse partition prune first, then segment-local interval prune before payload fetch;
- observability: sequential append on ingest and partition-level cold movement later.

Any single-class design pays the wrong lower bound for at least one of these workloads.

### 3. Smallest structure that realizes the optimum

The smallest structure is exactly the accepted class set above: topology, value/blob, embedding/ANN, temporal delta, and event-append segments under one transaction, isolation, and scheduling regime. That structure is small enough because each class is also the unit of compaction, repair, and cold movement. The earlier draft's "graph pages plus external adjacency" idea survives only as the topology class, not as a universal answer.

### 4. Prior art reused patternwise

physa-db reuses four patterns without inheriting any one engine wholesale:

- adjacency-local graph storage for traversal-heavy workloads;
- large-value separation for keeping hot metadata and cold payloads apart;
- graph-based ANN indexes for mutable dense-vector search;
- append-friendly lineage and interval pruning for temporal history.

The prior art is useful as a set of local techniques. The project still needs a custom composition because no single existing pattern covers graph locality, large-value tiers, ANN locality, temporal pushdown, and append-only observability in one substrate.

## Consequences

**Positive**
- Each workload family gets a physical class shaped to its dominant cost instead of inheriting a compromise.
- Blob tiers, temporal side indexes, and ANN segments become first-class durability objects rather than ad hoc side stores.
- WAL replay, compaction, and cold movement can operate per class, which improves fault isolation.
- Tenant isolation applies to every major storage root, not only namespaces.

**Negative**
- The storage engine now has more control-plane surfaces: class-aware compaction, replay, checksums, and quota enforcement.
- Cross-class transaction testing becomes more demanding because one logical write may touch topology, value, ANN, and temporal segments together.
- Thresholds and maintenance policies still need benchmark validation even though the architecture is now fixed.

## Open items

- Blob-tier thresholds are accepted with shipped defaults but remain sentinel-gated for exposure. Phase 6c benchmarking will validate write amplification, replay time, and local-vs-external fetch crossover before tenant-visible knobs are broadened.
- Temporal side-index constants such as list fragmentation limits and compaction rewrite budgets are accepted directionally and remain subject to the Phase 6c benchmark-tracking issue once filed.
- Event partition sizing, ANN rebuild cadence, and cross-class compaction fairness remain benchmark-tracked operational constants, not open architecture questions.

## FM coverage

- FM-008, FM-009: graph-native adjacency and columnar property storage
- FM-015, FM-100, FM-102: vector-aware storage roots and ANN locality
- FM-017, FM-110, FM-118: bi-temporal history and embedding-version navigation
- FM-114, FM-115: blob tiers and content-addressed dedup
- FM-123, FM-124: append-first event partitions with cold-tier support
- FM-041, FM-042, FM-126: class-aware memory, spill, and tenant-local secondary isolation

## References

- Lu et al., "WiscKey: Separating Keys from Values in SSD-conscious Storage", FAST 2016.
- Malkov and Yashunin, "Efficient and Robust Approximate Nearest Neighbor Search Using Hierarchical Navigable Small World Graphs", TPAMI 2018.
- Sadoghi et al., "L-Store: A Real-time OLTP and OLAP System", EDBT 2018.
- columnar graph execution paper, CIDR 2023.

## Changelog

- 2026-04-25: Accepted with revisions per Campaign M1-Lock synthesis (formerly Proposed pending feature lock).
