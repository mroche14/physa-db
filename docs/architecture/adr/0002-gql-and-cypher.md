# ADR-0002: physa-db supports GQL AND openCypher, no compromise

- **Status:** Proposed (pending M1 feature lock — see `AGENTS.md` §15)
- **Date:** 2026-04-20
- **Context issue:** _(to be filed as `type:feature area:query`)_

> **Note on status.** This ADR was initially drafted as *Accepted* on 2026-04-20, then downgraded to *Proposed* the same day when the project reformalised its features-first discipline (`AGENTS.md` §15, `docs/requirements/positioning.md`). The direction here — both dialects, shared IR — is very likely to survive M1 because it follows directly from the dual workload demand: Cypher for migrations from the incumbent, GQL for forward-compatibility and AI-agent-native extensions (`PHYSA` namespace for vector / temporal / hybrid operators in [`ai-agent-workloads.md`](../../requirements/ai-agent-workloads.md)). It will be promoted to *Accepted* when (a) the AI-native feature rows in `feature-matrix.md` are ratified and (b) the cross-dialect extension grammar is specified.

## Context

Two graph query languages matter in 2026:

- **openCypher** — de facto standard. Originated in Neo4j, now maintained as an open specification. Every serious graph DB either speaks it (Memgraph, RedisGraph/FalkorDB, AgensGraph) or bridges to it. The millions of lines of production Cypher out there define the migration market physa-db wants to capture.
- **GQL (ISO/IEC 39075:2024)** — the ISO-ratified standard graph query language, published April 2024. It is the long-term equilibrium: a single standard language that every compliant vendor must implement. Cypher is in effect a dialect that GQL generalised.

An earlier draft of this ADR framed this as a choice: "which one do we privilege?". The founder's explicit guidance rejects that framing:

> "Both, no compromise."

## Decision

physa-db implements **both GQL and openCypher** as first-class front-ends from day one of M2.

Architecturally:

1. A **shared logical plan IR** (`physa-query::plan::Logical`) is the single execution substrate.
2. Two parsers — `physa-query::parser::gql` and `physa-query::parser::cypher` — lower to that IR.
3. A **dialect tag** is attached to each query on ingress so error messages cite the source language.
4. **Semantic differences** (e.g. GQL's explicit graph pattern matching vs Cypher's pattern-match semantics in `OPTIONAL MATCH`) are handled in dialect-specific lowerings. Where a construct exists in only one language, the other simply rejects it at parse time with a clear error.
5. **Extensions** beyond the standards (e.g. vector search, time-travel) live in a `PHYSA` pseudo-namespace in both dialects to preserve portability.

## First-principles derivation

The irreducible constraint of a query layer is: **a query language is a surface, the plan IR is the substrate.** If we can map both surfaces to the same substrate faithfully, there is no inherent cost in supporting both — except engineering hours, which §12 of `AGENTS.md` grants.

A common failure mode in past projects (e.g. projects that tried to bolt Cypher onto an SQL engine) was a lossy IR that forced unnatural translations. We avoid it by designing the IR **natively for graphs**, expressive enough that GQL's `GRAPH_TABLE` and Cypher's `MATCH` both project cleanly.

## Consequences

**Positive**
- Full migration story from Neo4j (Cypher) AND from any GQL-compliant tool.
- Future-proof as Cypher stabilises within GQL (the two languages will converge over time — our IR absorbs that convergence at no cost).
- Dual-language test coverage catches IR bugs that a single dialect would miss.

**Negative**
- Two parsers, two sets of conformance tests, two documentation pipelines.
- Some semantic edge cases (null handling, list comprehension semantics) differ between dialects; we must document them explicitly.

Accepted trade-offs under `AGENTS.md` §12.

## Alternatives considered

- **Cypher-only at v1.0, GQL later** — rejected: founder's no-compromise directive. Also, GQL is the forward standard, and adding it later forces a retroactive IR redesign.
- **GQL-only, Cypher translator emitting GQL text** — rejected: translators strip semantics (error positions, feature detection), and users expect native Cypher error messages.

## References

- openCypher: https://opencypher.org/
- ISO/IEC 39075:2024 "Information technology — Database languages — GQL": https://www.iso.org/standard/76120.html
- Deutsch et al., *Graph Pattern Matching in GQL and SQL/PGQ*, SIGMOD 2022.
- Francis et al., *Cypher: An Evolving Query Language for Property Graphs*, SIGMOD 2018.
