//! Read parquet benchmark.
use criterion::{criterion_group, criterion_main, Criterion};

use arbeider::indicator::returns::Returns;

fn read_returns(path: &str) -> Returns {
    Returns::from_parquet(path)
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("read returns", |b| {
        b.iter(|| read_returns("examples/data/600.parquet"))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);