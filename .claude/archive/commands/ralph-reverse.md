---
description: Reverse engineer a project from its codebase. Traverses directories and processes 1-5 files per iteration to build a complete specification document.
arguments:
  - name: files_per_iteration
    description: Number of files to process per iteration (default: 3, range: 1-5)
    required: false
  - name: output_file
    description: Where to save the specs (default: .ralph/reverse-specs.md)
    required: false
  - name: exclude_patterns
    description: Glob patterns to exclude (default: node_modules/**, .git/**, target/**, **/*.lock)
    required: false
---

# Ralph Loop - Reverse Engineer Project

<function_calls>
<invoke name="Task">
<parameter name="subagent_type">general-purpose</parameter>
<parameter name="prompt">You are a Ralph Loop Reverse Engineer. Your job is to analyze an existing codebase and generate a complete specification document.

## CRITICAL: Fresh Context Architecture

Each iteration you start FRESH and must hydrate from the `.ralph/` folder to continue where you left off.

## Your Process

### PHASE 1: HYDRATION (Read State)

Read these files in order:
1. `.ralph/reverse-state.json` - Progress tracking
2. `.ralph/reverse-specs.md` - Accumulated specifications so far
3. `.ralph/reverse-scan.md` - Scan results and file queue

If files don't exist, this is the first iteration - go to PHASE 2: INITIAL SCAN.

### PHASE 2: INITIAL SCAN (First Iteration Only)

If this is the first iteration (no state files):

1. **Scan the project structure:**
   - Use Glob to find all source files
   - Exclude patterns: `$ARGUMENTS.exclude_patterns` or `node_modules/**, .git/**, target/**, build/**, dist/**, **/*.lock, **/node_modules/**`
   - Detect tech stack from files (package.json, Cargo.toml, etc.)

2. **Categorize files by type:**
   - Source code (.ts, .js, .rs, .py, .go, etc.)
   - Configuration (package.json, tsconfig.json, Cargo.toml, etc.)
   - Tests (*.test.*, *.spec.*, __tests__/, tests/)
   - Documentation (README.md, docs/, *.md)
   - Assets (images, styles, etc.)

3. **Create file processing queue:**
   - Prioritize: config → entry points → main source → tests → docs
   - Save queue to `.ralph/reverse-scan.md`

4. **Initialize state:**
   ```json
   {
     "iteration": 1,
     "files_processed": 0,
     "files_remaining": N,
     "current_batch": [],
     "tech_stack": {},
     "structure": {},
     "started_at": "timestamp"
   }
   ```

### PHASE 3: PROCESS FILES (Each Iteration)

1. **Read next batch of files:**
   - Process `$ARGUMENTS.files_per_iteration` files (default: 3, max: 5)
   - Read from queue in `.ralph/reverse-scan.md`
   - Update `.ralph/reverse-state.json` with current batch

2. **Analyze each file:**
   - Purpose and responsibility
   - Key functions/classes and their roles
   - Dependencies (imports, requires)
   - Exports/API surface
   - Patterns used (design patterns, architectural patterns)

3. **Update specifications:**
   - Append findings to `.ralph/reverse-specs.md`
   - Organize by module/component
   - Cross-reference dependencies

### PHASE 4: DEHYDRATION (Save State)

Before exiting, update ALL files:

1. **`.ralph/reverse-state.json`:**
   ```json
   {
     "iteration": N,
     "files_processed": M,
     "files_remaining": X,
     "current_batch": ["file1.ts", "file2.rs", ...],
     "tech_stack": {
       "language": "Rust",
       "framework": "Actix-web",
       "detected_from": ["Cargo.toml", "src/main.rs"]
     },
     "structure": {
       "directories": ["src/", tests/", ...],
       "entry_points": ["src/main.rs"],
       "modules": ["config", "database", "routes", ...]
     },
     "started_at": "timestamp",
     "last_updated": "timestamp"
   }
   ```

2. **`.ralph/reverse-specs.md`:**
   - Accumulate all findings
   - Format as comprehensive specification

3. **`.ralph/reverse-scan.md`:**
   - Update file queue (remove processed files)

4. **`.ralph/reverse-history/iteration-XXX.md`:**
   - Log what was processed this iteration

### PHASE 5: EXIT DECISION

**If all files processed:**
- Output `LOOP_REVERSE_COMPLETE`
- Finalize specification document
- Generate summary

**If files remain:**
- Output `LOOP_CONTINUE`
- Exit (next iteration will continue)

## Output Specification Format

The `.ralph/reverse-specs.md` should follow this structure:

```markdown
# Project Specification - Reverse Engineered

**Generated:** 2026-01-19
**Project:** [Project Name]
**Tech Stack:** [Detected stack]

## Executive Summary
[Brief overview of what this project does]

## Architecture Overview
[High-level architecture, patterns used]

## Tech Stack Details
- **Language:** [Language]
- **Framework:** [Framework]
- **Build System:** [Build tools]
- **Testing:** [Test frameworks]
- **Dependencies:** [Key dependencies]

## Project Structure
```
project-root/
├── src/
│   ├── main.ts          # Entry point
│   ├── config/          # Configuration
│   └── services/        # Business logic
└── tests/
```

## Components / Modules

### [Module Name]
**Location:** `src/module/file.ts`
**Purpose:** [What it does]
**Key Functions:**
- `functionName()` - [Purpose]
**Dependencies:**
- Imports from `other-module`
**Exports:**
- `exportedFunction` - [API surface]

## Data Flow
[How data flows through the system]

## API Surface (if applicable)
[Endpoints, functions, or public APIs]

## Configuration
[How configuration works]

## Testing Strategy
[How tests are organized]

## Build & Deployment
[How to build and deploy]

## Dependencies
| Package | Version | Purpose |
|---------|---------|---------|

## Known Patterns
[Design patterns, architectural patterns detected]

## Potential Issues / Technical Debt
[Code smells, anti-patterns, areas for improvement]
```

## File Exclusion Defaults

Default exclude patterns:
- `node_modules/**`
- `.git/**`
- `target/**`
- `build/**`
- `dist/**`
- `**/*.lock`
- `**/package-lock.json`
- `**/yarn.lock`
- `**/Cargo.lock`
- `.vscode/**`
- `.idea/**`

## Tech Stack Detection

Auto-detect from these files:
- `package.json` → Node.js/TypeScript
- `Cargo.toml` → Rust
- `pyproject.toml` / `requirements.txt` → Python
- `go.mod` → Go
- `pom.xml` → Java/Maven
- `build.gradle` → Java/Gradle

## BEGIN

1. Check if `.ralph/reverse-state.json` exists
2. If no → INITIAL SCAN phase
3. If yes → PROCESS FILES phase
4. Dehydrate state before exit
5. Output exit marker</parameter>
<parameter name="description">Reverse engineer project from codebase</parameter>
<parameter name="model">sonnet</parameter>
</invoke>
</function_calls>

---

**Reverse engineering started.** Check `.ralph/reverse-specs.md` for accumulated specifications.
