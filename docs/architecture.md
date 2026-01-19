# AI-Coreutils Architecture

System design, module organization, and data flow for AI-Coreutils.

## Overview

AI-Coreutils is a modern reimplementation of GNU core utilities designed specifically for AI agents. The architecture emphasizes:

- **Structured Output**: All utilities output JSONL (JSON Lines) format
- **Memory Efficiency**: Zero-copy operations with memory mapping
- **Performance**: SIMD-accelerated text processing
- **Async I/O**: Concurrent file operations with tokio
- **AI Integration**: Pattern detection and content analysis

## System Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                         CLI Binaries                             │
│  ┌──────┐ ┌──────┐ ┌──────┐ ┌──────┐ ┌──────────┐             │
│  │ls   │ │cat  │ │grep │ │wc   │ │analyze  │  ... (16 utils)  │
│  └──┬───┘ └──┬───┘ └──┬───┘ └──┬───┘ └────┬─────┘             │
└─────┼────────┼────────┼────────┼───────────┼──────────────────┘
      │        │        │        │           │
      └────────┴────────┴────────┴───────────┘
                      │
      ┌───────────────┴───────────────────────┐
      │         ai-coreutils Library          │
      └───────────────┬───────────────────────┘
                      │
      ┌───────────────┴───────────────────────┐
      │           Core Modules                │
      ├───────────────┬───────────────────────┤
      │               │                       │
┌─────┴─────┐   ┌────┴────┐   ┌────────────┐ │
│   memory  │   │  jsonl  │   │   error    │ │
│   module  │   │  module │   │  module    │ │
└───────────┘   └────┬────┘   └────────────┘ │
                      │                       │
┌─────────────────────┴─────────────────────┐ │
│           Advanced Modules                │ │
├─────────────┬─────────────┬──────────────┤ │
│             │             │              │ │
│  async_ops  │  simd_ops   │   ml_ops     │ │
│  (tokio)    │  (AVX2/SSE2)│  (patterns)  │ │
└─────────────┴─────────────┴──────────────┘ │
└─────────────────────────────────────────────┘
                      │
      ┌───────────────┴───────────────────────┐
      │         Dependencies                  │
      │  memmap2 │ clap │ serde │ tokio ...   │
      └───────────────────────────────────────┘
```

## Core Modules

### Memory Module (`src/memory.rs`)

Provides safe memory access with bounds checking.

**Key Components:**

```rust
pub struct SafeMemoryAccess {
    mmap: Mmap,
    size: usize,
}

impl SafeMemoryAccess {
    pub fn new(path: &Path) -> Result<Self>
    pub fn get(&self, offset: usize, len: usize) -> Option<&[u8]>
    pub fn size(&self) -> usize
    pub fn find_pattern(&self, pattern: &[u8]) -> Option<usize>
    pub fn count_byte(&self, byte: u8) -> usize
}
```

**Features:**
- Memory-mapped file access via `memmap2`
- Bounds-checked read operations
- SIMD-accelerated pattern search
- Byte counting for text processing

### JSONL Module (`src/jsonl.rs`)

Handles all JSONL output formatting.

**Key Components:**

```rust
pub enum JsonlRecord {
    FileEntry { ... },
    MatchRecord { ... },
    Result { data: serde_json::Value },
    Error { code: String, message: String },
    Info { data: serde_json::Value },
    Progress { current: usize, total: usize, message: String },
}

impl JsonlRecord {
    pub fn to_jsonl(&self) -> Result<String>
    pub fn result(data: serde_json::Value) -> Self
    pub fn error(message: String, code: String) -> Self
}
```

**Helper Functions:**

```rust
pub fn output_result(data: serde_json::Value) -> Result<()>
pub fn output_error(message: &str, code: &str, path: Option<&str>) -> Result<()>
pub fn output_info(data: serde_json::Value) -> Result<()>
pub fn output_progress(current: usize, total: usize, message: &str) -> Result<()>
```

### Error Module (`src/error.rs`)

Centralized error handling with thiserror.

```rust
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

## Advanced Modules

### Async Operations (`src/async_ops.rs`)

Concurrent file processing with tokio.

**Key Components:**

```rust
pub struct AsyncConfig {
    pub max_concurrent: usize,
    pub buffer_size: usize,
    pub progress: bool,
}

pub async fn async_read_file(path: &Path) -> Result<Vec<u8>>
pub async fn async_write_file(path: &Path, data: &[u8]) -> Result<()>
pub async fn async_walk_dir(dir: &Path) -> Result<Vec<PathBuf>>
pub async fn async_copy_file(src: &Path, dst: &Path) -> Result<()>
pub async fn async_process_files_concurrently(
    files: Vec<PathBuf>,
    config: AsyncConfig
) -> Result<Vec<JsonlRecord>>
```

**Data Flow:**

```
Files → Async Stream → Concurrent Processing → Results
         (buffer_unordered)
```

### SIMD Operations (`src/simd_ops.rs`)

Hardware-accelerated text processing.

**Key Components:**

```rust
pub struct SimdPatternSearcher { ... }
pub struct SimdByteCounter { ... }
pub struct SimdWhitespaceDetector { ... }
pub struct SimdTextProcessor { ... }

impl SimdPatternSearcher {
    pub fn find_first(&self, haystack: &[u8], needle: &[u8]) -> Option<usize>
}

impl SimdByteCounter {
    pub fn count_byte(&self, data: &[u8], byte: u8) -> usize
}
```

**SIMD Code Path:**

```
Data → CPU Feature Detection → AVX2/SSE2/Scalar → Result
```

### ML Operations (`src/ml_ops.rs`)

Pattern detection and content analysis.

**Key Components:**

```rust
pub struct PatternDetector {
    patterns: Vec<(PatternType, Regex)>,
    config: MlConfig,
}

pub struct FileClassifier;
pub struct ContentAnalysis;

impl PatternDetector {
    pub fn new() -> Self
    pub fn detect_patterns(&self, text: &str) -> Vec<PatternMatch>
    pub fn analyze_content(&self, text: &str, path: &Path) -> Result<ContentAnalysis>
}

impl FileClassifier {
    pub fn classify(path: &Path, content: &[u8]) -> Result<FileClassification>
}
```

**Supported Patterns:**
- Email addresses
- URLs
- IP addresses
- Phone numbers
- Social Security Numbers
- Credit card numbers
- UUIDs
- Dates
- Hex values
- Base64 encoded data
- File paths

## Utility Binary Structure

Each utility follows a consistent structure:

```rust
use ai_coreutils::{jsonl, Result};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "ai-utility")]
struct Cli {
    // Utility-specific options
    #[arg(short, long)]
    option: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Process input
    for item in &cli.items {
        match process_item(item, &cli) {
            Ok(result) => {
                jsonl::output_result(serde_json::json!(result))?;
            }
            Err(e) => {
                jsonl::output_error(&e.to_string(), "ERROR_CODE", None)?;
            }
        }
    }

    Ok(())
}
```

## Cross-Language Bindings

### Python Bindings (`src/python.rs`)

Uses PyO3 to expose Rust APIs to Python:

```rust
use pyo3::prelude::*;

#[pyclass]
pub struct SafeMemoryAccess {
    // ...
}

#[pymethods]
impl SafeMemoryAccess {
    #[new]
    pub fn new(path: &str) -> PyResult<Self> { ... }

    pub fn size(&self) -> usize { ... }

    pub fn get(&self, offset: usize, len: usize) -> PyResult<Vec<u8>> { ... }
}
```

### Node.js Bindings (`nodejs/src/lib.rs`)

Uses NAPI-RS to expose Rust APIs to Node.js:

```rust
use napi_derive::napi;

#[napi]
pub struct MemoryAccess {
    // ...
}

#[napi]
impl MemoryAccess {
    #[napi(constructor)]
    pub fn new(path: String) -> Result<Self> { ... }

    #[napi]
    pub fn size(&self) -> usize { ... }
}
```

## Data Flow Examples

### File Reading (ai-cat)

```
CLI Input → SafeMemoryAccess::new()
    ↓
Memory Mapping (memmap2)
    ↓
get(offset, len)
    ↓
JSONL Output
```

### Pattern Search (ai-grep)

```
CLI Input → SafeMemoryAccess
    ↓
find_pattern() → SIMD Search (AVX2/SSE2)
    ↓
Match Results
    ↓
JSONL Output with Context
```

### Async Processing (ai-cat --async)

```
CLI Input → Files List
    ↓
tokio::runtime::Runtime
    ↓
futures::stream (buffer_unordered)
    ↓
Concurrent async_read_file()
    ↓
Results Collection
    ↓
JSONL Output
```

## Performance Architecture

### Memory Access Strategy

| File Size | Strategy | Performance |
|-----------|----------|-------------|
| < 1MB | Standard I/O | Baseline |
| 1MB - 10MB | Memory Mapping | ~3x faster |
| > 10MB | Memory Mapping | ~10x faster |

### SIMD Acceleration

```
Text Data → CPU Feature Detection
    ↓
┌───────────┬──────────┬──────────┐
│   AVX2    │   SSE2   │  Scalar  │
│  (32-byte)│ (16-byte)│  (1-byte)│
└───────────┴──────────┴──────────┘
    ↓
Result (2-4x faster than scalar)
```

### Concurrent Processing

```
File List → Stream → Concurrent Tasks (tokio)
    ↓
┌─────────────────────────────────┐
│ Task 1 │ Task 2 │ ... │ Task N  │
└─────────────────────────────────┘
    ↓
Collected Results (3x faster)
```

## Module Dependencies

```
cli binaries
    ├── memory
    ├── jsonl
    ├── error
    ├── async_ops (optional)
    ├── simd_ops (via memory)
    └── ml_ops (optional)

memory
    ├── memmap2
    └── simd_ops

async_ops
    ├── tokio
    └── futures

simd_ops
    └── (std::arch - intrinsics)

ml_ops
    └── regex
```

## Extension Points

### Adding New Utilities

1. Create binary in `src/bin/ai-utility.rs`
2. Use shared modules (memory, jsonl, error)
3. Implement CLI with clap
4. Output JSONL format
5. Add tests in `tests/`

### Adding New Patterns

1. Add to `PatternType` enum in `src/ml_ops.rs`
2. Create regex pattern
3. Implement confidence calculation
4. Add tests

### Adding SIMD Operations

1. Implement in `src/simd_ops.rs`
2. Add CPU feature detection
3. Provide scalar fallback
4. Add benchmarks

## Testing Architecture

```
tests/
├── integration/
│   ├── test_ls.rs
│   ├── test_cat.rs
│   └── ...
└── benches/
    ├── memory_access.rs
    ├── jsonl_output.rs
    └── simd_performance.rs
```

## Build Configuration

```toml
[profile.release]
lto = true
codegen-units = 1
panic = "abort"
strip = true

[features]
default = []
python = ["pyo3"]
```

## Platform Support

| Platform | Status | Notes |
|----------|--------|-------|
| Linux x86_64 | ✅ Full | AVX2/SSE2 support |
| Linux aarch64 | ✅ Full | NEON support planned |
| macOS x86_64 | ✅ Full | AVX2/SSE2 support |
| macOS aarch64 | ✅ Full | NEON support planned |
| Windows x86_64 | ✅ Full | AVX2/SSE2 support |
