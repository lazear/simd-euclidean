use criterion::*;
use rand::*;
use simd_euclidean::*;

fn bench_random(c: &mut Criterion) {
    let mut rng = thread_rng();
    let input_sizes = [1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024];

    let mut inputs = Vec::new();
    for &i in input_sizes.into_iter() {
        let mut a = Vec::with_capacity(i);
        let mut b = Vec::with_capacity(i);

        for _ in 0..i {
            a.push(rng.gen::<f32>());
        }
        for _ in 0..i {
            b.push(rng.gen::<f32>());
        }

        inputs.push((i, a, b))
    }

    c.bench(
        "Random inputs f32",
        ParameterizedBenchmark::new(
            "Naive",
            |b, i| b.iter(|| Naive::distance(black_box(&i.1), black_box(&i.2))),
            inputs,
        )
        .with_function("Vectorized", |b, i| {
            b.iter(|| Vectorized::distance(black_box(&i.1), black_box(&i.2)))
        })
        .throughput(|s| Throughput::Elements(s.0 as u32)),
    );
}

criterion_group!(benches, bench_random);
criterion_main!(benches);
