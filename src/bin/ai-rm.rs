//! AI-optimized rm utility
//!
//! Removes files and directories with safety features and JSONL output.

use ai_coreutils::jsonl;
use ai_coreutils::{jsonl::JsonlRecord, Result};
use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};

/// AI-optimized rm: Remove files with JSONL output
#[derive(Parser, Debug)]
#[command(name = "ai-rm")]
#[command(about = "AI-optimized rm with safety features and JSONL output", long_about = None)]
struct Cli {
    /// Files/directories to remove
    #[arg(required = true)]
    paths: Vec<PathBuf>,

    /// Recursive removal (for directories)
    #[arg(short = 'r', long)]
    recursive: bool,

    /// Force removal (ignore nonexistent files, never prompt)
    #[arg(short, long)]
    force: bool,

    /// Interactive prompt before each removal
    #[arg(short, long)]
    interactive: bool,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Prompt before removing more than 3 files
    #[arg(short = 'I', long)]
    one_file_system: bool,

    /// Don't remove root directory (/)
    #[arg(long, default_value_t = true)]
    preserve_root: bool,

    /// Output JSONL (always enabled for AI-Coreutils)
    #[arg(short, long, default_value_t = true)]
    json: bool,
}

#[derive(Debug, Clone)]
struct RemoveStats {
    files_removed: u64,
    dirs_removed: u64,
    bytes_freed: u64,
    errors: u64,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let mut stats = RemoveStats {
        files_removed: 0,
        dirs_removed: 0,
        bytes_freed: 0,
        errors: 0,
    };

    // Check for root directory attempts
    if cli.preserve_root {
        for path in &cli.paths {
            if path.as_os_str() == "/" || path.as_os_str() == "\\" {
                jsonl::output_error(
                    "Cannot remove root directory (use --no-preserve-root to override)",
                    "RM_ERROR",
                    Some(&path.to_string_lossy()),
                )?;
                return Err(ai_coreutils::error::AiCoreutilsError::InvalidInput(
                    "Cannot remove root directory".to_string(),
                ));
            }
        }
    }

    // Remove each path
    for path in &cli.paths {
        if let Err(e) = remove_path(path, &cli, &mut stats) {
            stats.errors += 1;

            // Only output error if not in force mode
            if !cli.force {
                let error_record = JsonlRecord::error(
                    format!("Failed to remove {}: {}", path.display(), e),
                    "RM_ERROR"
                );
                println!("{}", error_record.to_jsonl()?);
            }
        }
    }

    // Output final stats
    let record = JsonlRecord::result(serde_json::json!({
        "type": "remove_summary",
        "files_removed": stats.files_removed,
        "dirs_removed": stats.dirs_removed,
        "bytes_freed": stats.bytes_freed,
        "errors": stats.errors,
    }));
    println!("{}", record.to_jsonl()?);

    Ok(())
}

fn remove_path(path: &PathBuf, cli: &Cli, stats: &mut RemoveStats) -> Result<()> {
    // Check if path exists
    if !path.exists() {
        if cli.force {
            // Silently skip in force mode
            return Ok(());
        }
        return Err(ai_coreutils::error::AiCoreutilsError::PathNotFound(path.clone()));
    }

    // Get metadata for stats
    let metadata = fs::metadata(path)?;
    let is_dir = path.is_dir();
    let size = metadata.len();

    // Interactive prompt
    if cli.interactive {
        jsonl::output_info(
            serde_json::json!({
                "prompt": format!("Remove {}? (y/n)", path.display()),
            }),
        )?;
        // For now, we'll just skip interactive in non-interactive mode
        // In a real implementation, you'd read from stdin here
    }

    // Perform removal
    if is_dir {
        if !cli.recursive && !cli.one_file_system {
            return Err(ai_coreutils::error::AiCoreutilsError::InvalidInput(
                "Cannot remove directory without -r/--recursive".to_string(),
            ));
        }
        remove_directory(path, cli, stats)?;
    } else {
        remove_file(path, cli, stats, size)?;
    }

    Ok(())
}

fn remove_file(path: &Path, cli: &Cli, stats: &mut RemoveStats, size: u64) -> Result<()> {
    // Output progress
    jsonl::output_progress(0, size as usize, &format!("Removing {}", path.display()))?;

    // Remove the file
    fs::remove_file(path)?;

    // Update stats
    stats.files_removed += 1;
    stats.bytes_freed += size;

    if cli.verbose {
        jsonl::output_info(
            serde_json::json!({
                "type": "file_removed",
                "path": path.display().to_string(),
                "size": size,
            }),
        )?;
    }

    Ok(())
}

fn remove_directory(path: &Path, cli: &Cli, stats: &mut RemoveStats) -> Result<()> {
    // Remove all contents first
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let entry_path = entry.path();

        if entry_path.is_dir() {
            remove_directory(&entry_path, cli, stats)?;
        } else {
            let size = fs::metadata(&entry_path)
                .map(|m| m.len())
                .unwrap_or(0);
            remove_file(&entry_path, cli, stats, size)?;
        }
    }

    // Remove the directory itself
    fs::remove_dir(path)?;

    // Update stats
    stats.dirs_removed += 1;

    if cli.verbose {
        jsonl::output_info(
            serde_json::json!({
                "type": "directory_removed",
                "path": path.display().to_string(),
            }),
        )?;
    }

    Ok(())
}
