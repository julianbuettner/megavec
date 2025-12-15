use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use megavec::{Lz4, Megavec};
use rand_chacha::{
    rand_core::{RngCore, SeedableRng},
    ChaCha12Rng,
};

const BLOCK_SIZES: &[usize] = &[64, 128, 256, 512, 1024];
const SAMPLE_LEN: usize = 50_000;

fn generate_data() -> Vec<u64> {
    let mut rng = ChaCha12Rng::from_seed([42; 32]);
    (0..SAMPLE_LEN).map(|_| rng.next_u64()).collect()
}

fn bench_push(c: &mut Criterion) {
    let data = generate_data();
    let mut group = c.benchmark_group("push");

    for &block in BLOCK_SIZES {
        group.bench_function(format!("block_{block}"), |b| {
            b.iter_batched(
                || (Megavec::new(Lz4, block), data.clone()),
                |(mut megavec, data)| {
                    for value in data {
                        megavec.push(value);
                    }
                },
                BatchSize::SmallInput,
            );
        });
    }

    group.finish();
}

fn bench_pop(c: &mut Criterion) {
    let data = generate_data();
    let mut group = c.benchmark_group("pop");

    for &block in BLOCK_SIZES {
        group.bench_function(format!("block_{block}"), |b| {
            b.iter_batched(
                || {
                    let mut megavec = Megavec::new(Lz4, block);
                    for &value in data.iter() {
                        megavec.push(value);
                    }
                    megavec
                },
                |mut megavec| {
                    while megavec.pop().is_some() {}
                },
                BatchSize::SmallInput,
            );
        });
    }

    group.finish();
}

criterion_group!(megavec_benches, bench_push, bench_pop);
criterion_main!(megavec_benches);

