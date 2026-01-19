# API Reference

Complete library API documentation for AI-Coreutils.

## Table of Contents

1. [Memory Access](#memory-access)
2. [JSONL Output](#jsonl-output)
3. [Error Handling](#error-handling)
4. [Async Operations](#async-operations)
5. [SIMD Operations](#simd-operations)
6. [ML Operations](#ml-operations)

## Memory Access

### `SafeMemoryAccess`

Safe, bounds-checked memory access with memory mapping support.

```rust
use ai_coreutils::memory::SafeMemoryAccess;

pub struct SafeMemoryAccess {
    // Private fields
}
```

#### Methods

##### `new`

```rust
pub fn new(path: &Path) -> Result<Self>
```

Creates a new memory-mapped file accessor.

**Arguments:**
- `path` - Path to the file

**Returns:**
- `Result<SafeMemoryAccess>` - The memory access object or error

**Example:**

```rust
use ai_coreutils::SafeMemoryAccess;
use std::path::Path;

let mem = SafeMemoryAccess::new(Path::new("file.txt"))?;
```

##### `get`

```rust
pub fn get(&self, offset: usize, len: usize) -> Option<&[u8]>
```

Reads a slice of data with bounds checking.

**Arguments:**
- `offset` - Byte offset to start reading
- `len` - Number of bytes to read

**Returns:**
- `Option<&[u8]>` - The data slice if within bounds, None otherwise

**Example:**

```rust
if let Some(data) = mem.get(0, 1024) {
    println!("Read {} bytes", data.len());
}
```

##### `size`

```rust
pub fn size(&self) -> usize
```

Returns the size of the memory-mapped region.

**Returns:**
- `usize` - Size in bytes

##### `find_pattern`

```rust
pub fn find_pattern(&self, pattern: &[u8]) -> Option<usize>
```

Searches for a byte pattern using SIMD acceleration.

**Arguments:**
- `pattern` - Byte pattern to search for

**Returns:**
- `Option<usize>` - First occurrence offset or None

##### `count_byte`

```rust
pub fn count_byte(&self, byte: u8) -> usize
```

Counts occurrences of a byte using SIMD acceleration.

**Arguments:**
- `byte` - Byte to count

**Returns:**
- `usize` - Number of occurrences

##### `count_text_metrics`

```rust
pub fn count_text_metrics(&self) -> (usize, usize, usize)
```

Counts lines, words, and bytes using SIMD acceleration.

**Returns:**
- `(usize, usize, usize)` - (lines, words, bytes)

## JSONL Output

### `JsonlRecord`

Enum representing all JSONL record types.

```rust
use ai_coreutils::jsonl::JsonlRecord;

pub enum JsonlRecord {
    FileEntry { ... },
    MatchRecord { ... },
    Result { data: serde_json::Value },
    Error { code: String, message: String },
    Info { data: serde_json::Value },
    Progress { current: usize, total: usize, message: String },
}
```

#### Methods

##### `to_jsonl`

```rust
pub fn to_jsonl(&self) -> Result<String>
```

Serializes the record to JSONL format.

**Returns:**
- `Result<String>` - JSONL string or error

##### `result`

```rust
pub fn result(data: serde_json::Value) -> Self
```

Creates a result record.

##### `error`

```rust
pub fn error(message: String, code: String) -> Self
```

Creates an error record.

### Helper Functions

#### `output_result`

```rust
pub fn output_result(data: serde_json::Value) -> Result<()>
```

Outputs a result record to stdout.

#### `output_error`

```rust
pub fn output_error(message: &str, code: &str, path: Option<&str>) -> Result<()>
```

Outputs an error record to stdout.

#### `output_info`

```rust
pub fn output_info(data: serde_json::Value) -> Result<()>
```

Outputs an info record to stdout.

#### `output_progress`

```rust
pub fn output_progress(current: usize, total: usize, message: &str) -> Result<()>
```

Outputs a progress record to stdout.

## Error Handling

### `AiCoreutilsError`

Error type for all AI-Coreutils operations.

```rust
use ai_coreutils::error::AiCoreutilsError;

#[derive(Error, Debug)]
pub enum AiCoreutilsError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Memory access error: {0}")]
    MemoryAccess(String),

    #[error("Path not found: {0}")]
    PathNotFound(PathBuf),

    #[error("Invalid input: {0}")]
    InvalidInput(String),
}
```

### `Result`

Type alias for Result with AiCoreutilsError.

```rust
pub type Result<T> = std::result::Result<T, AiCoreutilsError>;
```

## Async Operations

### `AsyncConfig`

Configuration for async operations.

```rust
use ai_coreutils::async_ops::AsyncConfig;

pub struct AsyncConfig {
    pub max_concurrent: usize,
    pub buffer_size: usize,
    pub progress: bool,
}
```

### Async Functions

#### `async_read_file`

```rust
pub async fn async_read_file(path: &Path) -> Result<Vec<u8>>
```

Asynchronously reads an entire file.

#### `async_write_file`

```rust
pub async fn async_write_file(path: &Path, data: &[u8]) -> Result<()>
```

Asynchronously writes data to a file.

#### `async_walk_dir`

```rust
pub async fn async_walk_dir(dir: &Path) -> Result<Vec<PathBuf>>
```

Asynchronously walks a directory tree.

#### `async_copy_file`

```rust
pub async fn async_copy_file(src: &Path, dst: &Path) -> Result<()>
```

Asynchronously copies a file.

#### `async_grep_file`

```rust
pub async fn async_grep_file(
    path: &Path,
    pattern: &str,
    case_insensitive: bool,
    invert_match: bool
) -> Result<Vec<GrepMatch>>
```

Asynchronously searches a file for a pattern.

#### `async_process_files_concurrently`

```rust
pub async fn async_process_files_concurrently(
    files: Vec<PathBuf>,
    config: AsyncConfig
) -> Result<Vec<JsonlRecord>>
```

Processes multiple files concurrently.

## SIMD Operations

### `SimdPatternSearcher`

SIMD-accelerated pattern searching.

```rust
use ai_coreutils::simd_ops::SimdPatternSearcher;

pub struct SimdPatternSearcher { }
```

#### Methods

##### `new`

```rust
pub fn new() -> Self
```

Creates a new pattern searcher with CPU feature detection.

##### `find_first`

```rust
pub fn find_first(&self, haystack: &[u8], needle: &[u8]) -> Option<usize>
```

Finds the first occurrence of a pattern.

### `SimdByteCounter`

SIMD-accelerated byte counting.

```rust
use ai_coreutils::simd_ops::SimdByteCounter;

pub struct SimdByteCounter { }
```

#### Methods

##### `count_byte`

```rust
pub fn count_byte(&self, data: &[u8], byte: u8) -> usize
```

Counts byte occurrences using SIMD.

## ML Operations

### `PatternDetector`

Pattern detection with regex-based matching.

```rust
use ai_coreutils::ml_ops::PatternDetector;

pub struct PatternDetector { }
```

#### Methods

##### `new`

```rust
pub fn new() -> Result<Self>
```

Creates a new pattern detector.

##### `with_config`

```rust
pub fn with_config(config: MlConfig) -> Result<Self>
```

Creates a pattern detector with custom configuration.

##### `detect_patterns`

```rust
pub fn detect_patterns(&self, text: &str) -> Vec<PatternMatch>
```

Detects all patterns in text.

##### `analyze_content`

```rust
pub fn analyze_content(&self, text: &str, path: &Path) -> Result<ContentAnalysis>
```

Analyzes content for patterns and statistics.

### `FileClassifier`

File type classification.

```rust
use ai_coreutils::ml_ops::FileClassifier;

pub struct FileClassifier;
```

#### Methods

##### `classify`

```rust
pub fn classify(path: &Path, content: &[u8]) -> Result<FileClassification>
```

Classifies a file by type.

### Types

#### `PatternMatch`

```rust
pub struct PatternMatch {
    pub pattern: String,
    pub matched_text: String,
    pub start: usize,
    pub end: usize,
    pub confidence: f64,
    pub pattern_type: PatternType,
}
```

#### `PatternType`

```rust
pub enum PatternType {
    Email,
    Url,
    IpAddress,
    PhoneNumber,
    Ssn,
    CreditCard,
    Uuid,
    Date,
    Hex,
    Base64,
    FilePath,
}
```

#### `FileClassification`

```rust
pub struct FileClassification {
    pub path: String,
    pub file_type: String,
    pub mime_type: String,
    pub encoding: String,
    pub is_binary: bool,
    pub language: Option<String>,
    pub confidence: f64,
}
```

#### `ContentAnalysis`

```rust
pub struct ContentAnalysis {
    pub total_patterns: usize,
    pub patterns_by_type: HashMap<String, usize>,
    pub matches: Vec<PatternMatch>,
    pub statistics: ContentStatistics,
    pub issues: Vec<String>,
}
```

#### `ContentStatistics`

```rust
pub struct ContentStatistics {
    pub lines: usize,
    pub words: usize,
    pub characters: usize,
    pub bytes: usize,
    pub avg_line_length: f64,
    pub max_line_length: usize,
    pub whitespace_ratio: f64,
    pub entropy: f64,
}
```

## Module Re-exports

The main library re-exports common types:

```rust
// In src/lib.rs
pub mod error;
pub mod jsonl;
pub mod memory;

pub use error::{AiCoreutilsError, Result};
pub use jsonl::{JsonlRecord, output_result, output_error, output_info, output_progress};
pub use memory::SafeMemoryAccess;

// Optional modules (feature-gated)
pub mod async_ops;
pub mod simd_ops;
pub mod ml_ops;
```

## Usage Examples

### Basic Memory Access

```rust
use ai_coreutils::SafeMemoryAccess;

let mem = SafeMemoryAccess::new("file.txt")?;
let size = mem.size();

if let Some(data) = mem.get(0, size.min(1024)) {
    println!("Read: {}", String::from_utf8_lossy(data));
}
```

### Pattern Detection

```rust
use ai_coreutils::ml_ops::PatternDetector;

let detector = PatternDetector::new()?;
let matches = detector.detect_patterns("Contact test@example.com");

for m in matches {
    println!("Found {} at {}:{}", m.pattern_type, m.start, m.end);
}
```

### File Classification

```rust
use ai_coreutils::ml_ops::FileClassifier;
use std::fs;

let content = fs::read("unknown_file.bin")?;
let classification = FileClassifier::classify(
    Path::new("unknown_file.bin"),
    &content
)?;

println!("Type: {}", classification.file_type);
println!("Binary: {}", classification.is_binary);
```

### Async Operations

```rust
use ai_coreutils::async_ops::{async_read_file, AsyncConfig};
use tokio::runtime::Runtime;

let rt = Runtime::new()?;
rt.block_on(async {
    let data = async_read_file(Path::new("large_file.txt")).await?;
    println!("Read {} bytes", data.len());
    Ok(())
});
```
