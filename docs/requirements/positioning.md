# Positioning — what physa-db is for

> The technical target. Paired with `initial-vision.md` (the commercial motivation).

## One-liner

**physa-db is an AI-agent-native graph database.**

It is a property graph engine whose primary workloads are those produced by agentic AI systems: retrieval-augmented generation, long-term agent memory, knowledge graphs, multi-modal asset organization, and agent-trace observability.

## Evolution of the vision

The founder's [`initial-vision.md`](../../initial-vision.md) (immutable) captured the **commercial** motivation: end the Neo4j pricing era so SaaS builders can use a graph DB as freely as Postgres. That remains true.

This file captures the **technical** target established on 2026-04-20: AI-agent workloads drive the feature set. Existing graph databases (Neo4j, Memgraph, Dgraph, TigerGraph, etc.) were designed in the pre-agent era; their storage layouts, query planners, and data types predate the workloads AI agents actually produce. We do not intend to be a faster re-implementation of yesterday's graph DB.

The two pillars compound:

| Motivation | Pillar |
|------------|--------|
| Commercial — the reason the project exists | End Neo4j lock-in; free, horizontally scalable, multi-tenant, Apache-2.0 |
| Technical — the reason a user would pick us | AI-agent-native features natively: vectors, hybrid retrieval, media assets, agent memory, provenance, streaming results |

## What "AI-agent-native" concretely means

We commit to being best-in-class at the workloads below. Specifics live in [`ai-agent-workloads.md`](./ai-agent-workloads.md), the authoritative feature source.

1. **Agent memory.** Episodic (what did the agent see) and semantic (what does the agent know) memory at the scale of millions of traces per tenant. TTL and forgetting semantics built in.
2. **Retrieval-augmented generation (RAG).** Hybrid retrieval that combines vector similarity and graph traversal in a single query plan, not two. Multi-hop retrieval optimised for the RAG access pattern.
3. **Knowledge graphs.** Entity resolution, relation extraction integration, provenance tracking, confidence scores, open-world assumption support.
4. **Multi-modal assets.** Native storage of blobs (images, audio, video, PDFs) with automatic chunking, deduplication, embedding hook points, and asset↔chunk↔embedding relationships.
5. **Agent observability.** Every tool call, thought, and output can be a node; every causal link an edge. High-throughput streaming ingest.
6. **Temporal reasoning.** As-of queries so an agent can reason about what it knew at time T.
7. **LLM-shaped outputs.** Results format-aware (JSON-LD, Markdown summaries) and token-budget-aware (truncation, ranking) for direct feed into a context window.

## What stays from the generic graph DB promise

- Full GQL (ISO/IEC 39075:2024) and openCypher support (ADR-0002).
- Bolt v5 wire protocol for Neo4j-driver compatibility.
- ACID transactions with MVCC snapshot isolation.
- Native multi-tenancy, horizontal scaling, online re-sharding.
- Apache-2.0 end-to-end, no enterprise-gated features.

These are necessary but not sufficient. They win migrations *from* Neo4j; AI-agent features win *new* workloads that never had a good graph DB answer.

## Non-overlap with other categories

- We are **graph+vector**, not a vector-only DB (use pgvector / Qdrant / LanceDB if you only want vectors).
- We are a **graph DB**, not a document store (use MongoDB / Postgres JSONB).
- We are a **property store with blob support**, not a blob store (use S3 / R2 for petabyte cold storage; physa-db stores references or hot blobs).

See [`non-goals.md`](./non-goals.md) for the full list.

## Why this positioning, why now

- Agentic AI is producing workloads that no 2010-era graph DB was designed for.
- The existing "solution" is to chain a vector DB + a graph DB + a blob store + an orchestration layer. That stack is brittle, slow, and expensive.
- First-principles: if the workloads want a unified substrate, build one.
- No incumbent has the freedom to redesign from scratch. We do.

## Compatibility with `AGENTS.md` §§11–12

The decision to build a unified AI-agent substrate is EXACTLY the kind of higher-complexity path §12 commits us to. It is also a first-principles derivation per §11: we look at the irreducible cost of agent queries (vector similarity + graph traversal + blob access) and conclude the theoretical optimum is a single engine, not three.

## Compatibility with `AGENTS.md` §15

Every subsequent architectural ADR must cite the FM-NNN features in `feature-matrix.md` it addresses. Architecture follows features; features follow workloads; workloads follow positioning (this document).
