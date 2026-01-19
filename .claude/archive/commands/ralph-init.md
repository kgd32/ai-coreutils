---
description: Initialize a Ralph Loop project by generating all required .ralph/ files from your project idea. Pass the idea as a string or reference an existing markdown file.
arguments:
  - name: idea
    description: Your project idea as a string, or path to a markdown file containing your project specifications (e.g., "a CLI tool for log analysis" or "./my-specs.md")
    required: true
  - name: tech_stack
    description: Optional tech stack hint (e.g., "rust", "typescript", "python", "go"). Auto-detected from project files if not provided.
    required: false
  - name: project_type
    description: Optional project type (cli, web, library, api, desktop). Helps with template selection.
    required: false
---

# Ralph Loop - Project Initialization

Initializing Ralph Loop project structure...

<function_calls>
<invoke name="Task">
<parameter name="subagent_type">general-purpose</parameter>
<parameter name="prompt">You are initializing a Ralph Loop project. Your job is to generate all required files from the user's project idea.

## CRITICAL: Read the Template First

Before generating anything, you MUST read:
1. `ralph_loop_template.md` - The master template for generating Ralph projects

## Your Task

Generate ALL required files for a Ralph Loop project based on the user's project idea.

### Input

**Project Idea:** $ARGUMENTS.idea

This can be:
- A string describing the project (e.g., "a CLI tool for log analysis using Python")
- A path to an existing markdown file (e.g., "./my-specs.md")

**Tech Stack (optional):** $ARGUMENTS.tech_stack
**Project Type (optional):** $ARGUMENTS.project_type

### What to Generate

You MUST create the following structure:

```
your-project/
├── CLAUDE.md              # AT ROOT - Knowledge base
└── .ralph/                # All Ralph state management
    ├── prompt.md          # Agent instructions
    ├── ralph.yml          # Task configuration
    ├── scratchpad.md      # Handoff notes
    ├── session.md         # Session state
    ├── state.json         # Structured state
    ├── history/           # Directory for iteration logs
    └── checkpoints/       # Directory for task snapshots
```

### Generation Steps

1. **Parse the input:**
   - If `$ARGUMENTS.idea` is a file path that exists, read it
   - Otherwise, treat it as a project description string

2. **Auto-detect tech stack** (if not provided):
   - Check for existing project files (package.json, Cargo.toml, etc.)
   - Set appropriate defaults based on what's found

3. **Generate each file:**

   **CLAUDE.md** (at project root):

   **IMPORTANT:** Handle existing CLAUDE.md correctly:
   - Check if `CLAUDE.md` already exists in the project root
   - If it EXISTS: **APPEND** Ralph Loop instructions to the end (preserve user's existing content)
   - If it DOESN'T exist: Create new file with full template

   **When appending to existing CLAUDE.md:**
   ```markdown
   ---
   ## Ralph Loop Integration

   This project uses Ralph Loop for autonomous development.

   ### Ralph Loop Workflow

   The Ralph Loop agent follows this process each iteration:

   1. **Orient Yourself:**
      - Read this CLAUDE.md for project knowledge
      - Check .ralph/session.md for iteration/phase
      - Check .ralph/ralph.yml for current tasks

   2. **Work on Task:**
      - Select next "todo" task from .ralph/ralph.yml
      - Update status to "in-progress"
      - Follow .ralph/prompt.md workflow
      - Update this CLAUDE.md with learnings

   3. **Dehydrate State:**
      - Update .ralph/scratchpad.md with handoff notes
      - Update .ralph/session.md (increment iteration)
      - Mark task as "done" in .ralph/ralph.yml

   ### Completion Markers

   Ralph Loop uses these markers:
   - `LOOP_COMPLETE_PHASE_1` - MVP complete
   - `LOOP_COMPLETE_PHASE_2` - Phase 2 complete
   - `LOOP_COMPLETE_PHASE_3` - Phase 3 complete
   - `LOOP_COMPLETE` - Project fully complete
   - `LOOP_BLOCKED: [reason]` - Task blocked

   ### Exit Detection

   Ralph Loop uses dual-condition exit detection:
   - Condition 1: `completion_indicators >= 2` (heuristic from output)
   - Condition 2: `EXIT_SIGNAL: true` (explicit confirmation)
   - BOTH must be true to exit

   ---
   ```

   **When creating new CLAUDE.md:**
   - Purpose and how to use
   - Tech stack realities section (what works/doesn't)
   - Template for working patterns
   - Template for failed approaches
   - Agent-to-agent messages section
   - Current state assessment
   - Ralph Loop Integration section (as shown above)

   **.ralph/prompt.md:**
   - Role & Context (project description)
   - Workflow sections:
     - Orient Yourself (Every Iteration)
     - Work on the Task
     - Commit Your Changes
     - Dehydrate State Before Exiting
     - Check Completion Status
   - Critical Guidelines:
     - Code Quality Standards
     - Project-Specific Requirements
     - Error Handling Pattern
   - Don't Do This / Do This sections
   - Success Criteria
   - Completion Markers

   **.ralph/ralph.yml:**
   - Project metadata (name, description, version)
   - Tasks broken down by phases:
     - PHASE 1: MVP (critical priority tasks)
     - PHASE 2: Enhanced Features (high priority)
     - PHASE 3: Polish & Scale (medium/low priority)
   - Each task needs:
     - id, title, priority, status (all start as "todo")
     - dependencies (array of task IDs)
     - acceptance_criteria (array)
     - subtasks (array)
     - estimated_hours (number)
   - Workflows section
   - Dependencies section
   - Testing section

   **.ralph/scratchpad.md:**
   - First Run section (no previous context)
   - Next Iteration Should section
   - Blockers section

   **.ralph/session.md:**
   - iteration: 0
   - phase: mvp
   - started_at: null
   - last_update: null
   - Current Status section

   **.ralph/state.json:**
   ```json
   {
     "iteration": 0,
     "phase": "mvp",
     "current_task_id": null,
     "current_task_status": null,
     "tasks": { "total": N, "done": 0, "in_progress": 0, "todo": N },
     "phases": {
       "current": "mvp",
       "mvp_complete": false,
       "phase_2_complete": false,
       "phase_3_complete": false
     },
     "tech_stack": { "language": "...", "detected_from": "..." },
     "last_updated": null,
     "exit_signal": false,
     "completion_indicators": 0,
     "last_iteration_status": null,
     "blocked": false,
     "blocker_reason": null,
     "circuit_breaker": {
       "state": "closed",
       "no_progress_count": 0,
       "same_error_count": 0,
       "last_error": null,
       "last_output_size": 0,
       "opened_at": null,
       "reason": null
     },
     "exit_conditions": {
       "consecutive_test_loops": 0,
       "consecutive_done_signals": 0,
       "total_iterations": 0,
       "test_only_iterations": 0,
       "test_percentage": 0
     }
   }
   ```

4. **Create directories:**
   - `.ralph/history/` (empty, for future iteration logs)
   - `.ralph/checkpoints/` (empty, for future task snapshots)

### Tech Stack Defaults

Use these defaults when auto-detecting:

| Detected From | Language | Build | Test | Lint |
|---------------|----------|-------|------|------|
| package.json | TypeScript/Node.js | `npm run build` | `npm test` | `eslint` |
| Cargo.toml | Rust | `cargo build` | `cargo test` | `cargo clippy` |
| pyproject.toml | Python | `python -m build` | `pytest` | `ruff check` |
| go.mod | Go | `go build` | `go test` | `golangci-lint` |

### Task Breakdown Guidelines

When generating ralph.yml tasks:

**PHASE 1 (MVP) - Critical:**
- Project setup & structure
- Core functionality
- Basic testing
- Initial documentation

**PHASE 2 (Enhanced) - High:**
- Additional features
- Error handling
- Performance optimization
- Enhanced testing

**PHASE 3 (Polish) - Medium/Low:**
- Advanced features
- Documentation polish
- Deployment setup
- Release preparation

### Output Format

After generating all files, output a summary:

```
✅ Ralph Loop project initialized!

Generated files:
- CLAUDE.md (appended if existing, created if new)
- .ralph/prompt.md
- .ralph/ralph.yml
- .ralph/scratchpad.md
- .ralph/session.md
- .ralph/state.json
- .ralph/history/ (directory)
- .ralph/checkpoints/ (directory)

Note: If CLAUDE.md already existed, Ralph Loop instructions
      were appended to preserve your existing content.

Next steps:
1. Review the generated files
2. Adjust .ralph/ralph.yml tasks if needed
3. Run /ralph to start autonomous development
```

## BEGIN GENERATION

1. Read ralph_loop_template.md for guidance
2. Parse the project idea from $ARGUMENTS.idea
3. Detect or use provided tech stack
4. Generate all required files
5. Create .ralph/ directories
6. Output summary</parameter>
<parameter name="description">Initialize Ralph Loop project</parameter>
<parameter name="model">sonnet</parameter>
</invoke>
</function_calls>

---

**Project structure initialized.** Review the generated files and run `/ralph` to start autonomous development.
