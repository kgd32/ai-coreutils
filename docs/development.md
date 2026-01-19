# Development Guide

Guide to building, testing, and contributing to AI-Coreutils.

## Table of Contents

1. [Building](#building)
2. [Testing](#testing)
3. [Development Workflow](#development-workflow)
4. [Code Style](#code-style)
5. [Adding Features](#adding-features)
6. [Contributing](#contributing)

## Building

### Prerequisites

- Rust 1.70 or later
- Git
- (Optional) Python 3.8+ for bindings
- (Optional) Node.js 18+ for bindings

### Clone Repository

```bash
git clone https://github.com/your-org/ai-coreutils
cd ai-coreutils
```

### Development Build

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Build specific binary
cargo build --bin ai-ls
```

### Build with Features

```bash
# Build with Python bindings
cargo build --features python

# Build all binaries
cargo build --bins
```

### Build Output

Binaries are placed in:
- Debug: `target/debug/ai-<utility>`
- Release: `target/release/ai-<utility>`

## Testing

### Run All Tests

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run tests in parallel
cargo test -- --test-threads=4
```

### Specific Tests

```bash
# Library tests only
cargo test --lib

# Integration tests only
cargo test --test '*'

# Specific test
cargo test test_memory_access
```

### Test Coverage

```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --out Html

# View coverage
open tarpaulin-report.html
```

### Benchmarks

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark
cargo bench --bench memory_access

# Save baseline
cargo bench --bench memory_access -- --save-baseline main

# Compare with baseline
cargo bench --bench memory_access -- --baseline main
```

## Development Workflow

### 1. Create Feature Branch

```bash
git checkout -b feature/new-utility
```

### 2. Make Changes

```bash
# Edit files
# ...

# Check compilation
cargo check

# Run tests
cargo test

# Format code
cargo fmt

# Run linter
cargo clippy
```

### 3. Commit Changes

```bash
git add .
git commit -m "feat: add new utility"
```

### 4. Push and Create PR

```bash
git push origin feature/new-utility
# Create PR on GitHub
```

## Code Style

### Formatting

```bash
# Format all code
cargo fmt

# Check formatting
cargo fmt -- --check
```

### Linting

```bash
# Run clippy
cargo clippy

# Fix clippy warnings
cargo clippy --fix
```

### Common Clippy Warnings

```rust
// DON'T - Redundant closure
let _ = (0..10).map(|x| x + 1).collect::<Vec<_>>();

// DO - Use direct variant
use AiCoreutilsError::Io;
let _ = (0..10).map(|x| x + 1).collect::<Vec<_>>();

// DON'T - Unnecessary cast
let x: u8 = 42;
let y = x as u8;

// DON'T - Ptr arg (use &Path instead &PathBuf)
fn foo(path: &PathBuf) { }
fn foo(path: &Path) { }  // DO
```

## Project Structure

```
ai-coreutils/
├── src/
│   ├── bin/              # Utility binaries
│   │   ├── ai-ls.rs
│   │   ├── ai-cat.rs
│   │   └── ...
│   ├── memory.rs         # Memory access module
│   ├── jsonl.rs          # JSONL output module
│   ├── error.rs          # Error types
│   ├── async_ops.rs      # Async operations
│   ├── simd_ops.rs       # SIMD operations
│   ├── ml_ops.rs         # ML operations
│   └── lib.rs            # Library entry point
├── tests/                # Integration tests
├── benches/              # Benchmarks
├── docs/                 # Documentation
├── nodejs/               # Node.js bindings
├── examples/             # Example code
├── Cargo.toml            # Dependencies
└── README.md             # Project readme
```

## Adding Features

### Adding a New Utility

1. Create binary file: `src/bin/ai-utility.rs`

```rust
use ai_coreutils::{jsonl, Result};
use clap::Parser;

/// Utility description
#[derive(Parser, Debug)]
#[command(name = "ai-utility")]
#[command(about = "Utility description", long_about = None)]
struct Cli {
    // Add CLI arguments
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    // Implement utility
    Ok(())
}
```

2. Add to `Cargo.toml`:

```toml
[[bin]]
name = "ai-utility"
path = "src/bin/ai-utility.rs"
```

3. Add tests in `tests/`

4. Add documentation in `docs/utilities/ai-utility.md`

### Adding a New Module

1. Create module file: `src/new_module.rs`

2. Export in `src/lib.rs`:

```rust
pub mod new_module;
```

3. Add tests

4. Add documentation

## Code Standards

### Error Handling

```rust
// Use Result<T> return type
fn do_something() -> Result<String> {
    // Use ? operator for propagation
    let data = std::fs::read_to_string("file.txt")?;
    Ok(data)
}

// Use map_err for custom errors
let file = File::open(path)
    .map_err(AiCoreutilsError::Io)?;
```

### Documentation

```rust
/// Brief description of the function.
///
/// More detailed explanation if needed.
///
/// # Arguments
///
/// * `arg1` - Description of arg1
///
/// # Returns
///
/// * `Result<T>` - Description of return value
///
/// # Examples
///
/// ```
/// use ai_coreutils::function_name;
/// let result = function_name()?;
/// ```
pub fn function_name(arg1: &str) -> Result<String> {
    // ...
}
```

### Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_functionality() {
        let result = function_name("input").unwrap();
        assert_eq!(result, "expected");
    }

    #[test]
    fn test_error_handling() {
        let result = function_name("");
        assert!(result.is_err());
    }
}
```

## Release Process

### Update Version

1. Update `Cargo.toml` version

```toml
[package]
version = "0.2.0"
```

2. Update CHANGELOG.md

3. Commit changes

```bash
git add .
git commit -m "chore: bump version to 0.2.0"
```

### Tag Release

```bash
git tag v0.2.0
git push origin v0.2.0
```

### Publish to Crates.io

```bash
# Login
cargo login

# Publish
cargo publish

# Publish with features
cargo publish --features python
```

## Contributing

### Bug Reports

Include:
- Rust version (`rustc --version`)
- Operating system
- Minimal reproduction code
- Expected vs actual behavior

### Feature Requests

Include:
- Use case description
- Proposed API/CLI changes
- Examples of how it would be used
- Potential implementation approach

### Pull Requests

Before submitting:
1. Run `cargo test` - all tests pass
2. Run `cargo clippy` - no warnings
3. Run `cargo fmt` - code formatted
4. Add tests for new features
5. Update documentation

PR Checklist:
- [ ] Tests added/updated
- [ ] Documentation updated
- [ ] CHANGELOG.md updated
- [ ] All tests pass
- [ ] No clippy warnings
- [ ] Code formatted

### Code Review

- Be constructive and respectful
- Focus on code quality and design
- Suggest improvements
- Ask questions for clarity

## Useful Commands

### Development

```bash
# Watch for changes and recompile
cargo watch -x build

# Run tests on file change
cargo watch -x test

# Format and check
cargo fmt && cargo clippy && cargo test
```

### Debugging

```bash
# Build with debug symbols
cargo build

# Run with debugger
rust-gdb target/debug/ai-ls

# Print backtrace
RUST_BACKTRACE=1 cargo run --bin ai-ls
```

### Documentation

```bash
# Generate documentation
cargo doc --no-deps --open

# Document all items (including private)
cargo doc --document-private-items
```

## Resources

### Rust Documentation

- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [API Guidelines](https://rust-lang.github.io/api-guidelines/)

### Dependencies

- [clap](https://docs.rs/clap/) - CLI argument parsing
- [memmap2](https://docs.rs/memmap2/) - Memory mapping
- [serde_json](https://docs.rs/serde_json/) - JSON serialization
- [tokio](https://docs.rs/tokio/) - Async runtime
- [regex](https://docs.rs/regex/) - Regular expressions

### Project Specific

- [gnu-core-utils.md](../gnu-core-utils.md) - Utility specifications
- [CLAUDE.md](../CLAUDE.md) - Project learnings
- [bindings.md](bindings.md) - Language bindings

## Getting Help

- GitHub Issues: https://github.com/your-org/ai-coreutils/issues
- Discussions: https://github.com/your-org/ai-coreutils/discussions
- Documentation: https://docs.ai-coreutils.dev

## License

MIT OR Apache-2.0 - See LICENSE files for details.
