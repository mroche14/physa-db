# ADR-0002: Equal GQL and openCypher frontends over one graph-native IR

- **Status:** Accepted
- **Date:** 2026-04-25
- **Features addressed:** FM-001, FM-002, FM-017, FM-121, FM-122, FM-130
- **Workloads addressed:** W-A, W-B, W-C, W-E, W-F
- **Context issue:** _(to be filed as `type:feature area:query`)_

## Context

physa-db must satisfy both positioning pillars at once: migration-grade language parity and AI-agent-native retrieval, evidence, and temporal semantics. Campaign M1-Lock fixed the product boundary: GQL and openCypher remain equal public frontends, while deterministic agent surfaces sit above a shared graph-native plan substrate. The architecture must therefore preserve source-language fidelity, keep extensions explicit, and make tool traffic safe by default. The summary below records that locked decision.

## Decision

physa-db keeps two first-class query frontends and one execution substrate.

1. GQL and openCypher parse independently and lower into one shared graph-native logical IR. No dialect is translated through the other.
2. Non-standard features live only in the `PHYSA` extension namespace so that temporal, retrieval, and evidence features stay explicit in both dialects.
3. Deterministic agent surfaces are part of the query architecture: `schema.describe`, `schema.prune`, `retrieval.search`, `query.read`, `query.write`, and `evidence.render` compile to validated IR templates over the same engine as handwritten queries.
4. Agent profiles are read-only by default. Any write-capable profile requires an explicit capability grant scoped by tenant, namespace, role, and schema epoch.
5. Bolt, HTTP/JSON, gRPC, and MCP are transports over the same validation boundary. Transport choice must not change query meaning, safety policy, or evidence format.
6. Evidence and schema exports are tenant-scoped, namespace-scoped, and role-scoped. Cross-tenant schema statistics and evidence payloads are unreachable from this layer.

## First-principles derivation

### 1. Irreducible constraints

1. The commercial pillar requires native openCypher compatibility, while the standards pillar requires native GQL support. Treating one as a translator target for the other loses source positions, dialect-specific errors, and feature detection.
2. Agent traffic is safety-sensitive: the same tool call must mean the same thing every time. A stochastic text-to-query surface cannot be the engine's semantic boundary.
3. A single round trip from an agent tool call already costs an RTT; adding a second parsing or translation stage adds avoidable latency and an avoidable failure mode.
4. Temporal and retrieval operators such as `AS OF`, `NEAREST`, and evidence export must preserve snapshot and plan context across every transport.

### 2. Theoretical optimum

The optimum is one semantic core and multiple syntax frontends. In cost terms, each request should pay for:

- one parse in its source language;
- one lowering into one shared IR;
- one validation pass against one schema epoch;
- one execution plan.

Any design that parses openCypher, emits GQL text, then reparses pays at least two parse/validate passes for one query and still loses exact source semantics. Any design that lets tools send free-form natural language instead of validated profiles pays an unbounded error surface.

### 3. Smallest structure that realizes the optimum

The smallest structure is:

- one `LogicalPlan` substrate expressive enough for both GQL and openCypher;
- two dialect frontends with dialect-tagged error reporting;
- one explicit extension namespace, `PHYSA`, shared across both dialects;
- one profile validator that turns agent tool calls into pre-validated IR templates;
- one structural signature for caching and replay keyed by `(tenant_id, schema_epoch, plan_shape, time_class, result_mode)`.

This is enough to keep language parity, deterministic agent access, and transport neutrality without multiplying execution semantics.

### 4. Prior art reused patternwise

The design follows the common pattern used by mature database engines that accept multiple frontends but converge on one logical algebra. On the language side, the useful inputs are the ISO GQL standard and the openCypher specification. On the agent side, the useful pattern is capability-scoped RPC and tool profiles rather than free-form text translation. physa-db reuses those patterns but keeps the graph IR, temporal operators, and evidence surfaces graph-native from the start.

## Consequences

**Positive**
- Migration users get native openCypher and standards users get native GQL without a second system hidden underneath.
- Agent traffic becomes reproducible: every tool call validates against the same schema epoch and policy rules as direct queries.
- `PHYSA` extensions make non-standard retrieval and temporal features explicit instead of silently overloading standard syntax.
- Evidence export, streaming, and cancellation share one substrate across Bolt, HTTP, gRPC, and MCP.

**Negative**
- physa-db must maintain two parser/conformance suites instead of one.
- Dialect-specific semantics still need explicit documentation where the standards differ.
- Tool-profile governance becomes part of the query surface, not just client packaging.

## Open items

- Schema export size budgets, sample-size controls, and evidence artifact payload limits remain benchmark-gated sentinel settings to be validated in the Phase 6c benchmark-tracking issue once filed.
- The exact JSON and Markdown wire schema for evidence artifacts is accepted in principle here but still needs its own compatibility test corpus before client SDKs freeze.

## FM coverage

- FM-001: native GQL support
- FM-002: native openCypher compatibility
- FM-017: temporal syntax must surface cleanly in both frontends
- FM-121: JSON-LD and Markdown result shaping belong to the deterministic surface
- FM-122: MCP support is transport plus policy, not a separate query engine
- FM-130: cancellation must work identically for typed tools and handwritten queries

## References

- ISO/IEC 39075:2024, Information technology - Database languages - GQL.
- openCypher specification, https://opencypher.org/
- Francis et al., "Cypher: An Evolving Query Language for Property Graphs", SIGMOD 2018.
- Deutsch et al., "Graph Pattern Matching in GQL and SQL/PGQ", SIGMOD 2022.

## Changelog

- 2026-04-25: Accepted with revisions per Campaign M1-Lock synthesis (formerly Proposed pending feature lock).
