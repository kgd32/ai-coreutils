//! AI-optimized grep utility
//!
//! Searches for patterns in files with structured JSONL output.

use ai_coreutils::{jsonl::JsonlRecord, memory::SafeMemoryAccess, Result};
use clap::Parser;
use std::path::PathBuf;

/// AI-optimized grep: Search files with JSONL output
#[derive(Parser, Debug)]
#[command(name = "ai-grep")]
#[command(about = "AI-optimized grep with structured output", long_about = None)]
struct Cli {
    /// Pattern to search for
    pattern: String,

    /// Files to search
    #[arg(required = true)]
    files: Vec<PathBuf>,

    /// Show line numbers
    #[arg(short, long)]
    line_number: bool,

    /// Show count of matches
    #[arg(short = 'c', long)]
    count: bool,

    /// Case insensitive search
    #[arg(short, long)]
    ignore_case: bool,

    /// Output JSONL (always enabled for AI agents)
    #[arg(short, long, default_value_t = true)]
    json: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    for file in &cli.files {
        if let Err(e) = grep_file(file, &cli) {
            let error_record = JsonlRecord::error(
                format!("Failed to search {}: {}", file.display(), e),
                "GREP_ERROR"
            );
            println!("{}", error_record.to_jsonl()?);
        }
    }

    Ok(())
}

fn grep_file(path: &PathBuf, cli: &Cli) -> Result<()> {
    // Use memory mapping for efficient searching
    let mem_access = SafeMemoryAccess::new(path)?;

    let content = if let Some(data) = mem_access.get(0, mem_access.size()) {
        String::from_utf8_lossy(data).to_string()
    } else {
        return Ok(());
    };

    let search_pattern = if cli.ignore_case {
        cli.pattern.to_lowercase()
    } else {
        cli.pattern.clone()
    };

    let mut match_count = 0;

    for (line_num, line) in content.lines().enumerate() {
        let search_line = if cli.ignore_case {
            line.to_lowercase()
        } else {
            line.to_string()
        };

        if search_line.contains(&search_pattern) {
            match_count += 1;

            if !cli.count {
                let match_start = search_line.find(&search_pattern).unwrap_or(0);
                let match_end = match_start + search_pattern.len();

                let record = JsonlRecord::MatchRecord {
                    timestamp: chrono::Utc::now(),
                    file: path.display().to_string(),
                    line_number: line_num + 1,
                    line_content: line.to_string(),
                    match_start,
                    match_end,
                };

                println!("{}", record.to_jsonl()?);
            }
        }
    }

    if cli.count {
        let record = JsonlRecord::result(serde_json::json!({
            "file": path.display().to_string(),
            "match_count": match_count,
        }));
        println!("{}", record.to_jsonl()?);
    }

    Ok(())
}
