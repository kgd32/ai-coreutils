// Comprehensive SIMD Performance Benchmarking Suite
//
// This benchmark compares scalar vs SIMD performance for all optimizations
// in AI-Coreutils. Tests different file sizes and CPU architectures.

use ai_coreutils::simd_ops::*;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use std::time::Duration;

// Test data sizes (small, medium, large, very large)
const SIZES: &[usize] = &[64, 1024, 10_240, 102_400, 1_048_576];

fn generate_test_data(size: usize) -> Vec<u8> {
    // Generate mixed ASCII and UTF-8 data
    let mut data = Vec::with_capacity(size);
    let mut i = 0;

    while i < size {
        if i % 10 == 0 {
            // Add some UTF-8 (Chinese characters: 世界 = 3 bytes each)
            data.extend_from_slice("世界".as_bytes());
            i += 6;
        } else if i % 5 == 0 {
            // Add newline
            data.push(b'\n');
            i += 1;
        } else {
            // Add ASCII text
            data.push(b'a' + (i % 26) as u8);
            i += 1;
        }
    }

    data.truncate(size);
    data
}

// Byte Counting Benchmarks

fn bench_byte_count_scalar(data: &[u8], target: u8) -> usize {
    data.iter().filter(|&&b| b == target).count()
}

fn bench_byte_count_simd(c: &mut Criterion) {
    let mut group = c.benchmark_group("byte_count");
    group.measurement_time(Duration::from_secs(5));
    group.sample_size(100);

    for size in SIZES {
        let data = generate_test_data(*size);
        let counter = SimdByteCounter::new();

        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_with_input(BenchmarkId::new("scalar", size), &data, |b, data| {
            b.iter(|| bench_byte_count_scalar(black_box(data), b'a'));
        });

        group.bench_with_input(BenchmarkId::new("simd", size), &data, |b, data| {
            b.iter(|| counter.count(black_box(data), b'a'));
        });
    }

    group.finish();
}

// Pattern Search Benchmarks

fn bench_pattern_search_scalar(data: &[u8], pattern: &[u8]) -> Option<usize> {
    data.windows(pattern.len()).position(|w| w == pattern)
}

fn bench_pattern_search(c: &mut Criterion) {
    let mut group = c.benchmark_group("pattern_search");
    group.measurement_time(Duration::from_secs(5));
    group.sample_size(100);

    for size in SIZES {
        let data = generate_test_data(*size);
        let pattern = b"world";
        let searcher = SimdPatternSearcher::new();

        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_with_input(BenchmarkId::new("scalar", size), &data, |b, data| {
            b.iter(|| bench_pattern_search_scalar(black_box(data), black_box(pattern)));
        });

        group.bench_with_input(BenchmarkId::new("simd", size), &data, |b, data| {
            b.iter(|| searcher.find_first(black_box(data), black_box(pattern)));
        });
    }

    group.finish();
}

// Newline Counting Benchmarks

fn bench_count_newlines_scalar(data: &[u8]) -> usize {
    data.iter().filter(|&&b| b == b'\n').count()
}

fn bench_count_newlines(c: &mut Criterion) {
    let mut group = c.benchmark_group("newline_count");
    group.measurement_time(Duration::from_secs(5));
    group.sample_size(100);

    for size in SIZES {
        let data = generate_test_data(*size);
        let counter = SimdByteCounter::new();

        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_with_input(BenchmarkId::new("scalar", size), &data, |b, data| {
            b.iter(|| bench_count_newlines_scalar(black_box(data)));
        });

        group.bench_with_input(BenchmarkId::new("simd", size), &data, |b, data| {
            b.iter(|| counter.count(black_box(data), b'\n'));
        });
    }

    group.finish();
}

// Word Counting Benchmarks

fn bench_count_words_scalar(data: &[u8]) -> usize {
    let mut count = 0;
    let mut in_word = false;

    for &byte in data.iter() {
        let is_whitespace = byte.is_ascii_whitespace();
        if is_whitespace {
            if in_word {
                count += 1;
                in_word = false;
            }
        } else {
            in_word = true;
        }
    }

    if in_word {
        count += 1;
    }

    count
}

fn bench_count_words(c: &mut Criterion) {
    let mut group = c.benchmark_group("word_count");
    group.measurement_time(Duration::from_secs(5));
    group.sample_size(100);

    for size in SIZES {
        let data = generate_test_data(*size);
        let detector = SimdWhitespaceDetector::new();

        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_with_input(BenchmarkId::new("scalar", size), &data, |b, data| {
            b.iter(|| bench_count_words_scalar(black_box(data)));
        });

        group.bench_with_input(BenchmarkId::new("simd", size), &data, |b, data| {
            b.iter(|| detector.count_words(black_box(data)));
        });
    }

    group.finish();
}

// UTF-8 Validation Benchmarks

fn bench_utf8_validate_scalar(data: &[u8]) -> (bool, Option<usize>) {
    let mut i = 0;

    while i < data.len() {
        let byte = data[i];

        if byte <= 0x7F {
            i += 1;
        } else if byte >= 0xC0 && byte <= 0xDF {
            if i + 1 >= data.len() {
                return (false, Some(i));
            }
            let byte2 = data[i + 1];
            if byte2 < 0x80 || byte2 > 0xBF || byte < 0xC2 {
                return (false, Some(i));
            }
            i += 2;
        } else if byte >= 0xE0 && byte <= 0xEF {
            if i + 2 >= data.len() {
                return (false, Some(i));
            }
            let byte2 = data[i + 1];
            let byte3 = data[i + 2];
            if byte2 < 0x80 || byte2 > 0xBF || byte3 < 0x80 || byte3 > 0xBF {
                return (false, Some(i + 1));
            }
            if byte == 0xE0 && byte2 < 0xA0 {
                return (false, Some(i));
            }
            if byte == 0xED && byte2 > 0x9F {
                return (false, Some(i));
            }
            i += 3;
        } else if byte >= 0xF0 && byte <= 0xF4 {
            if i + 3 >= data.len() {
                return (false, Some(i));
            }
            let byte2 = data[i + 1];
            let byte3 = data[i + 2];
            let byte4 = data[i + 3];
            if byte2 < 0x80 || byte2 > 0xBF ||
               byte3 < 0x80 || byte3 > 0xBF ||
               byte4 < 0x80 || byte4 > 0xBF {
                return (false, Some(i + 1));
            }
            if byte == 0xF0 && byte2 < 0x90 {
                return (false, Some(i));
            }
            if byte == 0xF4 && byte2 > 0x8F {
                return (false, Some(i));
            }
            i += 4;
        } else {
            return (false, Some(i));
        }
    }

    (true, None)
}

fn bench_utf8_validate(c: &mut Criterion) {
    let mut group = c.benchmark_group("utf8_validate");
    group.measurement_time(Duration::from_secs(5));
    group.sample_size(100);

    for size in SIZES {
        let data = generate_test_data(*size);
        let validator = SimdUtf8Validator::new();

        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_with_input(BenchmarkId::new("scalar", size), &data, |b, data| {
            b.iter(|| bench_utf8_validate_scalar(black_box(data)));
        });

        group.bench_with_input(BenchmarkId::new("simd", size), &data, |b, data| {
            b.iter(|| validator.validate(black_box(data)));
        });
    }

    group.finish();
}

// String Comparison Benchmarks

fn bench_string_compare_scalar(a: &[u8], b: &[u8]) -> std::cmp::Ordering {
    a.cmp(b)
}

fn bench_string_compare(c: &mut Criterion) {
    let mut group = c.benchmark_group("string_compare");
    group.measurement_time(Duration::from_secs(5));
    group.sample_size(100);

    for size in SIZES {
        let data_a = generate_test_data(*size);
        let data_b = generate_test_data(*size);
        let comparer = SimdStringComparer::new();

        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_with_input(BenchmarkId::new("scalar", size), &(&data_a, &data_b), |bencher, (a, b)| {
            bencher.iter(|| bench_string_compare_scalar(black_box(a), black_box(b)));
        });

        group.bench_with_input(BenchmarkId::new("simd", size), &(&data_a, &data_b), |bencher, (a, b)| {
            bencher.iter(|| comparer.compare(black_box(a), black_box(b)));
        });
    }

    group.finish();
}

// Case-Insensitive Search Benchmarks

fn bench_case_insensitive_scalar(data: &[u8], pattern: &[u8]) -> Option<usize> {
    data.windows(pattern.len())
        .position(|window| window.len() == pattern.len() &&
            window.iter().zip(pattern.iter()).all(|(a, b)| a.eq_ignore_ascii_case(b)))
}

fn bench_case_insensitive(c: &mut Criterion) {
    let mut group = c.benchmark_group("case_insensitive");
    group.measurement_time(Duration::from_secs(5));
    group.sample_size(100);

    for size in SIZES {
        let data = generate_test_data(*size);
        let pattern = b"WORLD";
        let folder = SimdCaseFolder::new();

        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_with_input(BenchmarkId::new("scalar", size), &data, |b, data| {
            b.iter(|| bench_case_insensitive_scalar(black_box(data), black_box(pattern)));
        });

        group.bench_with_input(BenchmarkId::new("simd", size), &data, |b, data| {
            b.iter(|| folder.find_caseless(black_box(data), black_box(pattern)));
        });
    }

    group.finish();
}

// Entropy Calculation Benchmarks

fn bench_entropy_scalar(data: &[u8]) -> f64 {
    use std::collections::HashMap;

    let mut char_counts = HashMap::new();
    for &byte in data.iter() {
        *char_counts.entry(byte).or_insert(0) += 1;
    }

    let length = data.len() as f64;
    let mut entropy = 0.0;

    for &count in char_counts.values() {
        if count > 0 {
            let probability = count as f64 / length;
            entropy -= probability * probability.log2();
        }
    }

    entropy
}

fn bench_entropy(c: &mut Criterion) {
    let mut group = c.benchmark_group("entropy");
    group.measurement_time(Duration::from_secs(5));
    group.sample_size(100);

    for size in SIZES {
        let data = generate_test_data(*size);
        let calculator = SimdEntropyCalculator::new();

        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_with_input(BenchmarkId::new("scalar", size), &data, |b, data| {
            b.iter(|| bench_entropy_scalar(black_box(data)));
        });

        group.bench_with_input(BenchmarkId::new("simd", size), &data, |b, data| {
            b.iter(|| calculator.calculate_entropy(black_box(data)));
        });
    }

    group.finish();
}

// Memory Copy Benchmarks

fn bench_memory_copy_scalar(dst: &mut [u8], src: &[u8]) -> usize {
    let count = src.len().min(dst.len());
    dst[..count].copy_from_slice(&src[..count]);
    count
}

fn bench_memory_copy(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_copy");
    group.measurement_time(Duration::from_secs(5));
    group.sample_size(100);

    for size in SIZES {
        let mem_ops = SimdMemoryOps::new();

        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_with_input(BenchmarkId::new("scalar", size), size, |bencher, len| {
            let mut dst = vec![0u8; *len];
            let src = generate_test_data(*len);
            bencher.iter(|| {
                bench_memory_copy_scalar(&mut dst, &src);
            });
        });

        group.bench_with_input(BenchmarkId::new("simd", size), size, |bencher, len| {
            let mut dst = vec![0u8; *len];
            let src = generate_test_data(*len);
            bencher.iter(|| {
                let _ = mem_ops.copy(&mut dst, &src);
            });
        });
    }

    group.finish();
}

// Hash Computation Benchmarks

fn bench_crc32_scalar(data: &[u8]) -> u32 {
    let mut crc: u32 = 0xFFFFFFFF;

    for &byte in data {
        crc ^= byte as u32;
        for _ in 0..8 {
            if crc & 1 == 1 {
                crc = (crc >> 1) ^ 0xEDB88320;
            } else {
                crc >>= 1;
            }
        }
    }

    !crc
}

fn bench_hash_computation(c: &mut Criterion) {
    let mut group = c.benchmark_group("hash");
    group.measurement_time(Duration::from_secs(5));
    group.sample_size(100);

    for size in SIZES {
        let data = generate_test_data(*size);
        let hasher = SimdHasher::new();

        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_with_input(BenchmarkId::new("scalar_crc32", size), &data, |b, data| {
            b.iter(|| bench_crc32_scalar(black_box(data)));
        });

        group.bench_with_input(BenchmarkId::new("simd_crc32", size), &data, |b, data| {
            b.iter(|| hasher.crc32(black_box(data)));
        });

        group.bench_with_input(BenchmarkId::new("rolling_hash", size), &data, |b, data| {
            b.iter(|| hasher.rolling_hash(black_box(data)));
        });
    }

    group.finish();
}

// Multi-Pattern Search Benchmarks

fn bench_multi_pattern_scalar(data: &[u8], patterns: &[&[u8]]) -> Vec<(usize, usize)> {
    let mut matches = Vec::new();

    for (pattern_idx, pattern) in patterns.iter().enumerate() {
        if let Some(pos) = data.windows(pattern.len()).position(|w| w == *pattern) {
            matches.push((pattern_idx, pos));
        }
    }

    matches
}

fn bench_multi_pattern_search(c: &mut Criterion) {
    let mut group = c.benchmark_group("multi_pattern");
    group.measurement_time(Duration::from_secs(5));
    group.sample_size(100);

    let patterns: Vec<&[u8]> = vec![b"hello", b"world", b"test", b"benchmark"];

    for size in SIZES {
        let data = generate_test_data(*size);
        let searcher = SimdMultiPatternSearcher::new(&patterns);

        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_with_input(BenchmarkId::new("scalar", size), &data, |b, data| {
            b.iter(|| bench_multi_pattern_scalar(black_box(data), black_box(&patterns)));
        });

        group.bench_with_input(BenchmarkId::new("simd", size), &data, |b, data| {
            b.iter(|| searcher.find_all(black_box(data)));
        });
    }

    group.finish();
}

// Comprehensive Benchmark Group

fn bench_comprehensive_suite(c: &mut Criterion) {
    bench_byte_count_simd(c);
    bench_pattern_search(c);
    bench_count_newlines(c);
    bench_count_words(c);
    bench_utf8_validate(c);
    bench_string_compare(c);
    bench_case_insensitive(c);
    bench_entropy(c);
    bench_memory_copy(c);
    bench_hash_computation(c);
    bench_multi_pattern_search(c);
}

criterion_group!(benches, bench_comprehensive_suite);
criterion_main!(benches);
