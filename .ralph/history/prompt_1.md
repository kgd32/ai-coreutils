# AI-Coreutils Development Agent Instructions

## Role & Context
You are developing AI-Coreutils, a modern implementation of GNU core utilities specifically designed for AI agents. This project uses Rust for performance and memory safety, provides structured JSONL output, and enables direct memory pointer access.

---

## Available Skills

The following skills are available. **Invoke them manually** when needed.

### Core Skills
- `skill: "dev-agent"` - Development work implementation
- `skill: "test-agent"` - Test execution and verification
- `skill: "phase-agent"` - Phase tracking and management
- `skill: "doc-agent"` - Documentation updates

### Skill Usage

**For development work:**
```
skill: "dev-agent" --task "<description>"
```

**For running tests:**
```
skill: "test-agent" --scope "<scope>"
```

**For phase tracking:**
```
skill: "phase-agent" --action "status"
skill: "phase-agent" --action "complete" --phase "N"
```

**For documentation updates:**
```
skill: "doc-agent" --target "<topic>"
```

**Note**: `auto-doc` runs automatically after work - no manual invocation needed.

---

## Mandatory Status Reporting

At the END of your response, ALWAYS include this status block:

```markdown
---RALPH_STATUS---
STATUS: IN_PROGRESS | COMPLETE | BLOCKED
CURRENT_TASK: {{TASK_ID}}
TASKS_COMPLETED_THIS_LOOP: {{N}}
FILES_MODIFIED: {{N}}
TESTS_STATUS: PASSING | FAILING | NOT_RUN
WORK_TYPE: DOCUMENTATION | IMPLEMENTATION | TESTING | REFACTORING
EXIT_SIGNAL: false | true
RECOMMENDATION: {{one line summary}}
---END_RALPH_STATUS---
```

**Field meanings:**
- `STATUS`: Current task status
- `CURRENT_TASK`: Task ID from ralph.yml
- `TASKS_COMPLETED_THIS_LOOP`: Number of subtasks completed this iteration
- `FILES_MODIFIED`: Number of files changed
- `TESTS_STATUS`: Test results (use NOT_RUN for non-code work)
- `WORK_TYPE`: Type of work performed
- `EXIT_SIGNAL`: Set to `true` when ALL exit criteria are met (see ralph.yml)
- `RECOMMENDATION`: One-line summary of what's next

---

## ✅ MANDATORY: Subtask Tracking

**CRITICAL**: You MUST mark subtasks as complete in `ralph.yml` as you finish them.

### How to Mark Subtasks Complete

When you complete a subtask, add a `✓` checkmark to the end of the subtask line in ralph.yml:

**Before:**
```yaml
subtasks:
  - "Implement SafeMemoryAccess struct"
  - "Add memory mapping support"
```

**After completing first subtask:**
```yaml
subtasks:
  - "Implement SafeMemoryAccess struct" ✓
  - "Add memory mapping support"
```

**Alternative formats (all acceptable):**
```yaml
subtasks:
  - "Complete implementation" ✓      # Checkmark
  - "~~Add tests~~"                  # Strikethrough
  - "[x] Write docs"                 # Checkbox style
```

### When to Mark Subtasks

Mark a subtask as complete when:
- ✅ The work is done and verified
- ✅ Tests pass (if applicable)
- ✅ Code compiles without warnings (if applicable)
- ✅ Documentation is accurate (if docs)

**DO THIS EVERY ITERATION** - Don't wait until the entire task is done!

---

## Your Workflow

### 1. Orient Yourself (Every Iteration)
- Read CLAUDE.md FIRST to understand what works and what doesn't
- Read gnu-core-utils.md for utility specifications (if implementing utilities)
- Check ralph.yml for current task, subtasks, and acceptance criteria
- Identify which subtasks are already marked complete (✓)
- Run `cargo check` to ensure project compiles
- Review recent commits with `git log --oneline -5`

### 2. Pick the Next Subtask

From ralph.yml, find the first subtask without a ✓ and work on it.

Example:
```yaml
subtasks:
  - "Implement SafeMemoryAccess struct" ✓      # Already done
  - "Add memory mapping support"               ← DO THIS ONE
  - "Implement bounds checking"
```

### 3. Execute the Subtask

- Check CLAUDE.md for relevant patterns
- Implement according to specifications (gnu-core-utils.md or task description)
- Add tests if applicable
- Verify the work is correct

### 4. Mark the Subtask Complete

Update ralph.yml to mark the subtask with ✓:
```yaml
subtasks:
  - "Implement SafeMemoryAccess struct" ✓
  - "Add memory mapping support" ✓              # Just completed!
  - "Implement bounds checking"                 ← Next one
```

### 5. Invoke Appropriate Skill

After completing work, invoke the relevant skill:
```
skill: "dev-agent" --task "completed [subtask]"
skill: "test-agent" --scope "[affected area]"
skill: "doc-agent" --target "completed [file]"
```

### 6. Update Scratchpad

Update `.ralph/scratchpad.md` with:
- What was accomplished
- Which subtasks were completed
- What was tried but didn't work
- What should be tackled next
- Any blockers encountered

### 7. Check Completion Status

From ralph.yml, check:
- Are all subtasks marked with ✓?
- Are all acceptance criteria met?
- Is the task ready to be marked "done"?

### 8. Commit Your Changes

```bash
git add .
git commit -m "feat: [brief description of change]"
# or
git commit -m "docs: [brief description]"
# or
git commit -m "fix: [brief description]"
```

---

## When to Set EXIT_SIGNAL: true

Set EXIT_SIGNAL to **true** in your RALPH_STATUS block when ALL of these conditions are met:

1. All subtasks in the current task are marked with ✓
2. All acceptance criteria in ralph.yml are satisfied
3. Tests are passing (or not applicable for documentation)
4. Code compiles without warnings (or not applicable)
5. The task is fully complete and ready to move to the next one

**Check ralph.yml** for the specific acceptance criteria of each task.

---

## Project Context

### Tech Stack
- Language: Rust
- Memory: memmap2 crate for memory mapping
- Serialization: serde_json for JSONL
- CLI: clap for argument parsing
- Async: tokio for concurrent operations
- SIMD: AVX2/SSE2 intrinsics (x86_64)

### Implemented Utilities (15 total)
- ls, cat, grep, touch, mkdir, rmdir, head, tail, wc
- cp, mv, rm, find, chmod, chown
- analyze (AI-enhanced analysis tool)

### Key Features
- JSONL structured output
- Memory-mapped file access for large files
- SIMD-accelerated text processing
- Async/await support for concurrent operations
- ML-based pattern detection and file classification
- Python (PyO3) and Node.js (NAPI-RS) bindings

---

## Critical Guidelines

### Code Quality Standards
- Use `cargo fmt` for formatting
- Use `cargo clippy` for linting
- All public functions must have documentation comments
- Error handling must use `Result<T, E>` pattern
- Memory safety is paramount - no unsafe code without justification

### AI-Coreutils Requirements
- All utilities must output JSONL by default
- Memory operations must use memmap2 for large files
- CLI interface must mirror GNU coreutils but with AI enhancements
- Library API must be thread-safe

### Error Handling Pattern
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AiCoreutilsError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Memory access error: {0}")]
    MemoryAccess(String),

    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, AiCoreutilsError>;
```

### CLI Pattern
```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "ai-coreutils")]
#[command(about = "AI-optimized core utilities")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(short, long)]
    pub json: bool, // Always true for AI agents

    #[arg(short, long)]
    pub mem_ptr: bool, // Enable memory pointer access
}
```

---

## Don't Do This
- Don't use unsafe code without extensive documentation
- Don't ignore error returns
- Don't break compatibility with GNU coreutils basics
- Don't implement memory operations without proper bounds checking
- Don't use blocking I/O in async contexts
- **Don't forget to mark subtasks complete with ✓**

## Do This
- Always benchmark new implementations
- Use memory mapping for files > 1MB
- Provide structured error messages in JSONL format
- Write comprehensive tests for edge cases
- Document all memory access patterns
- **Mark subtasks complete immediately after finishing them**

---

## When You're Stuck
1. **Check gnu-core-utils.md for utility specifications** (PRIMARY REFERENCE)
2. Check CLAUDE.md for similar solved problems
3. Look at GNU coreutils source for reference
4. Consider if the operation can be memory-mapped
5. Update scratchpad.md with the blocker
6. Mark task as "blocked" in ralph.yml with reason
7. **Mark any completed subtasks with ✓ before blocking**

---

## Quick Reference Commands

```bash
cargo build                    # Build project
cargo test                     # Run tests
cargo bench                    # Run benchmarks
cargo fmt                      # Format code
cargo clippy                   # Lint code
cargo doc --no-deps --open     # Generate and view rustdoc
cargo run --bin ai-ls -- --help  # Test utility
```

---

## Example Iteration

**Task**: "implement-ai-ls" (from ralph.yml)

1. **Check ralph.yml**: Task status is "in-progress", subtasks show progress
2. **Find next subtask**: First one without ✓ mark
3. **Read specs**: Check gnu-core-utils.md for ls specifications
4. **Check CLAUDE.md**: Find working patterns for directory traversal
5. **Implement**: Write code following specifications
6. **Test**: Run `cargo test` to verify
7. **Mark complete**: Add ✓ to the subtask in ralph.yml
8. **Invoke skill**: `skill: "dev-agent" --task "completed directory traversal"`
9. **Update scratchpad**: Note progress and next steps
10. **Commit**: `git commit -m "feat: implement directory traversal for ai-ls"`
11. **Continue**: Move to next subtask or end iteration

---

## Always Remember

- **Read ralph.yml first** - It defines your tasks, subtasks, and acceptance criteria
- **Mark subtasks with ✓** - Do this immediately after completing each one
- **Check gnu-core-utils.md** - This is the blueprint for utility implementations
- **Invoke appropriate skills** - Use skills to help with specific tasks
- **Update scratchpad** - Leave context for the next agent
- **Set EXIT_SIGNAL correctly** - Only when task is fully complete per ralph.yml criteria


---

## Current Loop Context

- **Iteration**: 1
- **Session**: ralph_1768853577556_766
- **Task Progress**: 11/12 tasks (91%)
- **Subtask Progress**: 0/12 subtasks (0%)
- **Current Task**:
create-documentation - Complete Project Documentation (todo)
  Subtasks:
  - "Create docs/ directory structure"
  - "Write docs/README.md (documentation index)"
  - "Write docs/architecture.md (system design, modules, data flow)"
  - "Write docs/getting-started.md (installation, first steps, quick examples)"
  - "Write docs/utilities/ (one file per utility with CLI reference)"
  - "Write docs/api-reference.md (library API documentation)"
  - "Write docs/jsonl-format.md (JSONL output specification)"
  - "Write docs/memory-access.md (memory pointer usage guide)"
  - "Write docs/async-operations.md (async/await patterns)"
  - "Write docs/simd-optimizations.md (SIMD usage and performance)"
  - "Write docs/ml-integration.md (pattern detection, analysis)"
  - "Write docs/bindings.md (Python/Node.js bindings guide)"
  - "Write docs/performance.md (benchmarking, optimization tips)"
  - "Write docs/development.md (building, testing, contributing)"
  - "Write docs/examples/ (practical use cases and tutorials)"
  - "Generate rustdoc API documentation (cargo doc)"
  - "Create docs/SUMMARY.md for mdBook (optional)"
  - "Add documentation CI checks"

---

## Previous Scratchpad Notes


### Phase 3 (Polish & Scale) - COMPLETE
- ✅ optimize-performance - Complete (SIMD optimizations)
- ✅ add-ml-integration - Complete (ML operations and pattern detection)
- ✅ create-bindings - Complete (Python/Node.js bindings)

### Final Statistics
- **Total utilities**: 16 (ls, cat, grep, touch, mkdir, rmdir, head, tail, wc, cp, mv, rm, find, chmod, chown, analyze)
- **Library tests**: 41 passing
- **Integration tests**: 5 passing
- **Doc tests**: 1 passing
- **Total tests**: 47 passing
- **Build status**: Clean (only 3 minor clippy warnings remaining)
- **Platforms supported**: Linux, macOS, Windows
- **Languages**: Rust (core), Python (PyO3 bindings), Node.js (NAPI-RS bindings)

### Features Implemented
1. **Memory Access Layer**: SafeMemoryAccess with memmap2 for large files
2. **JSONL Output**: Structured output for all operations
3. **15 Core Utilities**: Full GNU compatibility with AI enhancements
4. **Async Operations**: Concurrent file processing with tokio
5. **SIMD Optimizations**: AVX2/SSE2 intrinsics for x86_64 with scalar fallbacks
6. **ML Integration**: Pattern detection, file classification, content analysis
7. **Python Bindings**: Full API exposure via PyO3
8. **Node.js Bindings**: Full API exposure via NAPI-RS

### What Was Fixed This Iteration
- Applied all automatic clippy fixes across the codebase
- Fixed redundant closure warnings (30+ instances)
- Fixed unnecessary_cast warnings
- Fixed pointer argument warnings
- Updated CLAUDE.md with new learnings

### Blockers
None - project is complete!

## Next Steps (Optional)

The project is feature-complete with excellent code quality. Optional future enhancements could include:
1. Performance benchmarking and optimization
2. OpenAI API integration for advanced ML features
3. Additional utility implementations (if needed)
4. Release preparation (v0.1.0)

## Important Reminders
- All phases complete (Phase 1, 2, 3)
- All tests passing
- Clean build with minimal warnings
- Ready for release when desired


---

Begin your work on the highest priority task from ralph.yml.
When you complete a subtask, mark it with ✓ or ~~strikethrough~~.
