//! AI-optimized mv utility
//!
//! Moves and renames files and directories with progress tracking and JSONL output.

use ai_coreutils::jsonl;
use ai_coreutils::{jsonl::JsonlRecord, Result};
use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};

/// AI-optimized mv: Move/rename files with JSONL output
#[derive(Parser, Debug)]
#[command(name = "ai-mv")]
#[command(about = "AI-optimized mv with progress tracking and JSONL output", long_about = None)]
struct Cli {
    /// Source file(s) to move
    #[arg(required = true)]
    sources: Vec<PathBuf>,

    /// Destination path
    #[arg(required = true)]
    destination: PathBuf,

    /// Interactive prompt before overwrite
    #[arg(short, long)]
    interactive: bool,

    /// No clobber (don't overwrite existing files)
    #[arg(short, long)]
    no_clobber: bool,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Force overwrite (needed if destination exists and is not writable)
    #[arg(short, long)]
    force: bool,

    /// Output JSONL (always enabled for AI-Coreutils)
    #[arg(short, long, default_value_t = true)]
    json: bool,
}

#[derive(Debug, Clone)]
struct MoveStats {
    files_moved: u64,
    bytes_moved: u64,
    dirs_moved: u64,
    errors: u64,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let mut stats = MoveStats {
        files_moved: 0,
        bytes_moved: 0,
        dirs_moved: 0,
        errors: 0,
    };

    // Determine if destination is a directory
    let dest_is_dir = cli.destination.exists() && cli.destination.is_dir();

    // Handle multiple sources
    if cli.sources.len() > 1 {
        if !dest_is_dir {
            jsonl::output_error(
                "When moving multiple sources, destination must be a directory",
                "MV_ERROR",
                Some(&cli.destination.to_string_lossy()),
            )?;
            return Err(ai_coreutils::error::AiCoreutilsError::InvalidInput(
                "Destination must be a directory when moving multiple sources".to_string(),
            ));
        }

        for source in &cli.sources {
            if let Err(e) = move_path(
                source,
                &cli.destination.join(source.file_name().unwrap_or_default()),
                &cli,
                &mut stats,
            ) {
                stats.errors += 1;
                let error_record = JsonlRecord::error(
                    format!("Failed to move {}: {}", source.display(), e),
                    "MV_ERROR"
                );
                println!("{}", error_record.to_jsonl()?);
            }
        }
    } else {
        // Single source
        let source = &cli.sources[0];
        let dest = if dest_is_dir {
            cli.destination.join(source.file_name().unwrap_or_default())
        } else {
            cli.destination.clone()
        };

        if let Err(e) = move_path(source, &dest, &cli, &mut stats) {
            // stats.errors += 1; // Error is already returned below
            let error_record = JsonlRecord::error(
                format!("Failed to move {}: {}", source.display(), e),
                "MV_ERROR"
            );
            println!("{}", error_record.to_jsonl()?);
            return Err(e);
        }
    }

    // Output final stats
    let record = JsonlRecord::result(serde_json::json!({
        "type": "move_summary",
        "files_moved": stats.files_moved,
        "bytes_moved": stats.bytes_moved,
        "dirs_moved": stats.dirs_moved,
        "errors": stats.errors,
    }));
    println!("{}", record.to_jsonl()?);

    Ok(())
}

fn move_path(source: &PathBuf, dest: &PathBuf, cli: &Cli, stats: &mut MoveStats) -> Result<()> {
    // Check if source exists
    if !source.exists() {
        return Err(ai_coreutils::error::AiCoreutilsError::PathNotFound(source.clone()));
    }

    // Check if destination exists and no_clobber is set
    if dest.exists() && cli.no_clobber {
        return Ok(());
    }

    // Check if source and destination are the same
    if source == dest {
        return Err(ai_coreutils::error::AiCoreutilsError::InvalidInput(
            "Source and destination are the same".to_string(),
        ));
    }

    // Interactive prompt
    if cli.interactive && dest.exists() {
        jsonl::output_info(
            serde_json::json!({
                "prompt": format!("Overwrite {}? (y/n)", dest.display()),
            }),
        )?;
        // For now, we'll just skip interactive in non-interactive mode
        // In a real implementation, you'd read from stdin here
    }

    // Get file size for stats
    let file_size = if source.is_file() {
        fs::metadata(source)
            .map(|m| m.len())
            .unwrap_or(0)
    } else {
        0
    };

    // Try to perform the move
    let move_result = fs::rename(source, dest);

    if let Err(_e) = move_result {
        // If rename fails (cross-device), try copy + delete
        // This returns Ok(()) with stats already updated
        if source.is_dir() {
            move_directory_fallback(source, dest, cli, stats)?;
            return Ok(());
        } else {
            move_file_fallback(source, dest, cli, stats, file_size)?;
            return Ok(());
        }
    }

    // Normal move succeeded - update stats
    if source.is_file() {
        stats.files_moved += 1;
        stats.bytes_moved += file_size;
    } else {
        stats.dirs_moved += 1;
    }

    if cli.verbose {
        jsonl::output_info(
            serde_json::json!({
                "type": "path_moved",
                "source": source.display().to_string(),
                "dest": dest.display().to_string(),
            }),
        )?;
    }

    Ok(())
}

fn move_file_fallback(source: &Path, dest: &Path, cli: &Cli, stats: &mut MoveStats, file_size: u64) -> Result<()> {
    // Copy the file
    fs::copy(source, dest)?;

    // Remove the source
    fs::remove_file(source)?;

    // Update stats
    stats.files_moved += 1;
    stats.bytes_moved += file_size;

    if cli.verbose {
        jsonl::output_info(
            serde_json::json!({
                "type": "file_moved_fallback",
                "source": source.display().to_string(),
                "dest": dest.display().to_string(),
                "size": file_size,
            }),
        )?;
    }

    Ok(())
}

fn move_directory_fallback(
    source: &Path,
    dest: &Path,
    cli: &Cli,
    stats: &mut MoveStats,
) -> Result<()> {
    // Create destination directory
    fs::create_dir_all(dest)?;

    // Copy contents
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let source_path = entry.path();
        let dest_path = dest.join(entry.file_name());

        if source_path.is_dir() {
            move_directory_fallback(&source_path, &dest_path, cli, stats)?;
        } else {
            let file_size = fs::metadata(&source_path)
                .map(|m| m.len())
                .unwrap_or(0);
            move_file_fallback(&source_path, &dest_path, cli, stats, file_size)?;
        }
    }

    // Remove source directory
    fs::remove_dir_all(source)?;

    // Update stats
    stats.dirs_moved += 1;

    if cli.verbose {
        jsonl::output_info(
            serde_json::json!({
                "type": "directory_moved_fallback",
                "source": source.display().to_string(),
                "dest": dest.display().to_string(),
            }),
        )?;
    }

    Ok(())
}
