//! JSONL output formatter
//!
//! Provides structured JSONL output for all AI-Coreutils operations.

use crate::error::Result;
use crate::AiCoreutilsError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::io::Write;

/// JSONL record types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum JsonlRecord {
    /// Error record
    #[serde(rename = "error")]
    Error {
        /// Timestamp when the error occurred
        timestamp: DateTime<Utc>,
        /// Error message
        message: String,
        /// Error code
        code: String,
    },

    /// Result record
    #[serde(rename = "result")]
    Result {
        /// Timestamp when the result was generated
        timestamp: DateTime<Utc>,
        /// Result data
        data: serde_json::Value,
    },

    /// Metadata record
    #[serde(rename = "metadata")]
    Metadata {
        /// Timestamp when the metadata was generated
        timestamp: DateTime<Utc>,
        /// Metadata information
        info: serde_json::Value,
    },

    /// Progress record for long operations
    #[serde(rename = "progress")]
    Progress {
        /// Timestamp when the progress was reported
        timestamp: DateTime<Utc>,
        /// Current progress count
        current: usize,
        /// Total items to process
        total: usize,
        /// Progress message
        message: String,
    },

    /// File entry record (for directory listings)
    #[serde(rename = "file")]
    FileEntry {
        /// Timestamp when the file entry was recorded
        timestamp: DateTime<Utc>,
        /// File path
        path: String,
        /// File size in bytes
        size: u64,
        /// Last modification time
        modified: DateTime<Utc>,
        /// Whether this is a directory
        is_dir: bool,
        /// Whether this is a symbolic link
        is_symlink: bool,
        /// File permissions string
        permissions: String,
    },

    /// Match record (for grep operations)
    #[serde(rename = "match")]
    MatchRecord {
        /// Timestamp when the match was found
        timestamp: DateTime<Utc>,
        /// File path where match was found
        file: String,
        /// Line number of the match
        line_number: usize,
        /// Content of the line
        line_content: String,
        /// Start position of match within line
        match_start: usize,
        /// End position of match within line
        match_end: usize,
    },
}

impl JsonlRecord {
    /// Create a new error record
    pub fn error(message: impl Into<String>, code: impl Into<String>) -> Self {
        JsonlRecord::Error {
            timestamp: Utc::now(),
            message: message.into(),
            code: code.into(),
        }
    }

    /// Create a new result record
    pub fn result(data: serde_json::Value) -> Self {
        JsonlRecord::Result {
            timestamp: Utc::now(),
            data,
        }
    }

    /// Create a new metadata record
    pub fn metadata(info: serde_json::Value) -> Self {
        JsonlRecord::Metadata {
            timestamp: Utc::now(),
            info,
        }
    }

    /// Serialize to JSONL string
    pub fn to_jsonl(&self) -> Result<String> {
        serde_json::to_string(self).map_err(AiCoreutilsError::from)
    }
}

/// JSONL output handler
pub struct JsonlOutput<W: Write> {
    writer: W,
}

impl<W: Write> JsonlOutput<W> {
    /// Create a new JSONL output handler
    pub fn new(writer: W) -> Self {
        Self { writer }
    }

    /// Write a record to the output
    pub fn write_record(&mut self, record: &JsonlRecord) -> Result<()> {
        let jsonl = record.to_jsonl()?;
        writeln!(self.writer, "{}", jsonl)
            .map_err(AiCoreutilsError::Io)?;
        Ok(())
    }

    /// Flush the output
    pub fn flush(&mut self) -> Result<()> {
        self.writer.flush().map_err(AiCoreutilsError::Io)
    }

    /// Write multiple records efficiently
    pub fn write_records(&mut self, records: &[JsonlRecord]) -> Result<()> {
        for record in records {
            self.write_record(record)?;
        }
        Ok(())
    }
}

impl<W: Write> Drop for JsonlOutput<W> {
    fn drop(&mut self) {
        let _ = self.flush();
    }
}

/// Output an error record to stdout
pub fn output_error(message: &str, code: &str, path: Option<&str>) -> Result<()> {
    let mut output = JsonlOutput::new(std::io::stdout());
    let record = match path {
        Some(p) => JsonlRecord::error(
            format!("{}: {}", p, message),
            code
        ),
        None => JsonlRecord::error(message, code),
    };
    output.write_record(&record)?;
    output.flush()
}

/// Output a result record to stdout
pub fn output_result(data: serde_json::Value) -> Result<()> {
    let mut output = JsonlOutput::new(std::io::stdout());
    output.write_record(&JsonlRecord::result(data))?;
    output.flush()
}

/// Output a metadata record to stdout
pub fn output_info(info: serde_json::Value) -> Result<()> {
    let mut output = JsonlOutput::new(std::io::stdout());
    output.write_record(&JsonlRecord::metadata(info))?;
    output.flush()
}

/// Output a progress record to stdout
pub fn output_progress(current: usize, total: usize, message: &str) -> Result<()> {
    let mut output = JsonlOutput::new(std::io::stdout());
    output.write_record(&JsonlRecord::Progress {
        timestamp: Utc::now(),
        current,
        total,
        message: message.to_string(),
    })?;
    output.flush()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_record() {
        let record = JsonlRecord::error("Test error", "TEST_ERR");
        let jsonl = record.to_jsonl().unwrap();
        assert!(jsonl.contains("\"type\":\"error\""));
        assert!(jsonl.contains("Test error"));
    }

    #[test]
    fn test_result_record() {
        let record = JsonlRecord::result(serde_json::json!({"test": "value"}));
        let jsonl = record.to_jsonl().unwrap();
        assert!(jsonl.contains("\"type\":\"result\""));
        assert!(jsonl.contains("test"));
    }

    #[test]
    fn test_file_entry_record() {
        let record = JsonlRecord::FileEntry {
            timestamp: Utc::now(),
            path: "/test/path".to_string(),
            size: 1024,
            modified: Utc::now(),
            is_dir: false,
            is_symlink: false,
            permissions: "rw-r--r--".to_string(),
        };
        let jsonl = record.to_jsonl().unwrap();
        assert!(jsonl.contains("\"type\":\"file\""));
        assert!(jsonl.contains("/test/path"));
    }

    #[test]
    fn test_jsonl_output_to_vec() {
        let mut output = JsonlOutput::new(Vec::new());
        let record = JsonlRecord::error("Test error", "TEST_ERR");
        output.write_record(&record).unwrap();
        let result = String::from_utf8(output.writer.clone()).unwrap();
        assert!(result.contains("Test error"));
    }
}
