# Documentation Agent - Main Entry Point

> **Purpose**: Primary skill for managing Agent Village documentation with two operational modes
> **Expertise**: XState, TypeScript, Node.js, Agent Village architecture

---

## Core Principle: Document ALL Work

**CRITICAL**: This agent should be spawned after **ANY** work completion:

- **Every phase** (Phases 1-14 from roadmap)
- **Every task** (regardless of size or complexity)
- **Every feature** (no matter how small)
- **Every bug fix** (even trivial fixes)
- **Every test** (unit tests, E2E, verification)
- **Every refactor** (code cleanup, reorganization)
- **Every config change** (any configuration modification)

> **Rule**: If you changed code/files, spawn the doc-agent.

---

## Overview

The Documentation Agent provides two modes for keeping Agent Village documentation current:

1. **Sweep Mode**: Comprehensive documentation audit and update
2. **Targeted Mode**: Document specific feature/change/topic

Both modes include automatic improvements tracking and git commit/push.

---

## Invocation

### Mode 1: Comprehensive Sweep (Default)
```
skill: "doc-agent"
skill: "doc-agent" --mode "sweep"
```

Performs a full documentation audit and update across all files.

### Mode 2: Targeted Documentation
```
skill: "doc-agent" --mode "targeted" --target "<topic>"
```

Documents a specific feature, change, or topic.

### Examples

```bash
# Full documentation sweep
skill: "doc-agent"

# Document phase 7 completion
skill: "doc-agent" --mode "targeted" --target "phase 7 completion"

# Document bug fix
skill: "doc-agent" --mode "targeted" --target "crash recovery testing"

# Update architecture docs
skill: "doc-agent" --mode "targeted" --target "new planner machine pattern"
```

---

## Parameters

| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| `mode` | string | No | "sweep" | "sweep" or "targeted" |
| `target` | string | No* | - | Topic to document (required for targeted mode) |

*Required when `mode="targeted"`

---

## Behavior

### Mode 1: Sweep (Comprehensive)

```
skill: "doc-agent"
```

**Execution Flow**:

1. **Calls `doc-sweep` skill**
   - Scans all documentation files
   - Checks against code state (read-only)
   - Identifies outdated content
   - Updates status badges, dates, cross-references
   - Sorts session logs chronologically
   - Generates summary of changes

2. **Calls `doc-fixes` skill**
   - Categorizes any remaining issues
   - Updates `docs/IMPROVEMENTS.md`
   - Tracks critical/medium/low priority items

3. **Auto-commits and pushes**
   - Commits all documentation changes
   - Pushes to current branch
   - Commit message: `docs: [summary]`

### Mode 2: Targeted (Specific Topic)

```
skill: "doc-agent" --mode "targeted" --target "<topic>"
```

**Execution Flow**:

1. **Analyzes target**
   - Determines type (phase/feature/bug/testing)
   - Identifies affected documentation
   - Maps to relevant code files

2. **Calls `doc-update` skill**
   - Reads relevant code (read-only)
   - Updates affected documentation
   - Creates session log entry
   - Updates cross-references

3. **Calls `doc-fixes` skill**
   - Checks for related issues
   - Updates `docs/IMPROVEMENTS.md`
   - Categorizes any new issues found

4. **Auto-commits and pushes**
   - Commits all documentation changes
   - Pushes to current branch
   - Commit message: `docs: [target] - [summary]`

---

## Mode Selection Guide

### Use Sweep Mode When:

- After significant code changes (multiple files)
- Before releases or tags
- Periodic maintenance (weekly/monthly)
- After completing a development phase
- Documentation has fallen behind

### Use Targeted Mode When:

- Completing a specific phase
- Adding a single feature
- Fixing a specific bug
- Running verification tests
- Making architectural changes

---

## Output

### Console Output (Sweep Mode)
```
Documentation Agent: Sweep Mode
================================
Scanning documentation files...
Found 27 documentation files
Checking against code state...
Identified 15 issues
Updating documentation...
- Fixed status badges: 3
- Updated dates: 5
- Fixed cross-references: 4
- Sorted session logs: 2
- Created session log: sessions/2026-01-18_doc-sweep.md

Checking for improvements...
🔴 Critical: 2
🟡 Medium: 6
🟢 Low: 3
Updated: docs/IMPROVEMENTS.md

Committing and pushing...
[commit hash] docs: comprehensive sweep - status updates, dates, links
```

### Console Output (Targeted Mode)
```
Documentation Agent: Targeted Mode
==================================
Target: "phase 7 completion"
Analyzing target...
Reading relevant code...
Identifying affected docs...
Updating documentation...
- Updated: XSTATE-MIGRATION-PLAN.md (Phase 7 status)
- Created: PHASE-7-COMPLETE.md
- Updated: CLAUDE.md (current phase)
- Created: sessions/2026-01-18_phase-7-completion.md

Checking for improvements...
🔴 Critical: 0 new
🟡 Medium: 1 new (feature usage examples)
🟢 Low: 2 new (typos)
Updated: docs/IMPROVEMENTS.md

Committing and pushing...
[commit hash] docs: phase 7 completion - verification and documentation
```

---

## Expertise Context

The Documentation Agent has senior-level expertise in:

### XState v5
- State machine patterns and best practices
- Actor lifecycle management
- `matches()` vs `.value` comparison
- `waitFor` snapshot handling

### TypeScript
- Type system and patterns
- Import/export conventions
- File extension standards (`.ts` source, `.js` imports)

### Agent Village Architecture
- Orchestration layer (XState) separation
- A2A bridge pattern
- Task lifecycle management
- Mission control coordination

### Documentation Standards
- Status badge conventions (✅🚧🔴⚠️📖🔄)
- Cross-reference formatting
- Session log naming (`YYYY-MM-DD_HH-MM_topic.md`)
- Executive summary structure

---

## File Structure

```
.claude/
├── skills/
│   ├── doc-agent.md          # ← This file (main entry point)
│   ├── doc-sweep.md          # Comprehensive sweep logic
│   ├── doc-update.md         # Targeted update logic
│   ├── doc-fixes.md          # Improvements tracking
│   └── doc-expertise.md      # Shared knowledge base
└── doc-config.yml            # Configuration
```

---

## Configuration

The agent reads configuration from `.claude/doc-config.yml`:

```yaml
# Model Configuration
model:
  default: "claude-sonnet-4-5-20250514"  # Cost-effective for documentation
  fallback: "claude-3-5-sonnet-20241022"
  fast: "claude-3-5-haiku-20241022"

# Auto-sweep settings
auto_sweep:
  enabled: true
  triggers:
    - git_commit  # Run after commits
    - daily       # Run once per day

# Git settings
git:
  auto_commit: true
  auto_push: true
  commit_prefix: "docs:"
  branch: "xstate-migration-gemini"

# Improvements tracking
improvements:
  file: "docs/IMPROVEMENTS.md"
  auto_categorize: true
```

### Model Selection

The doc-agent uses **Claude Sonnet 4.5** by default for cost-effective documentation:

- **Default (Sonnet 4.5)**: Most documentation tasks
- **Fast (Haiku)**: Simple scans, duplicate detection
- **Fallback (Sonnet 3.5)**: If primary unavailable

Models are automatically selected based on task complexity.

---

## Git Workflow

All modes automatically commit and push:

```bash
# Stage documentation changes
git add docs/ sessions/ .claude/

# Commit with standard prefix
git commit -m "docs: [summary of changes]"

# Push to current branch
git push
```

### Branch Context
- **Current Branch**: `xstate-migration-gemini`
- **Main Branch**: `main`
- Always pushes to feature branch, never directly to main

---

## Read-Only Code Access

The Documentation Agent:
- ✅ **CAN**: Read any code file for understanding
- ✅ **CAN**: Write/edit documentation files
- ✅ **CAN**: Commit and push via git
- ❌ **CANNOT**: Write, edit, modify code files

### Allowed Write Locations
- `docs/*.md`
- `sessions/*.md`
- `README.md`
- `CLAUDE.md`
- `.claude/*`

---

## Related Skills

| Skill | Purpose |
|-------|---------|
| `doc-sweep` | Comprehensive documentation audit |
| `doc-update` | Targeted feature/topic documentation |
| `doc-fixes` | Track and categorize improvements |
| `doc-expertise` | Shared knowledge base |

---

## Common Usage Patterns

### After Completing Phase 7
```bash
skill: "doc-agent" --mode "targeted" --target "phase 7 completion"
```

### Weekly Documentation Maintenance
```bash
skill: "doc-agent"  # or skill: "doc-sweep"
```

### After Bug Fix
```bash
skill: "doc-agent" --mode "targeted" --target "bug fix: actor lifecycle"
```

### Before Release
```bash
skill: "doc-agent"  # Full sweep to ensure docs are current
```

### After Architecture Changes
```bash
skill: "doc-agent" --mode "targeted" --target "new planner machine pattern"
```

---

## Verification

After running, verify:

1. ✅ Documentation files updated correctly
2. ✅ Session log created with proper format
3. ✅ `docs/IMPROVEMENTS.md` updated
4. ✅ Git commit created with proper message
5. ✅ Changes pushed to remote
6. ✅ No code files modified

---

**Last Updated**: 2026-01-18
**Maintained By**: Documentation Agent System
