# Non-goals

> What physa-db explicitly will NOT try to be.

Non-goals are load-bearing. They protect the core promise by refusing adjacent ambitions that would dilute it.

## Not a standalone vector database

physa-db is **graph + vector**, not vector-only. If an application only needs vector similarity search with no graph structure, a dedicated vector store (pgvector, Qdrant, LanceDB, Weaviate as a vector index, Milvus) is the right tool. Our value comes from combining vectors, graph traversal, full-text, and scalar filters **in a single query plan** — if you never need that combination, you're paying for machinery you don't use.

**Why:** a focused vector DB will beat us on pure-ANN benchmarks because it pays no storage / concurrency / transaction tax. Our target is the *hybrid* benchmark where single-engine planning wins.

## Not a document store

We are a **graph database**. Properties can hold JSON and JSON-LD (required for agent observability and knowledge graphs), but physa-db is not a general-purpose document DB. If your data model is nested documents without relationships, use MongoDB, Couchbase, or Postgres with JSONB.

**Why:** document stores invest heavily in secondary indexes on arbitrary nested paths. That investment is misaligned with traversal-dominated workloads.

## Not an S3 replacement

physa-db stores blob **references and small inline payloads** (< 1 MB by default), with content-addressable dedup and presigned-URL generation. For petabyte cold storage, use S3, GCS, R2, or on-prem object storage — physa-db integrates with these via adapters (FM-114). We don't plan to become the authoritative store for large binary archives.

**Why:** object stores have spent a decade optimising for append-mostly, cheap-cold, eventually-consistent blob semantics. Re-implementing that isn't a graph-DB problem.

## Not a multi-model "everything" database

physa-db refuses the "graph + document + KV + time-series" multi-model marketing. We pick **graph as the primary model, with vectors and blobs as first-class adjacent capabilities** because those adjacencies are mandatory for AI-agent workloads. We do not extend this list. Adding a relational/tabular layer, a true document store, or a wide-column store is out of scope.

**Why:** multi-modal systems compromise on every model. Our performance bar requires vertical specialisation on the graph+vector+blob triad that AI agents actually need.

## Not an analytics warehouse

We target **mixed transactional + small-to-medium analytical workloads** (LDBC SNB Interactive + BI, plus AI-agent workloads). We do not target petabyte-scale OLAP over flat tables — that's Snowflake / BigQuery / ClickHouse territory.

**Why:** OLAP warehouses have a different cost model (columnar on object store with large fan-out); merging those is a distraction.

## Not a pure-analytical in-memory engine (AFM-011)

physa-db will not offer an "analytical mode" that disables ACID guarantees, logging, and crash recovery just to fit graphs into memory and run ingest faster. We guarantee durability and consistency.

**Why:** The operational overhead of managing separate analytical and transactional configurations, and the risk of data loss on crash, violates our commitment to predictable, reliable graph storage.

## Not a streaming engine

No native CEP (complex event processing). Change-data-capture output is a goal (post-M5). Stream processing on top of CDC is for Flink / Kafka / Materialize.

## Not a proprietary / EDK-fork licence

The code is Apache-2.0 end-to-end. There is no "enterprise edition" with gated features — multi-tenancy, clustering, per-tenant encryption keys, vector indices, MCP server are all in the OSS build. We will never adopt BSL, SSPL, or any "fair-code" licence.

**Why:** the founding premise of the project is precisely to un-do the prevailing pricing pattern (see [`./positioning.md`](./positioning.md) §1).

## Not a mobile / embedded-edge product (initially)

The embedded mode (library linked into a host process, see FM-039) is supported. But the targets (NVMe, 64 GB RAM) are datacentre-class. We don't target battery-powered devices or < 1 GB memory footprints.

## Not a Windows target

physa-db ships to **Unix servers** — Linux in production, macOS for developer parity. Windows is **not** a supported host or target platform. No `#[cfg(windows)]` branches, no PowerShell scripts, no Windows-specific filesystem handling.

**Why:** server graph databases run on Linux. Supporting Windows costs CI minutes, test surface, and filesystem-semantics bugs (case-insensitive FS, no `fork`, CRLF, path-length limits) in exchange for zero realistic users of a server DB. Dev on macOS works because Unix semantics are the same. If a dependency requires Windows workarounds, we drop the dependency.

## Not an LLM host

physa-db provides **hook points for embedding generation** (FM-117) and serves as a tool to agents via MCP (FM-122). It does **not** ship a bundled LLM, embedding model, or inference runtime. Users bring their own models.

**Why:** embedding-model choice evolves on a monthly cadence. Tying our release train to a model shipment would drag the database out of date. Hook points let users swap models without a DB upgrade.

## Not a blockchain

Self-explanatory.

## Not dependent on the JVM, Go, or any GC runtime

Pure Rust. This is an inviolable technology choice, not an engineering preference — GC pauses, memory bloat, and the need for complex heap tuning are specific problems we are solving. Operational simplicity is paramount; requiring specialized DBA skills just to maintain a stable baseline is a non-goal.

## Not beholden to ISO GQL if it diverges from user needs

We implement GQL (FM-001) because the standard aligns with our design and unlocks portability. If the ISO spec evolves in ways that harm users, we extend rather than regress — via the `PHYSA` extension namespace clearly documented and shared between both dialects.

## Not a generic KV-store backend (AFM-014)

physa-db uses a graph-native columnar adjacency layout. We do not layer our graph engine on top of a generic KV-store (like RocksDB or FoundationDB).

**Why:** Relational or KV-stores mapped to graphs are suboptimal for index-free adjacency and deep traversals, introducing unnecessary I/O overhead.

## Not a system that relies on hard memory aborts (AFM-015)

We do not use blunt memory trackers that abort queries simply because the process nears a limit.

**Why:** Aborting queries is a poor user experience. The engine must gracefully spill to NVMe or use memory-mapped structures to maintain stability under memory pressure.

## Not a proprietary query language ecosystem (AFM-016)

We natively support standard query languages: GQL and openCypher. We will not invent a new, proprietary query language.

**Why:** Standard languages reduce the learning curve, prevent vendor lock-in, and provide a seamless migration path from legacy systems.
