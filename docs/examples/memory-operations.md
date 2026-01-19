# Memory Operations Examples

Examples of using AI-Coreutils memory access features.

## Table of Contents

1. [Memory Mapping](#memory-mapping)
2. [Pattern Searching](#pattern-searching)
3. [Binary File Processing](#binary-file-processing)
4. [Performance Comparisons](#performance-comparisons)

## Memory Mapping

### Basic Memory Access

```rust
use ai_coreutils::SafeMemoryAccess;
use std::path::Path;

fn main() -> Result<()> {
    // Create memory-mapped file accessor
    let mem = SafeMemoryAccess::new(Path::new("large_file.txt"))?;

    // Get file size
    let size = mem.size();
    println!("File size: {} bytes", size);

    // Read first 1KB
    if let Some(data) = mem.get(0, 1024) {
        println!("First 1KB: {}", String::from_utf8_lossy(data));
    }

    Ok(())
}
```

### Random Access

```rust
use ai_coreutils::SafeMemoryAccess;

fn read_specific_offsets(mem: &SafeMemoryAccess) {
    let offsets = vec![0, 4096, 8192, 16384];

    for offset in offsets {
        if let Some(data) = mem.get(offset, 100) {
            println!("Offset {}: {}", offset, String::from_utf8_lossy(data));
        }
    }
}
```

### Streaming Large Files

```rust
use ai_coreutils::SafeMemoryAccess;

fn process_in_chunks(mem: &SafeMemoryAccess) -> Result<()> {
    let chunk_size = 1024 * 1024; // 1MB chunks
    let size = mem.size();

    for offset in (0..size).step_by(chunk_size) {
        let remaining = size - offset;
        let to_read = chunk_size.min(remaining);

        if let Some(chunk) = mem.get(offset, to_read) {
            // Process chunk
            process_chunk(chunk)?;
        }
    }

    Ok(())
}

fn process_chunk(chunk: &[u8]) -> Result<()> {
    // Your processing logic
    Ok(())
}
```

## Pattern Searching

### Find Pattern

```rust
use ai_coreutils::SafeMemoryAccess;

fn find_all_patterns(mem: &SafeMemoryAccess, pattern: &[u8]) -> Vec<usize> {
    let mut positions = Vec::new();
    let mut offset = 0;

    while let Some(pos) = mem.find_pattern_at(offset, pattern) {
        positions.push(pos);
        offset = pos + 1;
    }

    positions
}

// Note: find_pattern_at is not in the public API, but you can
// iterate manually using get() and searching
fn find_all_patterns_simple(mem: &SafeMemoryAccess, pattern: &[u8]) -> Vec<usize> {
    let size = mem.size();
    let mut positions = Vec::new();

    if let Some(data) = mem.get(0, size) {
        let mut pos = 0;
        while let Some(index) = data[pos..].windows(pattern.len()).position(|w| w == pattern) {
            positions.push(pos + index);
            pos += index + 1;
        }
    }

    positions
}
```

### Count Occurrences

```rust
use ai_coreutils::SafeMemoryAccess;

fn count_byte_occurrences(mem: &SafeMemoryAccess, byte: u8) -> usize {
    mem.count_byte(byte)
}

fn count_newlines(mem: &SafeMemoryAccess) -> usize {
    mem.count_byte(b'\n')
}

fn count_patterns(mem: &SafeMemoryAccess, pattern: &str) -> usize {
    if let Some(data) = mem.get(0, mem.size()) {
        let text = String::from_utf8_lossy(data);
        text.matches(pattern).count()
    } else {
        0
    }
}
```

### Text Metrics

```rust
use ai_coreutils::SafeMemoryAccess;

fn get_text_stats(mem: &SafeMemoryAccess) -> (usize, usize, usize) {
    // SIMD-accelerated text metrics
    mem.count_text_metrics()
}

fn print_file_stats(path: &Path) -> Result<()> {
    let mem = SafeMemoryAccess::new(path)?;
    let (lines, words, bytes) = mem.count_text_metrics();

    println!("File: {}", path.display());
    println!("Lines: {}", lines);
    println!("Words: {}", words);
    println!("Bytes: {}", bytes);

    Ok(())
}
```

## Binary File Processing

### Parse Binary Header

```rust
use ai_coreutils::SafeMemoryAccess;

#[derive(Debug)]
struct FileHeader {
    version: u32,
    count: u32,
    timestamp: u64,
}

fn read_header(mem: &SafeMemoryAccess) -> Option<FileHeader> {
    // Read first 16 bytes
    if let Some(data) = mem.get(0, 16) {
        let version = u32::from_le_bytes(data[0..4].try().ok()?);
        let count = u32::from_le_bytes(data[4..8].try().ok()?);
        let timestamp = u64::from_le_bytes(data[8..16].try().ok()?);

        Some(FileHeader { version, count, timestamp })
    } else {
        None
    }
}
```

### Process Records

```rust
use ai_coreutils::SafeMemoryAccess;

fn process_records(mem: &SafeMemoryAccess, record_size: usize) -> Result<()> {
    let size = mem.size();
    let header_size = 16; // Skip header
    let num_records = (size - header_size) / record_size;

    for i in 0..num_records {
        let offset = header_size + (i * record_size);

        if let Some(record) = mem.get(offset, record_size) {
            // Process record
            println!("Record {}: {} bytes", i, record.len());
        }
    }

    Ok(())
}
```

### Binary Search

```rust
use ai_coreutils::SafeMemoryAccess;

fn binary_search(mem: &SafeMemoryAccess, target: u32) -> Option<usize> {
    let size = mem.size();
    let record_size = 4; // u32 records
    let num_records = size / record_size;

    let mut left = 0;
    let mut right = num_records;

    while left < right {
        let mid = left + (right - left) / 2;
        let offset = mid * record_size;

        if let Some(data) = mem.get(offset, record_size) {
            let value = u32::from_le_bytes(data.try().ok()?);

            if value == target {
                return Some(mid);
            } else if value < target {
                left = mid + 1;
            } else {
                right = mid;
            }
        } else {
            return None;
        }
    }

    None
}
```

## Performance Comparisons

### Memory Mapping vs Standard I/O

```rust
use std::fs::File;
use std::io::Read;
use std::time::Instant;

fn benchmark_standard_io(path: &Path) -> std::time::Duration {
    let start = Instant::now();

    let mut file = File::open(path).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    start.elapsed()
}

fn benchmark_memory_mapping(path: &Path) -> std::time::Duration {
    let start = Instant::now();

    let mem = SafeMemoryAccess::new(path).unwrap();
    let size = mem.size();
    let _data = mem.get(0, size);

    start.elapsed()
}
```

### SIMD vs Scalar Counting

```rust
use ai_coreutils::{SafeMemoryAccess, simd_ops::SimdByteCounter};

fn benchmark_simd_count(mem: &SafeMemoryAccess) -> std::time::Duration {
    let start = Instant::now();

    let count = mem.count_byte(b'\n'); // SIMD-accelerated

    start.elapsed()
}

fn benchmark_scalar_count(data: &[u8]) -> std::time::Duration {
    let start = Instant::now();

    let count = data.iter().filter(|&&b| b == b'\n').count();

    start.elapsed()
}
```

## Real-World Examples

### Log File Parser

```rust
use ai_coreutils::SafeMemoryAccess;

struct LogEntry {
    timestamp: String,
    level: String,
    message: String,
}

fn parse_log_file(path: &Path) -> Result<Vec<LogEntry>> {
    let mem = SafeMemoryAccess::new(path)?;

    if let Some(data) = mem.get(0, mem.size()) {
        let text = String::from_utf8_lossy(data);
        let mut entries = Vec::new();

        for line in text.lines() {
            if let Some(entry) = parse_log_line(line) {
                entries.push(entry);
            }
        }

        Ok(entries)
    } else {
        Ok(Vec::new())
    }
}

fn parse_log_line(line: &str) -> Option<LogEntry> {
    // Parse log line format: [TIMESTAMP] [LEVEL] MESSAGE
    let parts: Vec<&str> = line.splitn(3, ']').collect();

    if parts.len() >= 3 {
        Some(LogEntry {
            timestamp: parts[0].trim_start_matches('[').to_string(),
            level: parts[1].trim_start_matches('[').to_string(),
            message: parts[2].trim().to_string(),
        })
    } else {
        None
    }
}
```

### Fast Text Search

```rust
use ai_coreutils::SafeMemoryAccess;

fn search_file_fast(path: &Path, pattern: &str) -> Vec<(usize, String)> {
    let mem = SafeMemoryAccess::new(path).unwrap();
    let mut results = Vec::new();

    if let Some(data) = mem.get(0, mem.size()) {
        let text = String::from_utf8_lossy(data);

        for (line_num, line) in text.lines().enumerate() {
            if line.contains(pattern) {
                results.push((line_num + 1, line.to_string()));
            }
        }
    }

    results
}
```

### Data Extraction

```rust
use ai_coreutils::SafeMemoryAccess;

fn extract_csv_column(path: &Path, column_index: usize) -> Vec<String> {
    let mem = SafeMemoryAccess::new(path).unwrap();
    let mut values = Vec::new();

    if let Some(data) = mem.get(0, mem.size()) {
        let text = String::from_utf8_lossy(data);

        for line in text.lines().skip(1) { // Skip header
            let columns: Vec<&str> = line.split(',').collect();

            if column_index < columns.len() {
                values.push(columns[column_index].trim().to_string());
            }
        }
    }

    values
}
```

### Large File Deduplication

```rust
use ai_coreutils::SafeMemoryAccess;
use std::collections::HashSet;

fn find_duplicates(files: Vec<PathBuf>) -> Result<Vec<PathBuf>> {
    let mut seen = HashSet::new();
    let mut duplicates = Vec::new();

    for file in files {
        let mem = SafeMemoryAccess::new(&file)?;

        // Use hash of first 1KB for quick comparison
        let sample_size = 1024.min(mem.size());
        if let Some(data) = mem.get(0, sample_size) {
            let hash = hash_bytes(data);

            if seen.contains(&hash) {
                duplicates.push(file);
            } else {
                seen.insert(hash);
            }
        }
    }

    Ok(duplicates)
}

fn hash_bytes(data: &[u8]) -> u64 {
    // Simple hash function
    data.iter().fold(0u64, |acc, &b| acc.wrapping_mul(31).wrapping_add(b as u64))
}
```

## Best Practices

### Memory Management

```rust
// Good: Reuse SafeMemoryAccess
fn process_multiple_reads(mem: &SafeMemoryAccess) {
    for offset in [0, 4096, 8192] {
        if let Some(data) = mem.get(offset, 100) {
            // Process data
        }
    }
}

// Bad: Creating multiple SafeMemoryAccess instances
fn process_multiple_reads_bad(path: &Path) {
    let mem1 = SafeMemoryAccess::new(path).unwrap();
    let _ = mem1.get(0, 100);

    let mem2 = SafeMemoryAccess::new(path).unwrap();
    let _ = mem2.get(4096, 100);
}
```

### Bounds Checking

```rust
// Good: Always check return value
if let Some(data) = mem.get(offset, len) {
    // Safe to use data
}

// Bad: Ignoring None case
let data = mem.get(offset, len).unwrap(); // May panic
```

### Chunking Strategy

```rust
// Good: Process in reasonable chunks
const CHUNK_SIZE: usize = 1024 * 1024; // 1MB

fn process_in_chunks(mem: &SafeMemoryAccess) {
    for offset in (0..mem.size()).step_by(CHUNK_SIZE) {
        let to_read = CHUNK_SIZE.min(mem.size() - offset);
        if let Some(chunk) = mem.get(offset, to_read) {
            process_chunk(chunk);
        }
    }
}

// Bad: Too small chunks (overhead)
const BAD_CHUNK: usize = 100; // Too small

// Bad: Too large chunks (memory pressure)
const BAD_CHUNK2: usize = 1024 * 1024 * 1024; // 1GB
```
