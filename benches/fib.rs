use algs::int::fibonacci;
use algs::string::atoi;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

fn criterion_benchmark_2(c: &mut Criterion) {
    c.bench_function("atoi", |b| {
        b.iter(|| atoi(black_box("4456478".to_string())))
    });
}

criterion_group!(benches, criterion_benchmark, criterion_benchmark_2);
criterion_main!(benches);
