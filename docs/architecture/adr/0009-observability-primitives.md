# ADR-0009: Observability primitives as a first-class event and evidence fabric

- **Status:** Accepted
- **Date:** 2026-04-25
- **Features addressed:** FM-030, FM-031, FM-032, FM-123, FM-124, FM-125, FM-127
- **Workloads addressed:** W-E, W-F
- **Context issue:** _(to be filed as `type:feature area:ops`)_

## Context

Agent observability is a locked workload family with throughput and retention demands that do not fit ordinary OLTP graph pages. Campaign M1-Lock also fixed several cross-cutting requirements that belong with this workload: per-tenant cost accounting, query-plan capture, agent-trace spans, and sentinel metrics for the accepted J-defaults. The engine therefore needs observability primitives as a first-class product surface rather than a thin export layer bolted onto general storage.

## Decision

A query plan is the operator tree chosen by the planner and executed by the runtime. physa-db adopts a first-class observability fabric with an append-only event store, causal trace graph, and structured evidence metrics.

1. Observability ingest uses tenant-scoped, time-partitioned append segments with transactional linkage to causal edges and payload records.
2. Log payloads use structured JSON-LD values. Payload storage and causal-edge storage may be physically separate, but they are query-linked by stable event IDs.
3. physa-db captures query plans, operator timings, policy hits, read path, and cancellation cause as first-class observability events when the caller asks for them or policy requires them.
4. Per-tenant cost accounting tracks CPU, memory, I/O, network egress, storage bytes, and cold-tier occupancy across query, retrieval, and ingest paths.
5. Sentinel metrics for the accepted J-defaults are mandatory: blob-tier write amplification and replay time, temporal pushdown overhead, ANN skew and mirror lag, hotspot-lane effectiveness, and follower-read lag and stale-hit rate.
6. Ingest compatibility is append-first and streaming-friendly. OTLP-compatible export/import is part of the architecture, not a best-effort adapter.

## First-principles derivation

### 1. Irreducible constraints

1. W-E targets `100k events/s` per tenant. At `1 KiB` per event before indexes, that is about `100 MiB/s` of raw ingest, so random in-place update paths are physically wrong.
2. Operators debug incidents by time range first and causal graph second. A design that starts with whole-graph traversal pays the wrong lower bound.
3. Query-plan and policy evidence matter only if they are captured in the same system that executes the plan; sidecar logging cannot guarantee semantic completeness.
4. Observability load must not starve transactional workloads. Cost accounting and admission control are therefore core architecture, not dashboards.

### 2. Theoretical optimum

The optimum is:

- sequential append on ingest;
- partition pruning by `(tenant, time_range, event_type)` before graph traversal;
- cheap reconstruction of causal paths through stable event IDs;
- one evidence stream that can explain what the planner and runtime actually did;
- explicit per-tenant budgets so one noisy tenant degrades inside its own envelope first.

Any design that stores observability purely as ordinary graph rows or exports it only after the fact pays too much write amplification and loses the causal evidence needed for debugging.

### 3. Smallest structure that realizes the optimum

The smallest structure is:

- per-tenant time partitions for events;
- append-first ingest journaling;
- stable event IDs linking payloads, causal edges, query plans, and evidence artifacts;
- per-tenant accounting counters and budgets;
- a sentinel-metric plane that reports whether the accepted architecture is still holding its assumptions.

That is enough to support query-time evidence, production debugging, and benchmark validation without turning the database into an external observability stack clone.

### 4. Prior art reused patternwise

physa-db reuses the broad industry pattern of append-only telemetry ingestion, time partitioning, structured log payloads, and trace spans. The novel requirement is keeping those observability artifacts under the same tenant, plan, evidence, and temporal semantics as the graph and retrieval engine itself.

## Consequences

**Positive**
- W-E gets a storage class and evidence surface designed for its actual write path.
- Query plans and runtime evidence become reproducible artifacts instead of ad hoc logs.
- Sentinel metrics create direct feedback loops for the accepted J-defaults.
- Per-tenant accounting and retention become enforceable rather than advisory.

**Negative**
- The product now owns an observability schema and retention policy surface.
- Capturing full evidence and plan details can increase storage and wire volume when enabled.
- Compression, indexing, and cold-tier movement policies need their own benchmark tuning.

## Open items

- Compression codec choices, asynchronous summary lag limits, and plan-capture default sampling rates remain benchmark-gated sentinel constants for the Phase 6c benchmark-tracking issue once filed.
- The exact OTLP mapping for plan and evidence events needs compatibility testing before SDK surfaces freeze.

## FM coverage

- FM-030, FM-031, FM-032: plan explanation, auditability, and OTLP-aligned telemetry
- FM-123, FM-124, FM-125: high-throughput streaming ingest, time-partitioned storage, and JSON-LD payloads
- FM-127: per-tenant audit and redaction posture applies to observability payloads too

## References

- OpenTelemetry protocol specification.
- Sadoghi et al., "L-Store: A Real-time OLTP and OLAP System", EDBT 2018.

## Changelog

- 2026-04-25: Accepted as part of the Campaign M1-Lock ADR expansion.
