# Performance targets

> Public performance contract for Campaign M1-Lock. Each target is tied to a workload family and to machine class `M`; rows that still lack a public first-principles bound stay explicitly benchmark-blocked instead of inventing numbers.

## Machine class M

Unless stated otherwise, all single-node targets assume:
- 16 physical x86_64 cores
- 128 GiB RAM
- PCIe Gen4 NVMe
- Linux 6.x

Cluster targets add a 3-node deployment of the same machine class inside one datacentre.

## Workload contract

| Workload | Operation | Target on machine class `M` | Bound source |
|----------|-----------|-----------------------------|--------------|
| W-A | Fact append with provenance and embedding reference on the single-shard path | `>= 75k facts/s`; `p95 commit <= 2 ms` | Phase 3 first-principles optimum: append-first fact storage plus commutative deltas on the common path |
| W-A | Top-20 semantic recall plus 2-hop recency / confidence expansion | `p95 <= 35 ms warm`; `p95 <= 90 ms cold` | Phase 3 theoretical bound from W-A constraints 1-5: segment-local ANN shortlist plus bounded graph expansion |
| W-B | Hybrid retrieval (`ANN + BM25 + <= 3 hops + token shaping`) | `p95 <= 60 ms warm`; `p95 <= 150 ms cold` | Phase 3 theoretical bound from W-B constraints 1-6: one optimizer-owned plan with bounded candidate inflation |
| W-B | First streamed evidence batch | First 5 results in `<= 20 ms warm` | Phase 3 theoretical bound from W-B constraint 4: stream before full materialization |
| W-C | 5-edge cyclic knowledge-graph pattern on a `100M`-edge snapshot | `p95 <= 150 ms warm` | Phase 3 theoretical bound from W-C constraint 1: WCOJ / factorized execution on cyclic plans |
| W-C | Large graph-algorithm throughput on `1B`-edge PageRank / SSSP / Louvain runs | `TBD (benchmark BNCH-001 required)` | Public corpus does not justify an honest edge/s number yet; Phase 6c must set the bound from benchmark evidence |
| W-D | Manifest commit for a `1 GiB` asset | `p95 <= 15 ms` for manifest commit; stage visible in `<= 250 ms` | Phase 3 theoretical bound from W-D constraints 1-6: manifest commit decoupled from heavy transforms |
| W-D | Local `blob-log` ingest throughput | `>= 600 MB/s` sustained local ingest | Phase 3 first-principles bound: sequential append with checksum / compression on local NVMe |
| W-E | OTLP ingest for `1 KiB` events batched by 256 | `100k events/s per tenant`; `p95 ACK <= 8 ms` | Locked workload target from W-E plus Phase 3 append-first event-store contract |
| W-E | Session trace reconstruction for a `10k`-event workflow | `p95 <= 120 ms` | Phase 3 theoretical bound from W-E constraints 1-6: prune by tenant and time before causal traversal |
| W-F | `AS OF` overhead on point lookup and hybrid retrieval | `<= 25%` over the equivalent non-temporal query | Phase 3 theoretical bound from W-F constraints 1-3: time pushdown must avoid late-filter blow-up |
| W-F | Historical scan cost over a `24h` window | `<= 1.5x` the equivalent live-scan cost | Phase 3 theoretical bound from W-F constraints 2-4: lineage / delta reconstruction instead of copy-on-write snapshots |

## Benchmark-blocked target

- `BNCH-001` — establish the honest throughput envelope for `1B`-edge PageRank, SSSP, and Louvain on machine class `M`, including edge/s and memory envelope without out-of-memory failure.

## Regression policy

- Any benchmarked row above becomes a regression gate once the corresponding harness lands.
- Benchmark-blocked rows do not gate CI until their `BNCH-NNN` issue is resolved with reproducible numbers.

## Methodology

1. Warm targets mean the relevant local segments and indexes are already resident on the serving node.
2. Cold targets mean the same logical path after cache misses or tier fetches force storage reads on the critical path.
3. Stored results must be reproducible from commit SHA on machine class `M` and checked in under `docs/benchmarks/results/YYYY-MM-DD-*.md` once the harness exists.

## Glossary

- `ANN` (approximate nearest neighbor): vector search that returns near candidates quickly without exact full-collection scan.
- `AS OF`: point-in-time read against bi-temporal history rather than only the latest committed state.
- `BM25` (Best Matching 25): lexical ranking function used for full-text retrieval.
- `Cold`: query path where cache misses or tier fetches force storage reads during execution.
- `Warm`: query path where the relevant local segments and indexes are already resident on the serving node.
