# ADR-0003: Custom graph-native storage engine, derived from first principles

- **Status:** Proposed (direction pending M1 feature lock — see `AGENTS.md` §15)
- **Date:** 2026-04-20
- **Context issue:** _(to be filed as `type:feature area:storage priority:p0`)_

> **Note on status.** Downgraded from *Accepted* on 2026-04-20 when features-first discipline was re-established. The first-principles derivation below assumes a workload mix dominated by graph traversal. AI-agent workloads ([`ai-agent-workloads.md`](../../requirements/ai-agent-workloads.md)) introduce additional shapes — dense vectors with ANN indices, large inline/external blobs, append-heavy observability traces, bi-temporal history — that may reshape the tiered layout (e.g. a third tier for vector-heavy nodes, dedicated blob-segment files, log-structured temporal layers). The promotion to *Accepted* is gated on the storage-layout ADR absorbing those workload constraints.

## Context

Every graph DB on the market either:

1. **Wraps a generic KV or page-oriented engine** (RocksDB, LMDB, sled, InnoDB, WiredTiger, FoundationDB). Examples: Dgraph (BadgerDB), ArangoDB (RocksDB), JanusGraph (Cassandra/HBase), SurrealDB (rocksdb / speedb).
2. **Builds its own engine** tuned for graph workloads. Examples: Neo4j (native adjacency), Memgraph (in-memory), TigerGraph (proprietary), KuzuDB (columnar, research-driven).

Wrapping a generic engine is cheaper in engineering hours; it also caps the performance ceiling at what that engine can do for point lookups and range scans, which is a bad fit for graph traversal. Graph workloads are dominated by **pointer chasing** — following edges — and the engines above are built for **range scans**.

The founder's guidance (`AGENTS.md` §§0, 11, 12):
- most efficient graph DB on the planet;
- first-principles thinking;
- no shortcuts, unlimited engineering budget.

## Decision

physa-db implements a **custom, graph-native storage engine** designed ground-up. We do NOT wrap RocksDB, sled, or LMDB as the primary store.

## First-principles derivation

Frame the problem by what the hardware actually forces us to pay.

### Irreducible costs

- **Follow one edge, cold**: one random NVMe 4 KB read ≈ 80 μs; one memory page fault ≈ 100 ns if in OS cache; one L3 hit ≈ 10 ns; one L1 hit ≈ 1 ns. Three orders of magnitude between the levels.
- **Neighbourhood locality**: in most real graphs, traversals access the out-neighbours of a node together. Co-locating those neighbours on a single cache line (L1: 64 B) or a single NVMe block (4 KB) amortises the fetch across many edges.
- **Graph skew**: degree distributions are power-law — a few nodes have millions of edges, most have very few. A one-size data layout is hostile to one of those regimes.

### Theoretical optimum

For **low-degree nodes**: store the node record *and* its out-adjacency inline in a single cache line. One read = the node and its edges.

For **high-degree nodes**: separate the adjacency from the node record, laid out as a compressed, columnar edge list (source implicit, destinations + edge properties in dictionary-encoded columns). Vectorised scans.

For **property access**: a columnar property store decoupled from the graph topology, so analytical queries don't pay for graph structure they don't read.

No generic KV store offers this. Wrapping one imposes its page structure on top of ours — a structural mismatch that caps performance.

### What prior art we reuse

- **LSM-tree insight** (RocksDB, LevelDB): out-of-place writes + background compaction = great write throughput with durable ordering. We adopt *the idea*; we don't adopt *the library*, because RocksDB serialises our graph-native tuples into opaque KV pairs and loses the layout we just fought to design.
- **Vectorised, cache-conscious scans** (MonetDB, DuckDB, KuzuDB): columnar + SIMD + morsel-driven parallelism. Directly applicable to our analytical (BI) path.
- **Copy-on-write B-tree** (LMDB, Btrfs): serializable snapshot isolation with cheap reads. We adopt the technique for our metadata layer.
- **io_uring** (Linux): batch async I/O without syscall overhead per op. We target it for the storage I/O path.

### What we'll build (outline; detailed ADRs to follow)

1. **Tiered node layout.** Low-degree (< 32 edges) nodes stored inline with their adjacency in a graph-native page. High-degree nodes point to an external adjacency chunk (columnar).
2. **Columnar property store.** Per-label, per-property column files with dictionary encoding.
3. **WAL + checkpointed pages** with per-tenant segment isolation for multi-tenancy.
4. **io_uring-backed block I/O** on Linux; AIO on others; synchronous fallback.
5. **MVCC (see ADR-0004) layered on top.**

## Consequences

**Positive**
- Performance ceiling is governed by the hardware, not by a generic wrapper.
- Graph-native layout makes traversal queries approach the theoretical minimum I/O.
- Tiered layout handles power-law skew natively (a top concern of real-world graphs).

**Negative**
- Massive engineering investment. We must build WAL, crash recovery, page cache, compaction, checksums ourselves.
- Correctness burden: a generic engine like RocksDB is battle-tested; our engine must earn that trust via property tests, fuzzers, Jepsen-style linearizability tests, and years of soak.
- Any bug in the storage layer is a data-loss bug. We adopt a **"prove it or don't ship it"** policy for every storage primitive: it ships with proptest + loom + fuzz + integration crash tests.

Accepted under `AGENTS.md` §§11, 12.

## Alternatives considered

- **Wrap RocksDB.** Rejected: caps performance; opaque to graph layouts; LSM compaction tuning is a full-time job even for dedicated projects.
- **Wrap LMDB.** Rejected: excellent for read-dominant workloads but single-writer; our multi-tenant, write-heavy SaaS target is a poor fit.
- **Wrap sled.** Rejected: sled is not production-ready for our scale; also still a KV store.
- **Fork KuzuDB's engine.** Tempting (columnar, graph-aware), but rejected: C++ codebase, licence & ergonomics mismatch, and forking locks us into someone else's architectural choices. We will study their papers and absorb the lessons into a Rust-native design.

## Open sub-ADRs (to be written under M2)

- ADR-0008: on-disk page format (header, checksum, version, tenant tag).
- ADR-0009: WAL format & group commit protocol.
- ADR-0010: compaction strategy.
- ADR-0011: tiered node layout thresholds (benchmark-derived).

## References

- Neo Technology, *The Neo4j Manual*, particularly on the "native graph storage" design.
- Köpcke, *KuzuDB: Efficient Columnar Graph Processing for Research Workloads*, CIDR 2023.
- Idreos et al., *The MonetDB Architecture*, IEEE Data Eng. Bull. 2012.
- Leis et al., *Morsel-driven Parallelism*, SIGMOD 2014.
- Kleppmann, *Designing Data-Intensive Applications*, chapters 3 & 7.
- Axboe, *Efficient IO with io_uring*, 2019.
