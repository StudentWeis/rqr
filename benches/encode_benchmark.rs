//! Benchmarks for QR encoding operations

use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use rqr::qr::encoder::QrEncoder;

fn bench_encode_short(c: &mut Criterion) {
    let encoder = QrEncoder::new(200, 10, "M").unwrap();

    c.bench_function("encode_short_text", |b| {
        b.iter(|| encoder.encode(black_box("Hello, World!")).unwrap())
    });
}

fn bench_encode_medium(c: &mut Criterion) {
    let encoder = QrEncoder::new(400, 10, "M").unwrap();
    let content = "a".repeat(100);

    c.bench_function("encode_medium_text", |b| {
        b.iter(|| encoder.encode(black_box(&content)).unwrap())
    });
}

fn bench_encode_long(c: &mut Criterion) {
    let encoder = QrEncoder::new(800, 10, "M").unwrap();
    let content = "a".repeat(500);

    c.bench_function("encode_long_text", |b| {
        b.iter(|| encoder.encode(black_box(&content)).unwrap())
    });
}

fn bench_encode_with_different_sizes(c: &mut Criterion) {
    let mut group = c.benchmark_group("encode_different_sizes");

    for size in [100, 200, 400, 800] {
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            let encoder = QrEncoder::new(size, 10, "M").unwrap();
            b.iter(|| encoder.encode(black_box("Benchmark content")).unwrap())
        });
    }

    group.finish();
}

fn bench_encode_with_different_error_correction(c: &mut Criterion) {
    let mut group = c.benchmark_group("encode_error_correction_levels");

    for level in ["L", "M", "Q", "H"] {
        group.bench_with_input(BenchmarkId::new("level", level), &level, |b, &level| {
            let encoder = QrEncoder::new(200, 10, level).unwrap();
            b.iter(|| encoder.encode(black_box("Test content")).unwrap())
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_encode_short,
    bench_encode_medium,
    bench_encode_long,
    bench_encode_with_different_sizes,
    bench_encode_with_different_error_correction
);
criterion_main!(benches);
