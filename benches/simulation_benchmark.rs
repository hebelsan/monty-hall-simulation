use criterion::{black_box, criterion_group, criterion_main, Criterion};
use montyrs::{simulate_sequential, simulate_parallel};

fn criterion_benchmark(c: &mut Criterion) {
    let input = 1000000;
    c.bench_function("simple", |b| b.iter(|| simulate_sequential(black_box(input))));
    c.bench_function("parallel", |b| b.iter(|| simulate_parallel(black_box(input))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);