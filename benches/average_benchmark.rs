use criterion::{criterion_group, criterion_main, Criterion};

use xrs::statistics::average;

fn benchmark_average(c: &mut Criterion) {
    let numbers: Vec<f64> = (1..=1000).map(|x| x as f64).collect();
    c.bench_function("average", |b| b.iter(|| average(&numbers)));
}

criterion_group!(benches, benchmark_average);
criterion_main!(benches);
