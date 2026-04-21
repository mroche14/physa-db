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
| FM-016 | query | Built-in graph algorithms (BFS, DFS, PageRank, SSSP, WCC, Louvain) | Parity | M6 | — | — |
| FM-017 | query | Temporal / as-of queries | Novel | M6 | — | — |
| FM-018 | query | Subgraph / snapshot materialisation | Novel | M8 | — | — |
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
| FM-107 | ai-native | Full-text (BM25) index + `RRF` / `HYBRID` scoring operators | Parity | M6 | — | — | W-B |
| FM-108 | ai-native | Multi-hop retrieval over heterogeneous schema (planner picks direction) | Novel | M4 | — | — | W-B, W-C |
| FM-109 | ai-native | Built-in graph algorithms: PageRank, SSSP, BFS/DFS with early termination, Louvain, Leiden | Parity | M6 | — | — | W-C |
| FM-110 | ai-native | Bi-temporal model (valid-time + transaction-time) + `AS OF` / `BETWEEN` | Novel | M6 | — | — | W-F |
| FM-111 | ai-native | Per-fact TTL / forgetting curves + reinforcement primitive | Novel | M3 | — | — | W-A |
| FM-112 | ai-native | Provenance (source, timestamp, extraction confidence) per node/edge | Novel | M3 | — | — | W-C |
| FM-113 | ai-native | Confidence scores with composition under traversal (`CONFIDENCE` type) | Novel | M6 | — | — | W-C |
| FM-114 | ai-native | Blob storage: inline (<1 MB) + external (S3-compatible) with content-addressing | Novel | M4 | — | — | W-D |
| FM-115 | ai-native | Content-addressable dedup for blobs and chunks | Novel | M4 | — | — | W-D |
| FM-116 | ai-native | Chunk hierarchy: `Asset -HAS_CHUNK-> Chunk` with chunk types (page, frame, segment, span) | Novel | M4 | — | — | W-D |
| FM-117 | ai-native | Embedding hook point at ingest (user supplies the model; DB provides the slot) | Novel | M3 | — | — | W-A, W-B, W-D |
| FM-118 | ai-native | Embedding model version registry (navigate chunk embeddings by model version) | Novel | M6 | — | — | W-D, W-F |
| FM-119 | ai-native | Streaming results so the LLM can start generating before retrieval completes | Novel | M4 | — | — | W-B |
| FM-120 | ai-native | Token-budget-aware result shaping (`CONTEXT_WINDOW(results, budget)`) | Novel | M4 | — | — | W-B |
| FM-121 | ai-native | `TO_JSONLD(node)` and Markdown summary output shaping | Novel | M4 | — | — | W-B, W-C |
| FM-122 | ai-native | MCP (Model Context Protocol) server — agents call physa-db as a tool directly | Novel | M4 | — | — | W-A, W-B, W-C, W-D, W-E |
| FM-123 | ai-native | High-throughput streaming ingest (AFM-029) (target 100k events/s per tenant on reference HW) | Novel | M4 | — | — | W-E |
| FM-124 | ai-native | Time-partitioned storage layout with cold-tier auto-demotion | Novel | M6 | — | — | W-E |
| FM-125 | ai-native | JSON-LD property type for structured log payloads | Novel | M4 | — | — | W-E |
| FM-126 | ai-native | Per-tenant vector isolation (one tenant's vectors never in another's index) | Parity | M5 | — | — | W-A, W-B |
| FM-127 | ai-native | PII marking on properties + redaction / mandatory encryption / audited reads | Parity | M5 | — | — | all |
| FM-128 | ai-native | Entity resolution helpers (`CANONICAL_OF` edge type + dedupe) | Novel | M6 | — | — | W-C |
| FM-129 | ai-native | Plan caching keyed by structural signature (not exact text) | Novel | M6 | — | — | W-B |
| FM-130 | ai-native | Cancellable queries (agents frequently abandon) | Parity | M4 | — | — | W-A, W-B |

_Add rows as research surfaces new requirements. Row IDs are stable once published — do not renumber._

_Tier column is the target milestone. Tiers may shift once M1 locks the feature set and M2 promotes the architecture ADRs — rows do not get renumbered, only re-tiered with a changelog entry._
