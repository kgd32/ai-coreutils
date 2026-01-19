//! Error types for AI-Coreutils
//!
//! Provides unified error handling across all utilities.

use std::path::PathBuf;
use thiserror::Error;

/// Main error type for AI-Coreutils
#[derive(Error, Debug)]
pub enum AiCoreutilsError {
    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Memory access error
    #[error("Memory access error: {0}")]
    MemoryAccess(String),

    /// JSON serialization error
    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),

    /// Path not found
    #[error("Path not found: {0}")]
    PathNotFound(PathBuf),

    /// Permission denied
    #[error("Permission denied: {0}")]
    PermissionDenied(PathBuf),

    /// Invalid input
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// Operation not supported
    #[error("Operation not supported: {0}")]
    NotSupported(String),

    /// WalkDir error
    #[error("Directory traversal error: {0}")]
    WalkDir(#[from] walkdir::Error),
}

/// Result type alias for AI-Coreutils
pub type Result<T> = std::result::Result<T, AiCoreutilsError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = AiCoreutilsError::InvalidInput("test".to_string());
        assert!(err.to_string().contains("Invalid input"));
    }

    #[test]
    fn test_error_from_io() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "test");
        let err: AiCoreutilsError = io_err.into();
        assert!(matches!(err, AiCoreutilsError::Io(_)));
    }
}
