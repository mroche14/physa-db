# ADR-0004: MVCC with HLC timestamps, WAL-backed commit, and explicit safe-time control

- **Status:** Accepted
- **Date:** 2026-04-25
- **Features addressed:** FM-005, FM-006, FM-007, FM-010, FM-017, FM-110, FM-111, FM-118, FM-123
- **Workloads addressed:** W-A, W-B, W-E, W-F
- **Context issue:** _(to be filed as `type:feature area:storage priority:p1`)_

## Context

The earlier draft settled only the isolation-level choice. Campaign M1-Lock showed that this was too narrow: the real decision spans timestamps, commit ordering, follower freshness, safe-time garbage collection, product-history retention, and hotspot control under power-law traffic. physa-db still needs readers that do not block writers, but it now also needs explicit semantics for temporal history and retrieval freshness. The accepted concurrency design therefore covers the full stack rather than isolation labels alone.

## Decision

MVCC = multi-version concurrency control: keep multiple versions of each row per write so readers see a snapshot without blocking writers. physa-db adopts MVCC with Hybrid Logical Clock (HLC) timestamps, snapshot isolation by default, and opt-in Serializable Snapshot Isolation (SSI) for transactions that require serializability.

1. Every committed version receives an HLC timestamp shared across commit ordering, temporal reconstruction, and follower-read freshness checks.
2. The local WAL is distinct from any replication log. A transaction commits only after its local WAL record is durable; replicated durability is layered above that rule rather than replacing it.
3. Single-shard writes use a one-phase commit fast path. Cross-shard writes escalate to two-phase commit only on the miss path.
4. Cell-level intent locks detect write-write conflicts without blocking readers. Readers always serve from a consistent snapshot.
5. Safe-time governs visibility garbage collection only. Product-history retention and cold-tier demotion are separate control planes with separate budgets and policies.
6. Strong reads are the semantic default. Bounded-staleness follower reads are an explicit opt-in per query or tool profile and must record read path, snapshot timestamp, and replica lag in evidence output.
7. Hotspot mitigation is a stack, not a single toggle: chunked neighborhoods are mandatory, mirror reads activate on read skew, and commutative update lanes activate only for explicitly declared commutative mutations such as reinforcement, counters, and recency bumps.

## First-principles derivation

### 1. Irreducible constraints

1. Readers for W-A, W-B, and W-F must not wait behind writers. If a retrieval call blocks on every concurrent fact append, the product fails its primary workloads.
2. Every durable transaction must pay at least one local journal sync or durable group-commit boundary. Pretending the replication log alone is enough couples crash recovery to the cluster path and weakens the single-node fast path.
3. Long-lived snapshots and product-history retention are different things. A transaction may need an old version for milliseconds, while a temporal query may need it for months.
4. A follower can serve a safe historical read only if it knows it has applied all commits up to the requested timestamp. That requires an explicit freshness contract, not a hidden routing heuristic.
5. Power-law hotspots create both read concentration and write concentration. One tactic does not cover both.

### 2. Theoretical optimum

The theoretical optimum is:

- readers pay zero lock wait and read one consistent snapshot;
- writers pay one durable local commit record plus the minimum extra coordination needed by their shard count;
- obsolete versions are reclaimed as soon as the oldest active snapshot and replica freshness floor permit;
- product history remains queryable under independent retention policy;
- hotspot reads and commutative hotspot writes avoid the general conflict path.

In other words, the lower bound is not "pick SI or SSI." It is "one timestamp domain, one durable local journal, one fast path for the common single-shard case, and separate controls for visibility, history, and cold storage."

### 3. Smallest structure that realizes the optimum

The smallest structure is:

- HLC-stamped version fragments;
- one local WAL per node for crash recovery and group commit;
- snapshot isolation as the default reader/writer contract, with SSI only when requested;
- safe-time watermarks for visibility GC;
- separate retention classes for history;
- explicit `read_consistency = strong | bounded_staleness(max_lag_ms)` routing;
- commutative update lanes beside, not inside, the general write-conflict path.

Removing any of those pieces pushes the cost back onto readers, hides staleness, or conflates concurrency cleanup with product retention.

### 4. Prior art reused patternwise

physa-db reuses the standard MVCC and SSI patterns for snapshot isolation and serializable upgrades, HLC-style timestamping for combining physical time with logical order, and the common database pattern of distinguishing local crash recovery journals from replication logs. The hotspot design borrows from escrow- or delta-style commutative updates and from read-replica mirror patterns, but keeps activation explicit and metric-gated.

## Consequences

**Positive**
- Readers never block writers on the common path.
- Single-shard writes avoid distributed commit overhead.
- Historical queries and follower reads share one timestamp vocabulary.
- Visibility GC, history retention, and cold movement can be tuned independently.
- Hotspot traffic gets distinct defenses for read skew and commutative write skew.

**Negative**
- HLC management, safe-time tracking, and SSI dependency checks add coordination complexity.
- Version fragments remain pinned when snapshots or legal-retention policies demand them.
- Operator surfaces for consistency level and hotspot metrics become part of the product contract.

## Open items

- HLC uncertainty budgets, safe-time lag thresholds, and commutative-lane admission rules remain sentinel-tracked constants to be validated in the Phase 6c benchmark-tracking issue once filed.
- Cross-shard 2PC batching strategy and recovery-path detail need implementation-level benchmark evidence, but they are no longer open architecture questions.
- Mirror-read activation thresholds remain metric-gated, not always-on defaults.

## FM coverage

- FM-005, FM-006: snapshot isolation default with serializable opt-in
- FM-007, FM-010: local WAL and crash recovery remain explicit
- FM-017, FM-110, FM-118: temporal history and embedding-version reads need one timestamp domain
- FM-111: reinforcement writes use commutative paths where declared safe
- FM-123: append-heavy observability ingest depends on the single-shard durable fast path

## References

- Bernstein and Goodman, "Multiversion Concurrency Control - Theory and Algorithms", TODS 1983.
- Cahill et al., "Serializable Isolation for Snapshot Databases", SIGMOD 2008.
- Kulkarni et al., "Logical Physical Clocks and Consistent Snapshots in Globally Distributed Databases", 2014.
- Diaconu et al., "Hekaton: SQL Server's Memory-Optimized OLTP Engine", SIGMOD 2013.

## Changelog

- 2026-04-25: Accepted with revisions per Campaign M1-Lock synthesis (formerly Proposed pending feature lock).
