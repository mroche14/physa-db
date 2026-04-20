# Stress tests

> Long-running, adversarial tests designed to surface bugs that unit/integration tests miss.

## Scenarios (to be implemented starting M2)

| Scenario | What it does | Signal |
|----------|--------------|--------|
| `smoke` | 60 s of mixed reads/writes. Fails fast on obvious regressions. | quick gate |
| `chaos` | Kills the daemon at random intervals during a write workload. Verifies WAL recovery. | crash safety |
| `soak` | 24 h of steady-state mixed workload. | resource leaks, fragmentation |
| `disk-full` | Fills the disk mid-write. Verifies graceful degradation and recovery. | out-of-space handling |
| `oom` | Restricts memory via cgroups. Verifies bounded memory usage. | memory pressure |
| `partition` | Simulates network partitions during cluster operations. | split-brain safety |
| `clock-skew` | Injects clock drift on cluster nodes. Verifies time-related invariants (MVCC watermarks). | timestamp correctness |
| `poison` | Injects fault-injection hooks to force error paths. | error path coverage |
| `supernode` | Builds a graph with a 10M-degree supernode; runs traversals. | hotspot handling |

## Running

```bash
just stress smoke        # CI gate
just stress chaos        # nightly
just stress soak         # release candidates only
```

## Harness

Implemented in `physa-cli` as the `physa-stress` subcommand. Each scenario is self-contained: it owns its dataset, its workload generator, its invariant checkers. No scenario may share state with another.

## Invariant checkers

Every scenario runs a background invariant checker:
- Storage: no torn pages after recovery.
- MVCC: no lost updates, no read of committed data that should be invisible at snapshot ts.
- Cluster: no committed write lost on leader change.

A scenario **fails** if any invariant fires at any point.
