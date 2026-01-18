# AI-Coreutils Skills

This directory contains the skills for AI-Coreutils development.

## Auto-Invoked Skills

### auto-doc.md ⚠️ ALWAYS ENABLED
- **Status**: Automatically invoked after all work
- **Purpose**: Maintains documentation without manual intervention
- **Trigger**: After tasks, commits, tests, errors
- **Configuration**: `.claude/doc-config.yml`

**NO MANUAL INVOCATION NEEDED** - This runs automatically.

## Primary Skills

### dev-agent.md
- **Purpose**: Development work implementation
- **Usage**: `skill: "dev-agent" --task "<description>"`
- **Auto-doc integration**: Invoked after code changes

### test-agent.md
- **Purpose**: Test execution and verification
- **Usage**: `skill: "test-agent" --scope "<scope>"`
- **Auto-doc integration**: Invoked after test runs

### phase-agent.md
- **Purpose**: Phase tracking and management
- **Usage**: `skill: "phase-agent" --action "<action>" --phase "<N>"`
- **Auto-doc integration**: Invoked after phase updates

### doc-agent.md
- **Purpose**: Manual documentation (sweep/targeted modes)
- **Usage**: `skill: "doc-agent"` or `skill: "doc-agent" --target "<topic>"`
- **Note**: Use for manual documentation sweeps; auto-doc handles routine updates

## Skill Integration

```
Work Flow:
┌─────────────┐
│  dev-agent  │ ──→ Code changes
└─────────────┘
       │
       ├──→ git commit
       │
       ▼
┌─────────────┐
│  auto-doc   │ ──→ Auto-generates documentation
└─────────────┘
       │
       ├──→ CLAUDE.md updated
       ├──→ ralph.yml updated
       ├──→ Session log created
       └──→ git commit (docs: ...)
```

## Configuration

- **doc-config.yml**: Auto-documentation configuration
- **CLAUDE.md**: Agent knowledge base (auto-updated)
- **ralph.yml**: Task tracking (auto-updated)
- **.agent/sessions/**: Session logs (auto-created)

## Quick Reference

```bash
# Development work
skill: "dev-agent" --task "implement ai-ls with walkdir"

# Run tests
skill: "test-agent" --scope "all"

# Check phase status
skill: "phase-agent" --action "status"

# Manual documentation sweep
skill: "doc-agent"

# Manual documentation for specific topic
skill: "doc-agent" --target "bug fix: memory mapping"
```

---

**Remember**: Auto-documentation is ALWAYS enabled. No manual invocation needed for routine documentation.
