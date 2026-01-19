//! AI-optimized find utility
//!
//! Searches for files in a directory hierarchy with JSONL output.

use ai_coreutils::jsonl;
use ai_coreutils::Result;
use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// AI-optimized find: Search files with JSONL output
#[derive(Parser, Debug)]
#[command(name = "ai-find")]
#[command(about = "AI-optimized find with structured output", long_about = None)]
struct Cli {
    /// Starting point(s) for search
    #[arg(default_value = ".")]
    paths: Vec<PathBuf>,

    /// Filter by name (supports wildcards)
    #[arg(short, long)]
    name: Option<String>,

    /// Filter by type (f=file, d=directory, l=symlink)
    #[arg(short, long)]
    #[arg(value_parser = parse_type_filter)]
    type_filter: Option<Vec<TypeFilter>>,

    /// Filter by minimum size (bytes, or suffix K/M/G)
    #[arg(long)]
    #[arg(value_parser = parse_size)]
    size_min: Option<u64>,

    /// Filter by maximum size (bytes, or suffix K/M/G)
    #[arg(long)]
    #[arg(value_parser = parse_size)]
    size_max: Option<u64>,

    /// Filter by extension
    #[arg(long)]
    ext: Option<String>,

    /// Filter by permission mode (e.g., 755, 644)
    #[arg(long, value_parser = parse_octal)]
    perm: Option<u32>,

    /// Maximum depth to search
    #[arg(short, long)]
    maxdepth: Option<usize>,

    /// Minimum depth to search
    #[arg(short, long)]
    mindepth: Option<usize>,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Output JSONL (always enabled for AI-Coreutils)
    #[arg(short, long, default_value_t = true)]
    json: bool,
}

#[derive(Debug, Clone, Copy)]
enum TypeFilter {
    File,
    Directory,
    Symlink,
}

fn parse_type_filter(s: &str) -> std::result::Result<Vec<TypeFilter>, String> {
    let mut filters = Vec::new();
    for c in s.chars() {
        match c {
            'f' | 'F' => filters.push(TypeFilter::File),
            'd' | 'D' => filters.push(TypeFilter::Directory),
            'l' | 'L' => filters.push(TypeFilter::Symlink),
            _ => return Err(format!("Invalid type filter: {}", c)),
        }
    }
    Ok(filters)
}

fn parse_size(s: &str) -> std::result::Result<u64, String> {
    let s = s.trim();
    let (num, suffix) = if s.ends_with('K') || s.ends_with('k') {
        (&s[..s.len()-1], 1024u64)
    } else if s.ends_with('M') || s.ends_with('m') {
        (&s[..s.len()-1], 1024 * 1024)
    } else if s.ends_with('G') || s.ends_with('g') {
        (&s[..s.len()-1], 1024 * 1024 * 1024)
    } else {
        (s, 1u64)
    };

    num.parse::<u64>()
        .map(|n| n * suffix)
        .map_err(|_| format!("Invalid size: {}", s))
}

fn parse_octal(s: &str) -> std::result::Result<u32, String> {
    u32::from_str_radix(s, 8)
        .map_err(|_| format!("Invalid octal number: {}", s))
}

#[derive(Debug, Clone)]
struct MatchStats {
    files_matched: u64,
    dirs_matched: u64,
    symlinks_matched: u64,
    searched: u64,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let mut stats = MatchStats {
        files_matched: 0,
        dirs_matched: 0,
        symlinks_matched: 0,
        searched: 0,
    };

    // Search each starting path
    for start_path in &cli.paths {
        find_in_directory(start_path, &cli, 0, &mut stats)?;
    }

    // Output final stats
    jsonl::output_result(serde_json::json!({
        "type": "find_summary",
        "files_matched": stats.files_matched,
        "dirs_matched": stats.dirs_matched,
        "symlinks_matched": stats.symlinks_matched,
        "searched": stats.searched,
    }))?;

    Ok(())
}

fn find_in_directory(
    path: &Path,
    cli: &Cli,
    depth: usize,
    stats: &mut MatchStats,
) -> Result<()> {
    // Check depth constraints
    if let Some(maxdepth) = cli.maxdepth {
        if depth > maxdepth {
            return Ok(());
        }
    }

    if let Some(mindepth) = cli.mindepth {
        if depth < mindepth {
            // Still need to traverse deeper
            if path.is_dir() {
                let entries = match fs::read_dir(path) {
                    Ok(e) => e,
                    Err(_) => return Ok(()),
                };

                for entry in entries {
                    let entry = entry?;
                    let entry_path = entry.path();
                    find_in_directory(&entry_path, cli, depth + 1, stats)?;
                }
            }
            return Ok(());
        }
    }

    // Check if current path matches
    if matches_filters(path, cli)? {
        output_match(path, cli)?;
        update_stats(path, stats);
    }

    stats.searched += 1;

    // Recurse into directories
    if path.is_dir() {
        let entries = match fs::read_dir(path) {
            Ok(e) => e,
            Err(_) => return Ok(()),
        };

        for entry in entries {
            let entry = entry?;
            let entry_path = entry.path();
            find_in_directory(&entry_path, cli, depth + 1, stats)?;
        }
    }

    Ok(())
}

fn matches_filters(path: &Path, cli: &Cli) -> Result<bool> {
    // Type filter
    if let Some(ref filters) = cli.type_filter {
        let matches_type = filters.iter().any(|&filter| {
            match filter {
                TypeFilter::File => path.is_file(),
                TypeFilter::Directory => path.is_dir(),
                TypeFilter::Symlink => path.is_symlink(),
            }
        });
        if !matches_type {
            return Ok(false);
        }
    }

    // Name filter (supports simple wildcard)
    if let Some(ref name_pattern) = cli.name {
        let file_name = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");

        if !matches_pattern(file_name, name_pattern) {
            return Ok(false);
        }
    }

    // Extension filter
    if let Some(ref ext) = cli.ext {
        let file_ext = path.extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");

        if file_ext != ext {
            return Ok(false);
        }
    }

    // Size filters (only for files)
    if path.is_file() {
        if let Ok(metadata) = fs::metadata(path) {
            let size = metadata.len();

            if let Some(min_size) = cli.size_min {
                if size < min_size {
                    return Ok(false);
                }
            }

            if let Some(max_size) = cli.size_max {
                if size > max_size {
                    return Ok(false);
                }
            }
        }
    }

    // Permission filter (Unix-only)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Some(perm) = cli.perm {
            if let Ok(metadata) = fs::metadata(path) {
                let mode = metadata.permissions().mode() & 0o777;
                if mode as u32 != perm {
                    return Ok(false);
                }
            }
        }
    }

    Ok(true)
}

fn matches_pattern(text: &str, pattern: &str) -> bool {
    // Simple wildcard matching: * matches any sequence, ? matches any single char
    if pattern == "*" {
        return true;
    }

    if pattern.contains('*') {
        let parts: Vec<&str> = pattern.split('*').collect();
        if parts.len() == 2 {
            let (prefix, suffix) = (parts[0], parts[1]);
            text.starts_with(prefix) && text.ends_with(suffix)
        } else {
            // For complex patterns, just do simple contains check
            let inner = pattern.replace('*', "");
            text.contains(&inner)
        }
    } else if pattern.contains('?') {
        if text.len() != pattern.len() {
            return false;
        }
        text.chars().zip(pattern.chars())
            .all(|(t, p)| p == '?' || t == p)
    } else {
        text == pattern
    }
}

fn output_match(path: &Path, cli: &Cli) -> Result<()> {
    let metadata = fs::metadata(path).ok();
    let file_type = if path.is_file() {
        "file"
    } else if path.is_dir() {
        "directory"
    } else if path.is_symlink() {
        "symlink"
    } else {
        "unknown"
    };

    let mut result = serde_json::json!({
        "type": "match",
        "path": path.display().to_string(),
        "file_type": file_type,
    });

    if let Some(meta) = metadata {
        result["size"] = serde_json::json!(meta.len());
        if let Ok(modified) = meta.modified() {
            if let Ok(datetime) = modified.duration_since(SystemTime::UNIX_EPOCH) {
                result["modified"] = serde_json::json!(datetime.as_secs());
            }
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mode = meta.permissions().mode() & 0o777;
            result["permissions"] = serde_json::json!(format!("{:03o}", mode));
        }
    }

    if let Some(name) = path.file_name() {
        result["name"] = serde_json::json!(name.to_string_lossy().to_string());
    }

    if let Some(parent) = path.parent() {
        result["parent"] = serde_json::json!(parent.display().to_string());
    }

    jsonl::output_result(result)?;

    if cli.verbose {
        // Verbose mode outputs additional info
        jsonl::output_info(serde_json::json!({
            "matched": path.display().to_string(),
        }))?;
    }

    Ok(())
}

fn update_stats(path: &Path, stats: &mut MatchStats) {
    if path.is_file() {
        stats.files_matched += 1;
    } else if path.is_dir() {
        stats.dirs_matched += 1;
    } else if path.is_symlink() {
        stats.symlinks_matched += 1;
    }
}
