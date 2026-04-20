---
name: run-bench
description: >
  Run physa-db benchmarks (criterion wall-time, iai-callgrind instruction-stable,
  or macro LDBC SNB / SNAP) and report numerical deltas against a stored
  baseline. The only legitimate source of "faster" claims. Required evidence
  for any PR labelled type:perf (AGENTS.md §§1.2, 8).
when_to_use: >
  "benchmark", "bench", "how fast", "perf test", before claiming a performance
  win, before merging a type:perf PR, when checking for regression.
argument-hint: "[micro|iai|macro] [baseline]"
user-invocable: true
allowed-tools:
  - Bash(just *)
  - Bash(cargo *)
  - Bash(git *)
  - Bash(lscpu)
  - Bash(uname *)
  - Bash(cat *)
  - Bash(rg *)
  - Read
---

# run-bench — reproducible performance measurement

**Mode and baseline:** $ARGUMENTS (defaults: `micro main`).

"Faster" without numbers is meaningless (`AGENTS.md` §1.2). Every perf
claim ships with the exact command, hardware, dataset, before/after
numbers, and std dev. This skill is the one legitimate path.

## Pick the mode

| Mode | Tool | What it measures | Stability |
|------|------|------------------|-----------|
| `micro` | `cargo criterion` | Wall-time for small functions | Hardware-dependent; noisy under load |
| `iai` | `cargo bench --bench iai` | Instruction count (Valgrind / iai-callgrind) | Instruction-stable; safe for CI gates |
| `macro` | `just bench-macro` | End-to-end LDBC SNB / SNAP queries | Dataset-dependent; run on fixed SF |

Use `iai` for CI regression gates. Use `micro` for local exploration.
Use `macro` for user-facing latency claims.

## 1 — Capture hardware context

```bash
echo "== Hardware =="
uname -a
lscpu | rg 'Model name|CPU\(s\)|MHz|Cache'
cat /proc/meminfo | head -n 3
cat /sys/block/nvme0n1/queue/rotational 2>/dev/null || echo "no nvme0n1"
```

Always include this block in the PR body. Benchmarks without hardware
context are irreproducible.

## 2 — Save a baseline (first run on a new machine)

```bash
# From main, uncontaminated by your PR
git switch main
just bench-save main
git switch -
```

`main` becomes the reference baseline. Re-save whenever `main`
advances significantly (major perf-landing PR).

## 3 — Run your benchmark

### Micro (criterion)

```bash
just bench
# -- or --
just bench-compare main   # also diffs against the main baseline
```

Criterion prints, per benchmark:

```
physa_core::node::lookup time:   [42.103 ns 42.234 ns 42.389 ns]
                         change: [-1.23% +0.12% +1.48%] (p = 0.85 > 0.05)
                         No change in performance detected.
```

The `[lo median hi]` triple is the 95 % confidence interval. The
`change` triple is the delta vs baseline.

### Instruction-stable (iai-callgrind)

```bash
just bench-iai
```

iai prints integer instruction counts. Zero variance run-to-run on
the same binary — use this for regression gates in CI.

### Macro (LDBC SNB / SNAP)

```bash
just bench-macro 1    # Scale Factor 1 (default)
just bench-macro 10   # SF10 for latency claims
```

Outputs per-query latency percentiles (p50 / p95 / p99) and aggregate
throughput.

## 4 — Interpret & decide

A change is a **regression** if:

- criterion / iai delta exceeds +2 % on any tracked bench
  (`AGENTS.md` §5 bench-regression rule), AND
- the confidence interval excludes zero (p < 0.05).

A change is a **win** if:

- delta is ≤ −X % where X is the improvement you claim,
- p < 0.05,
- iai confirms the direction (instructions drop too).

Report every tracked bench, even if it didn't change — silence on a
bench is suspicious.

## 5 — Report (paste into PR body under `## Benchmarks`)

```markdown
## Benchmarks

### Hardware
- CPU: <model>, N cores
- RAM: <GB>
- Storage: <NVMe model / ext4 | zfs | …>
- Kernel: <uname -r>

### Command
```bash
just bench-compare main
```

### Baseline
- main @ <commit sha>

### Results
| Benchmark | Before (median) | After (median) | Δ | p | Verdict |
|-----------|-----------------|----------------|---|---|---------|
| <name>    | 42.1 ns         | 38.2 ns        | −9.3 % | 0.002 | WIN |
| <name>    | 1.23 μs         | 1.24 μs        | +0.8 % | 0.42  | no change |
| …         | …               | …              | …      | …     | …       |

### iai confirmation (instruction counts)
| Benchmark | Before | After | Δ |
|-----------|--------|-------|---|
| <name>    | 1,242  | 1,117 | −10.0 % |

### Notes
<explain anomalies, e.g. "TLB-miss regime change around 10^5 entries">
```

## 6 — If you are checking for regression (not claiming a win)

Run `just bench-compare main` and verify no bench regresses > 2 %. If
any does, either fix or document the trade explicitly (a trade-off
that moves a slow path to a fast path is OK; a silent slow-down is
not).

## Before running

- Quiet the machine: close browsers, disable aggressive background
  jobs. `turbostat` fluctuations show up as 5–10 % noise.
- Pin governor: `sudo cpupower frequency-set -g performance` (or note
  the governor you ran under).
- Run at least 3 times. Report the median.

## What NOT to do

- Do not report a single run. Criterion needs ≥ 30 iterations for
  statistical validity; run its defaults.
- Do not claim "2x faster" based on a micro-bench alone. Back it with
  an iai-callgrind improvement AND a macro-bench improvement.
- Do not compare runs from different machines. Re-baseline on the
  target machine first.
- Do not skip the hardware block. A bench report without hardware
  context is unreviewable.
- Do not save a baseline from a tree that fails `just ci`. A baseline
  must be from green code.
