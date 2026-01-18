# Documentation Agent

> **Purpose**: Maintain AI-Coreutils documentation after all work
> **Expertise**: Rust, core utilities, documentation standards

---

## Core Principle: Document ALL Work

**CRITICAL**: Spawn after ANY work completion:
- Every task (regardless of size)
- Every feature (no matter how small)
- Every bug fix (even trivial ones)
- Every test run
- Every refactor

> **Rule**: If files were changed, documentation must be updated.

---

## Invocation

### Mode 1: Comprehensive Sweep
```
skill: "doc-agent"
```

Performs full documentation audit and update.

### Mode 2: Targeted Update
```
skill: "doc-agent" --target "<topic>"
```

Documents specific feature/change.

### Examples

```bash
# Full sweep
skill: "doc-agent"

# Document task completion
skill: "doc-agent" --target "ai-ls implementation complete"

# Document bug fix
skill: "doc-agent" --target "memory mapping fix"

# Phase completion
skill: "doc-agent" --target "phase 1 complete"
```

---

## Behavior

### Sweep Mode
1. Scan all documentation files
2. Check against code state (read-only)
3. Update status badges and dates
4. Fix broken cross-references
5. Update ralph.yml task statuses
6. Commit and push

### Targeted Mode
1. Analyze target topic
2. Read relevant code (read-only)
3. Update affected documentation
4. Create session log entry
5. Update CLAUDE.md with learnings
6. Commit and push

---

## Documentation Files

| File | Purpose | Can Modify |
|------|---------|------------|
| `CLAUDE.md` | Agent knowledge base | ✅ Yes |
| `ralph.yml` | Task tracking | ✅ Yes |
| `.agent/scratchpad.md` | Context for next agent | ✅ Yes |
| `.agent/sessions/*.md` | Session logs | ✅ Yes |
| `src/**/*.rs` | Source code | ❌ Read only |

---

## Output Format

```
Documentation Agent: [mode]
================================
Target: [topic if targeted]

Files Updated:
- CLAUDE.md (working patterns)
- ralph.yml (task status)
- .agent/sessions/YYYY-MM-DD_topic.md

Session log created: .agent/sessions/...
Committed and pushed.
```

---

## Session Log Format

```markdown
# Session: [Topic Title]

**Date**: YYYY-MM-DD
**Branch**: [current branch]

## Executive Summary
- **Status**: [Completed/Failed/In Progress]
- **Outcome**: [Brief result]

## Details
### Changes Made
[What was done]

### Learnings
[What was learned - added to CLAUDE.md]

### Next Steps
[What should be done next - added to scratchpad.md]
```

---

## Git Workflow

```bash
# Stage documentation changes
git add CLAUDE.md ralph.yml .agent/

# Commit with standard prefix
git commit -m "docs: [summary]"

# Push
git push
```

---

## Verification

After running, verify:
- ✅ Documentation files updated
- ✅ Session log created
- ✅ CLAUDE.md updated with learnings
- ✅ ralph.yml updated with task status
- ✅ Git commit created
- ✅ No code files modified

---

**Last Updated**: 2026-01-19
