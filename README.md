# physa-db

> **An open-source, Rust-native graph database purpose-built for AI-agent workloads — vector search, multi-hop retrieval, knowledge graphs, agent memory, media assets — with full GQL/openCypher compatibility and native multi-tenancy.**

**Status:** Pre-alpha. Architecture phase. Not usable yet.

## Why physa-db?

### The technical reason — AI-agent-native, by design

Agentic AI systems in 2026 produce workloads that no 2010-era graph DB was designed for: retrieval-augmented generation blending vectors and graph hops in one query, long-term agent memory with TTL and forgetting semantics, knowledge graphs with provenance and confidence, multi-modal asset stores, agent-trace observability, temporal reasoning. Today the "solution" is to chain a vector DB + a graph DB + a blob store + an orchestration layer. That stack is brittle, slow, and expensive.

physa-db is a single engine that does those workloads natively. See [`docs/requirements/positioning.md`](./docs/requirements/positioning.md) and [`docs/requirements/ai-agent-workloads.md`](./docs/requirements/ai-agent-workloads.md) for specifics.

### The commercial reason — end the pricing era

The graph database market is captive. The incumbent's licensing model makes it impossible to build a modern SaaS on top of it the way you'd build one on Postgres. OSS alternatives are either abandoned, non-Cypher, single-node, or slower. physa-db is Apache-2.0 end-to-end, with multi-tenancy and horizontal scaling natively — no enterprise-gated features. See [`docs/requirements/positioning.md`](./docs/requirements/positioning.md) §1.

## Two pillars, one database

| Pillar | Promise |
|--------|---------|
| **AI-agent-native** | Dense + sparse vectors as first-class property types, HNSW / IVF-PQ indices, hybrid (vector + graph + BM25) query plans, MCP server, media blob storage with chunk hierarchy, embedding model versioning, `AS OF` temporal queries, agent-observability ingest, streaming results, LLM-shaped output (JSON-LD + token-budget-aware truncation). |
| **Graph-DB parity** | Full **GQL (ISO/IEC 39075:2024) AND openCypher**, Bolt v5 wire protocol for Neo4j-driver compatibility, ACID transactions with MVCC snapshot isolation, horizontal scaling, online re-sharding, Apache-2.0, no enterprise tier. |

The AI-native features win *new* workloads that never had a good graph DB answer. The parity features win *migrations from* the incumbent. They compound.

## Development philosophy

This project is **AI-agent-first** in its development workflow, too. Documentation, tooling, and issue structure are designed so that AI coding agents (Claude Code, Codex, Cursor, etc.) can pick up well-scoped issues and ship PRs with minimal human intervention. Humans own vision, review, and merges; agents own implementation velocity.

See [`AGENTS.md`](./AGENTS.md) for the full agent contract, including the engineering discipline rules (§11 first-principles, §12 no-shortcuts, §15 features-before-architecture).

## Live project dashboard

Development progress (features in flight, benchmarks, open issues) is tracked on a public GitHub Pages dashboard: **(URL pending first deploy)**

## Quick links

- [`docs/requirements/positioning.md`](./docs/requirements/positioning.md) — positioning: commercial pillar (§1) + AI-agent-native technical pillar (§§2–7)
- [`docs/requirements/ai-agent-workloads.md`](./docs/requirements/ai-agent-workloads.md) — authoritative source of the AI-agent workloads that drive every feature
- [`docs/requirements/feature-matrix.md`](./docs/requirements/feature-matrix.md) — public feature list with tier and ADR links
- [`docs/requirements/non-goals.md`](./docs/requirements/non-goals.md) — what physa-db explicitly is NOT
- [`AGENTS.md`](./AGENTS.md) — instructions for AI agents
- [`ROADMAP.md`](./ROADMAP.md) — milestones
- [`docs/architecture/`](./docs/architecture/) — design docs & ADRs

## License

Apache-2.0.
