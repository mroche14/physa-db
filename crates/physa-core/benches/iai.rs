//! iai-callgrind instruction-count smoke bench.
//!
//! This is the regression oracle for the `bench-regression` PR gate
//! (AGENTS.md §5). Instruction counts are deterministic across runs on
//! the same binary — the gate fires when `delta > 2%`.
//!
//! Scaffolding only; grow the list of tracked benches as `physa-core`
//! adds measurable code paths.

use iai_callgrind::{library_benchmark, library_benchmark_group, main};

#[library_benchmark]
fn smoke_noop() -> u64 {
    std::hint::black_box(0_u64)
}

#[library_benchmark]
fn smoke_sum_1024() -> u64 {
    let mut acc: u64 = 0;
    for i in 0..1024_u64 {
        acc = acc.wrapping_add(std::hint::black_box(i));
    }
    acc
}

library_benchmark_group!(
    name = smoke;
    benchmarks = smoke_noop, smoke_sum_1024
);

main!(library_benchmark_groups = smoke);
