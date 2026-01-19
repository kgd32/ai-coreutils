//! Async operations for AI-Coreutils
//!
//! This module provides async/await variants of file system operations
//! for concurrent I/O processing and improved performance.

use crate::error::{AiCoreutilsError, Result};
use crate::jsonl;
use futures::stream::{self, StreamExt};
use std::path::{Path, PathBuf};
use tokio::fs;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};

/// Configuration for async operations
#[derive(Debug, Clone)]
pub struct AsyncConfig {
    /// Maximum concurrent operations
    pub max_concurrent: usize,
    /// Buffer size for I/O operations
    pub buffer_size: usize,
    /// Enable progress reporting
    pub progress: bool,
}

impl Default for AsyncConfig {
    fn default() -> Self {
        Self {
            max_concurrent: 10,
            buffer_size: 8192,
            progress: false,
        }
    }
}

/// Read a file asynchronously
pub async fn async_read_file(path: &Path) -> Result<Vec<u8>> {
    let mut file = fs::File::open(path)
        .await
        .map_err(|e| AiCoreutilsError::Io(e))?;
    let metadata = file.metadata().await.map_err(|e| AiCoreutilsError::Io(e))?;
    let size = metadata.len() as usize;

    let mut buffer = Vec::with_capacity(size);
    file.read_to_end(&mut buffer)
        .await
        .map_err(|e| AiCoreutilsError::Io(e))?;

    Ok(buffer)
}

/// Read a file as text asynchronously
pub async fn async_read_file_to_string(path: &Path) -> Result<String> {
    let contents = async_read_file(path).await?;
    String::from_utf8(contents).map_err(|e| AiCoreutilsError::InvalidInput(e.to_string()))
}

/// Write data to a file asynchronously
pub async fn async_write_file(path: &Path, data: &[u8]) -> Result<()> {
    let mut file = fs::File::create(path)
        .await
        .map_err(|e| AiCoreutilsError::Io(e))?;
    file.write_all(data)
        .await
        .map_err(|e| AiCoreutilsError::Io(e))?;
    file.flush().await.map_err(|e| AiCoreutilsError::Io(e))?;
    Ok(())
}

/// Append data to a file asynchronously
pub async fn async_append_file(path: &Path, data: &[u8]) -> Result<()> {
    let mut file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(path)
        .await
        .map_err(|e| AiCoreutilsError::Io(e))?;
    file.write_all(data)
        .await
        .map_err(|e| AiCoreutilsError::Io(e))?;
    file.flush().await.map_err(|e| AiCoreutilsError::Io(e))?;
    Ok(())
}

/// Read a file line by line asynchronously
pub async fn async_read_lines<F>(path: &Path, mut callback: F) -> Result<()>
where
    F: FnMut(usize, String) -> Result<()>,
{
    let file = fs::File::open(path)
        .await
        .map_err(|e| AiCoreutilsError::Io(e))?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let mut line_num = 0;
    while let Some(line) = lines
        .next_line()
        .await
        .map_err(|e| AiCoreutilsError::Io(e))?
    {
        line_num += 1;
        callback(line_num, line)?;
    }

    Ok(())
}

/// Recursively walk a directory asynchronously
pub async fn async_walk_dir(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut entries = Vec::new();

    async_walk_dir_recursive(dir, &mut entries).await?;

    Ok(entries)
}

/// Helper for recursive directory walking
fn async_walk_dir_recursive<'a>(
    dir: &'a Path,
    entries: &'a mut Vec<PathBuf>,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<()>> + 'a + Send>> {
    Box::pin(async move {
        let mut dir_entry = fs::read_dir(dir)
            .await
            .map_err(|e| AiCoreutilsError::Io(e))?;

        while let Some(entry) = dir_entry
            .next_entry()
            .await
            .map_err(|e| AiCoreutilsError::Io(e))?
        {
            let path = entry.path();
            let file_type = entry
                .file_type()
                .await
                .map_err(|e| AiCoreutilsError::Io(e))?;

            if file_type.is_dir() {
                async_walk_dir_recursive(&path, entries).await?;
            } else if file_type.is_file() {
                entries.push(path);
            }
        }

        Ok(())
    })
}

/// Process multiple files concurrently
pub async fn async_process_files_concurrently<F>(
    files: Vec<PathBuf>,
    config: &AsyncConfig,
    process_fn: F,
) -> Result<()>
where
    F: Fn(PathBuf) -> Result<()> + Send + Sync + 'static,
{
    let config = config.clone();

    // Report progress if enabled
    if config.progress {
        jsonl::output_info(serde_json::json!({
            "operation": "async_process_start",
            "file_count": files.len(),
            "max_concurrent": config.max_concurrent,
        }))?;
    }

    // Process files in batches
    let results = stream::iter(files)
        .map(|file| {
            let process_fn = &process_fn;
            async move {
                let result = process_fn(file.clone());
                (file, result)
            }
        })
        .buffer_unordered(config.max_concurrent)
        .collect::<Vec<_>>()
        .await;

    // Check results
    let mut success_count = 0;
    let mut error_count = 0;

    for (path, result) in results {
        match result {
            Ok(()) => success_count += 1,
            Err(e) => {
                error_count += 1;
                jsonl::output_error(
                    &format!("Error processing file: {}", e),
                    "PROCESSING_ERROR",
                    Some(&path.to_string_lossy()),
                )?;
            }
        }
    }

    // Report completion
    if config.progress {
        jsonl::output_info(serde_json::json!({
            "operation": "async_process_complete",
            "success_count": success_count,
            "error_count": error_count,
        }))?;
    }

    Ok(())
}

/// Copy a file asynchronously with progress
pub async fn async_copy_file(src: &Path, dest: &Path, config: &AsyncConfig) -> Result<u64> {
    let mut src_file = fs::File::open(src)
        .await
        .map_err(|e| AiCoreutilsError::Io(e))?;

    let metadata = src_file
        .metadata()
        .await
        .map_err(|e| AiCoreutilsError::Io(e))?;
    let total_size = metadata.len();

    let mut dest_file = fs::File::create(dest)
        .await
        .map_err(|e| AiCoreutilsError::Io(e))?;

    let mut buffer = vec![0u8; config.buffer_size];
    let mut copied: u64 = 0;

    loop {
        let n = src_file
            .read(&mut buffer)
            .await
            .map_err(|e| AiCoreutilsError::Io(e))?;

        if n == 0 {
            break;
        }

        dest_file
            .write_all(&buffer[..n])
            .await
            .map_err(|e| AiCoreutilsError::Io(e))?;

        copied += n as u64;

        if config.progress && copied % (1024 * 1024) == 0 {
            jsonl::output_progress(copied as usize, total_size as usize, "Copying file")?;
        }
    }

    dest_file
        .flush()
        .await
        .map_err(|e| AiCoreutilsError::Io(e))?;

    if config.progress {
        jsonl::output_info(serde_json::json!({
            "operation": "copy_complete",
            "source": src.display().to_string(),
            "destination": dest.display().to_string(),
            "bytes_copied": copied,
        }))?;
    }

    Ok(copied)
}

/// Count lines, words, and bytes in a file asynchronously
pub async fn async_wc(path: &Path) -> Result<WcCounts> {
    let file = fs::File::open(path)
        .await
        .map_err(|e| AiCoreutilsError::Io(e))?;
    let reader = BufReader::new(file);

    let mut lines = 0u64;
    let mut words = 0u64;
    let mut bytes = 0u64;
    let mut in_word = false;

    let mut line_reader = reader.lines();
    while let Some(line) = line_reader
        .next_line()
        .await
        .map_err(|e| AiCoreutilsError::Io(e))?
    {
        lines += 1;
        bytes += line.len() as u64 + 1; // +1 for newline

        for c in line.chars() {
            if c.is_whitespace() {
                if in_word {
                    words += 1;
                    in_word = false;
                }
            } else {
                in_word = true;
            }
        }

        // Count word at end of line
        if in_word {
            words += 1;
            in_word = false;
        }
    }

    // Handle last word if file doesn't end with newline
    if in_word {
        words += 1;
    }

    Ok(WcCounts { lines, words, bytes })
}

/// Word count results
#[derive(Debug, Clone)]
pub struct WcCounts {
    /// Number of lines
    pub lines: u64,
    /// Number of words
    pub words: u64,
    /// Number of bytes
    pub bytes: u64,
}

/// Search for a pattern in a file asynchronously
pub async fn async_grep_file(
    path: &Path,
    pattern: &str,
    case_insensitive: bool,
    invert_match: bool,
) -> Result<Vec<GrepMatch>> {
    let contents = async_read_file_to_string(path).await?;
    let search_pattern = if case_insensitive {
        pattern.to_lowercase()
    } else {
        pattern.to_string()
    };

    let mut matches = Vec::new();

    for (line_num, line) in contents.lines().enumerate() {
        let search_line = if case_insensitive {
            line.to_lowercase()
        } else {
            line.to_string()
        };

        let is_match = search_line.contains(&search_pattern);
        let should_include = if invert_match { !is_match } else { is_match };

        if should_include {
            matches.push(GrepMatch {
                line_number: line_num + 1,
                line: line.to_string(),
                path: path.to_path_buf(),
            });
        }
    }

    Ok(matches)
}

/// Grep match result
#[derive(Debug, Clone)]
pub struct GrepMatch {
    /// Line number (1-indexed)
    pub line_number: usize,
    /// Matching line content
    pub line: String,
    /// Path to the file containing the match
    pub path: PathBuf,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn test_async_read_write_file() {
        let temp_file = NamedTempFile::new().unwrap();
        let data = b"Hello, async world!";

        async_write_file(temp_file.path(), data).await.unwrap();

        let read_data = async_read_file(temp_file.path()).await.unwrap();
        assert_eq!(read_data, data);
    }

    #[tokio::test]
    async fn test_async_read_lines() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "Line 1").unwrap();
        writeln!(temp_file, "Line 2").unwrap();
        writeln!(temp_file, "Line 3").unwrap();

        let mut lines = Vec::new();
        async_read_lines(temp_file.path(), |num, line| {
            lines.push((num, line));
            Ok(())
        })
        .await
        .unwrap();

        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0].0, 1);
        assert_eq!(lines[0].1, "Line 1");
    }

    #[tokio::test]
    async fn test_async_wc() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "Hello world").unwrap();
        writeln!(temp_file, "Second line").unwrap();

        let counts = async_wc(temp_file.path()).await.unwrap();
        assert_eq!(counts.lines, 2);
        assert_eq!(counts.words, 4);
    }

    #[tokio::test]
    async fn test_async_grep() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "Hello world").unwrap();
        writeln!(temp_file, "Hello there").unwrap();
        writeln!(temp_file, "Goodbye").unwrap();

        let matches = async_grep_file(temp_file.path(), "Hello", false, false)
            .await
            .unwrap();

        assert_eq!(matches.len(), 2);
        assert_eq!(matches[0].line_number, 1);
        assert!(matches[0].line.contains("Hello"));
    }

    #[tokio::test]
    async fn test_async_grep_case_insensitive() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "HELLO world").unwrap();
        writeln!(temp_file, "hello there").unwrap();

        let matches = async_grep_file(temp_file.path(), "hello", true, false)
            .await
            .unwrap();

        assert_eq!(matches.len(), 2);
    }

    #[tokio::test]
    async fn test_async_grep_invert() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "Hello world").unwrap();
        writeln!(temp_file, "Goodbye").unwrap();

        let matches = async_grep_file(temp_file.path(), "Hello", false, true)
            .await
            .unwrap();

        assert_eq!(matches.len(), 1);
        assert!(matches[0].line.contains("Goodbye"));
    }
}
