# AI-agent workloads

> The authoritative source of what agentic-AI applications ask from a graph database. Every feature in `feature-matrix.md` traces back to a workload described here.

This file is **input to architecture**, not architecture itself. Architectural ADRs cite it; it does not cite them.

---

## Workload taxonomy

We group AI-agent workloads into six families. Any agentic system in production in 2026 is some blend of these.

| ID | Family | Short description |
|----|--------|-------------------|
| W-A | Agent memory | Episodic + semantic memory for long-running agents |
| W-B | Retrieval-augmented generation (RAG) | Hybrid vector + graph retrieval feeding LLM context windows |
| W-C | Knowledge graphs | Curated entities, relations, provenance, confidence |
| W-D | Multi-modal assets | Images, audio, video, PDFs with chunks + embeddings + relationships |
| W-E | Agent observability | Traces of tool calls, thoughts, outputs as a queryable graph |
| W-F | Temporal reasoning | As-of queries over any of the above |

## W-A — Agent memory

**What it looks like.** An agent runs continuously. Every interaction produces facts ("the user mentioned X", "I called tool Y at time T", "the result was Z"). Over months the agent accumulates millions of facts, some stable (preferences), some transient (working context).

**Read patterns.** Semantic retrieval (give me facts similar to the current situation), recency-weighted (prefer recent), decaying (forget old unless reinforced), graph-walk (facts connected to the current entity).

**Write patterns.** Append-heavy, continuous. Each fact is small. Occasional updates to confidence/recency scores.

**Required primitives.**
- Vector-typed properties on fact nodes (the embedding of the fact).
- Relationships linking facts to entities, times, sources, and other facts.
- TTL / forgetting curves per fact.
- Confidence scores per fact, updatable without rewriting the node.
- Reinforcement primitive: "touching" a fact extends its TTL.

**Non-trivial.** Supernodes (a frequently-mentioned entity) receive huge fan-in; our storage layout must handle this without lock contention (see ADR-0004).

## W-B — Retrieval-augmented generation (RAG)

**What it looks like.** Before the LLM produces a response, the system retrieves the top-K most relevant facts/documents and inserts them into the context window.

**Two retrieval modes.** Vector similarity (semantic) and graph traversal (structural). Mature RAG systems **combine** them.

**Read patterns.**
- Top-K ANN over dense embeddings (k typically 10–100).
- Followed by 1–3 graph hops to fetch related context.
- Followed by reranking with a cross-encoder (external model) using the retrieved bundle.
- Fusion (Reciprocal Rank Fusion, hybrid scoring) to combine vector + BM25 + graph-based scores.

**Write patterns.** Light. Embeddings are regenerated when source text changes.

**Required primitives.**
- HNSW or IVF index on vector properties.
- Hybrid query plan: ANN → graph expansion → rerank, all in one query.
- Full-text (BM25) index on textual properties.
- Streaming results so the LLM can start generating before retrieval fully completes.
- Token-budget-aware truncation in result shaping.
- Multiple concurrent embedding model versions (the query vector is from model V2, the index has V1 vectors — handle gracefully).

**Non-trivial.** Graph traversal over heterogeneous schema (documents → authors → topics → citations) needs the planner to pick the right traversal direction per query.

## W-C — Knowledge graphs

**What it looks like.** Curated entities (people, companies, chemicals, genes) with typed relations. Populated by ingestion pipelines (possibly LLM-powered), queried by agents for grounded reasoning.

**Read patterns.** Pattern matching (GQL `GRAPH_TABLE` / Cypher `MATCH`), subgraph retrieval, centrality queries, community detection.

**Write patterns.** Bulk ingest from pipelines; incremental updates as new sources arrive.

**Required primitives.**
- Entity resolution support (a `CANONICAL_OF` edge type + dedupe helpers).
- Provenance per edge/node (source, timestamp, extraction confidence).
- Confidence scores that compose under traversal (path score = product of edge confidences).
- Open-world assumption: missing facts ≠ negative facts.
- Built-in graph algorithms: PageRank, SSSP, BFS/DFS with early termination, community detection (Louvain, Leiden).

**Non-trivial.** KGs evolve. Historical queries ("what did we believe about X on 2025-11-01?") need temporal layers — see W-F.

## W-D — Multi-modal assets

**What it looks like.** An agent ingests a PDF, a video, a podcast. It needs to store the raw asset, its chunks (pages, frames, segments), and their embeddings. Later it queries "find me the video segment closest to this description".

**Read patterns.** Vector similarity over chunk embeddings, then walk to the parent asset, then fetch the original bytes or a link to them.

**Write patterns.** Bursty — a whole asset comes in at once. Chunking + embedding generation dominates ingest time.

**Required primitives.**
- Blob storage: inline for small assets (< 1 MB), external reference (S3-compatible) for large.
- Content-addressable storage for automatic deduplication.
- Chunk hierarchy: `Asset --HAS_CHUNK-> Chunk` edges.
- Chunk types: page (PDF), frame (video), segment (audio with timestamps), span (text).
- Embedding attached to each chunk as a vector property.
- Hook point at ingest for the user's embedding model. The DB does not ship a model; it provides the slot.
- Metadata extraction hooks (MIME, duration, resolution, page count).
- Presigned-URL generation for external blob fetch.

**Non-trivial.** Large assets + embeddings per chunk inflate storage fast. Quota + compression are first-class.

## W-E — Agent observability

**What it looks like.** Every tool call an agent makes, every thought, every output is a node. Every causal link is an edge. Operators query this graph to debug, audit, and improve agents.

**Read patterns.** Time-range scans, path queries (what led to this failure), aggregate metrics (tool call p95 latency by type).

**Write patterns.** Very heavy append. Thousands of events per agent per session.

**Required primitives.**
- High ingest throughput (target: 100k events/s per tenant on reference hardware).
- Structured log payload as JSON-LD property.
- Time-partitioned storage layout (observability data is time-range queried).
- Retention policies per tenant.
- Streaming ingestion API (gRPC + OTLP compatibility).

**Non-trivial.** Agent-trace graphs are enormous and mostly never queried. Need cold-tier storage with automatic demotion.

## W-F — Temporal reasoning

**What it looks like.** Any of W-A through W-E, with an `AS OF <timestamp>` clause. "What did the KG say about X on this date?" "What tools did the agent call in this window?" "What was the embedding of this chunk under model V1?"

**Read patterns.** Point-in-time snapshot + range queries over history.

**Write patterns.** Same as the underlying workload, plus versioning of updates.

**Required primitives.**
- Bi-temporal model (valid-time + transaction-time per fact).
- `AS OF` syntax in GQL and Cypher (as PHYSA extensions if standards don't cover it).
- Efficient snapshot reconstruction (log-structured, not copy-on-write-per-version).
- Embedding version tracking: a chunk can have multiple embeddings from different model versions, navigable by time.

**Non-trivial.** Temporal + MVCC compose into a 2D time model. Must not double the storage.

---

## Derived requirements (cross-cutting)

### Data types (first-class property types)

| Type | Notes |
|------|-------|
| `INT`, `FLOAT`, `BOOL`, `STRING`, `BYTES` | Standard. |
| `TIMESTAMP`, `DURATION` | With timezone + as-of semantics. |
| `VECTOR<F16/F32, DIM>` | Dense. Fixed dim per property. |
| `SPARSE_VECTOR` | For SPLADE-style retrieval. |
| `BLOB(inline | external)` | With content hash. |
| `JSON` / `JSON-LD` | For semi-structured payloads and observability. |
| `GEO` | Point + region. |
| `CONFIDENCE` | Float [0,1] with composition under traversal. |

### Operators (first-class query-language citizens)

- Distance/similarity: `COSINE(a, b)`, `DOT(a, b)`, `L2(a, b)`, `HAMMING(a, b)`, `JACCARD(a, b)`.
- Retrieval: `NEAREST(vector, K)`, `WITHIN_DISTANCE(vector, radius)`.
- Graph algorithms: `PAGERANK(graph)`, `COMMUNITIES(graph, algorithm)`, `SHORTEST_PATH(a, b)`, `BFS(root, depth)`.
- Scoring: `RRF(score_sets)`, `HYBRID(scores, weights)`.
- Temporal: `AS OF <ts>`, `BETWEEN <ts1> AND <ts2>`.
- Shaping: `CONTEXT_WINDOW(results, token_budget)`, `TO_JSONLD(node)`.

### Indices

- B-tree / hash — scalar properties.
- HNSW — dense vectors, approximate.
- IVF-PQ — dense vectors, memory-efficient alternative for huge collections.
- Full-text (BM25) — strings.
- Geo (R-tree / s2) — `GEO` type.
- Temporal (segment tree) — `TIMESTAMP` columns used in `AS OF`.

### Execution

- Hybrid query plans that mix graph expansion, vector search, full-text, and scalar filters in one plan.
- Vectorised / morsel-driven execution for analytical queries.
- Streaming results.
- Cancellable queries (agents frequently abandon).
- Plan caching keyed by structural signature (not exact text).

### Storage

- Tiered layout (see ADR-0003 first-principles outline) — now also tiered by blob size and vector dimensionality.
- Columnar properties, including vector columns stored as contiguous float arrays for SIMD.
- External blob store adapters (S3, GCS, R2, local disk).
- Per-tenant encryption keys (relevant for PII in embeddings).

### Protocols

- Bolt v5 for migration-friendly driver compatibility.
- HTTP/JSON query endpoint.
- gRPC with reflection.
- **MCP (Model Context Protocol) server** so agents can treat physa-db as a tool directly.
- Streaming (HTTP/2 server-sent events, gRPC server streaming).

### Observability on our own workloads

- Provenance per query result (which nodes/edges contributed).
- Embedding model version tagged per vector.
- Slow-query log with plan explanation.
- OTLP export (metrics + traces + logs).

### Security

- Per-tenant embedding isolation (one tenant's vectors are never in another's index).
- PII handling: mark properties as PII; enforce redaction or mandatory encryption.
- RBAC per tenant, per namespace, per property.
- Audit log for all reads of PII-marked properties.

---

## Mapping to feature-matrix

Every requirement above is encoded as one or more FM-NNN rows in [`feature-matrix.md`](./feature-matrix.md). When a workload here gains a new sub-requirement, a new FM row is added. When an FM row is shipped, it references back to the workload(s) it serves. This file and the feature matrix evolve together.
