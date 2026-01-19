//! AI-optimized ls utility
//!
//! Lists directory contents with structured JSONL output.

use ai_coreutils::{jsonl::JsonlRecord, Result};
use chrono::{DateTime, Utc};
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

    /// Human-readable sizes
    #[arg(short, long)]
    human_readable: bool,

    /// Recursive listing
    #[arg(short = 'R', long)]
    recursive: bool,

    /// Sort by modification time
    #[arg(short, long)]
    sort_time: bool,

    /// Sort by size
    #[arg(short = 'S', long)]
    sort_size: bool,

    /// Reverse sort order
    #[arg(short, long)]
    reverse: bool,

    /// Output JSONL (always enabled for AI agents)
    #[arg(short, long, default_value_t = true)]
    json: bool,
}

#[derive(Debug, Clone)]
struct FileInfo {
    path: PathBuf,
    name: String,
    size: u64,
    modified: DateTime<Utc>,
    is_dir: bool,
    is_symlink: bool,
    is_hidden: bool,
    permissions: String,
}

impl FileInfo {
    fn from_entry(entry: &walkdir::DirEntry) -> Result<Self> {
        let metadata = entry.metadata()?;
        let path = entry.path().to_path_buf();
        let name = entry.file_name().to_string_lossy().to_string();

        // Check if hidden (starts with .)
        let is_hidden = name.starts_with('.');

        // Get permissions (Unix-specific with cfg_attr, simplified for cross-platform)
        #[cfg(unix)]
        let permissions = {
            use std::os::unix::fs::PermissionsExt;
            metadata.permissions()
                .mode()
                .map(|m| format!("{:o}", m & 0o777))
                .unwrap_or_else(|_| "??????????".to_string())
        };
        #[cfg(not(unix))]
        let permissions = "??????????".to_string();

        // Get modified time
        let modified = metadata.modified()
            .ok()
            .and_then(|t| {
                let secs = t.duration_since(std::time::UNIX_EPOCH).ok()?.as_secs();
                DateTime::from_timestamp(secs as i64, 0)
            })
            .unwrap_or_else(|| Utc::now());

        Ok(Self {
            path,
            name,
            size: metadata.len(),
            modified,
            is_dir: metadata.is_dir(),
            is_symlink: metadata.is_symlink(),
            is_hidden,
            permissions,
        })
    }

    fn to_jsonl_record(&self, show_long: bool, human_readable: bool) -> JsonlRecord {
        let size_str = if human_readable {
            format_size(self.size)
        } else {
            self.size.to_string()
        };

        let path_str = self.path.display().to_string();

        if show_long {
            JsonlRecord::result(serde_json::json!({
                "type": "file",
                "timestamp": Utc::now(),
                "path": path_str,
                "name": self.name,
                "size": self.size,
                "size_human": size_str,
                "modified": self.modified.to_rfc3339(),
                "is_dir": self.is_dir,
                "is_symlink": self.is_symlink,
                "is_hidden": self.is_hidden,
                "permissions": self.permissions,
            }))
        } else {
            JsonlRecord::FileEntry {
                timestamp: Utc::now(),
                path: path_str,
                size: self.size,
                modified: self.modified,
                is_dir: self.is_dir,
                is_symlink: self.is_symlink,
                permissions: self.permissions.clone(),
            }
        }
    }
}

fn format_size(size: u64) -> String {
    const THRESHOLD: u64 = 1024;
    const UNITS: &[&str] = &["B", "K", "M", "G", "T", "P"];

    let mut size_f = size as f64;
    let mut unit_index = 0;

    while size_f >= THRESHOLD as f64 && unit_index < UNITS.len() - 1 {
        size_f /= THRESHOLD as f64;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{}{}", size, UNITS[unit_index])
    } else {
        format!("{:.1}{}", size_f, UNITS[unit_index])
    }
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

fn list_path(path: &PathBuf, cli: &Cli) -> Result<()> {
    let mut entries = Vec::new();

    // Build walkdir iterator
    let mut walker = if path.is_dir() {
        walkdir::WalkDir::new(path)
    } else {
        // Single file
        let metadata = std::fs::metadata(path)?;
        let file_info = FileInfo {
            path: path.clone(),
            name: path.file_name().unwrap_or_default().to_string_lossy().to_string(),
            size: metadata.len(),
            modified: Utc::now(),
            is_dir: metadata.is_dir(),
            is_symlink: metadata.is_symlink(),
            is_hidden: false,
            permissions: "??????????".to_string(),
        };
        entries.push(file_info);

        output_entries(&entries, cli)?;
        return Ok(());
    };

    // Configure walker
    if cli.recursive {
        walker = walker.max_depth(usize::MAX);
    } else {
        walker = walker.max_depth(1);
    }

    // Collect entries
    let result = walker.into_iter().collect::<Vec<_>>();

    for entry in result {
        let entry = entry?;

        // Skip hidden files unless --all is specified
        let file_name = entry.file_name().to_string_lossy();
        if !cli.all && file_name.starts_with('.') {
            continue;
        }

        match FileInfo::from_entry(&entry) {
            Ok(info) => entries.push(info),
            Err(_) => continue, // Skip entries we can't read
        }
    }

    // Sort entries
    sort_entries(&mut entries, cli);

    // Output entries
    output_entries(&entries, cli)?;

    Ok(())
}

fn sort_entries(entries: &mut Vec<FileInfo>, cli: &Cli) {
    use std::cmp::Ordering;

    entries.sort_by(|a, b| {
        let mut ordering = if cli.sort_time {
            b.modified.cmp(&a.modified)
        } else if cli.sort_size {
            b.size.cmp(&a.size)
        } else {
            a.name.cmp(&b.name)
        };

        if cli.reverse {
            ordering = ordering.reverse();
        }

        // Always sort directories first within same ordering
        if ordering == Ordering::Equal {
            if a.is_dir && !b.is_dir {
                Ordering::Less
            } else if !a.is_dir && b.is_dir {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        } else {
            ordering
        }
    });
}

fn output_entries(entries: &[FileInfo], cli: &Cli) -> Result<()> {
    for entry in entries {
        let record = entry.to_jsonl_record(cli.long, cli.human_readable);
        println!("{}", record.to_jsonl()?);
    }
    Ok(())
}
