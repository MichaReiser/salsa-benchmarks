use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use new_salsa::TextDatabase;

fn compare(c: &mut Criterion) {
    let mut group = c.benchmark_group("compare");
    let mut db = TextDatabase::default();
    for n in &[10, 100] {
        group.bench_function(BenchmarkId::new("new_salsa", n), |b| {
            let text = std::iter::repeat("A").take(*n).collect::<String>();
            b.iter(|| new_salsa::run(&mut db, text.clone()))
        });
    }

    group.finish();
}

criterion_group!(benches, compare);
criterion_main!(benches);
