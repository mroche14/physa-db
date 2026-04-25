# ADR-0007: Deterministic agent surfaces and evidence trace contract

- **Status:** Accepted
- **Date:** 2026-04-25
- **Features addressed:** FM-106, FM-119, FM-120, FM-121, FM-122, FM-130
- **Workloads addressed:** W-A, W-B, W-C, W-D, W-E, W-F
- **Context issue:** _(to be filed as `type:feature area:ai-native`)_

## Context

Campaign M1-Lock fixed the boundary between the core database and agent-facing integration. physa-db must expose deterministic schema, retrieval, and evidence surfaces in-core, but it must not turn model-specific query generation or provider-hosted completion into mandatory engine behavior. That boundary is larger than the query dialect choice in ADR-0002 and smaller than a full in-core agent runtime. This ADR captures that cross-cutting contract.

## Decision

physa-db adopts transport-neutral, deterministic agent surfaces with a first-class evidence trace contract.

1. MCP is transport plus policy, not semantics. The same agent surface must be available over MCP, HTTP/gRPC, and direct driver calls.
2. The supported public tool surface is explicit: `schema.describe`, `schema.prune`, `retrieval.search`, `query.read`, `query.write`, and `evidence.render`.
3. Every tool call validates against `(tenant_id, namespace_id, role_id, schema_epoch)` before it becomes an executable plan.
4. Read-only is the default profile posture. Write-capable profiles require explicit grants and bounded target scopes.
5. GraphRAG-style retrieval is a bridge to the planner, not a sidecar service: semantic retrieval, graph expansion, result shaping, and evidence emission operate over one shared identity space and one plan.
6. The evidence artifact is first-class output. It may include contributing node and edge IDs, scores, pruning reasons, snapshot timestamps, partition IDs, read path, and policy hits, but it must never contain chain-of-thought text.
7. Tool surfaces expose sample-size, token-budget, and freshness-budget controls explicitly; hidden heuristics are not part of the contract.

## First-principles derivation

### 1. Irreducible constraints

1. Agent workloads need deterministic schema and evidence surfaces to stay reproducible under retries, audits, and policy review.
2. Retrieval output must fit a token budget, so result shaping belongs in the same boundary as retrieval and evidence, not in an opaque client shim.
3. Hidden write capability is unsafe. The default public posture must therefore be read-only, with explicit capability elevation for writes.
4. A tool call that goes through one transport today and another tomorrow must still produce the same semantic result under the same schema epoch.

### 2. Theoretical optimum

The optimum is one transport-neutral, validated tool contract that compiles directly to the same planner and executor used by handwritten queries. That means one validation pass, one plan, one evidence schema, and one policy surface. Any design that leaves retrieval or evidence as a sidecar service pays avoidable serialization cost and creates semantic drift between agent calls and direct database queries.

### 3. Smallest structure that realizes the optimum

The smallest structure is:

- one tool-profile schema with explicit scopes and safety modes;
- one schema exporter that emits tenant-scoped, role-scoped summaries;
- one retrieval bridge into the optimizer;
- one evidence artifact schema shared across transports;
- one read-only default with explicit write grants.

This is enough to make agent interaction deterministic without pushing provider-specific model execution into the database core.

### 4. Prior art reused patternwise

physa-db reuses the general pattern of capability-scoped tool APIs, transport-neutral RPC contracts, and provenance-rich retrieval traces. The useful lesson is that transport alone is too thin; stable schema, safety, and evidence surfaces must be part of the product contract. The database keeps those deterministic surfaces in-core and leaves stochastic translation and model hosting outboard.

## Consequences

**Positive**
- Agent integrations become auditable and reproducible.
- Retrieval, token shaping, freshness, and evidence operate over the same plan and identity space.
- Public safety posture is clear: read-only by default, explicit writes only.
- Client SDKs can target one stable contract across transports.

**Negative**
- physa-db must version and test tool schemas and evidence payloads as public interfaces.
- Schema and evidence export budgets become part of compatibility review.
- Client authors cannot assume that free-form text-to-query is supported as an engine primitive.

## Open items

- Evidence payload size caps, sample-size defaults, and token-budget heuristics remain benchmark-gated sentinel settings for the Phase 6c benchmark-tracking issue once filed.
- Client SDK ergonomics for the read-only/write-capable profile split remain packaging work, not open architecture.

## FM coverage

- FM-106: retrieval must enter the planner as a first-class operation
- FM-119, FM-120: streaming and token-budget-aware shaping are surface-level behaviors
- FM-121: JSON-LD and Markdown rendering are part of the deterministic output contract
- FM-122: MCP support is transport plus policy over this surface
- FM-130: cancellation is part of the same tool contract

## References

- Model Context Protocol specification.
- Cormack, Clarke, and Buettcher, "Reciprocal Rank Fusion Outperforms Condorcet and Individual Rank Learning Methods", SIGIR 2009.

## Changelog

- 2026-04-25: Accepted as part of the Campaign M1-Lock ADR expansion.
