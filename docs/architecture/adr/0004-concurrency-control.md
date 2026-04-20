# ADR-0004: Concurrency control — MVCC snapshot isolation (with opt-in serializable)

- **Status:** Proposed (pending M1 feature lock — see `AGENTS.md` §15)
- **Date:** 2026-04-20
- **Context issue:** _(to be filed as `type:feature area:storage priority:p1`)_

> **Note on status.** Downgraded from *Accepted* on 2026-04-20 under the features-first rule. MVCC-SI (+opt-in SSI) remains the presumptive choice because the AI-agent workloads reinforce it: agent-memory traces and observability ingest are heavy append streams, RAG retrieval is read-dominant and must not block writers, and bi-temporal reasoning (W-F) naturally extends MVCC's version chain into a two-dimensional (valid-time × transaction-time) model. Promotion to *Accepted* depends on the temporal ADR that specifies how bi-temporal coordinates layer onto the MVCC timestamp space without doubling storage.

## Context

"Concurrency control" is how a database lets many transactions touch shared data at once without producing incorrect results. Two dominant approaches exist; one of them was under review and the founder asked for an explanation before choosing. This ADR serves double duty as both decision and primer.

### What the two approaches mean

Imagine two bank-transfer transactions hitting the same account simultaneously:

- `T1`: move £10 from Alice to Bob.
- `T2`: move £5 from Alice to Carol.

**Option A — Optimistic Concurrency Control (OCC).**
Each transaction:
1. Reads whatever it needs, no locks, proceeds as if it were alone.
2. At commit time, the DB validates: "did anyone else write to the rows I read since I started?"
3. If yes, the transaction aborts and the client retries. If no, it commits.

Think of OCC as "forgiveness, not permission". Cheap when conflicts are rare. Catastrophic when they're common — retries pile up and throughput collapses. Hotspot keys (a global counter, a trending user) become hell.

**Option B — Multi-Version Concurrency Control (MVCC).**
Every write creates a **new version** of the row, tagged with the transaction ID that created it. The old version is not overwritten until no active transaction still needs it. Readers get a **consistent snapshot** of the database as of the moment their transaction started, so:
- Readers never block writers.
- Writers never block readers.
- Write-write conflicts are still possible, but they're detected by the version timestamps, and only the *actual* writers contend — not every reader.

Postgres, MySQL InnoDB, CockroachDB, TiKV, FoundationDB, SQL Server, and Oracle all use MVCC variants.

### Isolation levels

Different MVCC implementations offer different guarantees:

- **Read Committed**: you see committed data as of each statement. Weakest useful level.
- **Snapshot Isolation (SI)**: you see the database as of one consistent moment (your transaction's start), even across many statements. This is what most "MVCC" engines mean in practice. Anomaly: *write skew* (two transactions each read rows the other would invalidate and both succeed).
- **Serializable Snapshot Isolation (SSI)**: SI plus a conflict detector that prevents write skew. Slower to commit, but equivalent to "one transaction at a time".

## Decision

physa-db uses **MVCC with Snapshot Isolation by default**, and an **opt-in Serializable** mode (specifically, Serializable Snapshot Isolation — SSI) selectable per transaction.

- Default = SI (what Postgres calls `REPEATABLE READ`).
- `BEGIN SERIALIZABLE` (or the GQL/Cypher equivalent) upgrades the transaction to SSI.
- Read-only transactions always read from a consistent snapshot (free).

### First-principles derivation

Graph queries tend to:
1. **Read a lot** (traversal can touch millions of edges) and **write a little** (update a property, add an edge).
2. **Mix read-heavy analytical** (BI queries) with **transactional updates** (interactive workloads).
3. Have **locality-sensitive hotspots** (popular nodes, supernodes) that would be under lock contention in a pessimistic scheme.

Irreducible constraint: readers must not block on writers. OCC fails because contention on supernodes causes retry storms; a pessimistic lock scheme fails for the same reason. MVCC is the only option that mathematically decouples readers from writers.

SI is strictly preferable to Read Committed (same cost, stronger guarantee). SSI is strictly preferable to SI where correctness under concurrency matters (adds a commit-time cost but prevents write skew). Offering both lets users choose the cost/safety point per transaction.

### Garbage collection of old versions

Old versions are retired when **no active transaction has a snapshot older than the version's `commit_ts`**. This is tracked via the global `oldest_active_snapshot` watermark. GC is incremental, piggy-backed on compaction.

### Write-set intent locks (very fine-grained)

To detect write-write conflicts early, a write takes an **intent lock** on the (node, property) or (edge) cell. No reader blocks on an intent lock. Two writers racing on the same cell lose the late-commit race (one aborts and retries). Supernode reads are never blocked.

## Consequences

**Positive**
- Readers and writers never block each other.
- Snapshots are free (no copy-on-write overhead for readers).
- SSI available for cases that need true serializability (financial transfers, inventory, etc.).
- Natural fit with our custom storage engine's append-friendly layout.

**Negative**
- Version chains cost storage until GC runs; long-running read transactions can pin old versions.
- SSI's conflict-tracking adds commit-time overhead; users pay for the safety they opt into.
- Implementation complexity is high (timestamp ordering, GC, SSI conflict graph).

Accepted under `AGENTS.md` §§11, 12.

## Alternatives considered

- **Strict 2PL (pessimistic locking).** Rejected: supernode reads under constant write would starve. Mainstream graph DBs that use locks often advertise "avoid high-degree hubs" as guidance, which we refuse to inherit.
- **Pure OCC.** Rejected: same hotspot problem, inverted.
- **Single-writer, multi-reader (LMDB-style).** Rejected: write throughput ceiling is too low for multi-tenant SaaS.
- **HTAP via dual store.** Rejected: scope creep; MVCC can serve both OLTP and modest OLAP workloads from one store.

## References

- Kung & Robinson, *On Optimistic Methods for Concurrency Control*, TODS 1981.
- Bernstein & Goodman, *Multiversion Concurrency Control — Theory and Algorithms*, TODS 1983.
- Cahill et al., *Serializable Isolation for Snapshot Databases*, SIGMOD 2008 (the SSI paper).
- PostgreSQL MVCC internals: https://www.postgresql.org/docs/current/mvcc-intro.html
- Kleppmann, *Designing Data-Intensive Applications*, Chapter 7.
