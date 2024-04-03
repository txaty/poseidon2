use criterion::{black_box, Criterion, criterion_group, criterion_main};

use zkhash::{
    fields::goldilocks::FpGoldiLocks,
    poseidon2::{poseidon2::Poseidon2, poseidon2_instance_goldilocks::{POSEIDON2_GOLDILOCKS_12_PARAMS, POSEIDON2_GOLDILOCKS_16_PARAMS, POSEIDON2_GOLDILOCKS_20_PARAMS, POSEIDON2_GOLDILOCKS_8_PARAMS}},
};

type Scalar = FpGoldiLocks;

fn poseidon2_goldilocks_8(c: &mut Criterion) {
    let poseidon = Poseidon2::new(&POSEIDON2_GOLDILOCKS_8_PARAMS);
    let t = poseidon.get_t();
    let input: Vec<Scalar> = (0..t).map(|i| Scalar::from(i as u64)).collect();

    c.bench_function("goldilocks_8", move |bench| {
        bench.iter(|| {
            let perm = poseidon.permutation(black_box(&input));
            black_box(perm)
        });
    });
}

fn poseidon2_goldilocks_12(c: &mut Criterion) {
    let poseidon = Poseidon2::new(&POSEIDON2_GOLDILOCKS_12_PARAMS);
    let t = poseidon.get_t();
    let input: Vec<Scalar> = (0..t).map(|i| Scalar::from(i as u64)).collect();

    c.bench_function("goldilocks_12", move |bench| {
        bench.iter(|| {
            let perm = poseidon.permutation(black_box(&input));
            black_box(perm)
        });
    });
}

fn poseidon2_goldilocks_16(c: &mut Criterion) {
    let poseidon = Poseidon2::new(&POSEIDON2_GOLDILOCKS_16_PARAMS);
    let t = poseidon.get_t();
    let input: Vec<Scalar> = (0..t).map(|i| Scalar::from(i as u64)).collect();

    c.bench_function("goldilocks_16", move |bench| {
        bench.iter(|| {
            let perm = poseidon.permutation(black_box(&input));
            black_box(perm)
        });
    });
}

fn poseidon2_goldilocks_20(c: &mut Criterion) {
    let poseidon = Poseidon2::new(&POSEIDON2_GOLDILOCKS_20_PARAMS);
    let t = poseidon.get_t();
    let input: Vec<Scalar> = (0..t).map(|i| Scalar::from(i as u64)).collect();

    c.bench_function("goldilocks_20", move |bench| {
        bench.iter(|| {
            let perm = poseidon.permutation(black_box(&input));
            black_box(perm)
        });
    });
}

fn criterion_benchmark_poseidon2(c: &mut Criterion) {
    poseidon2_goldilocks_8(c);
    poseidon2_goldilocks_12(c);
    poseidon2_goldilocks_16(c);
    poseidon2_goldilocks_20(c);
}

criterion_group!(benches, criterion_benchmark_poseidon2);
criterion_main!(benches);
