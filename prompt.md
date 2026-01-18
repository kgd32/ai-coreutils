# AI-Coreutils Development Agent Instructions

## Role & Context
You are developing AI-Coreutils, a modern implementation of GNU core utilities specifically designed for AI agents. This project uses Rust for performance and memory safety, provides structured JSONL output, and enables direct memory pointer access.

---

## ⚠️ MANDATORY: Auto-Documentation Policy

**CRITICAL**: ALL agents and developers MUST follow the auto-documentation policy for AI-Coreutils.

### Auto-Documentation is ALWAYS Enabled

The **auto-doc** skill is **automatically invoked** after EVERY work session:

- ✅ After completing ANY task (regardless of size)
- ✅ After making ANY code changes
- ✅ After running tests
- ✅ After fixing bugs
- ✅ After refactoring code
- ✅ Before committing changes

### Configuration

Auto-documentation is configured in `.claude/doc-config.yml`:

```yaml
auto_document:
  enabled: true  # NEVER set to false
  triggers:
    - after_task
    - after_commit
    - before_commit
    - on_error
```

### What Gets Auto-Documented

1. **CLAUDE.md** - Working patterns, learnings, Rust-specific knowledge
2. **ralph.yml** - Task status updates (todo → in-progress → done)
3. **.agent/scratchpad.md** - Context for next agent
4. **.agent/sessions/** - Session log with:
   - Executive summary
   - What was done
   - Learnings
   - Next steps

### NO MANUAL INVOCATION NEEDED

**You do NOT need to manually invoke documentation.** It happens automatically.
Just complete your work, and the documentation will be generated and committed.

### Verification

After any work session, verify:
- ✅ CLAUDE.md updated with learnings
- ✅ ralph.yml task status updated
- ✅ Session log created in `.agent/sessions/`
- ✅ Documentation committed with "docs:" prefix

---

## Your Workflow

### 1. Orient Yourself (Every Iteration)
- Read CLAUDE.md FIRST to understand what works and what doesn't
- **Read gnu-core-utils.md for utility specifications** ⚠️ THIS IS OUR BLUEPRINT
- Check project state and current phase in ralph.yml
- Identify current task from ralph.yml
- Run `cargo check` to ensure project compiles
- Review recent commits with `git log --oneline -5`

### 2. Work on the Task
- **Read gnu-core-utils.md for the utility you're implementing** ⚠️ MANDATORY
- Update status in ralph.yml from "todo" to "in-progress"
- Check CLAUDE.md for relevant patterns and working solutions
- Implement according to gnu-core-utils.md specifications
- Add AI enhancements (JSONL output, memory access, etc.)
- Implement with tests (use `cargo test` to verify)
- Document findings in scratchpad.md
- Update CLAUDE.md with new learnings
- Update ralph.yml status to "done" when complete

### 3. Commit Your Changes
```bash
git add .
git commit -m "feat: [brief description of change]"
```

### 4. Leave Context for Next Agent
Update .agent/scratchpad.md with:
- What was accomplished
- What was tried but didn't work
- What should be tackled next
- Any blockers encountered

### 5. Check Completion Status
- All tasks in current phase marked "done"?
- Tests passing?
- Benchmarks meet performance targets?
- Ready for next phase?

## Critical Guidelines

### Code Quality Standards
- Use `cargo fmt` for formatting
- Use `cargo clippy` for linting
- All public functions must have documentation comments
- Error handling must use `Result<T, E>` pattern
- Memory safety is paramount - no unsafe code without justification

### AI-Coreutils Specific Requirements
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

## Don't Do This
- Don't use unsafe code without extensive documentation
- Don't ignore error returns
- Don't break compatibility with GNU coreutils basics
- Don't implement memory operations without proper bounds checking
- Don't use blocking I/O in async contexts

## Do This
- Always benchmark new implementations
- Use memory mapping for files > 1MB
- Provide structured error messages in JSONL format
- Write comprehensive tests for edge cases
- Document all memory access patterns

## When You're Stuck
1. **Check gnu-core-utils.md for utility specifications** ⚠️ PRIMARY REFERENCE
2. Check CLAUDE.md for similar solved problems
3. Look at GNU coreutils source for reference
4. Consider if the operation can be memory-mapped
5. Update scratchpad.md with the blocker
6. Mark task as "blocked" in ralph.yml with reason

## Success Criteria
- Code compiles without warnings
- All tests pass
- Performance benchmarks meet targets
- JSONL output is valid and complete
- Memory operations are safe and efficient

## Example Iteration
Task: Implement ai-ls with JSONL output
1. Check ralph.yml: task "implement-ai-ls" is todo
2. Update to in-progress
3. **Read gnu-core-utils.md section on `ls` for specifications** ⚠️ BLUEPRINT
4. Check CLAUDE.md for directory traversal patterns
5. Implement according to gnu-core-utils.md specs + AI enhancements
6. Add JSONL output formatting
7. Write tests
8. Run `cargo test`
9. Update CLAUDE.md with learnings
10. Mark task as done
11. Commit changes

## Quick Reference Commands
```bash
cargo build                    # Build project
cargo test                     # Run tests
cargo bench                    # Run benchmarks
cargo fmt                      # Format code
cargo clippy                   # Lint code
cargo run --bin ai-ls -- --help  # Test utility
```

## Completion Markers
LOOP_COMPLETE_PHASE_1    # MVP complete
LOOP_COMPLETE_PHASE_2    # Enhanced features complete
LOOP_COMPLETE_PHASE_3    # Project complete
LOOP_BLOCKED: [reason]   # Task blocked
