# E2E Test Fixtures

This directory contains test fixtures for end-to-end testing of AI-Coreutils.

## Fixture Files

- `empty.txt` - Empty file for edge case testing
- `small.txt` - Small file (< 1KB)
- `medium.txt` - Medium file (~50 bytes)
- `large.txt` - Large file (> 100KB)
- `unicode.txt` - Unicode content with special characters
- `binary.dat` - Binary data for testing binary file handling
- `patterns.txt` - File with various pattern types (email, URL, IP, etc.)
- `multiline.txt` - Multi-line text for context testing
- `special-chars.txt` - Special characters and escape sequences
- `json-sample.jsonl` - Sample JSONL output
- `code-sample.rs` - Rust source code for syntax highlighting tests
- `config-file.ini` - Configuration file format

## Directory Structure

- `nested/` - Nested directory structure for recursive tests
- `readonly/` - Read-only files (on supported systems)
- `symlinks/` - Symbolic link tests (on supported systems)
