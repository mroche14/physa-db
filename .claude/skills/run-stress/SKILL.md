---
name: run-stress
description: >
  Run a physa-db stress scenario (smoke, chaos, soak, disk-full, oom,
  partition, clock-skew, poison, supernode) and report the invariant-checker
  verdict. Stress tests are the primary evidence that storage / MVCC / cluster
  changes don't regress under adversarial conditions. Mandatory evidence for
  any PR touching those subsystems (AGENTS.md §5).
when_to_use: >
  "run stress", "stress test", "chaos test", "verify under load", after
  landing storage / MVCC / cluster changes, before claiming a concurrency
  change is done.
argument-hint: "[scenario] [duration-override]"
user-invocable: true
allowed-tools:
  - Bash(just *)
  - Bash(cargo *)
  - Bash(tail *)
  - Bash(rg *)
  - Read
---

# run-stress — adversarial scenario runner

**Scenario:** $ARGUMENTS (defaults to `smoke` if empty).

Stress scenarios surface bugs that unit and integration tests miss: crash
recovery, soak-leak, hotspots, clock skew, network partitions. A scenario
run is a signal, not a ceremony — read the output carefully.

## Pick the right scenario

| Scenario | Duration | When to use |
|----------|----------|-------------|
| `smoke` | 60 s | CI gate; every PR touching storage or concurrency |
| `chaos` | ~5 min | Nightly; PR that changes recovery / WAL / crash path |
| `soak` | 24 h | Release-candidate only; long-run memory + fragmentation |
| `disk-full` | ~2 min | PR that changes on-disk format or space bookkeeping |
| `oom` | ~2 min | PR that changes buffer pools or in-memory caches |
| `partition` | ~5 min | PR that changes cluster behaviour |
| `clock-skew` | ~3 min | PR that touches MVCC timestamps or leases |
| `poison` | ~5 min | PR that introduces or changes error paths |
| `supernode` | ~10 min | PR that touches adjacency layout or traversal |

See [`tests/stress/README.md`](../../../tests/stress/README.md) for the
invariant list each scenario enforces.

## Run

```bash
just stress {{scenario}}
```

Under the hood this invokes the `physa-stress` subcommand of `physa-cli`
which owns the dataset, workload generator, and invariant checker for
the chosen scenario.

For long scenarios (`soak`, `supernode`), run in the background and
monitor:

```bash
just stress soak > /tmp/soak-$(date +%Y%m%d).log 2>&1 &
tail -f /tmp/soak-$(date +%Y%m%d).log
```

## Verify invariants

Every scenario runs a background invariant checker
([`tests/stress/README.md`](../../../tests/stress/README.md)). A scenario
**fails** if any invariant fires at any point. Specifically:

- **Storage:** no torn pages after recovery.
- **MVCC:** no lost updates; no read of committed data invisible at the
  snapshot ts.
- **Cluster:** no committed write lost on leader change.
- **Memory:** no unbounded growth during `soak`.

Look for a final block like:

```
==== SCENARIO: chaos ====
Duration: 312.04 s
Iterations: 54,821
Invariant failures: 0
Verdict: PASS
```

If a scenario reports any failure, collect:
- The scenario log (save to `/tmp/` or an issue attachment).
- The seed used (scenarios are deterministic given a seed — capture it
  so the human can reproduce).
- The last ~200 lines before the failure.
- The state of the working tree (`git rev-parse HEAD`, `git status`).

File an issue with labels `type:bug priority:p1 area:storage|cluster
status:needs-review`.

## Report format

Output:

```
## Stress run — {{scenario}}

- Started: <timestamp>
- Ended:   <timestamp>
- Duration: <seconds>
- Iterations: <N>
- Invariant failures: <N>
- Verdict: PASS | FAIL
- Log: <path>

### If FAIL
- First failing invariant: <name>
- Seed for reproduction: <int>
- Suspected cause (brief): ...
- Filed issue: #NNN
```

Paste this into the PR body under `## Stress` (required for any PR
touching concurrency-sensitive code, `AGENTS.md` §8).

## Before running

- Ensure the working tree is clean of uncommitted changes OR that you've
  noted the tree state (stress runs modify databases in `tests/stress/`
  working dirs; a dirty tree can confuse a later bisect).
- For long scenarios, confirm you have disk space (`df -h` ≥ 20 GB free
  for `soak`).
- For `supernode`, confirm RAM ≥ 32 GB; the scenario builds a 10M-degree
  hub.

## What NOT to do

- Do not report PASS without quoting the invariant checker line. A
  scenario that completed without "Verdict: PASS" printed has an
  uncaught failure.
- Do not pick `smoke` for a cluster change — it won't exercise the
  relevant code paths.
- Do not shorten a scenario below its documented duration to save time.
  A `chaos` run under 60 s is less informative than no run at all.
- Do not commit if any scenario you ran reports FAIL without also
  filing the issue and marking the PR `status:blocked`.
