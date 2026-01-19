---
name: auto-doc
description: AUTOMATICALLY invoked after all work to maintain documentation. Updates CLAUDE.md with learnings, ralph.yml with task status, creates session logs, and commits documentation. Never needs manual invocation - runs automatically after tasks, commits, tests, and errors.
---

# Auto-Documentation Skill

> **Purpose**: AUTOMATICALLY invoked after all work to maintain documentation
> **Behavior**: Reads doc-config.yml and documents all changes
> **Status**: ALWAYS ENABLED for AI-Coreutils

---

## ⚠️ CRITICAL: AUTO-INVOKED SKILL ⚠️

**This skill is AUTOMATICALLY invoked after EVERY work session:**

- ✅ After completing ANY task
- ✅ After making ANY code changes
- ✅ After running tests
- ✅ After fixing bugs
- ✅ After refactoring
- ✅ Before committing

**No manual invocation needed - this happens automatically.**

---

## Configuration

Reads configuration from `.claude/doc-config.yml`:

```yaml
auto_document:
  enabled: true  # ALWAYS ON
  triggers:
    - after_task
    - after_commit
    - before_commit
    - on_error
```

---

## Auto-Documentation Behavior

### Step 1: Detect Changes
- Scan for modified files
- Identify what was done
- Determine task type

### Step 2: Generate Documentation
Update these files automatically:
1. **CLAUDE.md** - Add working patterns, learnings
2. **ralph.yml** - Update task status
3. **.agent/scratchpad.md** - Context for next agent
4. **.agent/sessions/** - Create session log

### Step 3: Quality Checks
- Validate cross-references
- Update stale dates
- Check documentation coverage
- Categorize any issues

### Step 4: Commit Documentation
```bash
git add CLAUDE.md ralph.yml .agent/
git commit -m "docs: [auto-generated] [summary]"
```

---

## What Gets Documented

### Code Changes
- New functions/structs/modules
- Modified implementations
- Added tests
- Bug fixes

### Learnings
- Working patterns discovered
- Rust-specific patterns
- Common pitfalls
- Performance optimizations

### Task Progress
- Task status updates
- Phase progress
- Blockers encountered
- Next steps

---

## Session Log Format

Auto-generated session logs include:

```markdown
# Session: [Topic]

**Date**: YYYY-MM-DD
**Duration**: [time spent]
**Task**: [from ralph.yml]

## Executive Summary
- **Status**: ✅ Complete | 🚧 In Progress | 🔴 Failed
- **Outcome**: [brief result]
- **Files Modified**: [count]

## What Was Done
- [Change 1]
- [Change 2]
- ...

## Learnings
### Working Patterns
[Pattern discovered]

### Rust-Specific
[Rust pattern notes]

## Next Steps
- [ ] [Next task]
- [ ] [Follow-up item]

## Files Changed
- `src/file.rs` - [description]
- `tests/test.rs` - [description]
```

---

## Integration Points

Automatically invoked by:
- **dev-agent** → After code changes
- **test-agent** → After test runs
- **phase-agent** → After phase updates
- **Manual work** → Before any commit

---

## Configuration Override

To disable auto-documentation (NOT RECOMMENDED):
```yaml
# In .claude/doc-config.yml
auto_document:
  enabled: false  # ⚠️ Only for testing
```

**Normal operation should ALWAYS have `enabled: true`**

---

## Manual Invocation

While auto-invoked, can also be run manually:

```bash
# Force documentation update
skill: "auto-doc"

# Document specific topic
skill: "auto-doc" --topic "custom topic"
```

---

## Output

```
Auto-Documentation: Triggered
================================
Changes detected: [files]
Documentation updated:
- CLAUDE.md (working patterns)
- ralph.yml (task status)
- .agent/sessions/2026-01- HH-MM_topic.md

Session log created: .agent/sessions/...
Committed: [hash]
```

---

## Quality Checks

Performs automatically:
- ✅ Cross-reference validation
- ✅ Date freshness check
- ✅ Terminology consistency
- ✅ Documentation coverage (target: 80%)

---

## Troubleshooting

### Documentation Not Generated
- Check `.claude/doc-config.yml` has `enabled: true`
- Verify `.agent/` directory exists
- Check file permissions

### Session Log Not Created
- Verify `.agent/sessions/` directory exists
- Check disk space
- Review error messages

### Commit Failed
- Check git status
- Verify remote configured
- Check branch permissions

---

**Configuration**: `.claude/doc-config.yml`
**Status**: Always enabled
**Last Updated**: 2026-01-19
