use criterion::{criterion_group, criterion_main, Criterion};
use rand::{RngCore, SeedableRng};

fn random_values(seed: u64, n: usize) -> impl Iterator<Item = u32> {
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);

    std::iter::from_fn(move || Some(rng.next_u32())).take(n)
}

fn range_set(input: impl Iterator<Item = u32>) {
    let mut set = range_set::Set::default();
    input.for_each(|x| {
        set.insert(x);
    });
}

fn ranged_set(input: impl Iterator<Item = u32>) {
    let mut set = ranged_set::RangedSet::new();
    input.for_each(|x| {
        set.insert(x);
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("RangeSet Comparison");

    let seed = 12345;

    for n in [1000, 10_000, 100_000, 1_000_000] {
        group.bench_with_input("ranged-set", &n, |b, &n| {
            b.iter_batched(
                || random_values(seed, n),
                |i| {
                    ranged_set(i);
                },
                criterion::BatchSize::SmallInput,
            )
        });
        group.bench_with_input("range-set", &n, |b, &n| {
            b.iter_batched(
                || random_values(seed, n),
                |i| {
                    range_set(i);
                },
                criterion::BatchSize::SmallInput,
            )
        });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
