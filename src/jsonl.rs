//! JSONL output formatter
//!
//! Provides structured JSONL output for all AI-Coreutils operations.

use crate::error::Result;
use crate::{AiCoreutilsError, Result as CrateResult};
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
        timestamp: DateTime<Utc>,
        message: String,
        code: String,
    },

    /// Result record
    #[serde(rename = "result")]
    Result {
        timestamp: DateTime<Utc>,
        data: serde_json::Value,
    },

    /// Metadata record
    #[serde(rename = "metadata")]
    Metadata {
        timestamp: DateTime<Utc>,
        info: serde_json::Value,
    },

    /// Progress record for long operations
    #[serde(rename = "progress")]
    Progress {
        timestamp: DateTime<Utc>,
        current: usize,
        total: usize,
        message: String,
    },

    /// File entry record (for directory listings)
    #[serde(rename = "file")]
    FileEntry {
        timestamp: DateTime<Utc>,
        path: String,
        size: u64,
        modified: DateTime<Utc>,
        is_dir: bool,
        is_symlink: bool,
        permissions: String,
    },

    /// Match record (for grep operations)
    #[serde(rename = "match")]
    MatchRecord {
        timestamp: DateTime<Utc>,
        file: String,
        line_number: usize,
        line_content: String,
        match_start: usize,
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
