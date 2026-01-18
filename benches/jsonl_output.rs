//! Benchmark for JSONL output operations

use ai_coreutils::jsonl::{JsonlOutput, JsonlRecord};
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use std::io::Cursor;

fn bench_jsonl_serialization(c: &mut Criterion) {
    let mut group = c.benchmark_group("jsonl_serialization");

    for record_count in [10, 100, 1000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(record_count), record_count, |b, &count| {
            let records: Vec<JsonlRecord> = (0..count)
                .map(|_| JsonlRecord::result(serde_json::json!({"test": "value"})))
                .collect();

            b.iter(|| {
                for record in &records {
                    let _jsonl = record.to_jsonl().unwrap();
                    black_box(&_jsonl);
                }
            });
        });
    }

    group.finish();
}

fn bench_jsonl_output_write(c: &mut Criterion) {
    let mut group = c.benchmark_group("jsonl_output_write");

    for record_count in [10, 100, 1000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(record_count), record_count, |b, &count| {
            let records: Vec<JsonlRecord> = (0..count)
                .map(|_| JsonlRecord::result(serde_json::json!({"test": "value"})))
                .collect();

            b.iter(|| {
                let buffer = Cursor::new(Vec::new());
                let mut output = JsonlOutput::new(buffer);
                let _ = output.write_records(&records);
                black_box(&output);
            });
        });
    }

    group.finish();
}

fn bench_jsonl_file_entry(c: &mut Criterion) {
    c.bench_function("jsonl_file_entry", |b| {
        b.iter(|| {
            let record = JsonlRecord::FileEntry {
                timestamp: chrono::Utc::now(),
                path: "/test/path/to/file.txt".to_string(),
                size: 1024,
                modified: chrono::Utc::now(),
                is_dir: false,
                is_symlink: false,
                permissions: "rw-r--r--".to_string(),
            };
            let _jsonl = record.to_jsonl().unwrap();
            black_box(&_jsonl);
        });
    });
}

criterion_group!(
    benches,
    bench_jsonl_serialization,
    bench_jsonl_output_write,
    bench_jsonl_file_entry
);
criterion_main!(benches);
