# physa-db Roadmap

> A living document. Each milestone links to a GitHub Milestone of the same name once the repo is pushed.

Milestones are numbered, not dated — we ship when ready, but keep velocity visible on the dashboard.

Guiding principles (see `AGENTS.md` §§0, 11, 12, 15):
- **Features before architecture.** M1 locks features; M2 promotes ADRs from *Proposed* to *Accepted*.
- **First-principles thinking** over analogy.
- **Unlimited engineering budget, no shortcuts.**
- **Correctness is non-negotiable; performance is the moat.**

### Causal chain (enforced by `AGENTS.md` §15)

```
positioning.md → ai-agent-workloads.md → feature-matrix.md → ADRs → code
```

M1 locks the first three. M2 locks the fourth. M3+ is build.

---

## M0 — Foundation (in progress)

Goal: make the repo navigable by both humans and agents; all conventions in place.

- [x] `initial-vision.md` captured (immutable)
- [x] `AGENTS.md` authored (v3 with §§11, 12, 15 rules: first-principles, no-shortcuts, features-before-architecture)
- [x] Repository skeleton created
- [x] Cargo workspace initialised with empty crates
- [x] License (Apache-2.0), `CONTRIBUTING.md`, `SECURITY.md`
- [x] ADR-0001 project tracking — GitHub Issues + Projects v2 + static dashboard snapshot
- [x] ADR-0002…0005 drafted as *Proposed* (pending M1 feature lock); ADR-0006 *Accepted* (research privacy is independent)
- [x] `docs/requirements/positioning.md` — AI-agent-native technical positioning
- [x] `docs/requirements/ai-agent-workloads.md` — authoritative source of six workload families + derived requirements
- [x] `justfile` + `.mise.toml` + `xtask/` dev workflow
- [x] `release-plz` configured for automated versioning
- [x] `private/` gitignored for competitive research; `docs/requirements/` for public synthesis
- [ ] CI: `fmt`, `clippy`, `test`, `bench-regression-guard`, `fuzz-smoke` on push and PR
- [ ] Issue + PR templates live on the pushed repo
- [ ] GitHub Pages dashboard MVP deployed
- [ ] Labels provisioned

## M1 — Feature lock (positioning → workloads → feature matrix)

Goal: every feature physa-db will ship is captured as an `FM-NNN` row, each traced to a workload in `ai-agent-workloads.md` or to the commercial positioning. Private research fuels the output; no competitor attribution in public files.

**Exit criterion:** `feature-matrix.md` is ratified and frozen for M2 entry. Any later row addition requires a new research cycle.

- [ ] Private competitor profiles for all tracked competitors (one file per codename in `private/research/competitors/`)
- [ ] Private pain-point mining complete across Reddit / X / HN / forum / GH issues
- [ ] LDBC SNB and SNAP dataset ingestion harness (no DB yet — just loaders + checkers)
- [ ] **AI-agent workload refinement:** expand `ai-agent-workloads.md` with concrete numbers (ingest rates, vector dims, chunk sizes, retention periods) derived from real agent frameworks (LangChain, LlamaIndex, MCP clients, observability tools)
- [ ] **Public output:** `docs/requirements/feature-matrix.md` completed — both parity rows (FM-001…099) and AI-native rows (FM-100…) with workload references and tier assignments
- [ ] **Public output:** `docs/requirements/performance-targets.md` with numerical targets for LDBC SNB + AI-native benchmarks (ANN recall vs latency, hybrid-query p95, ingest throughput for agent traces)
- [ ] **Public output:** `docs/requirements/non-goals.md` refined (graph+vector not vector-only, property store with blobs not S3, etc.)
- [ ] Research-surfaced ADRs filed as drafts (ADR-0007+) — stay in Draft until M1 exits

## M2 — Architecture lock (feature matrix → ADRs)

Goal: every architectural ADR is promoted from *Proposed* to *Accepted* by citing the FM rows it addresses; no premature choices remain.

**Exit criterion:** every *Proposed* ADR is either promoted, reshaped, or rejected, and the set of *Accepted* ADRs covers every FM row that needs architectural backing.

- [ ] Promote ADR-0002 (GQL + Cypher) with cross-dialect extension grammar for the `PHYSA` namespace (vector, temporal, hybrid operators)
- [ ] Promote ADR-0003 (storage) after specifying the vector + blob + temporal tiers alongside the tiered node layout
- [ ] Promote ADR-0004 (MVCC) with the bi-temporal extension ADR that layers valid-time × transaction-time on the version chain
- [ ] Promote ADR-0005 (sharding) after specifying how ANN indices span shards and how blob/observability data partitions
- [ ] New ADRs for AI-native subsystems: vector index (HNSW/IVF-PQ), blob storage adapter, hybrid query planner, MCP server, streaming result pipeline, embedding-version registry

## M3 — Embedded single-node kernel

Goal: a library that stores a graph on disk, executes read queries, beats the fastest published graph DB on a chosen micro-benchmark, AND serves a representative AI-agent workload end-to-end.

- [ ] Property graph data model (nodes, relationships, properties, labels, types)
- [ ] Durable storage with WAL + crash recovery (first-principles custom layout — see ADR-0003 post-promotion)
- [ ] MVCC transactional layer (see ADR-0004 post-promotion)
- [ ] **GQL + openCypher** parsers sharing a logical plan IR (ADR-0002 post-promotion, both from the start)
- [ ] Vector property type, HNSW index, `NEAREST(vector, K)` operator
- [ ] Query planner + physical executor for the read path (hybrid plans: ANN → graph expansion)
- [ ] Index: label + property B-tree index
- [ ] LDBC SNB IC-1 … IC-14 passing against SF1 dataset
- [ ] Representative RAG benchmark passing (hybrid vector + graph retrieval over a known dataset)
- [ ] Benchmark: wins on a documented workload vs the current fastest OSS option
- [ ] Property tests pass on the storage codec under fuzz & proptest
- [ ] Stress scenarios: disk-full, process-kill, partial-write, clock-skew, supernode — all pass

## M4 — Server, Bolt protocol, and MCP

Goal: a running daemon that Neo4j drivers AND AI agents can connect to directly.

- [ ] Bolt v5 protocol server (network, handshake, message framing)
- [ ] HTTP/JSON query endpoint with server-sent events streaming
- [ ] **MCP (Model Context Protocol) server** — agents can treat physa-db as a tool without glue code
- [ ] gRPC endpoint with reflection (incl. streaming ingest for observability workloads)
- [ ] Auth (basic + token + OIDC bridge stub)
- [ ] Connection pooling, backpressure, graceful shutdown
- [ ] Compatibility: official Neo4j Java/Python/JS drivers can connect and run `MATCH (n) RETURN n LIMIT 10` plus a representative LDBC SNB subset
- [ ] `physa` CLI for admin tasks (import, backup, restore, dump, inspect)
- [ ] Native Rust client (`physa-client`) with parity to `physa-cli`

## M5 — Clustering + native multi-tenancy

Goal: horizontal scaling and multi-tenant isolation, both natively and without enterprise gating.

- [ ] Raft consensus for metadata (+ Jepsen-style linearizability tests)
- [ ] Sharding strategy per ADR-0005 (abstracted from API, smart inside the planner)
- [ ] Replication (leader + followers, follower reads, cross-region async)
- [ ] Tenant isolation: namespaces with per-tenant quotas, RBAC, per-tenant encryption keys
- [ ] Per-tenant vector index isolation (one tenant's embeddings are never in another's ANN space)
- [ ] Online re-sharding with zero-downtime
- [ ] Chaos testing via `turmoil` + external Jepsen run
- [ ] Multi-region disaster recovery story documented & tested

## M6 — Performance surge + AI-native depth

Goal: win every public benchmark we publish, including hybrid AI-agent workloads.

- [ ] Columnar property store with dictionary encoding (incl. vector columns as contiguous float arrays for SIMD)
- [ ] Vectorised / morsel-driven execution for analytical (BI) queries
- [ ] `io_uring` / `AIO` storage backend on Linux
- [ ] Query result caching with plan-level invalidation
- [ ] Compiled query plans (`cranelift` JIT or AoT codegen)
- [ ] NUMA-aware scheduling
- [ ] IVF-PQ index for memory-efficient ANN over huge vector collections
- [ ] Full-text (BM25) index + hybrid scoring (RRF) in one plan
- [ ] Publish a reproducible benchmark head-to-head vs top competitors on RAG, agent-memory, and observability workloads

## M7 — Ecosystem & v1.0

Goal: first production-grade release.

- [ ] Full GQL (ISO/IEC 39075:2024) conformance
- [ ] Drivers: Rust, Python, Node, Go, Java (all native — no Bolt bridge)
- [ ] Managed cloud reference deployment (Terraform + Helm + operator)
- [ ] Migration tooling from dominant competitor dump formats
- [ ] Security audit (external)
- [ ] v1.0.0 release

## M8 — Beyond parity

Goal: features that no current graph DB offers, selected via first-principles on what users actually need.

Examples (non-exhaustive, guided by `docs/requirements/`):
- [ ] Bi-temporal `AS OF` queries across all workload families (W-F completeness)
- [ ] Incremental materialised views over subgraphs
- [ ] Embedded WASM UDFs with sandboxing (incl. user-supplied embedding functions)
- [ ] Built-in federated querying across physa-db instances
- [ ] Content-addressable blob dedup at scale with zero-copy fanout
- [ ] Confidence-aware traversal (path-score composition) as a query primitive

---

## Cross-cutting tracks (always-on)

- **Research track.** Continuous private mining of competitor releases, papers (SIGMOD, VLDB, CIDR, HotOS), community pain points. Synthesis surfaces publicly in `docs/requirements/`.
- **Benchmarks track.** Every PR labeled `type:perf` ships reproducible numbers. Nightly `bench-regression` job on `main` — any regression > 2% blocks the next release PR.
- **Stress track.** Weekly long-soak run on `main`. `tests/stress/` grows with every concurrency/cluster feature.
- **Docs track.** Every shipped feature ships user-facing docs in the same PR.
- **DX track.** Every new dev workflow gets a `just` recipe. No undocumented incantations.
