# ADR-0005: Sharding — fully abstracted at the user surface, graph-aware internally

- **Status:** Proposed (direction pending M1 feature lock — see `AGENTS.md` §15)
- **Date:** 2026-04-20
- **Context issue:** _(to be filed as `type:feature area:cluster priority:p1`)_

> **Note on status.** Downgraded from *Accepted* on 2026-04-20 under the features-first rule. The shard-transparent surface is likely to survive, but the *internal* sharding scheme must now account for AI-agent workloads ([`ai-agent-workloads.md`](../../requirements/ai-agent-workloads.md)): vector ANN indices that span shards (a query must fan-out and merge top-K), blob storage that may live entirely off-shard in an object store, agent-observability time-partitioned data whose ideal layout is temporal rather than graph-cut. Promotion to *Accepted* is gated on the partitioner ADR showing these shapes do not regress (or, if they do, explicitly documenting the trade).

## Context

Horizontal scaling requires splitting the graph across many nodes. The user-facing question the founder raised was:

> "Can the complexity be fully abstracted?"

Answer: **yes at the user surface, no at the planner and operator surface**. Nobody should write a different query just because the graph is sharded; but the query planner and the operator must be shard-aware to produce good plans and healthy clusters. This ADR commits to that layering.

## Decision

1. **User-facing API is shard-transparent.** Any GQL or Cypher query that works on a single-node deployment works identically on a 1000-shard cluster, with identical semantics. No new syntax. No "shard key" hints in queries.
2. **Query planner is shard-aware.** It pushes down predicates to shards, minimises cross-shard hops, uses bloom filters for fan-out reduction, and caches shard-affinity statistics.
3. **Sharding scheme is graph-aware and adaptive.** Initial placement uses **streaming edge-cut partitioning** (LDG / Fennel style) to keep neighbourhoods co-located. An online re-balancer migrates partitions when hotspots emerge. Random hash partitioning is available as a fallback mode for workloads that don't benefit from locality (e.g. pure random-access OLTP).
4. **Metadata via Raft, data via sharded consensus groups.** Metadata (schema, tenants, partition map) lives in a Raft group. Each data partition lives in its own Raft group (`multi-Raft`). Cross-partition transactions use two-phase commit over the partition groups when strong consistency is required; eventual consistency with bounded staleness is offered as an opt-in for read-scale.
5. **Tenants as a sharding axis.** A tenant can be pinned to a single shard (cheapest, strongest locality) or spread across shards (needed beyond a scale threshold). Tenant migration is online and automatic.

## First-principles derivation

The irreducible costs of a distributed graph query:

- **Cross-node hop** costs ≈ 100 μs within a datacentre (RDMA / 25 Gb/s) to 50 ms cross-region.
- **A 3-hop traversal on a badly partitioned graph** becomes 3 × 100 μs = 300 μs of wait; the same traversal if co-located = tens of microseconds.
- **Broadcast to N shards** costs N × log-factor on the bandwidth bus and wastes (N − k) / N of work when only k shards matter.

Therefore: the sharding algorithm must maximise *edge cuts minimised* (neighbourhoods together) and the planner must minimise *broadcast fan-out*. Hash partitioning is fine when queries are per-key; it is terrible for traversal-heavy workloads (the dominant case).

Users don't know the partition map and shouldn't need to — that's the abstraction. The planner does know it, and the operator can inspect/override it.

## Consequences

**Positive**
- Users write the same query regardless of scale.
- Performance stays close to the theoretical cross-node-bounded minimum on real workloads.
- Tenant-as-shard is the simple case; horizontal scaling within a big tenant is the hard case, handled by the same machinery.

**Negative**
- Graph partitioning is NP-hard; streaming approximations have worst-case pathological layouts. Mitigation: online re-balancer.
- `multi-Raft` is complex to operate (many consensus groups). Tooling investment required (dashboards, per-group health, leader balancing).
- Cross-shard transactions cost a two-phase commit round-trip. We document this cost and offer per-transaction `AFFINITY` hints when the application *knows* everything is co-located.

Accepted under `AGENTS.md` §§11, 12.

## What stays out of the user surface

- No `CREATE SHARD`, `USE SHARD`, or sharding syntax in GQL/Cypher.
- No manual rebalancing command — admins can *suggest* moves; they cannot fracture invariants.
- No partition key in the data model (unlike Cassandra's `PRIMARY KEY`).

## Alternatives considered

- **Hash partitioning only.** Rejected: fails on traversal locality.
- **User-specified partition keys.** Rejected: violates "fully abstracted" directive.
- **Share-everything (Aurora-style shared storage).** Interesting, but compute-storage separation adds its own latency floor and capex assumptions. Deferred to a possible future ADR; not in the critical path.
- **Gossip-based eventual consistency for data.** Rejected as the default: correctness-first project (§1.3). Offered as an opt-in for stale-read workloads (FM-024).

## Open sub-ADRs (to be written under M4)

- ADR-0012: streaming partitioner algorithm choice (LDG vs Fennel vs METIS-stream).
- ADR-0013: re-balancer trigger heuristics and cost model.
- ADR-0014: two-phase commit protocol for cross-partition transactions.
- ADR-0015: tenant affinity and migration strategy.

## References

- Stanton & Kliot, *Streaming Graph Partitioning for Large Distributed Graphs*, KDD 2012 (LDG).
- Tsourakakis et al., *FENNEL: Streaming Graph Partitioning for Massive Scale Graphs*, WSDM 2014.
- Ongaro & Ousterhout, *In Search of an Understandable Consensus Algorithm* (Raft), ATC 2014.
- Huang et al., *TiKV: A Distributed Transactional Key-Value Database*, VLDB 2020 (multi-Raft).
