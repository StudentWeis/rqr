//! Benchmarks for QR decoding operations

use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use rqr::qr::{decoder::QrDecoder, encoder::QrEncoder};

fn create_test_image(content: &str, size: u32) -> image::DynamicImage {
    let encoder = QrEncoder::new(size, 10, "M").unwrap();
    let qr_code = encoder.encode(content).unwrap();
    encoder.to_image(&qr_code).unwrap()
}

fn bench_decode_short(c: &mut Criterion) {
    let image = create_test_image("Hello", 200);
    let decoder = QrDecoder::new();

    c.bench_function("decode_short_text", |b| {
        b.iter(|| decoder.decode_from_image(black_box(image.clone())).unwrap())
    });
}

fn bench_decode_medium(c: &mut Criterion) {
    let image = create_test_image(&"a".repeat(100), 400);
    let decoder = QrDecoder::new();

    c.bench_function("decode_medium_text", |b| {
        b.iter(|| decoder.decode_from_image(black_box(image.clone())).unwrap())
    });
}

fn bench_decode_long(c: &mut Criterion) {
    let image = create_test_image(&"a".repeat(500), 800);
    let decoder = QrDecoder::new();

    c.bench_function("decode_long_text", |b| {
        b.iter(|| decoder.decode_from_image(black_box(image.clone())).unwrap())
    });
}

fn bench_decode_with_different_sizes(c: &mut Criterion) {
    let mut group = c.benchmark_group("decode_different_sizes");

    for size in [100, 200, 400, 800] {
        let image = create_test_image("Benchmark", size);
        group.bench_with_input(BenchmarkId::from_parameter(size), &image, |b, img| {
            let decoder = QrDecoder::new();
            b.iter(|| decoder.decode_from_image(black_box(img.clone())).unwrap())
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_decode_short,
    bench_decode_medium,
    bench_decode_long,
    bench_decode_with_different_sizes
);
criterion_main!(benches);
