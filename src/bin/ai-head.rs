use ai_coreutils::{jsonl, memory::SafeMemoryAccess, Result};
use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, Read, Write};
use std::path::PathBuf;

/// AI-optimized head utility - Output first part of files
///
/// This utility extends GNU head with:
/// - JSONL structured output
/// - Memory-mapped file access for large files
/// - Detailed metadata
#[derive(Parser, Debug)]
#[command(name = "ai-head")]
#[command(about = "Output first part of files", long_about = None)]
struct Cli {
    /// Files to read
    #[arg(required = false)]
    files: Vec<PathBuf>,

    /// Number of lines to show
    #[arg(short = 'n', long, default_value = "10")]
    lines: usize,

    /// Number of bytes to show
    #[arg(short = 'c', long)]
    bytes: Option<usize>,

    /// Quiet mode - don't print file headers
    #[arg(short, long)]
    quiet: bool,

    /// Verbose mode - always print file headers
    #[arg(short = 'v', long)]
    verbose: bool,

    /// Zero-terminated output
    #[arg(short = 'z', long)]
    zero_terminated: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // If no files specified, read from stdin
    if cli.files.is_empty() {
        handle_stdin(&cli)?;
        return Ok(());
    }

    let use_bytes = cli.bytes.is_some();
    let count = cli.bytes.unwrap_or(cli.lines);

    // Output start message
    jsonl::output_progress(0, cli.files.len(), "Starting head operation")?;

    for (index, file) in cli.files.iter().enumerate() {
        // Update progress
        jsonl::output_progress(
            index + 1,
            cli.files.len(),
            &format!("Processing: {}", file.display()),
        )?;

        // Print header if needed
        let show_header = cli.verbose || (cli.files.len() > 1 && !cli.quiet);

        if show_header {
            println!("==> {} <==", file.display());
        }

        match head_file(file, count, use_bytes, cli.zero_terminated) {
            Ok(bytes_read) => {
                jsonl::output_info(serde_json::json!({
                    "file": file.display().to_string(),
                    "operation": "head",
                    "unit": if use_bytes { "bytes" } else { "lines" },
                    "count": count,
                    "bytes_read": bytes_read,
                }))?;
            }
            Err(e) => {
                jsonl::output_error(
                    &format!("Failed to read {}: {}", file.display(), e),
                    "HEAD_ERROR",
                    Some(file.display().to_string().as_str()),
                )?;
            }
        }

        // Add separator between files
        if show_header && index < cli.files.len() - 1 {
            println!();
        }
    }

    Ok(())
}

fn handle_stdin(cli: &Cli) -> Result<()> {
    let mut stdin = io::stdin();
    let use_bytes = cli.bytes.is_some();
    let count = cli.bytes.unwrap_or(cli.lines);

    if use_bytes {
        let mut buffer = vec![0u8; count.min(1024 * 1024)]; // Max 1MB buffer
        let n = stdin.read(&mut buffer)?;
        buffer.truncate(n);
        io::stdout().write_all(&buffer)?;
    } else {
        let separator = if cli.zero_terminated { b'\0' } else { b'\n' };
        let reader = stdin.lock();
        let mut line_reader = io::BufReader::new(reader);
        let mut line = Vec::new();

        for _ in 0..count {
            line.clear();
            let n = line_reader.read_until(separator, &mut line)?;
            if n == 0 {
                break;
            }
            io::stdout().write_all(&line)?;
        }
    }

    Ok(())
}

fn head_file(
    file: &PathBuf,
    count: usize,
    use_bytes: bool,
    zero_terminated: bool,
) -> Result<usize> {
    // Try to use memory mapping for files
    if let Ok(mmap) = SafeMemoryAccess::new(file) {
        return head_mmap(&mmap, count, use_bytes, zero_terminated);
    }

    // Fall back to standard I/O
    let mut f = File::open(file).map_err(|e| ai_coreutils::AiCoreutilsError::Io(e))?;

    if use_bytes {
        let mut buffer = vec![0u8; count.min(1024 * 1024)]; // Max 1MB buffer
        let n = f.read(&mut buffer)?;
        buffer.truncate(n);
        io::stdout().write_all(&buffer)?;
        return Ok(n);
    }

    // Read lines
    let separator = if zero_terminated { b'\0' } else { b'\n' };
    let reader = io::BufReader::new(f);
    let mut line_reader = io::BufReader::new(reader);
    let mut line = Vec::new();
    let mut bytes_read = 0;

    for _ in 0..count {
        line.clear();
        let n = line_reader.read_until(separator, &mut line)?;
        if n == 0 {
            break;
        }
        bytes_read += n;
        io::stdout().write_all(&line)?;
    }

    Ok(bytes_read)
}

fn head_mmap(
    mmap: &SafeMemoryAccess,
    count: usize,
    use_bytes: bool,
    zero_terminated: bool,
) -> Result<usize> {
    let size = mmap.size();

    if use_bytes {
        // Read first N bytes
        let bytes_to_read = count.min(size);
        if let Some(data) = mmap.get(0, bytes_to_read) {
            io::stdout().write_all(data)?;
            return Ok(bytes_to_read);
        }
        return Ok(0);
    }

    // Read first N lines
    let separator = if zero_terminated { 0 } else { b'\n' };
    let mut lines_found = 0;
    let mut last_end = 0;

    // Scan through memory looking for line separators
    for i in 0..size {
        let byte = mmap.get(i, 1).map(|bytes| bytes[0]);

        if byte == Some(separator) || byte == Some(b'\n') {
            lines_found += 1;
            last_end = i + 1;

            if lines_found >= count {
                break;
            }
        }
    }

    // Output the data
    if last_end > 0 {
        if let Some(data) = mmap.get(0, last_end) {
            io::stdout().write_all(data)?;
            return Ok(last_end);
        }
    }

    Ok(0)
}
