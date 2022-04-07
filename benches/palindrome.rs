use algs::int::palindrome;
use algs::int::palindrome_fast;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

fn bench_fibs(c: &mut Criterion) {
    let mut group = c.benchmark_group("palindrome comparison");
    for i in [
        101u64,
        125521u64,
        15645645678964561645u64,
        11111111111111111111u64,
    ]
    .iter()
    {
        group.bench_with_input(BenchmarkId::new("My implementation", i), i, |b, i| {
            b.iter(|| palindrome(*i))
        });
        group.bench_with_input(BenchmarkId::new("Leetcode solution", i), i, |b, i| {
            b.iter(|| palindrome_fast(*i))
        });
    }
    group.finish();
}

criterion_group!(benches, bench_fibs);
criterion_main!(benches);
