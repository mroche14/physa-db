# Performance targets

> Testable assertions. Every target is verified by `just bench-macro` against a known dataset and hardware profile.

## Reference hardware

Unless stated otherwise, numbers target a single x86_64 node with:
- 16 physical cores (Zen 4 or Sapphire Rapids class), SMT enabled
- 64 GB DDR5 ECC
- 3.84 TB PCIe Gen4 NVMe (read ≥ 6 GB/s, 500k IOPS)
- Linux 6.x, ext4 or xfs

Cluster targets add: 3× single-node spec, 25 Gb/s interconnect, same datacentre.

## LDBC SNB Interactive (transactional)

Scale factor, query, target p95 latency (cold / warm cache):

| SF | IC-1 | IC-2 | IC-3 | IC-4 | IC-5 | IC-6 | IC-7 | IC-8 | IC-9 | IC-10 | IC-11 | IC-12 | IC-13 | IC-14 |
|----|------|------|------|------|------|------|------|------|------|-------|-------|-------|-------|-------|
| SF1 | TBD | TBD | TBD | TBD | TBD | TBD | TBD | TBD | TBD | TBD | TBD | TBD | TBD | TBD |
| SF10 | TBD | TBD | TBD | TBD | TBD | TBD | TBD | TBD | TBD | TBD | TBD | TBD | TBD | TBD |
| SF100 | TBD | TBD | TBD | TBD | TBD | TBD | TBD | TBD | TBD | TBD | TBD | TBD | TBD | TBD |

Filled during M1 based on first-principles derivation (what's the theoretical minimum given cache-line reads + NVMe latency for each query shape?).

## LDBC SNB BI (analytical)

Target throughput queries / minute on SF100, warm cache: TBD per query.

## Cold start & recovery

- **Cold start to first query** on a 1 TB graph: TBD (target: single-digit seconds).
- **Cold start memory footprint**: < 100 MB baseline (no JVM bloat).
- **Crash recovery** after kill -9 with WAL intact: TBD (target: sub-second on SF10).
- **Catchup** of a fresh follower from a 1 TB leader: TBD (target: saturate the network link).

## Write throughput

- **Single-node bulk load**: TBD GB/s sustained.
- **Single-node transactional writes (small tx)**: TBD tx/s at p99 < 10ms.
- **Multi-node writes (3-node cluster, single-region)**: TBD tx/s.

## Horizontal scale efficiency

On a read-dominant workload, speedup vs single-node:
- 3 nodes: ≥ 2.7× (90% efficiency target)
- 10 nodes: ≥ 8× (80% efficiency)
- 100 nodes: ≥ 60× (60% efficiency)

## Regression budget

CI fails any PR that regresses any cell in this document by > **2%** without an accompanying ADR justifying the trade-off.

## Methodology

All numbers are:
1. Median of ≥ 10 runs after warm-up.
2. Produced by `just bench-macro` and checked in under `docs/benchmarks/results/YYYY-MM-DD-*.md`.
3. Reproducible from commit SHA on the reference hardware.
ardware.
