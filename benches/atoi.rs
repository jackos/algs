use algs::string::atoi;
use algs::string::atoi_alt::{a1, a2};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

fn bench_fibs(c: &mut Criterion) {
    let mut group = c.benchmark_group("atoi comparison");
    for i in [
        "0".to_string(),
        "123456".to_string(),
        "12345674567456789456789".to_string(),
    ]
    .iter()
    {
        group.bench_with_input(BenchmarkId::new("My implementation", i), i, |b, i| {
            b.iter(|| atoi(i.to_string()))
        });
        group.bench_with_input(BenchmarkId::new("Leetcode solution", i), i, |b, i| {
            b.iter(|| a2(i.to_string()))
        });
    }
    group.finish();
}

criterion_group!(benches, bench_fibs);
criterion_main!(benches);
