# ADR-0010: Multi-tenancy isolation through tenant-local budgets, keys, and scheduling

- **Status:** Accepted
- **Date:** 2026-04-25
- **Features addressed:** FM-011, FM-026, FM-027, FM-028, FM-029, FM-041, FM-042, FM-043, FM-124, FM-126, FM-127
- **Workloads addressed:** W-A, W-B, W-C, W-D, W-E, W-F
- **Context issue:** _(to be filed as `type:feature area:tenancy`)_

## Context

Native multi-tenancy is part of the commercial pillar and also a hard technical requirement for agent workloads. Campaign M1-Lock made the scope sharper: isolation must apply not only to namespaces but also to secondary indexes, blob quotas, retention windows, encryption keys, follower reads, and noisy-neighbor control. That scope is cross-cutting enough to require its own ADR. Without it, "multi-tenant" would describe only naming, not real resource isolation.

## Decision

physa-db adopts tenant-local isolation as a first-class control plane across storage, query, and cluster paths.

1. Every tenant has explicit budgets for CPU, memory, I/O, storage bytes, ANN roots, blob occupancy, ingest rate, and retained-history bytes.
2. Encryption at rest uses tenant-local key material and key-version metadata. Key rotation and audit remain per tenant, not global best effort.
3. Query governance is explicit: concurrency ceilings, scan budgets, token budgets, timeout envelopes, and cancellation policies resolve by tenant and role before execution starts.
4. Admission control and fair scheduling apply across partition classes so a heavy observability or asset-ingest tenant degrades inside its own budget before it steals capacity from others.
5. Secondary access paths are tenant-local by construction: ANN, full-text, blob dedupe domains, retention classes, and evidence payloads do not cross tenant boundaries.
6. Backup, restore, and retention schedules remain tenant-scoped so one tenant's long retention horizon does not pin another tenant's data or keys.

## First-principles derivation

### 1. Irreducible constraints

1. W-E alone can target `100 MiB/s` of raw event ingest per tenant at `100k events/s` with `1 KiB` events. One tenant can therefore consume a meaningful fraction of a node unless isolation is real.
2. ANN, blob, and cold-history footprints grow by different slopes than graph topology. Namespace-only isolation does not constrain those paths.
3. Encryption, redaction, and audited reads are meaningless if key domains or audit scopes cross tenants by accident.
4. A noisy neighbor problem is a scheduler problem before it is a dashboard problem. If the runtime has no tenant-local budgets, it cannot enforce fairness later.

### 2. Theoretical optimum

The optimum is for every major cost center to resolve through tenant-local policy before work begins. That means:

- budget check before execution;
- tenant-local keys before persistence;
- tenant-local index roots before indexing;
- tenant-local retention and backup policy before cold movement or deletion.

Any design that isolates only namespaces and leaves indexes, budgets, or keys global pays the wrong lower bound on interference and auditability.

### 3. Smallest structure that realizes the optimum

The smallest structure is:

- one tenant governor that owns budgets, admission control, and fair scheduling;
- tenant-local key metadata for encrypted roots;
- tenant-local secondary roots and dedupe domains;
- tenant-local retention and backup control;
- tenant-scoped evidence and audit output.

That is the minimum structure that turns "multi-tenant" into a real runtime guarantee instead of a naming convention.

### 4. Prior art reused patternwise

physa-db reuses the common patterns of per-tenant resource governors, hierarchical fair scheduling, tenant-local encryption domains, and per-tenant audit trails. The important lesson is that those mechanisms must cover every secondary path, not just the primary row store. The accepted design therefore applies isolation to vector roots, blobs, retention, and evidence as well as to query namespaces.

## Consequences

**Positive**
- Tenant isolation becomes enforceable across all major hot paths.
- Noisy-neighbor defense is architectural rather than operational folklore.
- Encryption, auditing, and backup boundaries align with the product's tenant model.
- Future managed-service features have a clean runtime substrate.

**Negative**
- The tenant governor becomes a critical control-plane module with broad blast radius if wrong.
- Operators must reason about more budgets and scheduling metrics than in a namespace-only design.
- Some global pooling efficiency is intentionally traded away for isolation.

## Open items

- Default budget sizes, fairness weights, and spill thresholds remain benchmark-gated sentinel constants for the Phase 6c benchmark-tracking issue once filed.
- Cross-region backup-key handling and hosted-service control-plane integration remain future work on top of this isolation model.

## FM coverage

- FM-011: per-tenant encryption at rest
- FM-026, FM-027, FM-028, FM-029: native namespaces, quotas, RBAC, and backup schedules
- FM-041, FM-042: tenant-aware memory and spill behavior
- FM-043: external auth still resolves to tenant-local policy
- FM-124, FM-126, FM-127: tenant-local cold history, vector isolation, and PII handling

## References

- Zaharia et al., "Delay Scheduling: A Simple Technique for Achieving Locality and Fairness in Cluster Scheduling", EuroSys 2010.
- Gulati et al., "PARDA: Proportional Allocation of Resources for Distributed Storage Access", FAST 2009.

## Changelog

- 2026-04-25: Accepted as part of the Campaign M1-Lock ADR expansion.
