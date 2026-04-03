use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

use nat_set::ResourceSet;
use nat_set::hash_set_set::HashSetSet;
use nat_set::interval_set::IntervalSet;
use nat_set::bit_set_set::BitSetSet;

// Données de test réutilisables
fn  small() -> Vec<u32> { (0..10).collect() }
fn medium() -> Vec<u32> { (0..1_000).collect() }
// fn  large() -> Vec<u32> { (0..100_000).collect() }

// Benchmark générique sur n'importe quel ResourceSet
fn bench_impl<S: ResourceSet>(c: &mut Criterion, group_name: &str) {
    let mut g = c.benchmark_group(group_name);

    for (label, data) in [
        ("small",   small()),
        ("medium", medium()),
        // ("large",   large()),
    ] {
        let a = S::from_iter(data.iter().copied());
        let b = S::from_iter(data.iter().step_by(2).copied()); // sous-ensemble

        // from_iter
        g.bench_with_input(BenchmarkId::new("from_iter", label), &data, |bench, d| {
            bench.iter(|| S::from_iter(black_box(d.iter().copied())))
        });

        // contains
        g.bench_with_input(BenchmarkId::new("contains", label), &data, |bench, d| {
            bench.iter(|| d.iter().all(|&x| black_box(a.contains(x))))
        });

        // union
        g.bench_with_input(BenchmarkId::new("union", label), &(), |bench, _| {
            bench.iter(|| black_box(a.union(&b)))
        });

        // intersection
        g.bench_with_input(BenchmarkId::new("intersection", label), &(), |bench, _| {
            bench.iter(|| black_box(a.intersection(&b)))
        });

        // difference
        g.bench_with_input(BenchmarkId::new("difference", label), &(), |bench, _| {
            bench.iter(|| black_box(a.difference(&b)))
        });
    }

    g.finish();
}

fn benchmarks(c: &mut Criterion) {
    bench_impl::<HashSetSet>  (c, "HashSetSet"  );
    bench_impl::<IntervalSet> (c, "IntervalSet" );
    bench_impl::<BitSetSet>   (c, "BitSetSet"   );
}

criterion_group!(benches, benchmarks);
criterion_main!(benches);