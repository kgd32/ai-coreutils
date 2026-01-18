//! AI-optimized ls utility
//!
//! Lists directory contents with structured JSONL output.

use ai_coreutils::{jsonl::JsonlRecord, Result};
use clap::Parser;
use std::path::PathBuf;

/// AI-optimized ls: List directory contents with JSONL output
#[derive(Parser, Debug)]
#[command(name = "ai-ls")]
#[command(about = "AI-optimized ls with JSONL output", long_about = None)]
struct Cli {
    /// Paths to list
    #[arg(default_value = ".")]
    paths: Vec<PathBuf>,

    /// Show all files (including hidden)
    #[arg(short, long)]
    all: bool,

    /// Long format with detailed metadata
    #[arg(short, long)]
    long: bool,

    /// Recursive listing
    #[arg(short, long)]
    recursive: bool,

    /// Output JSONL (always enabled for AI agents)
    #[arg(short, long, default_value_t = true)]
    json: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    for path in &cli.paths {
        if let Err(e) = list_path(path, &cli) {
            let error_record = JsonlRecord::error(
                format!("Failed to list {}: {}", path.display(), e),
                "LS_ERROR"
            );
            println!("{}", error_record.to_jsonl()?);
        }
    }

    Ok(())
}

fn list_path(path: &PathBuf, _cli: &Cli) -> Result<()> {
    // Placeholder implementation - will be expanded in next phase
    let metadata = ai_coreutils::fs_utils::get_file_metadata(path)?;

    let record = JsonlRecord::FileEntry {
        timestamp: chrono::Utc::now(),
        path: path.display().to_string(),
        size: metadata["size"].as_u64().unwrap_or(0),
        modified: chrono::Utc::now(),
        is_dir: metadata["is_dir"].as_bool().unwrap_or(false),
        is_symlink: metadata["is_symlink"].as_bool().unwrap_or(false),
        permissions: "rw-r--r--".to_string(),
    };

    println!("{}", record.to_jsonl()?);

    Ok(())
}
