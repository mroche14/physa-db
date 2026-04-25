# ADR-0005: Tenant-first hybrid partition classes with explicit replica roles

- **Status:** Accepted
- **Date:** 2026-04-25
- **Features addressed:** FM-021, FM-022, FM-023, FM-024, FM-025, FM-026, FM-027, FM-123, FM-124, FM-126
- **Workloads addressed:** W-A, W-B, W-D, W-E, W-F
- **Context issue:** _(to be filed as `type:feature area:cluster priority:p1`)_

## Context

physa-db still promises shard-transparent user semantics, but Campaign M1-Lock showed that one implicit "graph-aware partitioner" is not enough. Graph topology, ANN locality, time-partitioned observability, and blob manifests have different optimal partition axes. The accepted cluster decision therefore keeps a single logical namespace while allowing multiple physical partition classes under one tenant-first routing model. This is the minimum structure that preserves locality without hiding staleness or cross-tenant interference.

## Decision

physa-db replaces the old single-partitioner model with tenant-first hybrid partition classes and explicit replica roles.

1. Tenant is the first routing key. By default, one tenant owns its own partition set; small tenants stay pinned and large tenants scale out inside their own partition space.
2. Partition classes are explicit: `graph`, `ann`, `event_time`, and `blob_manifest`. Each class can choose placement and rebalance policy according to its access path.
3. Graph partitions keep graph-aware placement, chunk-aware hot splitting, and mirror hooks for read-heavy supernodes.
4. ANN placement defaults to graph-home-shard co-location. Hot ANN subsets may be mirrored on evidence, and fully independent ANN placement is an exception for large-tenant memory pressure rather than the planner default.
5. Replica roles are explicit: `voter`, `learner`, and `witness`. Strong reads route to the authoritative replica; follower reads require explicit bounded-staleness opt-in.
6. The policy hook for partition-class assignment is part of the public cluster architecture:

```rust
pub trait PartitionClassPolicy {
    fn assign_class(
        &self,
        tenant: TenantId,
        object: &PlacementObject,
        metrics: &SentinelMetrics,
    ) -> PartitionClass;
}
```

`PlacementObject` carries object family, model/version scope, time range, and blob-manifest hints; `SentinelMetrics` carries skew, fanout, lag, and budget pressure needed for class-aware routing.

## First-principles derivation

### 1. Irreducible constraints

1. An extra cross-partition handoff in the common ANN-to-graph retrieval path adds a network RTT and a second failure point to nearly every W-A and W-B query.
2. Time-range observability queries first prune by tenant and time, not by graph cut. Using only graph edge-cut logic for W-E forces the wrong partition shape.
3. Large tenants can create ANN memory skew that is not visible in graph-edge counts alone.
4. Tenant quotas and noisy-neighbor defenses lose force if multiple tenants are mixed into the same physical hot path by default.
5. Follower reads are useful for scaling reads, but stale semantics must stay explicit and measurable.

### 2. Theoretical optimum

The optimum is one logical namespace and multiple physical classes. The common path should keep ANN and first graph expansion local, event scans should prune by time partition before graph work, and blob manifests should stay near metadata-hot reads without dragging large payload placement into the same rebalance policy. Replica placement should expose cheap read scale without silently changing consistency semantics.

Any "one partitioner fits all" design pays the wrong lower bound for at least one class: graph cuts are wrong for event scans, independent ANN everywhere is wrong for hybrid retrieval, and mixed-tenant hot partitions are wrong for isolation.

### 3. Smallest structure that realizes the optimum

The smallest structure is:

- tenant-first routing;
- four physical partition classes;
- class-aware replica roles;
- explicit bounded-staleness follower reads;
- one policy trait that assigns placement class from object shape and live metrics.

That is enough to keep user semantics shard-transparent while allowing the storage substrate to use the right partition shape for each access path.

### 4. Prior art reused patternwise

physa-db reuses established patterns from distributed databases: consensus groups for metadata and partitions, learner and witness style replicas, graph-aware edge-cut placement where topology is the primary access path, and mirror-based read offload for read-skewed hot ranges. The novelty is not any one ingredient; it is applying those ingredients by partition class under a tenant-first contract.

## Consequences

**Positive**
- Hybrid retrieval keeps its common ANN-to-graph path local.
- Observability partitions can optimize for append and time pruning without corrupting graph placement.
- Tenant isolation and quota enforcement stay load-bearing at the routing layer.
- Replica roles and follower-read semantics become explicit and auditable.

**Negative**
- Placement, rebalance, and routing logic become class-aware instead of uniform.
- Operators must monitor more metrics: skew, lag, cross-partition fanout, and tenant budget pressure.
- ANN mirrors and hot splits add maintenance traffic when activated.

## Open items

- Mirror activation, hot-split thresholds, and the exceptional path for fully independent ANN placement remain benchmark-gated sentinel settings for the Phase 6c benchmark-tracking issue once filed.
- Cross-region policies for FM-024 and FM-025 remain future cluster work on top of this partition-class foundation.
- The default size at which a pinned tenant graduates to a multi-partition tenant remains an operational constant, not an architecture gap.

## FM coverage

- FM-021, FM-022, FM-023: metadata consensus, transparent scale-out, and online re-sharding
- FM-024, FM-025: replica roles are the basis for future regional semantics
- FM-026, FM-027: tenant-first routing and resource boundaries
- FM-123, FM-124: event-time partitions and cold-tier movement
- FM-126: tenant-local ANN roots and follower-read visibility rules

## References

- Ongaro and Ousterhout, "In Search of an Understandable Consensus Algorithm", 2014.
- Corbett et al., "Spanner: Google's Globally-Distributed Database", OSDI 2012.
- Malkov and Yashunin, "Efficient and Robust Approximate Nearest Neighbor Search Using Hierarchical Navigable Small World Graphs", TPAMI 2018.

## Changelog

- 2026-04-25: Accepted with revisions per Campaign M1-Lock synthesis (formerly Proposed pending feature lock).
