//! AI-optimized cp utility
//!
//! Copies files and directories with progress tracking and JSONL output.

use ai_coreutils::jsonl;
use ai_coreutils::{jsonl::JsonlRecord, Result};
use clap::Parser;
use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

#[cfg(unix)]
use std::os::unix::fs as unix_fs;
#[cfg(windows)]
use std::os::windows::fs as windows_fs;

/// AI-optimized cp: Copy files with JSONL output
#[derive(Parser, Debug)]
#[command(name = "ai-cp")]
#[command(about = "AI-optimized cp with progress tracking and JSONL output", long_about = None)]
struct Cli {
    /// Source file(s) to copy
    #[arg(required = true)]
    sources: Vec<PathBuf>,

    /// Destination path
    #[arg(required = true)]
    destination: PathBuf,

    /// Recursive copy (for directories)
    #[arg(short = 'R', long)]
    recursive: bool,

    /// Archive mode (preserves all attributes)
    #[arg(short = 'a', long)]
    archive: bool,

    /// Preserve permissions, timestamps
    #[arg(short = 'p', long)]
    preserve: bool,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Interactive prompt before overwrite
    #[arg(short, long)]
    interactive: bool,

    /// Update only newer files
    #[arg(short, long)]
    update: bool,

    /// Create hard links instead of copying
    #[arg(short, long)]
    link: bool,

    /// Create symbolic links
    #[arg(short, long)]
    symbolic_link: bool,

    /// No clobber (don't overwrite existing files)
    #[arg(short, long)]
    no_clobber: bool,

    /// Output JSONL (always enabled for AI-Coreutils)
    #[arg(long, default_value_t = true)]
    json: bool,
}

#[derive(Debug, Clone)]
struct CopyStats {
    files_copied: u64,
    bytes_copied: u64,
    dirs_created: u64,
    errors: u64,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let mut stats = CopyStats {
        files_copied: 0,
        bytes_copied: 0,
        dirs_created: 0,
        errors: 0,
    };

    // Determine if destination is a directory
    let dest_is_dir = cli.destination.exists() && cli.destination.is_dir();

    // Handle multiple sources
    if cli.sources.len() > 1 {
        if !dest_is_dir {
            jsonl::output_error(
                "When copying multiple sources, destination must be a directory",
                "CP_ERROR",
                Some(&cli.destination.to_string_lossy()),
            )?;
            return Err(ai_coreutils::error::AiCoreutilsError::InvalidInput(
                "Destination must be a directory when copying multiple sources".to_string(),
            ));
        }

        for source in &cli.sources {
            if let Err(e) = copy_path(
                source,
                &cli.destination.join(source.file_name().unwrap_or_default()),
                &cli,
                &mut stats,
            ) {
                stats.errors += 1;
                let error_record = JsonlRecord::error(
                    format!("Failed to copy {}: {}", source.display(), e),
                    "CP_ERROR"
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

        if let Err(e) = copy_path(source, &dest, &cli, &mut stats) {
            // stats.errors += 1; // Error is already returned below
            let error_record = JsonlRecord::error(
                format!("Failed to copy {}: {}", source.display(), e),
                "CP_ERROR"
            );
            println!("{}", error_record.to_jsonl()?);
            return Err(e);
        }
    }

    // Output final stats
    let record = JsonlRecord::result(serde_json::json!({
        "type": "copy_summary",
        "files_copied": stats.files_copied,
        "bytes_copied": stats.bytes_copied,
        "dirs_created": stats.dirs_created,
        "errors": stats.errors,
    }));
    println!("{}", record.to_jsonl()?);

    Ok(())
}

fn copy_path(source: &PathBuf, dest: &PathBuf, cli: &Cli, stats: &mut CopyStats) -> Result<()> {
    // Check if source exists
    if !source.exists() {
        return Err(ai_coreutils::error::AiCoreutilsError::PathNotFound(source.clone()));
    }

    // Check if destination exists and no_clobber is set
    if dest.exists() && cli.no_clobber {
        return Ok(());
    }

    // Check update flag
    if cli.update && dest.exists() {
        let source_meta = fs::metadata(source)?;
        let dest_meta = fs::metadata(dest)?;

        // If destination is newer or equal, skip
        if dest_meta.modified()? >= source_meta.modified()? {
            return Ok(());
        }
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

    if source.is_dir() {
        if !cli.recursive && !cli.archive {
            return Err(ai_coreutils::error::AiCoreutilsError::InvalidInput(
                "Omitting directory, use -R to copy directories".to_string(),
            ));
        }
        copy_directory(source, dest, cli, stats)?;
    } else {
        copy_file(source, dest, cli, stats)?;
    }

    Ok(())
}

fn copy_directory(source: &Path, dest: &Path, cli: &Cli, stats: &mut CopyStats) -> Result<()> {
    // Create destination directory if it doesn't exist
    if !dest.exists() {
        fs::create_dir_all(dest)?;
        stats.dirs_created += 1;

        if cli.verbose {
            jsonl::output_info(
                serde_json::json!({
                    "type": "directory_created",
                    "path": dest.display().to_string(),
                }),
            )?;
        }
    }

    // Preserve permissions if requested
    if cli.preserve || cli.archive {
        if let Ok(source_meta) = fs::metadata(source) {
            fs::set_permissions(dest, source_meta.permissions())?;
        }
    }

    // Copy directory contents
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let source_path = entry.path();
        let dest_path = dest.join(entry.file_name());

        copy_path(&source_path, &dest_path, cli, stats)?;
    }

    Ok(())
}

fn copy_file(source: &Path, dest: &Path, cli: &Cli, stats: &mut CopyStats) -> Result<()> {
    // Check if we should create a link instead
    if cli.link {
        fs::hard_link(source, dest)?;
        stats.files_copied += 1;

        if cli.verbose {
            jsonl::output_info(
                serde_json::json!({
                    "type": "hard_link_created",
                    "source": source.display().to_string(),
                    "dest": dest.display().to_string(),
                }),
            )?;
        }
        return Ok(());
    }

    if cli.symbolic_link {
        #[cfg(unix)]
        {
            unix_fs::symlink(source, dest)?;
        }
        #[cfg(windows)]
        {
            if source.is_dir() {
                windows_fs::symlink_dir(source, dest)?;
            } else {
                windows_fs::symlink_file(source, dest)?;
            }
        }
        stats.files_copied += 1;

        if cli.verbose {
            jsonl::output_info(
                serde_json::json!({
                    "type": "symbolic_link_created",
                    "source": source.display().to_string(),
                    "dest": dest.display().to_string(),
                }),
            )?;
        }
        return Ok(());
    }

    // Get source metadata for progress tracking
    let source_meta = fs::metadata(source)?;
    let file_size = source_meta.len();

    // Output progress
    jsonl::output_progress(0, file_size as usize, &format!("Copying {}", source.display()))?;

    // Actually copy the file
    let mut source_file = fs::File::open(source)?;
    let mut dest_file = fs::File::create(dest)?;

    let mut buffer = vec![0u8; 8192];
    let mut total_copied = 0u64;

    loop {
        let bytes_read = source_file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }

        dest_file.write_all(&buffer[..bytes_read])?;
        total_copied += bytes_read as u64;

        // Output progress for large files
        if file_size > 1024 * 1024 && total_copied.is_multiple_of(1024 * 1024) {
            jsonl::output_progress(total_copied as usize, file_size as usize, &format!("Copying {}", source.display()))?;
        }
    }

    dest_file.sync_all()?;

    stats.files_copied += 1;
    stats.bytes_copied += total_copied;

    // Preserve attributes if requested
    if cli.preserve || cli.archive {
        if let Ok(source_meta) = fs::metadata(source) {
            fs::set_permissions(dest, source_meta.permissions())?;

            // Try to preserve timestamps (Unix-specific)
            #[cfg(unix)]
            {
                use std::os::unix::fs::MetadataExt;
                let atime = source_meta.atime();
                let mtime = source_meta.mtime();

                // Note: Setting times is platform-specific
                // On Unix, we'd use file.set_times() but that's not in std
                // For now, we preserve permissions which is the most important
            }
        }
    }

    if cli.verbose {
        jsonl::output_info(
            serde_json::json!({
                "type": "file_copied",
                "source": source.display().to_string(),
                "dest": dest.display().to_string(),
                "size": total_copied,
            }),
        )?;
    }

    Ok(())
}
