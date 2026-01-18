//! Benchmark for memory access operations

use ai_coreutils::memory::SafeMemoryAccess;
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use tempfile::NamedTempFile;
use std::io::Write;

fn bench_memory_access_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_access_creation");

    for size in [1024, 10_240, 102_400, 1_048_576].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let mut temp_file = NamedTempFile::new().unwrap();
            let data = vec![b'A'; size];
            temp_file.write_all(&data).unwrap();

            b.iter(|| {
                let _access = SafeMemoryAccess::new(temp_file.path()).unwrap();
                black_box(&_access);
            });
        });
    }

    group.finish();
}

fn bench_memory_read(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_read");

    for size in [1024, 10_240, 102_400, 1_048_576].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let mut temp_file = NamedTempFile::new().unwrap();
            let data = vec![b'A'; size];
            temp_file.write_all(&data).unwrap();

            let access = SafeMemoryAccess::new(temp_file.path()).unwrap();

            b.iter(|| {
                let _data = access.get(0, size.min(1024));
                black_box(&_data);
            });
        });
    }

    group.finish();
}

fn bench_pattern_search(c: &mut Criterion) {
    let mut temp_file = NamedTempFile::new().unwrap();
    let data = b"Hello World. Hello World. Hello World. ".repeat(10_000);
    temp_file.write_all(&data).unwrap();

    let access = SafeMemoryAccess::new(temp_file.path()).unwrap();

    c.bench_function("pattern_search", |b| {
        b.iter(|| {
            let _matches = access.find_pattern(b"Hello");
            black_box(&_matches);
        });
    });
}

criterion_group!(
    benches,
    bench_memory_access_creation,
    bench_memory_read,
    bench_pattern_search
);
criterion_main!(benches);
