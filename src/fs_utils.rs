//! File system utilities
//!
//! Common file system operations used across AI-Coreutils.

use crate::error::{AiCoreutilsError, Result};
use std::fs;
use std::path::{Path, PathBuf};

/// Get file metadata as a structured value
pub fn get_file_metadata(path: &Path) -> Result<serde_json::Value> {
    let metadata = fs::metadata(path)
        .map_err(AiCoreutilsError::Io)?;

    Ok(serde_json::json!({
        "size": metadata.len(),
        "is_dir": metadata.is_dir(),
        "is_file": metadata.is_file(),
        "is_symlink": metadata.is_symlink(),
        "modified": metadata.modified()
            .ok()
            .and_then(|t| {
                let secs = t.duration_since(std::time::UNIX_EPOCH).ok()?.as_secs();
                Some(secs)
            }),
        "readonly": metadata.permissions().readonly(),
    }))
}

/// Check if a path exists
pub fn path_exists(path: &Path) -> bool {
    path.exists()
}

/// Validate that a path is accessible
pub fn validate_path(path: &Path) -> Result<()> {
    if !path.exists() {
        return Err(AiCoreutilsError::PathNotFound(path.to_path_buf()));
    }

    // Try to access the path to check permissions
    fs::metadata(path)
        .map_err(|e| match e.kind() {
            std::io::ErrorKind::PermissionDenied => {
                AiCoreutilsError::PermissionDenied(path.to_path_buf())
            }
            _ => AiCoreutilsError::Io(e),
        })?;

    Ok(())
}

/// Resolve a path to its absolute form
pub fn resolve_path(path: &Path) -> Result<PathBuf> {
    path.canonicalize()
        .map_err(AiCoreutilsError::Io)
}

/// Check if a path is within a base directory (for security)
pub fn is_path_within_base(path: &Path, base: &Path) -> bool {
    let resolved_path = match path.canonicalize() {
        Ok(p) => p,
        Err(_) => return false,
    };

    let resolved_base = match base.canonicalize() {
        Ok(p) => p,
        Err(_) => return false,
    };

    resolved_path.starts_with(&resolved_base)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_get_file_metadata() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");

        fs::write(&file_path, b"Hello, World!").unwrap();

        let metadata = get_file_metadata(&file_path).unwrap();
        assert_eq!(metadata["size"], 13);
        assert_eq!(metadata["is_file"], true);
        assert_eq!(metadata["is_dir"], false);
    }

    #[test]
    fn test_validate_path() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");

        fs::write(&file_path, b"test").unwrap();

        assert!(validate_path(&file_path).is_ok());

        let non_existent = temp_dir.path().join("nonexistent.txt");
        assert!(validate_path(&non_existent).is_err());
    }

    #[test]
    fn test_is_path_within_base() {
        let temp_dir = TempDir::new().unwrap();
        let base = temp_dir.path();

        let safe_path = base.join("safe.txt");
        let unsafe_path = temp_dir.path().join("../outside.txt");

        // Create the safe path
        fs::write(&safe_path, b"safe").unwrap();

        assert!(is_path_within_base(&safe_path, base));
        assert!(!is_path_within_base(&unsafe_path, base));
    }
}
