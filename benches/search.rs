//! Index benchmarks

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use interpolation_search::{InterpolationFactor, InterpolationSearch};
use rand::{distr::Uniform, rngs::StdRng, Rng, SeedableRng};

fn bench_search(c: &mut Criterion) {
    bench_search_type(c, "Vec<u32>", |x| x);
    bench_search_type(c, "Vec<u32>, expensive Ord", ExpensiveOrd);
    bench_search_type(c, "Vec<u32>, expensive factor", ExpensiveFactor);
    bench_search_type(
        c,
        "Vec<u32>, expensive Ord and factor",
        ExpensiveOrdAndFactor,
    );
}

fn bench_search_type<T: Ord + InterpolationFactor>(
    c: &mut Criterion,
    desc: &str,
    mut mapper: impl FnMut(u32) -> T,
) {
    let mut group = c.benchmark_group(desc);

    for i in (1..5).map(|n| 100_usize.pow(n)) {
        let (vec, target) = create_sample(i, &mut mapper);
        let _ = group.bench_function(BenchmarkId::new("binary_search", i), |b| {
            b.iter(|| {
                _ = vec.binary_search(&target);
            });
        });
        let _ = group.bench_function(BenchmarkId::new("interpolation_search", i), |b| {
            b.iter(|| {
                _ = vec.interpolation_search(&target);
            });
        });
    }
}

fn create_sample<T: Ord>(count: usize, mut mapper: impl FnMut(u32) -> T) -> (Vec<T>, T) {
    let mut rng = StdRng::seed_from_u64(5);

    let uniform = Uniform::try_from(0..100_000_000).unwrap();
    let target = mapper(rng.sample(uniform));
    let mut vec: Vec<T> = rng.sample_iter(uniform).take(count).map(mapper).collect();
    vec.sort_unstable();
    (vec, target)
}

#[derive(PartialEq, Eq)]
struct ExpensiveOrd(u32);

impl PartialOrd for ExpensiveOrd {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ExpensiveOrd {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let pow_self: f64 = 1.01_f64.powf(self.0 as f64);
        let pow_other: f64 = 1.01_f64.powf(other.0 as f64);
        pow_self.total_cmp(&pow_other)
    }
}

impl InterpolationFactor for ExpensiveOrd {
    fn interpolation_factor(&self, a: &Self, b: &Self) -> f32 {
        self.0.interpolation_factor(&a.0, &b.0)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct ExpensiveFactor(u32);
impl InterpolationFactor for ExpensiveFactor {
    fn interpolation_factor(&self, a: &Self, b: &Self) -> f32 {
        let pow_self = 1.01_f64.powf(self.0 as f64) as u64;
        let pow_a = 1.01_f64.powf(a.0 as f64) as u64;
        let pow_b = 1.01_f64.powf(b.0 as f64) as u64;
        self.0.interpolation_factor(&a.0, &b.0)
            + 0.0000001 * pow_self.interpolation_factor(&pow_a, &pow_b)
    }
}

#[derive(PartialEq, Eq)]
struct ExpensiveOrdAndFactor(u32);

impl PartialOrd for ExpensiveOrdAndFactor {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ExpensiveOrdAndFactor {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        ExpensiveOrd(self.0).cmp(&ExpensiveOrd(other.0))
    }
}

impl InterpolationFactor for ExpensiveOrdAndFactor {
    fn interpolation_factor(&self, a: &Self, b: &Self) -> f32 {
        ExpensiveFactor(self.0).interpolation_factor(&ExpensiveFactor(a.0), &ExpensiveFactor(b.0))
    }
}
criterion_group!(benches, bench_search);
criterion_main!(benches);
