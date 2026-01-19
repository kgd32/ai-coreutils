//! Benchmark for SIMD operations
//!
//! Compares SIMD-accelerated operations against scalar implementations

use ai_coreutils::memory::SafeMemoryAccess;
use ai_coreutils::simd_ops::{SimdByteCounter, SimdTextProcessor};
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::io::Write;
use tempfile::NamedTempFile;

fn bench_pattern_search_small(c: &mut Criterion) {
    let mut group = c.benchmark_group("pattern_search_small");

    let mut temp_file = NamedTempFile::new().unwrap();
    let data = b"Hello World. Hello World. Hello World. ".repeat(100);
    temp_file.write_all(&data).unwrap();

    let access = SafeMemoryAccess::new(temp_file.path()).unwrap();
    let pattern = b"Hello";

    group.bench_function("simd", |b| {
        b.iter(|| {
            let _matches = access.find_pattern(black_box(pattern));
            black_box(&_matches);
        });
    });

    group.finish();
}

fn bench_pattern_search_large(c: &mut Criterion) {
    let mut group = c.benchmark_group("pattern_search_large");

    let mut temp_file = NamedTempFile::new().unwrap();
    let data = b"The quick brown fox jumps over the lazy dog. ".repeat(10_000);
    temp_file.write_all(&data).unwrap();

    let access = SafeMemoryAccess::new(temp_file.path()).unwrap();
    let pattern = b"fox";

    group.bench_function("simd", |b| {
        b.iter(|| {
            let _matches = access.find_pattern(black_box(pattern));
            black_box(&_matches);
        });
    });

    group.finish();
}

fn bench_byte_count_small(c: &mut Criterion) {
    let mut group = c.benchmark_group("byte_count_small");

    let data = b"Hello World! Hello World! ".repeat(100);

    let counter = SimdByteCounter::new();

    group.bench_function("simd", |b| {
        b.iter(|| {
            let _count = counter.count(black_box(&data), b'l');
            black_box(&_count);
        });
    });

    group.finish();
}

fn bench_byte_count_large(c: &mut Criterion) {
    let mut group = c.benchmark_group("byte_count_large");

    let data = b"The quick brown fox jumps over the lazy dog. ".repeat(100_000);

    let counter = SimdByteCounter::new();

    group.bench_function("simd", |b| {
        b.iter(|| {
            let _count = counter.count(black_box(&data), b'o');
            black_box(&_count);
        });
    });

    group.finish();
}

fn bench_text_analyze_small(c: &mut Criterion) {
    let mut group = c.benchmark_group("text_analyze_small");

    let data = b"Hello world\nThis is a test\nAnother line here\n".repeat(10);

    let processor = SimdTextProcessor::new();

    group.bench_function("simd", |b| {
        b.iter(|| {
            let _metrics = processor.analyze(black_box(&data));
            black_box(&_metrics);
        });
    });

    group.finish();
}

fn bench_text_analyze_large(c: &mut Criterion) {
    let mut group = c.benchmark_group("text_analyze_large");

    let data = b"The quick brown fox jumps over the lazy dog.\n".repeat(100_000);

    let processor = SimdTextProcessor::new();

    group.bench_function("simd", |b| {
        b.iter(|| {
            let _metrics = processor.analyze(black_box(&data));
            black_box(&_metrics);
        });
    });

    group.finish();
}

fn bench_memory_access_with_simd(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_access_with_simd");

    for size in [1024, 10_240, 102_400, 1_048_576].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let mut temp_file = NamedTempFile::new().unwrap();
            let data = vec![b'A'; size];
            temp_file.write_all(&data).unwrap();

            let access = SafeMemoryAccess::new(temp_file.path()).unwrap();

            b.iter(|| {
                let _count = access.count_byte(black_box(b'A'));
                black_box(&_count);
            });
        });
    }

    group.finish();
}

fn bench_newline_counting(c: &mut Criterion) {
    let mut group = c.benchmark_group("newline_counting");

    let data = b"Line 1\nLine 2\nLine 3\nLine 4\nLine 5\n".repeat(10_000);

    let processor = SimdTextProcessor::new();

    group.bench_function("simd", |b| {
        b.iter(|| {
            let _metrics = processor.analyze(black_box(&data));
            black_box(&_metrics);
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_pattern_search_small,
    bench_pattern_search_large,
    bench_byte_count_small,
    bench_byte_count_large,
    bench_text_analyze_small,
    bench_text_analyze_large,
    bench_memory_access_with_simd,
    bench_newline_counting
);
criterion_main!(benches);
