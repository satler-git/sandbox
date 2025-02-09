use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::seq::SliceRandom;
use rayon::slice::ParallelSliceMut as _;

fn generate_data(len: usize) -> Vec<usize> {
    let mut rng = rand::rng();
    let mut nums: Vec<usize> = (1..len).collect();
    nums.shuffle(&mut rng);
    nums
}

fn bm1(c: &mut Criterion) {
    let data = generate_data(20000);

    let mut group = c.benchmark_group("sort_comparison");
    group.measurement_time(std::time::Duration::from_secs(5));

    group.bench_function("std::sort", |b| {
        b.iter(|| {
            let mut v = data.clone();
            v.sort();
            black_box(v);
        })
    });

    group.bench_function("rayon::par_sort", |b| {
        b.iter(|| {
            let mut v = data.clone();
            v.par_sort();
            black_box(v);
        })
    });

    group.finish();
}

criterion_group!(benches, bm1);
criterion_main!(benches);
