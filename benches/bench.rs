use ark_std::test_rng;
use criterion::{criterion_group, criterion_main, Criterion};
use ff::Field;
use halo2_proofs::halo2curves::bn256::Fr;

const SIZE: usize = 65536;

fn bench_fr_ops(c: &mut Criterion) {
    let mut rng = test_rng();
    let a = (0..SIZE).map(|_| Fr::random(&mut rng)).collect::<Vec<_>>();
    let b = (0..SIZE).map(|_| Fr::random(&mut rng)).collect::<Vec<_>>();

    c.bench_function("BN254 Fr Add", |bencher| {
        bencher.iter(|| {
            a.iter()
                .zip(b.iter())
                .map(|(&a, &b)| a + b)
                .collect::<Vec<_>>()
        })
    });

    c.bench_function("BN254 Fr Mul", |bencher| {
        bencher.iter(|| {
            a.iter()
                .zip(b.iter())
                .map(|(&a, &b)| a * b)
                .collect::<Vec<_>>()
        })
    });
}

criterion_group!(benches, bench_fr_ops);
criterion_main!(benches);
