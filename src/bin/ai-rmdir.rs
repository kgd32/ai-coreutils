use ai_coreutils::{AiCoreutilsError, jsonl, Result};
use clap::Parser;
use std::fs;
use std::path::PathBuf;

/// AI-optimized rmdir utility - Remove empty directories
///
/// This utility extends GNU rmdir with:
/// - JSONL structured output
/// - Batch directory removal
/// - Detailed metadata
#[derive(Parser, Debug)]
#[command(name = "ai-rmdir")]
#[command(about = "Remove empty directories", long_about = None)]
struct Cli {
    /// Directories to remove
    #[arg(required = true)]
    directories: Vec<PathBuf>,

    /// Remove parent directories if empty
    #[arg(short, long)]
    parents: bool,

    /// Ignore failures caused by non-empty directories
    #[arg(long)]
    ignore_fail_on_non_empty: bool,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Output start message
    jsonl::output_progress(0, cli.directories.len(), "Starting rmdir operation")?;

    let mut success_count = 0;
    let mut error_count = 0;

    for (index, dir) in cli.directories.iter().enumerate() {
        // Update progress
        jsonl::output_progress(
            index + 1,
            cli.directories.len(),
            &format!("Removing: {}", dir.display()),
        )?;

        match remove_directory(dir, &cli) {
            Ok(metadata) => {
                success_count += 1;

                if cli.verbose {
                    jsonl::output_info(serde_json::json!({
                        "directory": dir.display().to_string(),
                        "operation": "removed",
                        "path": metadata.path,
                        "parents_removed": metadata.parents_removed,
                    }))?;
                }
            }
            Err(e) => {
                error_count += 1;

                // Check if this is a "directory not empty" error and we should ignore it
                let is_non_empty_error = e.to_string().contains("directory not empty")
                    || e.to_string().contains("Directory not empty");

                if !is_non_empty_error || !cli.ignore_fail_on_non_empty {
                    jsonl::output_error(
                        &format!("Failed to remove directory {}: {}", dir.display(), e),
                        "RMDIR_ERROR",
                        Some(dir.display().to_string().as_str()),
                    )?;
                } else {
                    // Treat ignored errors as successes
                    success_count += 1;
                    error_count -= 1;
                }
            }
        }
    }

    // Output summary
    jsonl::output_info(serde_json::json!({
        "operation": "rmdir_summary",
        "total_directories": cli.directories.len(),
        "successful": success_count,
        "errors": error_count,
    }))?;

    Ok(())
}

struct RemovalMetadata {
    path: String,
    parents_removed: Vec<String>,
}

fn remove_directory(dir: &PathBuf, cli: &Cli) -> Result<RemovalMetadata> {
    // Check if directory exists
    if !dir.exists() {
        return Err(AiCoreutilsError::PathNotFound(dir.clone()));
    }

    // Check if it's a directory
    if !dir.is_dir() {
        return Err(AiCoreutilsError::InvalidInput(
            format!("Not a directory: {}", dir.display())
        ));
    }

    // Check if directory is empty
    let is_empty = dir.read_dir()
        .map(|mut entries| entries.next().is_none())
        .unwrap_or(false);

    if !is_empty {
        return Err(AiCoreutilsError::InvalidInput(
            format!("Directory not empty: {}", dir.display())
        ));
    }

    // Remove the directory
    fs::remove_dir(dir)
        .map_err(AiCoreutilsError::Io)?;

    let mut parents_removed = Vec::new();

    // Remove parent directories if requested
    if cli.parents {
        let mut current = dir.parent();

        while let Some(parent) = current {
            if !parent.exists() {
                break;
            }

            // Check if parent is empty
            let parent_is_empty = parent.read_dir()
                .map(|mut entries| entries.next().is_none())
                .unwrap_or(false);

            if !parent_is_empty {
                break;
            }

            // Remove parent
            fs::remove_dir(parent)
                .map_err(AiCoreutilsError::Io)?;

            parents_removed.push(parent.display().to_string());
            current = parent.parent();
        }
    }

    Ok(RemovalMetadata {
        path: dir.display().to_string(),
        parents_removed,
    })
}
