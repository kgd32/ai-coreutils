# Async Workflows Examples

Examples of async/await patterns with AI-Coreutils.

## Table of Contents

1. [Concurrent File Processing](#concurrent-file-processing)
2. [Stream Processing](#stream-processing)
3. [Error Handling](#error-handling)
4. [Performance Patterns](#performance-patterns)

## Concurrent File Processing

### Basic Concurrent Reading

```rust
use ai_coreutils::async_ops::async_read_file;
use futures::future::join_all;

async fn read_multiple_files(files: Vec<PathBuf>) -> Result<Vec<Vec<u8>>> {
    let futures: Vec<_> = files
        .iter()
        .map(|file| async_read_file(file))
        .collect();

    let results = join_all(futures).await;

    // Handle errors
    let mut contents = Vec::new();
    for result in results {
        contents.push(result?);
    }

    Ok(contents)
}
```

### Concurrent Search

```rust
use ai_coreutils::async_ops::{async_grep_file, async_walk_dir};
use futures::stream::{self, StreamExt};

async fn search_concurrent(
    directory: &Path,
    pattern: &str
) -> Result<Vec<PathBuf>> {
    let files = async_walk_dir(directory).await?;
    let pattern = pattern.to_string();

    let results = stream::iter(files)
        .map(move |file| {
            let pattern = pattern.clone();
            async move {
                let matches = async_grep_file(&file, &pattern, true, false).await?;
                Ok((file, matches.len()))
            }
        })
        .buffer_unordered(10) // Process 10 files concurrently
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

### Batch Processing

```rust
use ai_coreutils::async_ops::{async_read_file, AsyncConfig};
use std::sync::Arc;

async fn process_batches(files: Vec<PathBuf>) -> Result<()> {
    let config = Arc::new(AsyncConfig {
        max_concurrent: 10,
        buffer_size: 8192,
        progress: true,
    });

    for chunk in files.chunks(50) {
        let futures: Vec<_> = chunk
            .iter()
            .map(|file| {
                let config = config.clone();
                async move {
                    let data = async_read_file(file).await?;
                    // Process data
                    Ok(())
                }
            })
            .collect();

        futures::future::join_all(futures).await;
    }

    Ok(())
}
```

## Stream Processing

### Real-time Processing

```rust
use futures::stream::{self, StreamExt};

async fn process_stream(files: Vec<PathBuf>) -> Result<()> {
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

### Pipeline Processing

```rust
use futures::stream::{self, StreamExt};

async fn pipeline_processing(files: Vec<PathBuf>) -> Result<()> {
    let stage1 = stream::iter(files)
        .map(|file| async move {
            async_read_file(&file).await
        })
        .buffer_unordered(10);

    let stage2 = stage1.map(|result| async move {
        let data = result?;
        // Process data
        Ok(data)
    });

    let stage3 = stage2.buffer_unordered(5);

    stage3.for_each(|result| async {
        match result {
            Ok(data) => {
                // Output result
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    })
    .await;

    Ok(())
}
```

## Error Handling

### Collecting Errors

```rust
use futures::stream::{self, StreamExt};

async fn process_with_errors(files: Vec<PathBuf>) -> (Vec<PathBuf>, Vec<(PathBuf, Error)>) {
    let results = stream::iter(files)
        .map(|file| async move {
            let result = async_read_file(&file).await;
            (file, result)
        })
        .buffer_unordered(10)
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

### Retry Logic

```rust
async fn read_with_retry(path: &Path, max_retries: usize) -> Result<Vec<u8>> {
    let mut attempt = 0;

    loop {
        match async_read_file(path).await {
            Ok(data) => return Ok(data),
            Err(e) if attempt < max_retries => {
                attempt += 1;
                tokio::time::sleep(tokio::time::Duration::from_millis(100 * attempt as u64)).await;
            }
            Err(e) => return Err(e),
        }
    }
}
```

## Performance Patterns

### Tuning Concurrency

```rust
use ai_coreutils::async_ops::AsyncConfig;

fn config_for_small_files() -> AsyncConfig {
    AsyncConfig {
        max_concurrent: 50,  // Many small files
        buffer_size: 4096,   // Small buffer
        progress: false,
    }
}

fn config_for_large_files() -> AsyncConfig {
    AsyncConfig {
        max_concurrent: 5,   // Few large files
        buffer_size: 65536,  // Large buffer
        progress: true,      // Show progress
    }
}
```

### Backpressure

```rust
use futures::stream::{self, StreamExt};

async fn process_with_backpressure(files: Vec<PathBuf>) -> Result<()> {
    let results = stream::iter(files)
        .map(|file| async move {
            let data = async_read_file(&file).await?;
            // Simulate processing
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            Ok(data.len())
        })
        .buffer_unordered(5) // Limit concurrency
        .collect::<Vec<_>>()
        .await;

    Ok(())
}
```

### Rate Limiting

```rust
use tokio::sync::Semaphore;
use futures::stream::{self, StreamExt};

async fn process_with_rate_limit(files: Vec<PathBuf>) -> Result<()> {
    let semaphore = Arc::new(Semaphore::new(10)); // Max 10 concurrent

    let results = stream::iter(files)
        .map(|file| {
            let semaphore = semaphore.clone();
            async move {
                let _permit = semaphore.acquire().await?;
                async_read_file(&file).await
            }
        })
        .buffer_unordered(10)
        .collect::<Vec<_>>()
        .await;

    Ok(())
}
```

## Real-World Examples

### Log Aggregation

```rust
use ai_coreutils::async_ops::{async_read_file, async_walk_dir};

async fn aggregate_logs(log_dir: &Path) -> Result<Vec<String>> {
    let files = async_walk_dir(log_dir).await?;
    let log_files: Vec<_> = files
        .into_iter()
        .filter(|f| f.extension().map_or(false, |e| e == "log"))
        .collect();

    let results = stream::iter(log_files)
        .map(|file| async move {
            let data = async_read_file(&file).await?;
            let text = String::from_utf8_lossy(&data).to_string();
            Ok(text)
        })
        .buffer_unordered(10)
        .collect::<Vec<_>>()
        .await;

    let mut all_lines = Vec::new();
    for result in results {
        let text = result?;
        all_lines.extend(text.lines().map(|s| s.to_string()));
    }

    Ok(all_lines)
}
```

### Parallel Search

```rust
async fn parallel_search(
    files: Vec<PathBuf>,
    patterns: Vec<String>
) -> Result<HashMap<String, Vec<PathBuf>>> {
    let pattern_stream = stream::iter(patterns);
    let file_stream = stream::iter(files);

    // Cartesian product: search all patterns in all files
    let results = pattern_stream
        .map(move |pattern| {
            let files = files.clone();
            async move {
                let matching_files = search_files_for_pattern(&files, &pattern).await?;
                Ok((pattern, matching_files))
            }
        })
        .buffer_unordered(5)
        .collect::<Vec<_>>()
        .await;

    let mut results_map = HashMap::new();
    for result in results {
        let (pattern, files) = result?;
        results_map.insert(pattern, files);
    }

    Ok(results_map)
}

async fn search_files_for_pattern(
    files: &[PathBuf],
    pattern: &str
) -> Result<Vec<PathBuf>> {
    let pattern = pattern.to_string();
    let results = stream::iter(files.iter().cloned())
        .map(|file| {
            let pattern = pattern.clone();
            async move {
                let matches = async_grep_file(&file, &pattern, true, false).await?;
                Ok((file, matches.len()))
            }
        })
        .buffer_unordered(10)
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

### Batch Data Processing

```rust
async fn process_data_batches(
    input_dir: &Path,
    output_dir: &Path,
    batch_size: usize
) -> Result<()> {
    let files = async_walk_dir(input_dir).await?;

    for chunk in files.chunks(batch_size) {
        let results = stream::iter(chunk.iter().cloned())
            .map(|file| {
                let output_dir = output_dir.to_path_buf();
                async move {
                    let data = async_read_file(&file).await?;

                    // Process data
                    let processed = process_data(&data)?;

                    // Write output
                    let output_path = output_dir.join(file.file_name().unwrap());
                    async_write_file(&output_path, &processed).await?;

                    Ok(())
                }
            })
            .buffer_unordered(10)
            .collect::<Vec<_>>()
            .await;

        // Check for errors
        for result in results {
            result?;
        }
    }

    Ok(())
}

fn process_data(data: &[u8]) -> Result<Vec<u8>> {
    // Your processing logic
    Ok(data.to_vec())
}
```

### Concurrent File Copy

```rust
use ai_coreutils::async_ops::{async_copy_file, async_walk_dir};

async fn concurrent_copy(
    source_dir: &Path,
    dest_dir: &Path,
    max_concurrent: usize
) -> Result<()> {
    let files = async_walk_dir(source_dir).await?;

    let results = stream::iter(files)
        .map(|file| {
            let source_dir = source_dir.to_path_buf();
            let dest_dir = dest_dir.to_path_buf();

            async move {
                let relative_path = file.strip_prefix(&source_dir)?;
                let dest_path = dest_dir.join(relative_path);

                // Create parent directory
                if let Some(parent) = dest_path.parent() {
                    tokio::fs::create_dir_all(parent).await?;
                }

                async_copy_file(&file, &dest_path).await
            }
        })
        .buffer_unordered(max_concurrent)
        .collect::<Vec<_>>()
        .await;

    // Check for errors
    for result in results {
        result?;
    }

    Ok(())
}
```

## CLI Examples

### Async Cat with Concurrency

```bash
# Process 20 files concurrently
ai-cat --async --max-concurrent 20 *.log

# Use default concurrency (10)
ai-cat --async *.txt
```

### Async Grep with Concurrency

```bash
# Search with high concurrency
ai-grep --async --max-concurrent 50 -r "pattern" /large/dir

# Default concurrency
ai-grep --async -r "pattern" /medium/dir
```

## Best Practices

1. **Choose appropriate concurrency** based on file size
2. **Handle errors gracefully** - don't let one failure stop all
3. **Use streams** for large collections
4. **Tune buffer sizes** based on expected file size
5. **Monitor progress** for long operations
6. **Use rate limiting** to avoid overwhelming resources
