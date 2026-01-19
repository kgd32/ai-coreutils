---
name: dev-agent
description: Primary development skill for implementing features, writing code, fixing bugs, and making changes to the AI-Coreutils Rust project. Invokes auto-doc automatically after committing changes. Use for any code work including utilities, tests, benchmarks, or refactoring.
---

# Development Agent

> **Purpose**: Primary skill for AI-Coreutils development tasks
> **Expertise**: Rust, memory mapping, JSONL, CLI utilities

---

## ⚠️ Auto-Documentation Integration

**IMPORTANT**: After completing any work, the **auto-doc** skill is automatically invoked.

You do NOT need to manually run documentation. It happens automatically after:
- Code changes are committed
- Tasks are completed
- Tests are run

The auto-doc skill will:
1. Update CLAUDE.md with learnings
2. Update ralph.yml with task status
3. Create session log in `.agent/sessions/`
4. Commit documentation changes

---

## Core Principle: Develop with Auto-Documentation

**CRITICAL**: Focus on your work. Documentation happens automatically.

Just:
- Write quality code
- Run tests
- Commit changes

The auto-doc skill handles the rest.

---

## Invocation

```
skill: "dev-agent" --task "<task_description>"
```

### Examples

```bash
# Implement ai-ls
skill: "dev-agent" --task "Implement ai-ls with walkdir and JSONL output"

# Fix memory mapping bug
skill: "dev-agent" --task "Fix memory mapping issue on large files"

# Add benchmark
skill: "dev-agent" --task "Add memory access benchmark"
```

---

## Behavior

### Step 1: Understand the Task
- Read CLAUDE.md for context
- **Read gnu-core-utils.md for utility specifications** ⚠️ BLUEPRINT
- Read ralph.yml for task status
- Read scratchpad.md for previous context

### Step 2: Implement
- Implement according to gnu-core-utils.md specifications
- Add AI enhancements (JSONL, memory access, etc.)
- Write code following Rust best practices
- Use `cargo check` to verify compilation
- Use `cargo test` to verify tests pass
- Use `cargo clippy` for linting

### Step 3: Commit
```bash
git add .
git commit -m "feat: [description]"
git push
```

### Step 4: Auto-Documentation (Automatic)
- The auto-doc skill is automatically invoked
- CLAUDE.md updated with learnings
- ralph.yml updated with task status
- Session log created
- Documentation committed separately

---

## Rust Development Guidelines

### Memory Safety
- Always use bounds checking
- Prefer safe abstractions over `unsafe`
- Use `memmap2` for large files

### Error Handling
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
```

### JSONL Output
All utilities must output JSONL format:
```json
{"type": "result", "data": {...}}
{"type": "error", "message": "..."}
```

### CLI Pattern
```rust
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "ai-utility")]
struct Cli {
    #[arg(short, long)]
    json: bool,
}
```

---

## Common Commands

```bash
# Check compilation
cargo check

# Run tests
cargo test

# Run specific test
cargo test test_name

# Build release
cargo build --release

# Run benchmarks
cargo bench

# Format code
cargo fmt

# Lint code
cargo clippy

# Test utility
cargo run --bin ai-ls -- --help
```

---

## Project Structure

```
ai-coreutils/
├── src/
│   ├── lib.rs              # Library entry point
│   ├── error.rs            # Error types
│   ├── jsonl.rs            # JSONL output
│   ├── memory.rs           # Memory access
│   └── bin/
│       ├── ai-ls.rs        # ls utility
│       ├── ai-cat.rs       # cat utility
│       └── ai-grep.rs      # grep utility
├── tests/                  # Integration tests
├── benches/                # Benchmarks
├── CLAUDE.md               # Agent knowledge
├── ralph.yml               # Task tracking
└── .agent/
    ├── scratchpad.md       # Context
    └── sessions/           # Session logs
```

---

## Verification

Before completing a task:
- ✅ Code compiles without warnings (or only expected ones)
- ✅ Tests pass
- ✅ Documentation updated
- ✅ ralph.yml status updated
- ✅ Changes committed

---

## ⚠️ Blueprint Reference

**CRITICAL**: gnu-core-utils.md is the PRIMARY SPECIFICATION for all utilities.

Before implementing any utility:
1. Read the utility's section in gnu-core-utils.md
2. Understand the specifications (options, behavior, exit codes)
3. Implement according to specs
4. Add AI enhancements (JSONL, memory access, etc.)

**gnu-core-utils.md is our blueprint - always reference it.**

---

**Last Updated**: 2026-01-19
