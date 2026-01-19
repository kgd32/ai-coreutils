---
name: phase-agent
description: Track and manage development phases for AI-Coreutils. Check phase status, start new phases, complete phases, or block phases on issues. Use to coordinate progress across MVP (Phase 1), Enhanced Features (Phase 2), and Polish & Scale (Phase 3).
---

# Phase Agent

> **Purpose**: Track and manage AI-Coreutils development phases
> **Expertise**: Project management, task coordination

---

## Core Principle: Phased Development

AI-Coreutils is developed in phases:
- **Phase 1 (MVP)**: Core utilities with basic functionality
- **Phase 2 (Enhanced)**: Additional utilities and async support
- **Phase 3 (Polish)**: Performance and ML integration

---

## Invocation

```
skill: "phase-agent" --action "<action>" --phase "<N>"
```

### Examples

```bash
# Check current phase status
skill: "phase-agent" --action "status"

# Start next phase
skill: "phase-agent" --action "start" --phase "2"

# Complete phase
skill: "phase-agent" --action "complete" --phase "1"

# Block phase on issue
skill: "phase-agent" --action "block" --reason "memory mapping issue"
```

---

## Behavior

### Status Action
- Read ralph.yml for current phase
- List all tasks in current phase
- Show completion status

### Start Action
- Verify previous phase complete
- Update CLAUDE.md with new phase
- Create session log for phase start

### Complete Action
- Verify all phase tasks done
- Run all tests
- Update documentation
- Commit phase completion

### Block Action
- Document blocker in ralph.yml
- Update scratchpad.md
- Create session log with issue

---

## Phase Checklists

### Phase 1 (MVP)
- [ ] Project setup
- [ ] Memory access layer
- [ ] JSONL output formatter
- [ ] ai-ls utility
- [ ] ai-cat utility
- [ ] ai-grep utility

### Phase 2 (Enhanced)
- [ ] Remaining utilities (find, mv, cp, rm, mkdir, touch, etc.)
- [ ] Async support
- [ ] Performance benchmarks

### Phase 3 (Polish)
- [ ] SIMD optimizations
- [ ] ML integration
- [ ] Cross-language bindings

---

## Output Format

```
Phase Agent: [action]
=======================
Current Phase: N
Phase Status: [In Progress/Complete/Blocked]

Tasks in Phase:
- [ ] task 1 (todo)
- [x] task 2 (done)
- [ ] task 3 (in-progress)

Completion: X%
```

---

## Blocker Format

```
BLOCKED: [reason]
==================
Phase: N
Blocked: YYYY-MM-DD
Reason: [description]

Resolution:
- [ ] Step 1
- [ ] Step 2
```

---

**Last Updated**: 2026-01-19
