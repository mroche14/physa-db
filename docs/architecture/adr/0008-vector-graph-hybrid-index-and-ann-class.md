# ADR-0008: Shared-ID vector-graph index fabric with segment-local ANN

- **Status:** Accepted
- **Date:** 2026-04-25
- **Features addressed:** FM-015, FM-100, FM-102, FM-103, FM-104, FM-105, FM-106, FM-107, FM-108, FM-117, FM-118, FM-119, FM-120, FM-126, FM-129
- **Workloads addressed:** W-A, W-B, W-D, W-F
- **Context issue:** _(to be filed as `type:feature area:ai-native`)_

## Context

Hybrid retrieval is now a locked workload, not an optional extension. physa-db therefore needs one index fabric that can support vector, text, scalar, and temporal pruning over shared identities, instead of isolating vector search behind a separate architecture silo. Campaign M1-Lock also resolved the ANN placement default: graph-home locality first, mirrors on evidence, and independent placement only as an exception. This ADR captures that cross-cutting index decision.

## Decision

ANN means approximate nearest neighbor search: return near vectors quickly without exact full-collection scan. HNSW means Hierarchical Navigable Small World, a graph-based ANN index that gives strong recall-latency tradeoffs on mutable dense-vector collections.

1. physa-db adopts a shared-ID index fabric. Vector, full-text, scalar, and temporal indexes must resolve to the same tenant-scoped object IDs.
2. Dense-vector ANN uses segment-local HNSW as the baseline. IVF-PQ is the later memory-efficiency extension, not the M1 default.
3. ANN roots are tenant-scoped and model-version-scoped. One tenant's vectors never share an ANN root with another tenant.
4. The common placement rule is co-location with the graph home shard so that ANN shortlist and first graph expansion remain local. Read-only mirrors may serve hot subsets when evidence justifies them.
5. Temporal predicates push down into hybrid retrieval. ANN, BM25, and graph expansion may not fetch candidates first and filter time later.
6. Retrieval, graph expansion, rerank preparation, streaming, and evidence emission execute as one optimizer-owned plan instead of a vector sidecar followed by application glue.

## First-principles derivation

### 1. Irreducible constraints

1. Exact dense retrieval over `N` vectors is `O(N * d)`, which is the wrong lower bound once collections are large.
2. A `1536`-dimension `F16` vector is about `3 KiB`; contiguous segment-local storage materially improves cache and prefetch behavior.
3. W-A and W-B both do ANN first and graph expansion immediately after. A forced cross-partition handoff in that path adds a network RTT and extra failure surface to the common query shape.
4. Model-versioned embeddings and temporal reads mean the same logical object can have multiple eligible vectors over time. Tenant and version scoping are therefore architectural, not optional metadata.
5. Full-text, vector, scalar, and graph filters must fuse before result shaping to preserve token budget and evidence quality.

### 2. Theoretical optimum

The optimum is:

- sublinear ANN candidate generation;
- early fusion with shared IDs and shared plan context;
- local graph expansion on the common path;
- temporal pruning before payload fetch;
- streaming of the first evidence batch before the full result set is materialized.

Any architecture that externalizes vector search by default pays a remote hop on the common path and loses deterministic plan ownership over pruning, shaping, and evidence.

### 3. Smallest structure that realizes the optimum

The smallest structure is:

- one shared-ID index fabric;
- one segment-local HNSW family for dense vectors;
- one BM25 family for text;
- one scalar family for ordinary filters;
- one temporal side-index family for `AS OF` and `BETWEEN`;
- one planner that can fuse all of them into one physical plan.

That is the minimum structure that keeps hybrid retrieval local, tenant-safe, and temporally correct.

### 4. Prior art reused patternwise

physa-db reuses HNSW as the dense-vector baseline, BM25 plus rank-fusion style combination for lexical and semantic retrieval, and the standard pattern of shared object IDs across index families. The reuse is deliberate and narrow: the engine borrows mature index shapes, but keeps the fusion, temporal pushdown, and placement policy under one graph-native optimizer.

## Consequences

**Positive**
- Hybrid retrieval becomes a first-class database capability instead of client orchestration.
- ANN-to-graph locality stays on the fast path.
- Tenant and model-version isolation remain load-bearing in every vector root.
- Evidence artifacts can explain fused ranking and pruning because one plan owns the full pipeline.

**Negative**
- Segment-local ANN maintenance and mirror freshness tracking add background work.
- The optimizer must reason about more index families in one cost model.
- IVF-PQ and other later families still need separate benchmark evidence before promotion.

## Open items

- Mirror activation thresholds, segment rebuild cadence, and the exceptional path for fully independent ANN placement remain sentinel-gated constants for the Phase 6c benchmark-tracking issue once filed.
- IVF-PQ promotion stays benchmark-driven after the HNSW baseline lands; this ADR does not pre-accept its operating thresholds.

## FM coverage

- FM-015, FM-100, FM-102, FM-103, FM-104, FM-105: vector types, ANN, and similarity operators
- FM-106, FM-107, FM-108: one-plan hybrid retrieval, lexical fusion, and multi-hop schema traversal
- FM-117, FM-118: embedding ingest hooks and model-version navigation
- FM-119, FM-120: streaming and token-budget shaping
- FM-126, FM-129: tenant-safe vector roots and structural-signature plan caching

## References

- Malkov and Yashunin, "Efficient and Robust Approximate Nearest Neighbor Search Using Hierarchical Navigable Small World Graphs", TPAMI 2018.
- Cormack, Clarke, and Buettcher, "Reciprocal Rank Fusion Outperforms Condorcet and Individual Rank Learning Methods", SIGIR 2009.

## Changelog

- 2026-04-25: Accepted as part of the Campaign M1-Lock ADR expansion.
