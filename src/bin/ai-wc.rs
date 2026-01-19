use ai_coreutils::{jsonl, memory::SafeMemoryAccess, Result};
use clap::Parser;
use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;

/// AI-optimized wc utility - Word, line, character count
///
/// This utility extends GNU wc with:
/// - JSONL structured output
/// - Memory-mapped file access for large files
/// - Detailed statistics
#[derive(Parser, Debug)]
#[command(name = "ai-wc")]
#[command(about = "Print newline, word, and byte counts for each file", long_about = None)]
struct Cli {
    /// Files to count
    #[arg(required = false)]
    files: Vec<PathBuf>,

    /// Count lines only
    #[arg(short = 'l', long)]
    lines_only: bool,

    /// Count words only
    #[arg(short = 'w', long)]
    words_only: bool,

    /// Count bytes only
    #[arg(short = 'c', long)]
    bytes_only: bool,

    /// Count characters only
    #[arg(short = 'm', long)]
    chars_only: bool,

    /// Print maximum line length
    #[arg(short = 'L', long)]
    max_line_length: bool,
}

#[derive(Debug, Default)]
struct Counts {
    lines: usize,
    words: usize,
    bytes: usize,
    chars: usize,
    max_line_length: usize,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // If no files specified, read from stdin
    if cli.files.is_empty() {
        let counts = count_stdin(&cli)?;
        print_counts(&counts, "stdin", &cli);
        jsonl::output_info(serde_json::json!({
            "file": "stdin",
            "operation": "wc",
            "lines": counts.lines,
            "words": counts.words,
            "bytes": counts.bytes,
            "chars": counts.chars,
            "max_line_length": counts.max_line_length,
        }))?;
        return Ok(());
    }

    // Output start message
    jsonl::output_progress(0, cli.files.len(), "Starting wc operation")?;

    let mut total_counts = Counts::default();

    for (index, file) in cli.files.iter().enumerate() {
        // Update progress
        jsonl::output_progress(
            index + 1,
            cli.files.len(),
            &format!("Processing: {}", file.display()),
        )?;

        match count_file(file, &cli) {
            Ok(counts) => {
                print_counts(&counts, &file.display().to_string(), &cli);

                total_counts.lines += counts.lines;
                total_counts.words += counts.words;
                total_counts.bytes += counts.bytes;
                total_counts.chars += counts.chars;
                total_counts.max_line_length = total_counts.max_line_length.max(counts.max_line_length);

                jsonl::output_info(serde_json::json!({
                    "file": file.display().to_string(),
                    "operation": "wc",
                    "lines": counts.lines,
                    "words": counts.words,
                    "bytes": counts.bytes,
                    "chars": counts.chars,
                    "max_line_length": counts.max_line_length,
                }))?;
            }
            Err(e) => {
                jsonl::output_error(
                    &format!("Failed to count {}: {}", file.display(), e),
                    "WC_ERROR",
                    Some(file.display().to_string().as_str()),
                )?;
            }
        }
    }

    // Print total if multiple files
    if cli.files.len() > 1 {
        print_counts(&total_counts, "total", &cli);
    }

    Ok(())
}

fn count_stdin(cli: &Cli) -> Result<Counts> {
    let mut stdin = io::stdin();
    let mut buffer = Vec::new();
    stdin.read_to_end(&mut buffer)?;

    count_bytes(&buffer, cli)
}

fn count_file(file: &PathBuf, cli: &Cli) -> Result<Counts> {
    // Try to use memory mapping for files
    if let Ok(mmap) = SafeMemoryAccess::new(file) {
        return count_mmap(&mmap, cli);
    }

    // Fall back to standard I/O
    let mut f = File::open(file).map_err(|e| ai_coreutils::AiCoreutilsError::Io(e))?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).map_err(|e| ai_coreutils::AiCoreutilsError::Io(e))?;

    count_bytes(&buffer, cli)
}

fn count_mmap(mmap: &SafeMemoryAccess, _cli: &Cli) -> Result<Counts> {
    let size = mmap.size();
    let data = if let Some(d) = mmap.get(0, size) {
        d
    } else {
        return Ok(Counts::default());
    };

    // Use SIMD-accelerated text metrics for basic counts
    let (lines, words, bytes) = mmap.count_text_metrics();

    let mut counts = Counts::default();
    counts.lines = lines;
    counts.words = words;
    counts.bytes = bytes;
    counts.chars = bytes; // For ASCII, chars == bytes

    // Still need to calculate max line length
    let mut current_line_length = 0;
    for &byte in data.iter() {
        if byte == b'\n' {
            counts.max_line_length = counts.max_line_length.max(current_line_length);
            current_line_length = 0;
        } else if byte != b'\r' {
            current_line_length += 1;
        }
    }
    counts.max_line_length = counts.max_line_length.max(current_line_length);

    Ok(counts)
}

fn count_bytes(data: &[u8], _cli: &Cli) -> Result<Counts> {
    let mut counts = Counts::default();
    counts.bytes = data.len();
    counts.chars = data.len(); // For ASCII, chars == bytes; UTF-8 would need proper handling

    let mut in_word = false;
    let mut current_line_length = 0;

    for &byte in data.iter() {
        // Track line length
        if byte == b'\n' {
            counts.lines += 1;
            counts.max_line_length = counts.max_line_length.max(current_line_length);
            current_line_length = 0;
        } else if byte != b'\r' {
            current_line_length += 1;
        }

        // Track words (whitespace-separated)
        let is_whitespace = byte == b' ' || byte == b'\t' || byte == b'\n' || byte == b'\r';
        if is_whitespace {
            in_word = false;
        } else if !in_word {
            in_word = true;
            counts.words += 1;
        }
    }

    // Don't forget the last line if it doesn't end with newline
    counts.max_line_length = counts.max_line_length.max(current_line_length);

    Ok(counts)
}

fn print_counts(counts: &Counts, name: &str, cli: &Cli) {
    let mut parts = Vec::new();

    if cli.bytes_only {
        parts.push(format!("{:7}", counts.bytes));
    } else if cli.chars_only {
        parts.push(format!("{:7}", counts.chars));
    } else if cli.lines_only {
        parts.push(format!("{:7}", counts.lines));
    } else if cli.words_only {
        parts.push(format!("{:7}", counts.words));
    } else if cli.max_line_length {
        parts.push(format!("{:7}", counts.max_line_length));
    } else {
        // Default: show lines, words, bytes
        parts.push(format!("{:7}", counts.lines));
        parts.push(format!("{:7}", counts.words));
        parts.push(format!("{:7}", counts.bytes));
    }

    parts.push(name.to_string());
    println!("{}", parts.join("  "));
}
