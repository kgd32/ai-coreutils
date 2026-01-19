# Async Operations Guide

Guide to async/await patterns and concurrent file processing in AI-Coreutils.

## Overview

AI-Coreutils supports async/await operations for concurrent file processing using Tokio. This provides:

- **3x faster** processing for multiple files
- **Non-blocking I/O** for better responsiveness
- **Concurrent operations** with configurable limits
- **Streaming results** as they complete

## Enabling Async Mode

### CLI Usage

```bash
# Enable async mode for multiple files
ai-cat --async *.log

# Specify concurrency limit
ai-cat --async --max-concurrent 20 *.log

# Recursive async grep
ai-grep --async -r "pattern" /large/directory
```

### Library Usage

```rust
use ai_coreutils::async_ops::{async_read_file, AsyncConfig};
use tokio::runtime::Runtime;

#[tokio::main]
async fn main() -> Result<()> {
    // Async file reading
    let data = async_read_file(Path::new("file.txt")).await?;

    // Concurrent processing
    let files = vec![PathBuf::from("a.txt"), PathBuf::from("b.txt")];
    let config = AsyncConfig {
        max_concurrent: 10,
        buffer_size: 8192,
        progress: false,
    };

    let results = async_process_files_concurrently(files, config).await?;
    Ok(())
}
```

## Async API

### Configuration

```rust
use ai_coreutils::async_ops::AsyncConfig;

pub struct AsyncConfig {
    /// Maximum concurrent operations
    pub max_concurrent: usize,

    /// Buffer size for I/O operations
    pub buffer_size: usize,

    /// Enable progress reporting
    pub progress: bool,
}

// Default configuration
let config = AsyncConfig {
    max_concurrent: 10,
    buffer_size: 8192,
    progress: false,
};
```

### Async Functions

#### `async_read_file`

```rust
pub async fn async_read_file(path: &Path) -> Result<Vec<u8>>
```

Asynchronously reads an entire file into memory.

```rust
use ai_coreutils::async_ops::async_read_file;

let data = async_read_file(Path::new("file.txt")).await?;
```

#### `async_write_file`

```rust
pub async fn async_write_file(path: &Path, data: &[u8]) -> Result<()>
```

Asynchronously writes data to a file.

```rust
use ai_coreutils::async_ops::async_write_file;

async_write_file(Path::new("output.txt"), b"Hello, world!").await?;
```

#### `async_read_lines`

```rust
pub async fn async_read_lines(path: &Path) -> Result<Vec<String>>
```

Asynchronously reads a file as lines.

```rust
use ai_coreutils::async_ops::async_read_lines;

let lines = async_read_lines(Path::new("file.txt")).await?;
```

#### `async_walk_dir`

```rust
pub async fn async_walk_dir(dir: &Path) -> Result<Vec<PathBuf>>
```

Asynchronously walks a directory tree.

```rust
use ai_coreutils::async_ops::async_walk_dir;

let files = async_walk_dir(Path::new("./src")).await?;
```

#### `async_copy_file`

```rust
pub async fn async_copy_file(src: &Path, dst: &Path) -> Result<()>
```

Asynchronously copies a file.

```rust
use ai_coreutils::async_ops::async_copy_file;

async_copy_file(Path::new("src.txt"), Path::new("dst.txt")).await?;
```

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

```rust
use ai_coreutils::async_ops::async_grep_file;

let matches = async_grep_file(
    Path::new("file.txt"),
    "error",
    true,  // case insensitive
    false  // don't invert
).await?;
```

#### `async_wc`

```rust
pub async fn async_wc(path: &Path) -> Result<WordCount>
```

Asynchronously counts lines, words, and bytes.

```rust
use ai_coreutils::async_ops::async_wc;

let counts = async_wc(Path::new("file.txt")).await?;
```

#### `async_process_files_concurrently`

```rust
pub async fn async_process_files_concurrently(
    files: Vec<PathBuf>,
    config: AsyncConfig
) -> Result<Vec<JsonlRecord>>
```

Processes multiple files concurrently with a limit.

```rust
use ai_coreutils::async_ops::{async_process_files_concurrently, AsyncConfig};

let files = vec![
    PathBuf::from("a.txt"),
    PathBuf::from("b.txt"),
    PathBuf::from("c.txt"),
];

let config = AsyncConfig {
    max_concurrent: 5,
    buffer_size: 8192,
    progress: true,
};

let results = async_process_files_concurrently(files, config).await?;
```

## Concurrent Processing

### Stream-based Processing

```rust
use futures::stream::{self, StreamExt};

async fn process_files_concurrent(files: Vec<PathBuf>) -> Result<()> {
    let config = AsyncConfig {
        max_concurrent: 10,
        buffer_size: 8192,
        progress: false,
    };

    let results = stream::iter(files)
        .map(|file| async move {
            let data = async_read_file(&file).await?;
            Ok((file, data))
        })
        .buffer_unordered(config.max_concurrent)
        .collect::<Vec<_>>()
        .await;

    for result in results {
        match result {
            Ok((file, data)) => {
                println!("Processed {}: {} bytes", file.display(), data.len());
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }

    Ok(())
}
```

### Batch Processing

```rust
async fn process_in_batches(files: Vec<PathBuf>, batch_size: usize) -> Result<()> {
    for chunk in files.chunks(batch_size) {
        let futures: Vec<_> = chunk.iter()
            .map(|file| async_read_file(file))
            .collect();

        let results = futures::future::join_all(futures).await;

        for result in results {
            match result {
                Ok(data) => {
                    // Process data
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
        }
    }

    Ok(())
}
```

## Performance Tuning

### Concurrency Limits

```rust
// Few large files
let config = AsyncConfig {
    max_concurrent: 5,   // Lower limit for large files
    buffer_size: 65536,  // Larger buffer for large files
    progress: false,
};

// Many small files
let config = AsyncConfig {
    max_concurrent: 50,  // Higher limit for small files
    buffer_size: 4096,   // Smaller buffer for small files
    progress: false,
};
```

### Buffer Sizes

| Use Case | Buffer Size | Reason |
|----------|-------------|--------|
| Small files (< 1MB) | 4KB | Minimal overhead |
| Medium files (1-100MB) | 8KB | Balanced |
| Large files (> 100MB) | 64KB | Better throughput |

## Use Cases

### Log Aggregation

```rust
async fn aggregate_logs(log_files: Vec<PathBuf>) -> Result<Vec<String>> {
    let mut all_lines = Vec::new();

    for chunk in log_files.chunks(10) {
        let futures: Vec<_> = chunk.iter()
            .map(|file| async_read_lines(file))
            .collect();

        let results = futures::future::join_all(futures).await;

        for result in results {
            all_lines.extend(result?);
        }
    }

    Ok(all_lines)
}
```

### Concurrent Search

```rust
async fn search_concurrent(
    files: Vec<PathBuf>,
    pattern: &str
) -> Result<Vec<PathBuf>> {
    let pattern = pattern.to_string();
    let config = AsyncConfig {
        max_concurrent: 20,
        buffer_size: 8192,
        progress: false,
    };

    let results = stream::iter(files)
        .map(|file| {
            let pattern = pattern.clone();
            async move {
                let matches = async_grep_file(&file, &pattern, true, false).await?;
                Ok((file, matches.len()))
            }
        })
        .buffer_unordered(config.max_concurrent)
        .collect::<Vec<_>>()
        .await;

    let matching_files: Vec<_> = results
        .into_iter()
        .filter_map(|r| r.ok())
        .filter(|(_, count)| *count > 0)
        .map(|(file, _)| file)
        .collect();

    Ok(matching_files)
}
```

### Concurrent Copy

```rust
async fn copy_concurrent(
    sources: Vec<(PathBuf, PathBuf)>
) -> Result<()> {
    let config = AsyncConfig {
        max_concurrent: 10,
        buffer_size: 65536,
        progress: true,
    };

    stream::iter(sources)
        .map(|(src, dst)| async move {
            async_copy_file(&src, &dst).await?;
            Ok((src, dst))
        })
        .buffer_unordered(config.max_concurrent)
        .collect::<Vec<_>>()
        .await;

    Ok(())
}
```

## Error Handling

### Collecting Errors

```rust
async fn process_with_error_collection(
    files: Vec<PathBuf>
) -> (Vec<PathBuf>, Vec<(PathBuf, Error)>) {
    let config = AsyncConfig {
        max_concurrent: 10,
        buffer_size: 8192,
        progress: false,
    };

    let results = stream::iter(files)
        .map(|file| async move {
            let result = async_read_file(&file).await;
            (file, result)
        })
        .buffer_unordered(config.max_concurrent)
        .collect::<Vec<_>>()
        .await;

    let mut success = Vec::new();
    let mut errors = Vec::new();

    for (file, result) in results {
        match result {
            Ok(_) => success.push(file),
            Err(e) => errors.push((file, e)),
        }
    }

    (success, errors)
}
```

## Runtime Management

### Creating a Runtime

```rust
use tokio::runtime::Runtime;

let rt = Runtime::new()?;
rt.block_on(async {
    // Your async code here
    Ok(())
});
```

### Current Thread Runtime

```rust
use tokio::runtime::Builder;

let rt = Builder::new_current_thread()
    .enable_all()
    .build()?;

rt.block_on(async {
    // Single-threaded async code
    Ok(())
});
```

## Best Practices

1. **Use appropriate concurrency**: More isn't always better
2. **Handle errors gracefully**: Don't let one failure stop all operations
3. **Process in chunks**: For very large file lists
4. **Monitor progress**: Enable progress for long operations
5. **Tune buffer sizes**: Match buffer size to expected file size

## Troubleshooting

### Too Many Open Files

```rust
// Reduce concurrency
let config = AsyncConfig {
    max_concurrent: 50,  // Reduce from higher value
    buffer_size: 8192,
    progress: false,
};
```

### Memory Pressure

```rust
// Process in smaller batches
for chunk in files.chunks(100) {
    process_batch(chunk).await?;
}
```

### Slow Performance

```rust
// Increase buffer size for large files
let config = AsyncConfig {
    max_concurrent: 10,
    buffer_size: 65536,  // 64KB buffer
    progress: false,
};
```
