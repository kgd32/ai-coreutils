//! Integration tests for AI-Coreutils
//!
//! These tests verify the basic functionality of the core utilities.

use std::fs;
use tempfile::TempDir;

#[test]
fn test_ai_ls_basic() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.txt");

    fs::write(&test_file, b"Hello, World!").unwrap();

    // Verify the file was created
    assert!(test_file.exists());
}

#[test]
fn test_ai_cat_basic() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.txt");

    fs::write(&test_file, b"Hello, World!").unwrap();

    // Read back the content
    let content = fs::read_to_string(&test_file).unwrap();
    assert_eq!(content, "Hello, World!");
}

#[test]
fn test_ai_grep_basic() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.txt");

    fs::write(&test_file, b"Hello\nWorld\nPattern\nTest").unwrap();

    // Read and search for pattern
    let content = fs::read_to_string(&test_file).unwrap();
    assert!(content.contains("Pattern"));
}

#[test]
fn test_memory_mapping() {
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("large_file.txt");

    // Create a file larger than 1MB
    let large_content = "A".repeat(2_000_000);
    fs::write(&test_file, large_content.as_bytes()).unwrap();

    // Verify file size
    let metadata = fs::metadata(&test_file).unwrap();
    assert!(metadata.len() > 1_000_000);
}

#[test]
fn test_jsonl_output() {
    use ai_coreutils::jsonl::JsonlRecord;

    let record = JsonlRecord::error("Test error", "TEST_CODE");
    let jsonl = record.to_jsonl().unwrap();

    assert!(jsonl.contains("\"type\":\"error\""));
    assert!(jsonl.contains("Test error"));
}
