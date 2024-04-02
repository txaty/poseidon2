use criterion::{black_box, criterion_group, criterion_main, Criterion};
use zkhash::{
    fields::bn256::FpBN256,
    poseidon2::{poseidon2::Poseidon2, poseidon2_instance_bn256::POSEIDON2_BN256_PARAMS},
};

type Scalar = FpBN256;

fn poseidon2_bn256(c: &mut Criterion) {
    let poseidon = Poseidon2::new(&POSEIDON2_BN256_PARAMS);
    let t = poseidon.get_t();
    let input: Vec<Scalar> = (0..t).map(|i| Scalar::from(i as u64)).collect();

    c.bench_function("bn256_different_input_size", move |bench| {
        bench.iter(|| {
            let perm = poseidon.permutation(black_box(&input));
            black_box(perm)
        });
    });
}

fn criterion_benchmark_poseidon2(c: &mut Criterion) {
    poseidon2_bn256(c);
}

criterion_group!(benches, criterion_benchmark_poseidon2);
criterion_main!(benches);
