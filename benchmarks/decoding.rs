use std::num::NonZeroUsize;

use criterion::BenchmarkGroup;
use pccc::{Bit, DecodingAlgo, Interleaver};
use rand::{Rng, SeedableRng};

criterion::criterion_main!(benches);
criterion::criterion_group!(benches, decode_f32, decode_f64);

fn decode<F: num_traits::Float>(mut group: BenchmarkGroup<'_, criterion::measurement::WallTime>) {
    let rng = rand::rngs::StdRng::seed_from_u64(0);

    // Test the speeds at various block sizes
    for block_size in [64, 128, 256, 512, 1024, 2048, 4096] {
        let random_data = rng
            .clone()
            .sample_iter::<bool, _>(rand::distr::StandardUniform)
            .take(block_size)
            .map(Bit::from)
            .collect::<Vec<_>>();

        let interleaver = Interleaver::random(NonZeroUsize::new(block_size).unwrap());
        let code_polynomials = [0o13, 0o15];

        // Encode the random data and convert it into "soft bits" which are simply +-1
        let code_bits_llr = pccc::encoder(&random_data, &interleaver, &code_polynomials)
            .unwrap()
            .into_iter()
            .map(|x| F::from(bool::from(x) as i8 * -2 + 1).unwrap())
            .collect::<Vec<_>>();

        // Sanity check to make sure it's working
        let bits_out = pccc::decoder(
            &code_bits_llr,
            &interleaver,
            &code_polynomials,
            DecodingAlgo::LinearLogMAP(8),
        )
        .unwrap();
        assert_eq!(&bits_out, &random_data);

        // Benchmark the decoding speeds with this float type
        group.bench_function(format!("Block Size `{block_size}`"), |b| {
            b.iter(|| {
                pccc::decoder(
                    &code_bits_llr,
                    &interleaver,
                    &code_polynomials,
                    DecodingAlgo::LinearLogMAP(8),
                )
                .unwrap();
            });
        });
    }
}

pub fn decode_f32(c: &mut criterion::Criterion) {
    let group = c.benchmark_group("decode-f32");
    decode::<f32>(group);
}

pub fn decode_f64(c: &mut criterion::Criterion) {
    let group = c.benchmark_group("decode-f64");
    decode::<f64>(group);
}
