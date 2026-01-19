use ai_coreutils::{AiCoreutilsError, jsonl, Result};
use clap::Parser;
use std::fs;
use std::path::PathBuf;

/// AI-optimized mkdir utility - Create directories
///
/// This utility extends GNU mkdir with:
/// - JSONL structured output
/// - Batch directory creation
/// - Detailed metadata
#[derive(Parser, Debug)]
#[command(name = "ai-mkdir")]
#[command(about = "Create directories", long_about = None)]
struct Cli {
    /// Directories to create
    #[arg(required = true)]
    directories: Vec<PathBuf>,

    /// Create parent directories as needed
    #[arg(short, long)]
    parents: bool,

    /// Set file mode (as in chmod), not supported on Windows
    #[arg(short = 'm', long, value_name = "MODE")]
    mode: Option<String>,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Output start message
    jsonl::output_progress(0, cli.directories.len(), "Starting mkdir operation")?;

    let mut success_count = 0;
    let mut error_count = 0;

    for (index, dir) in cli.directories.iter().enumerate() {
        // Update progress
        jsonl::output_progress(
            index + 1,
            cli.directories.len(),
            &format!("Creating: {}", dir.display()),
        )?;

        match create_directory(dir, &cli) {
            Ok(metadata) => {
                success_count += 1;

                if cli.verbose {
                    jsonl::output_info(serde_json::json!({
                        "directory": dir.display().to_string(),
                        "operation": "created",
                        "path": dir.display().to_string(),
                        "is_dir": metadata.is_dir,
                    }))?;
                }
            }
            Err(e) => {
                error_count += 1;
                jsonl::output_error(
                    &format!("Failed to create directory {}: {}", dir.display(), e),
                    "MKDIR_ERROR",
                    Some(dir.display().to_string().as_str()),
                )?;
            }
        }
    }

    // Output summary
    jsonl::output_info(serde_json::json!({
        "operation": "mkdir_summary",
        "total_directories": cli.directories.len(),
        "successful": success_count,
        "errors": error_count,
    }))?;

    Ok(())
}

struct DirectoryMetadata {
    is_dir: bool,
}

fn create_directory(dir: &PathBuf, cli: &Cli) -> Result<DirectoryMetadata> {
    // Check if directory already exists
    if dir.exists() {
        if !cli.parents {
            return Err(AiCoreutilsError::InvalidInput(
                format!("Directory already exists: {}", dir.display())
            ));
        }
        // With -p, existing directory is OK
        return Ok(DirectoryMetadata { is_dir: true });
    }

    // Create directory
    if cli.parents {
        fs::create_dir_all(dir)
            .map_err(AiCoreutilsError::Io)?;
    } else {
        fs::create_dir(dir)
            .map_err(AiCoreutilsError::Io)?;
    }

    // Note: Setting mode is platform-specific and not fully supported here
    // On Unix systems, you'd use std::os::unix::fs::PermissionsExt

    Ok(DirectoryMetadata { is_dir: true })
}
