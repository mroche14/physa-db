# Feature matrix

> The running list of capabilities physa-db will ship. Synthesis of research; no competitor attribution.

**Legend.**
- **Parity** — feature exists in mature competitors; we must have it to be considered.
- **Novel** — we believe we can do materially better than any existing solution.
- **Stretch** — ambitious; shipped only if first-principles analysis supports it.

Each row links to the governing ADR (if any), the tracking issue (once filed), and — for AI-native rows — the **workload family** from [`ai-agent-workloads.md`](./ai-agent-workloads.md) (W-A … W-F) that motivates it. Per `AGENTS.md` §15, a row without a workload or commercial anchor is premature.

**ID ranges.**
- `FM-001 … FM-099` — graph-DB parity and platform baseline (commercial pillar).
- `FM-100 … FM-199` — AI-agent-native features (technical pillar, anchored in `ai-agent-workloads.md`).

| ID | Area | Feature | Kind | Tier | ADR | Issue |
|----|------|---------|------|------|-----|-------|
| FM-001 | query | GQL (ISO/IEC 39075:2024) support | Parity | M3 | ADR-0002 | — |
| FM-002 | query | openCypher compatibility (AFM-020) | Parity | M3 | ADR-0002 | — |
| FM-003 | server | Bolt v5 wire protocol | Parity | M4 | — | — |
| FM-004 | server | HTTP/JSON query endpoint | Parity | M4 | — | — |
| FM-005 | storage | ACID transactions with MVCC snapshot isolation | Parity | M3 | ADR-0004 | — |
| FM-006 | storage | Serializable isolation option | Parity | M5 | ADR-0004 | — |
| FM-007 | storage | Write-ahead log with crash recovery | Parity | M3 | — | — |
| FM-008 | storage | Graph-native columnar adjacency store | Novel | M3 | ADR-0003 | — |
| FM-009 | storage | Columnar property store with dictionary encoding | Novel | M6 | ADR-0003 | — |
| FM-010 | storage | Online backup + point-in-time recovery | Parity | M5 | — | — |
| FM-011 | storage | Encryption at rest (per-tenant keys) | Parity | M5 | — | — |
| FM-012 | storage | TLS in transit | Parity | M4 | — | — |
| FM-013 | query | B-tree + hash indices | Parity | M3 | — | — |
| FM-014 | query | Full-text search on properties | Parity | M6 | — | — |
| FM-015 | query | Native vector search integrated with graph traversal | Novel | M3 | — | — |
| FM-016 | query | Built-in graph algorithms (BFS, DFS, PageRank, SSSP, WCC, Louvain) (AFM-028) | Parity | M6 | — | — |
| FM-017 | query | Temporal / as-of queries | Novel | M6 | — | — |
| FM-018 | query | Subgraph / snapshot materialisation (AFM-057) | Novel | M8 | — | — |
| FM-019 | query | Compiled query plans (JIT via `cranelift` or AoT codegen) | Novel | M6 | — | — |
| FM-020 | query | Vectorised execution for analytical workloads | Novel | M6 | — | — |
| FM-021 | cluster | Raft-based metadata consensus (AFM-011) | Parity | M5 | ADR-0005 | — |
| FM-022 | cluster | Transparent horizontal scaling | Parity | M5 | ADR-0005 | — |
| FM-023 | cluster | Online re-sharding with zero downtime | Novel | M5 | ADR-0005 | — |
| FM-024 | cluster | Multi-region async replication | Parity | M6 | — | — |
| FM-025 | cluster | Cross-region strong-consistency option | Stretch | M7 | — | — |
| FM-026 | tenancy | Native multi-tenancy with namespaces | Parity | M5 | — | — |
| FM-027 | tenancy | Per-tenant resource quotas | Parity | M5 | — | — |
| FM-028 | tenancy | Per-tenant RBAC (AFM-017) | Parity | M5 | — | — |
| FM-029 | tenancy | Per-tenant backup schedules | Parity | M5 | — | — |
| FM-030 | ops | Slow-query log with plan explanation | Parity | M4 | — | — |
| FM-031 | ops | Structured, queryable audit log | Parity | M5 | — | — |
| FM-032 | ops | OpenTelemetry export (metrics + traces + logs) | Parity | M4 | — | — |
| FM-033 | ops | Live query cancellation | Parity | M4 | — | — |
| FM-034 | ops | Schema migrations with versioning | Parity | M5 | — | — |
| FM-035 | ecosystem | Native drivers: Rust, Python, Node, Go, Java | Parity | M7 | — | — |
| FM-036 | ecosystem | Migration tool from dominant competitor dump format | Parity | M7 | — | — |
| FM-037 | ecosystem | Kubernetes operator | Parity | M7 | — | — |
| FM-038 | ecosystem | Terraform provider | Parity | M7 | — | — |
| FM-039 | dx | Embedded mode (library usage, no server) (AFM-003) | Parity | M3 | — | — |
| FM-040 | dx | Reproducible benchmark suite public to users | Novel | M6 | — | — |
| FM-041 | ops | Zero-tuning auto-configured memory management | Novel | M4 | — | — |
| FM-042 | ops | Graceful disk spill for memory pressure (no hard aborts) | Novel | M4 | — | — |
| FM-043 | tenancy | External auth providers: SSO / LDAP / OIDC / SAML (AFM-019) | Parity | M5 | — | — |
| FM-044 | ops | Fully managed cloud service (AFM-032) | Parity | M7 | — | — |

## AI-agent-native rows (FM-100…)

Every row below cites the workload family (from [`ai-agent-workloads.md`](./ai-agent-workloads.md)) that motivates it.

| ID | Area | Feature | Kind | Tier | ADR | Issue | Workload |
|----|------|---------|------|------|-----|-------|----------|
| FM-100 | ai-native | `VECTOR<F16/F32, DIM>` first-class property type | Novel | M3 | — | — | W-A, W-B, W-D |
| FM-101 | ai-native | `SPARSE_VECTOR` first-class property type (SPLADE-style) | Novel | M6 | — | — | W-B |
| FM-102 | ai-native | HNSW index for dense vectors (AFM-024) | Parity | M3 | — | — | W-A, W-B, W-D |
| FM-103 | ai-native | IVF-PQ index for memory-efficient ANN at scale | Novel | M6 | — | — | W-B, W-D |
| FM-104 | ai-native | Similarity operators: `COSINE`, `DOT`, `L2`, `HAMMING`, `JACCARD` | Parity | M3 | — | — | W-A, W-B |
| FM-105 | ai-native | Retrieval operators: `NEAREST(v, K)`, `WITHIN_DISTANCE(v, r)` | Parity | M3 | — | — | W-B |
| FM-106 | ai-native | Hybrid query plans (ANN → graph expansion → rerank) in one plan | Novel | M3 | — | — | W-B |
| FM-107 | ai-native | Full-text (BM25) index with shared-ID `RRF` / `HYBRID` scoring inside one optimizer-owned plan | Parity | M4 | ADR-0008 | — | W-B |
| FM-108 | ai-native | Multi-hop retrieval over heterogeneous schema (planner picks direction) | Novel | M4 | — | — | W-B, W-C |
| FM-109 | ai-native | Built-in graph algorithms: PageRank, SSSP, BFS/DFS with early termination, Louvain, Leiden (AFM-028) | Parity | M6 | — | — | W-C |
| FM-110 | ai-native | Bi-temporal model (valid-time + transaction-time) with `AS OF` / `BETWEEN` pushdown across ANN, BM25, and graph access paths | Novel | M6 | ADR-0004 | — | W-F |
| FM-111 | ai-native | Per-fact TTL / forgetting curves + reinforcement primitive | Novel | M3 | — | — | W-A |
| FM-112 | ai-native | Provenance (source, timestamp, extraction confidence) per node/edge | Novel | M3 | — | — | W-C |
| FM-113 | ai-native | Confidence scores with composition under traversal (`CONFIDENCE` type) | Novel | M6 | — | — | W-C |
| FM-114 | ai-native | Blob storage via content-addressed manifest with size-tiered placement: inline, local `blob-log`, and external object reference | Novel | M4 | ADR-0003 | — | W-D |
| FM-115 | ai-native | Content-addressable dedup for blobs and chunks | Novel | M4 | — | — | W-D |
| FM-116 | ai-native | Chunk hierarchy: `Asset -HAS_CHUNK-> Chunk` with chunk types (page, frame, segment, span) | Novel | M4 | — | — | W-D |
| FM-117 | ai-native | Ingest stage-hook contract for embeddings, extraction, chunk revisions, and provider provenance; physa supplies orchestration, not bundled models | Novel | M3 | ADR-0008 | — | W-A, W-B, W-D |
| FM-118 | ai-native | Embedding model version registry (navigate chunk embeddings by model version) | Novel | M6 | — | — | W-D, W-F |
| FM-119 | ai-native | Streaming results so the LLM can start generating before retrieval completes | Novel | M4 | — | — | W-B |
| FM-120 | ai-native | Token-budget tooling boundary: token counting, chunk-by-token-limit APIs, and output shaping | Novel | M4 | ADR-0007 | — | W-B |
| FM-121 | ai-native | `TO_JSONLD(node)` and Markdown summary output shaping | Novel | M4 | — | — | W-B, W-C |
| FM-122 | ai-native | MCP server with stdio and HTTP transports, auth, schema discovery, read-only default, write-scoped profiles, sample-size controls, and health/readiness semantics | Novel | M4 | ADR-0007 | — | W-A, W-B, W-C, W-D, W-E |
| FM-123 | ai-native | Append-only tenant-scoped ingest lane with async secondary indexing and explicit ACK / lag budget | Novel | M4 | ADR-0009 | — | W-E |
| FM-124 | ai-native | `event-store` partition class with cold-tier mover and tenant retention classes | Novel | M6 | ADR-0009 | — | W-E |
| FM-125 | ai-native | JSON-LD property type for structured log payloads | Novel | M4 | — | — | W-E |
| FM-126 | ai-native | Per-tenant isolation for all secondary roots and projections: ANN, text, temporal, and derived structures never cross tenant boundaries | Parity | M5 | ADR-0010 | — | W-A, W-B |
| FM-127 | ai-native | PII marking on properties + redaction / mandatory encryption / audited reads | Parity | M5 | — | — | all |
| FM-128 | ai-native | Entity resolution via canonicalization edges, exact/fuzzy/embedding prefilters, merge policy, and provenance-preserving merges | Novel | M6 | — | — | W-C |
| FM-129 | ai-native | Plan caching keyed by structural signature (not exact text) | Novel | M6 | — | — | W-B |
| FM-130 | ai-native | Cancellable queries (agents frequently abandon) | Parity | M4 | — | — | W-A, W-B |
| FM-131 | ai-native | Tenant-scoped schema and ontology introspection in JSON / Markdown under explicit token budget | Novel | M1 | ADR-0007 | — | W-B, W-C, W-E |
| FM-132 | ai-native | Validated semantic tool profiles for retrieval, neighborhood expansion, path search, `query.read`, and role-scoped `query.write` | Novel | M1 | ADR-0007 | — | W-A, W-B, W-C, W-E |
| FM-133 | ai-native | Structured evidence artifact output with contributing IDs, scores, pruning reasons, snapshot timestamps, partition IDs, and policy hits | Novel | M1 | ADR-0007 | — | W-B, W-C, W-E, W-F |
| FM-134 | ai-native | First-class graph ingestion pipeline with manifest commit, stable chunk IDs, durable stage DAG, schema-guided extraction hooks, entity-resolution write-back, and idempotent retries | Novel | M2 | — | — | W-C, W-D |
| FM-135 | ai-native | First-party context-graph surface for sessions, messages, observations, actions, long-term facts, reinforcement, and reasoning traces | Novel | M1 | — | — | W-A, W-E |
| FM-136 | ai-native | Planner-selected WCOJ / factorized execution for cyclic and high-branching query shapes | Novel | M1 | — | — | W-B, W-C |
| FM-137 | ai-native | Safe-time watermark, pinned-snapshot metrics, retention classes, and cold-move policy as separate controls | Novel | M1 | ADR-0004 | — | W-A, W-E, W-F |
| FM-138 | ai-native | Per-tenant admission control for CPU, memory, spill, WAL, compaction, extraction, and ingest budgets | Novel | M1 | ADR-0010 | — | all |
| FM-139 | ai-native | Tenant-first hybrid partitioning with graph, ANN, event-time, and blob-manifest classes plus `voter` / `learner` / `witness` replica roles | Novel | M2 | ADR-0005 | — | W-A, W-B, W-D, W-E |
| FM-140 | ai-native | Hotspot mitigation via chunked neighborhoods, hot-split triggers, mirror reads, and commutative update lanes | Novel | M1 | ADR-0005 | — | W-A, W-C, W-E |

## Campaign M1-Lock acceptance criteria

These rows changed or were added during Campaign M1-Lock. `Locked` means the public contract is set; future changes require a new synthesis or ADR-backed revision.

| ID | Acceptance criterion | Tier | Status |
|----|----------------------|------|--------|
| FM-107 | A single plan may fuse BM25, vector candidates, and graph expansion over shared tenant-scoped IDs; `RRF` / `HYBRID` scoring is computed inside the optimizer, not as client-side post-processing. | M4 | Locked |
| FM-110 | Plans using `AS OF` or `BETWEEN` must prune by time before ANN, BM25, or graph fanout; late temporal filtering is invalid except for an internal correctness-only fallback path. | M6 | Locked |
| FM-114 | Shipped defaults are `<= 16 KiB` inline, `> 16 KiB && <= 1 MiB` in a local `blob-log`, and `> 1 MiB` by external object reference; every byte tier hangs off a content-addressed manifest and tenant-local dedupe domain. | M4 | Locked |
| FM-117 | One durable stage contract covers embedding, extraction, chunk revision metadata, and provider provenance; model execution may be external, but retries, orchestration, and graph write-back remain in-core. | M3 | Locked |
| FM-120 | Public APIs expose token counting, chunk-by-token-limit operations, and output shaping under an explicit token budget; truncation-only behavior is insufficient. | M4 | Locked |
| FM-122 | The tool surface ships over stdio and HTTP with auth, schema discovery, read-only-by-default profiles, explicit write scopes, sample-size controls, and deployment-safe health / readiness behavior. | M4 | Locked |
| FM-123 | Tenant-scoped ingest ACK follows durable append; secondary indexes build asynchronously with explicit lag accounting and visible lag budget. | M4 | Locked |
| FM-124 | Observability data lives in `event-store` time partitions with separate cold-move and tenant retention classes; history retention is not conflated with visibility GC. | M6 | Locked |
| FM-126 | ANN, BM25, temporal side indexes, projections, and evidence outputs are single-tenant by construction; no cross-tenant roots or postings exist. | M5 | Locked |
| FM-128 | Resolution uses canonicalization edges plus exact, fuzzy, and embedding prefilters; merges preserve provenance and apply a declared merge policy instead of destructive rewrite. | M6 | Locked |
| FM-131 | Emit tenant-scoped schema and ontology summaries, canonicalization edges, temporal fields, and edge-type stats in JSON / Markdown under an explicit token budget, with stable IDs and zero cross-tenant leakage. | M1 | Locked |
| FM-132 | Expose validated tool profiles for retrieval, neighborhood expansion, path search, `query.read`, and role-scoped `query.write`; every call preflights against schema epoch and safety policy. | M1 | Locked |
| FM-133 | Any query may request contributing node and edge IDs, scores, pruning reasons, snapshot timestamps, partition IDs, read path, and policy hits as structured output, without chain-of-thought text. | M1 | Locked |
| FM-134 | Manifest commit, stable chunk IDs, durable stage DAG, schema-guided extraction hooks, entity-resolution write-back, and idempotent retries are first-class; model execution may be external, but graph-state ownership remains in-core. | M2 | Locked |
| FM-135 | Ship a first-party canonical context-graph schema and tool surface for sessions, messages, observations, actions, long-term facts, reinforcement, and reasoning traces on top of general graph primitives. | M1 | Locked |
| FM-136 | The planner chooses WCOJ / factorized operators on qualifying cyclic or high-branching patterns, and benchmark evidence must show lower peak memory than binary joins without regressing simple path queries. | M1 | Locked |
| FM-137 | Expose safe-time watermark, pinned-snapshot metrics, retention classes, and cold-move policy as separate tunable controls; visibility GC, product retention, and cold demotion remain distinct. | M1 | Locked |
| FM-138 | Enforce per-tenant CPU, memory, spill, WAL, compaction, extraction, and ingest budgets, with mixed-workload replay showing no starvation across foreground and background work classes. | M1 | Locked |
| FM-139 | Graph, ANN, event-time, and blob-manifest partitions place independently under one logical namespace; replica roles include `voter`, `learner`, and `witness`; strong reads are default and follower reads require explicit bounded-staleness opt-in. | M2 | Locked |
| FM-140 | Chunked neighborhoods are mandatory; hot-split triggers, mirror reads, and commutative update lanes must exist and be benchmarked under skewed fan-in / fan-out without abort storms on declared commutative paths. | M1 | Locked |

## Glossary

- `ANN` (approximate nearest neighbor): search that returns near vectors quickly without exact full-collection scan.
- `BM25` (Best Matching 25): lexical ranking function over tokenized text used for full-text retrieval.
- `Blob-log` (local append-only value log): local storage tier for payloads too large to inline but too small to externalize by default.
- `Canonicalization edge`: graph edge linking an alias, mention, or duplicate record to the canonical entity it resolves to.
- `Factorized execution`: execution strategy that stores shared intermediate structure once instead of repeating the same tuples.
- `HLC` (hybrid logical clock): timestamp scheme that combines physical time with logical counters for ordered commits and freshness checks.
- `RRF` (Reciprocal Rank Fusion): rank-combination method that blends multiple scored candidate lists into one retrieval order.
- `Safe-time`: replica or partition watermark before which a snapshot is fully visible and some obsolete versions may be reclaimed.
- `Schema epoch`: monotonically increasing schema version used to reject tool calls compiled against stale structure or policy state.
- `WAL` (write-ahead log): durable journal flushed before a change is treated as committed.
- `WCOJ` (worst-case-optimal join): multiway join strategy that avoids binary-join blow-ups on cyclic or high-branching patterns.
- `Zone map`: compact min/max summary used to skip entire partitions or stripes during time pruning.

_Add rows as research surfaces new requirements. Row IDs are stable once published — do not renumber._

_Tier column is the target milestone. Tiers may shift once M1 locks the feature set and M2 promotes the architecture ADRs — rows do not get renumbered, only re-tiered with a changelog entry._
