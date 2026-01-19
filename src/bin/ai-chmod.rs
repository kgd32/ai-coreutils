//! AI-optimized chmod utility
//!
//! Changes file permissions with JSONL output.

use ai_coreutils::jsonl;
use ai_coreutils::Result;
use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};

/// AI-optimized chmod: Change permissions with JSONL output
#[derive(Parser, Debug)]
#[command(name = "ai-chmod")]
#[command(about = "AI-optimized chmod with structured output", long_about = None)]
struct Cli {
    /// Permission changes (octal mode or symbolic mode)
    #[arg(required = true)]
    mode: String,

    /// Files/directories to modify
    #[arg(required = true)]
    paths: Vec<PathBuf>,

    /// Recursive permission change
    #[arg(short = 'R', long)]
    recursive: bool,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Produce output in JSONL format (always enabled)
    #[arg(long, default_value_t = true)]
    json: bool,

    /// Changes ownership if file is a symbolic link
    #[arg(short, long)]
    #[cfg(unix)]
    symbolic_link: bool,
}

#[derive(Debug, Clone)]
struct ChmodStats {
    files_modified: u64,
    dirs_modified: u64,
    errors: u64,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let mut stats = ChmodStats {
        files_modified: 0,
        dirs_modified: 0,
        errors: 0,
    };

    // Parse the mode specification
    let mode_spec = parse_mode(&cli.mode)?;

    // Apply permissions to each path
    for path in &cli.paths {
        if let Err(e) = change_permissions(path, &cli, &mode_spec, &mut stats) {
            stats.errors += 1;
            jsonl::output_error(
                &format!("Failed to change permissions for {}: {}", path.display(), e),
                "CHMOD_ERROR",
                Some(&path.to_string_lossy()),
            )?;
        }
    }

    // Output final stats
    jsonl::output_result(serde_json::json!({
        "type": "chmod_summary",
        "files_modified": stats.files_modified,
        "dirs_modified": stats.dirs_modified,
        "errors": stats.errors,
        "mode": cli.mode,
    }))?;

    Ok(())
}

#[derive(Debug, Clone)]
enum ModeSpec {
    Absolute(u32),
    #[allow(dead_code)]
    Symbolic {
        who: Option<char>,
        op: char,
        permissions: u32,
    },
}

fn parse_mode(mode_str: &str) -> Result<ModeSpec> {
    // Check if it's an octal mode (e.g., "755", "644")
    if mode_str.chars().all(|c| c.is_ascii_digit()) {
        let mode = u32::from_str_radix(mode_str, 8)
            .map_err(|_| ai_coreutils::error::AiCoreutilsError::InvalidInput(
                format!("Invalid octal mode: {}", mode_str)
            ))?;
        return Ok(ModeSpec::Absolute(mode));
    }

    // Otherwise it's a symbolic mode (e.g., "u+x", "go=rwx")
    if let Some((who, op, permissions)) = parse_symbolic_mode(mode_str)? {
        return Ok(ModeSpec::Symbolic { who, op, permissions });
    }

    Err(ai_coreutils::error::AiCoreutilsError::InvalidInput(
        format!("Invalid mode specification: {}", mode_str)
    ))
}

fn parse_symbolic_mode(mode_str: &str) -> Result<Option<(Option<char>, char, u32)>> {
    let chars: Vec<char> = mode_str.chars().collect();

    // Parse who part (u, g, o, a)
    let mut idx = 0;
    let mut who = None;
    while idx < chars.len() {
        match chars[idx] {
            'u' | 'g' | 'o' | 'a' => {
                who = Some(chars[idx]);
                idx += 1;
            }
            '+' | '-' | '=' => break,
            _ => {
                return Err(ai_coreutils::error::AiCoreutilsError::InvalidInput(
                    format!("Invalid who in mode: {}", mode_str)
                ));
            }
        }
    }

    if idx >= chars.len() {
        return Err(ai_coreutils::error::AiCoreutilsError::InvalidInput(
            format!("Missing operator in mode: {}", mode_str)
        ));
    }

    // Parse operator (+, -, =)
    let op = chars[idx];
    if !matches!(op, '+' | '-' | '=') {
        return Err(ai_coreutils::error::AiCoreutilsError::InvalidInput(
            format!("Invalid operator in mode: {}", mode_str)
        ));
    }
    idx += 1;

    // Parse permissions (r, w, x, X)
    let mut permissions = 0u32;
    while idx < chars.len() {
        match chars[idx] {
            'r' => permissions |= 0o444,
            'w' => permissions |= 0o222,
            'x' => permissions |= 0o111,
            'X' => {
                // X is special: execute only if directory or already executable
                permissions |= 0o111;
            }
            's' | 't' => {
                // Special bits - for now, just skip
                // In a full implementation, these would set setuid/setgid/sticky
            }
            _ => {
                return Err(ai_coreutils::error::AiCoreutilsError::InvalidInput(
                    format!("Invalid permission in mode: {}", mode_str)
                ));
            }
        }
        idx += 1;
    }

    Ok(Some((who, op, permissions)))
}

fn change_permissions(
    path: &Path,
    cli: &Cli,
    mode_spec: &ModeSpec,
    stats: &mut ChmodStats,
) -> Result<()> {
    // Check if path exists
    if !path.exists() {
        return Err(ai_coreutils::error::AiCoreutilsError::PathNotFound(path.to_path_buf()));
    }

    let is_dir = path.is_dir();

    // Get current permissions
    let metadata = fs::metadata(path)?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        let current_mode = metadata.permissions().mode();
        let new_mode = calculate_new_mode(current_mode, mode_spec)?;

        // Set new permissions
        let mut new_perms = metadata.permissions().clone();
        new_perms.set_mode(new_mode);
        fs::set_permissions(path, new_perms)?;

        // Update stats
        if is_dir {
            stats.dirs_modified += 1;
        } else {
            stats.files_modified += 1;
        }

        if cli.verbose {
            jsonl::output_info(serde_json::json!({
                "type": "permissions_changed",
                "path": path.display().to_string(),
                "old_mode": format!("{:04o}", current_mode & 0o7777),
                "new_mode": format!("{:04o}", new_mode & 0o7777),
            }))?;
        }
    }

    #[cfg(windows)]
    {
        // On Windows, chmod is more limited
        // We can only set readonly flag
        if let ModeSpec::Absolute(mode) = mode_spec {
            let readonly = (mode & 0o222) == 0; // No write permission = readonly
            let mut perms = metadata.permissions();
            perms.set_readonly(readonly);
            fs::set_permissions(path, perms)?;

            if is_dir {
                stats.dirs_modified += 1;
            } else {
                stats.files_modified += 1;
            }

            if cli.verbose {
                jsonl::output_info(serde_json::json!({
                    "type": "permissions_changed",
                    "path": path.display().to_string(),
                    "readonly": readonly,
                }))?;
            }
        }
    }

    // Recursive handling
    if is_dir && cli.recursive {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();

            change_permissions(&entry_path, cli, mode_spec, stats)?;
        }
    }

    Ok(())
}

#[allow(dead_code)]
fn calculate_new_mode(current_mode: u32, mode_spec: &ModeSpec) -> Result<u32> {
    match mode_spec {
        ModeSpec::Absolute(mode) => {
            // Absolute mode: replace the lower 12 bits (preserving file type bits)
            Ok((current_mode & 0o770000) | (mode & 0o7777))
        }
        ModeSpec::Symbolic { who, op, permissions } => {
            let mut new_mode = current_mode;

            // Determine which bits to modify
            let mask = match who {
                Some('u') => 0o4700,  // User bits
                Some('g') => 0o2070,  // Group bits
                Some('o') => 0o1007,  // Other bits
                Some('a') | None => 0o7777,  // All bits
                _ => return Err(ai_coreutils::error::AiCoreutilsError::InvalidInput(
                    "Invalid who in symbolic mode".to_string()
                )),
            };

            match op {
                '+' => {
                    // Add permissions
                    new_mode |= permissions & mask;
                }
                '-' => {
                    // Remove permissions
                    new_mode &= !(permissions & mask);
                }
                '=' => {
                    // Set exact permissions
                    new_mode = (new_mode & !mask) | (permissions & mask);
                }
                _ => {
                    return Err(ai_coreutils::error::AiCoreutilsError::InvalidInput(
                        "Invalid operator in symbolic mode".to_string()
                    ));
                }
            }

            Ok(new_mode)
        }
    }
}
