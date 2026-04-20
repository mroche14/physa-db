//! Criterion wall-time smoke bench.
//!
//! Scaffolding only: ensures the `bench-nightly` pipeline has something to
//! measure before real storage/query benchmarks land. Replace with real
//! benches as `physa-core` grows; do not delete the file — the CI workflow
//! expects at least one criterion target.

use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};

fn smoke_noop(c: &mut Criterion) {
    c.bench_function("smoke_noop", |b| b.iter(|| black_box(0_u64)));
}

fn smoke_sum_1024(c: &mut Criterion) {
    c.bench_function("smoke_sum_1024", |b| {
        b.iter(|| {
            let mut acc: u64 = 0;
            for i in 0..1024_u64 {
                acc = acc.wrapping_add(black_box(i));
            }
            acc
        });
    });
}

criterion_group!(smoke, smoke_noop, smoke_sum_1024);
criterion_main!(smoke);
