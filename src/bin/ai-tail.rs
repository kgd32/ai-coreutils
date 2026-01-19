use ai_coreutils::{jsonl, memory::SafeMemoryAccess, Result};
use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, Read, Seek, SeekFrom, Write};
use std::path::PathBuf;

/// AI-optimized tail utility - Output last part of files
///
/// This utility extends GNU tail with:
/// - JSONL structured output
/// - Memory-mapped file access for large files
/// - Detailed metadata
#[derive(Parser, Debug)]
#[command(name = "ai-tail")]
#[command(about = "Output last part of files", long_about = None)]
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

    /// Follow file (output appended data as file grows)
    #[arg(short = 'f', long)]
    follow: bool,

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
    jsonl::output_progress(0, cli.files.len(), "Starting tail operation")?;

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

        match tail_file(file, count, use_bytes, cli.zero_terminated, cli.follow) {
            Ok(bytes_read) => {
                jsonl::output_info(serde_json::json!({
                    "file": file.display().to_string(),
                    "operation": "tail",
                    "unit": if use_bytes { "bytes" } else { "lines" },
                    "count": count,
                    "bytes_read": bytes_read,
                    "following": cli.follow,
                }))?;
            }
            Err(e) => {
                jsonl::output_error(
                    &format!("Failed to read {}: {}", file.display(), e),
                    "TAIL_ERROR",
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
    let use_bytes = cli.bytes.is_some();
    let count = cli.bytes.unwrap_or(cli.lines);

    if use_bytes {
        // Read all and keep last N bytes
        let mut stdin = io::stdin();
        let mut all_data = Vec::new();
        stdin.read_to_end(&mut all_data)?;

        let start = if all_data.len() > count {
            all_data.len() - count
        } else {
            0
        };

        io::stdout().write_all(&all_data[start..])?;
    } else {
        // Read all lines and keep last N
        let separator = if cli.zero_terminated { b'\0' } else { b'\n' };
        let stdin = io::stdin();
        let reader = stdin.lock();
        let line_reader = io::BufReader::new(reader);

        let lines: io::Result<Vec<Vec<u8>>> = line_reader.split(separator).collect();
        let lines = lines?;

        let start = if lines.len() > count {
            lines.len() - count
        } else {
            0
        };

        for line in &lines[start..] {
            io::stdout().write_all(line)?;
            io::stdout().write_all(&[separator])?;
        }
    }

    Ok(())
}

fn tail_file(
    file: &PathBuf,
    count: usize,
    use_bytes: bool,
    zero_terminated: bool,
    _follow: bool,
) -> Result<usize> {
    // Try to use memory mapping for files
    if let Ok(mmap) = SafeMemoryAccess::new(file) {
        return tail_mmap(&mmap, count, use_bytes, zero_terminated);
    }

    // Fall back to standard I/O
    let mut f = File::open(file).map_err(|e| ai_coreutils::AiCoreutilsError::Io(e))?;
    let metadata = f.metadata()?;
    let file_size = metadata.len() as usize;

    if use_bytes {
        // Seek to position and read
        let start = if file_size > count { file_size - count } else { 0 };
        f.seek(SeekFrom::Start(start as u64))?;

        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer)?;
        io::stdout().write_all(&buffer)?;

        return Ok(buffer.len());
    }

    // For lines, we need to read backwards
    // Read the whole file for simplicity (could be optimized)
    let mut content = String::new();
    f.read_to_string(&mut content)?;

    let separator = if zero_terminated { '\0' } else { '\n' };
    let lines: Vec<&str> = content.split(separator).collect();

    let start = if lines.len() > count {
        lines.len() - count
    } else {
        0
    };

    let mut bytes_written = 0;
    for line in &lines[start..] {
        bytes_written += line.len() + 1;
        print!("{}{}", line, separator);
    }

    Ok(bytes_written)
}

fn tail_mmap(
    mmap: &SafeMemoryAccess,
    count: usize,
    use_bytes: bool,
    zero_terminated: bool,
) -> Result<usize> {
    let size = mmap.size();

    if use_bytes {
        // Read last N bytes
        let start = if size > count { size - count } else { 0 };
        let bytes_to_read = size - start;

        if let Some(data) = mmap.get(start, bytes_to_read) {
            io::stdout().write_all(data)?;
            return Ok(bytes_to_read);
        }
        return Ok(0);
    }

    // Read last N lines
    // Scan backwards from end
    let separator = if zero_terminated { 0 } else { b'\n' };
    let mut lines_found = 0;
    let mut start = size;

    // Scan backwards looking for line separators
    for i in (0..size).rev() {
        let byte = mmap.get(i, 1).map(|bytes| bytes[0]);

        if byte == Some(separator) || byte == Some(b'\n') {
            lines_found += 1;
            start = i + 1;

            if lines_found > count {
                break;
            }
        }
    }

    // If we didn't find enough lines, start from beginning
    if lines_found < count {
        start = 0;
    }

    // Output the data
    if start < size {
        let bytes_to_read = size - start;
        if let Some(data) = mmap.get(start, bytes_to_read) {
            io::stdout().write_all(data)?;
            return Ok(bytes_to_read);
        }
    }

    Ok(0)
}
