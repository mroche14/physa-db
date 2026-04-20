# Architecture

This directory holds the living design documentation for physa-db.

## Structure

- `adr/` — numbered Architecture Decision Records. Immutable once accepted; supersede instead of edit.
- `diagrams/` — Mermaid / D2 / SVG architecture diagrams.
- `unsafe-allowlist.md` — justified list of crates allowed to use `unsafe` code.
- Subsystem docs (one file per major component): `storage.md`, `query.md`, `cluster.md`, `wire-protocol.md`, `multi-tenancy.md`, `observability.md`.

## ADR lifecycle

1. **Draft** a new ADR as `adr/NNNN-kebab-title.md` using the template below.
2. Status starts as **Proposed**.
3. Discussion happens on the related GitHub issue (link it).
4. Once merged, status becomes **Accepted**.
5. If a later ADR invalidates this one, mark this one **Superseded by ADR-NNNN** — do NOT edit the body.

## ADR template

```markdown
# ADR-NNNN: <title>

- **Status:** Proposed | Accepted | Superseded by ADR-XXXX
- **Date:** YYYY-MM-DD
- **Context issue:** #NNN

## Context
What is the problem, the constraints, the forces at play?

## Decision
What did we decide? State it clearly, in the present tense.

## Consequences
Positive and negative. What gets easier? What gets harder?

## Alternatives considered
Each alternative, with a one-line reason we rejected it.

## References
Papers, blog posts, competitor implementations.
```
