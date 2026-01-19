//! AI-optimized chown utility
//!
//! Changes file owner and group with JSONL output.

use ai_coreutils::jsonl;
use ai_coreutils::Result;
use clap::Parser;
use std::path::PathBuf;

/// AI-optimized chown: Change ownership with JSONL output
#[derive(Parser, Debug)]
#[command(name = "ai-chown")]
#[command(about = "AI-optimized chown with structured output", long_about = None)]
struct Cli {
    /// Owner specification (user[:group])
    #[arg(required = true)]
    owner: String,

    /// Files/directories to modify
    #[arg(required = true)]
    paths: Vec<PathBuf>,

    /// Recursive ownership change
    #[arg(short = 'R', long)]
    recursive: bool,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Produce output in JSONL format (always enabled)
    #[arg(short, long, default_value_t = true)]
    json: bool,
}

#[derive(Debug, Clone)]
struct OwnerSpec {
    #[allow(dead_code)]
    uid: Option<u32>,
    #[allow(dead_code)]
    gid: Option<u32>,
}

#[derive(Debug, Clone)]
struct ChownStats {
    files_modified: u64,
    dirs_modified: u64,
    errors: u64,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let mut stats = ChownStats {
        files_modified: 0,
        dirs_modified: 0,
        errors: 0,
    };

    // Parse the owner specification
    let _owner_spec = parse_owner(&cli.owner)?;

    #[cfg(unix)]
    {
        // Apply ownership changes to each path
        for path in &cli.paths {
            if let Err(e) = change_ownership(path, &cli, &owner_spec, &mut stats) {
                stats.errors += 1;
                jsonl::output_error(
                    &format!("Failed to change ownership for {}: {}", path.display(), e),
                    "CHOWN_ERROR",
                    Some(&path.to_string_lossy()),
                )?;
            }
        }
    }

    #[cfg(windows)]
    {
        // On Windows, chown is not supported in the same way
        // We output a message explaining this
        jsonl::output_info(serde_json::json!({
            "type": "platform_info",
            "message": "chown is not supported on Windows - file ownership is managed differently",
        }))?;

        // Still iterate through paths to count them
        for path in &cli.paths {
            if path.exists() {
                if path.is_file() {
                    stats.files_modified += 1;
                } else {
                    stats.dirs_modified += 1;
                }
            }
        }
    }

    // Output final stats
    jsonl::output_result(serde_json::json!({
        "type": "chown_summary",
        "files_modified": stats.files_modified,
        "dirs_modified": stats.dirs_modified,
        "errors": stats.errors,
        "owner": cli.owner,
    }))?;

    Ok(())
}

fn parse_owner(owner_str: &str) -> Result<OwnerSpec> {
    let parts: Vec<&str> = owner_str.split(':').collect();

    let uid = if !parts[0].is_empty() {
        Some(parse_user_id(parts[0])?)
    } else {
        None
    };

    let gid = if parts.len() > 1 && !parts[1].is_empty() {
        Some(parse_group_id(parts[1])?)
    } else {
        None
    };

    if uid.is_none() && gid.is_none() {
        return Err(ai_coreutils::error::AiCoreutilsError::InvalidInput(
            "Invalid owner specification".to_string()
        ));
    }

    Ok(OwnerSpec { uid, gid })
}

#[cfg(unix)]
fn parse_user_id(user: &str) -> Result<u32> {
    use std::os::unix::fs::MetadataExt;

    // Try parsing as numeric UID first
    if let Ok(uid) = user.parse::<u32>() {
        return Ok(uid);
    }

    // Try to look up username
    #[cfg(feature = "user_lookup")]
    {
        // In a full implementation, you'd use the `users` crate or similar
        // For now, return an error
        return Err(ai_coreutils::error::AiCoreutilsError::InvalidInput(
            format!("Username lookup not implemented: {}", user)
        ));
    }

    #[cfg(not(feature = "user_lookup"))]
    {
        // Can't look up usernames without additional dependencies
        // Try parsing as number or fail
        user.parse::<u32>()
            .map_err(|_| ai_coreutils::error::AiCoreutilsError::InvalidInput(
                format!("Invalid UID or username not found: {}", user)
            ))
    }
}

#[cfg(windows)]
fn parse_user_id(user: &str) -> Result<u32> {
    // On Windows, we don't have the same concept of UIDs
    // Just try to parse as a number
    user.parse::<u32>()
        .map_err(|_| ai_coreutils::error::AiCoreutilsError::InvalidInput(
            format!("Invalid UID: {}", user)
        ))
}

#[cfg(unix)]
fn parse_group_id(group: &str) -> Result<u32> {
    // Try parsing as numeric GID first
    if let Ok(gid) = group.parse::<u32>() {
        return Ok(gid);
    }

    // Try to look up group name
    #[cfg(feature = "user_lookup")]
    {
        // In a full implementation, you'd use the `users` crate or similar
        return Err(ai_coreutils::error::AiCoreutilsError::InvalidInput(
            format!("Group lookup not implemented: {}", group)
        ));
    }

    #[cfg(not(feature = "user_lookup"))]
    {
        group.parse::<u32>()
            .map_err(|_| ai_coreutils::error::AiCoreutilsError::InvalidInput(
                format!("Invalid GID or group not found: {}", group)
            ))
    }
}

#[cfg(windows)]
fn parse_group_id(group: &str) -> Result<u32> {
    // On Windows, we don't have the same concept of GIDs
    group.parse::<u32>()
        .map_err(|_| ai_coreutils::error::AiCoreutilsError::InvalidInput(
            format!("Invalid GID: {}", group)
        ))
}

#[cfg(unix)]
fn change_ownership(
    path: &Path,
    cli: &Cli,
    owner_spec: &OwnerSpec,
    stats: &mut ChownStats,
) -> Result<()> {
    use std::os::unix::fs::MetadataExt;

    // Check if path exists
    if !path.exists() {
        return Err(ai_coreutils::error::AiCoreutilsError::PathNotFound(path.to_path_buf()));
    }

    let is_dir = path.is_dir();

    // Get current ownership
    let metadata = fs::metadata(path)?;
    let current_uid = metadata.uid();
    let current_gid = metadata.gid();

    let new_uid = owner_spec.uid.unwrap_or(current_uid);
    let new_gid = owner_spec.gid.unwrap_or(current_gid);

    // Change ownership using chown system call
    unsafe {
        use libc::{chown, strlen};
        use std::ffi::CString;

        let path_cstr = CString::new(path.to_string_lossy().as_ref())
            .map_err(|_| ai_coreutils::error::AiCoreutilsError::InvalidInput(
                "Invalid path for chown".to_string()
            ))?;

        let result = chown(
            path_cstr.as_ptr(),
            new_uid,
            new_gid,
        );

        if result != 0 {
            return Err(ai_coreutils::error::AiCoreutilsError::Io(
                std::io::Error::last_os_error()
            ));
        }
    }

    // Update stats
    if is_dir {
        stats.dirs_modified += 1;
    } else {
        stats.files_modified += 1;
    }

    if cli.verbose {
        jsonl::output_info(serde_json::json!({
            "type": "ownership_changed",
            "path": path.display().to_string(),
            "old_uid": current_uid,
            "old_gid": current_gid,
            "new_uid": new_uid,
            "new_gid": new_gid,
        }))?;
    }

    // Recursive handling
    if is_dir && cli.recursive {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();

            change_ownership(&entry_path, cli, owner_spec, stats)?;
        }
    }

    Ok(())
}
