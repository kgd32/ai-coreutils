# Ralph Loop Hooks

This directory contains hook scripts for the Ralph Loop autonomous development agent.

## Installed Hooks

### Post-Commit Hook

**Location:** `.git/hooks/post-commit`

**Trigger:** Automatically runs after every `git commit`

**Purpose:** Displays Ralph Loop status dashboard showing:
- Current iteration number
- Current phase (mvp, phase_2, phase_3)
- Task being worked on
- Tasks summary (total, done, in-progress, todo)
- Overall progress bar
- Tech stack detection
- Blockers (if any)

**Scripts:**
- `.claude/hooks/ralph-status.sh` - Unix/Linux/macOS script
- `.claude/hooks/ralph-status.cmd` - Windows script

## Configuration

Hook behavior is documented in `.claude/hooks/hooks.json`.

## Manual Trigger

You can manually display Ralph Loop status anytime using:

```bash
/ralph-status
```

Or directly via the scripts:

```bash
# Unix/Linux/macOS
bash .claude/hooks/ralph-status.sh

# Windows
cmd.exe /c .claude\hooks\ralph-status.cmd
```

## Output Example

```
┌────────────────────────────────────────────────────────────────────┐
│                         🤖 Ralph Loop Status                       │
├────────────────────────────────────────────────────────────────────┤
│                                                                    │
│  Session                                                           │
│  ┌────────────────────────────────────────────────────────────┐    │
│  │  Iteration: 5                                               │    │
│  │  Phase: mvp                                                 │    │
│  │  Last Update: 2026-01-19 10:30:00 UTC                       │    │
│  └────────────────────────────────────────────────────────────┘    │
│                                                                    │
│  Current Task                                                      │
│  ┌────────────────────────────────────────────────────────────┐    │
│  │  ⚑ implement-jsonl-output (in-progress)                     │    │
│  └────────────────────────────────────────────────────────────┘    │
│                                                                    │
│  Tasks Summary                                                     │
│  ┌────────────────────────────────────────────────────────────┐    │
│  │  Total: 10  │  Done: 2  │  In Progress: 1  │  Todo: 7        │    │
│  └────────────────────────────────────────────────────────────┘    │
│                                                                    │
│  Overall Progress:  ██░░░░░░░░░░░░░░░░░░░  20% (2/10 tasks)        │
│                                                                    │
│  Tech Stack                                                        │
│  ┌────────────────────────────────────────────────────────────┐    │
│  │  Language: Rust                                              │    │
│  └────────────────────────────────────────────────────────────┘    │
│                                                                    │
└────────────────────────────────────────────────────────────────────┘
```

## Symbols

- `✓` - Done task
- `⚑` - In-progress task
- `○` - Todo task
- `🚧` - Blocked task
- `█` - Progress bar filled
- `░` - Progress bar empty

## Disable Hook

To temporarily disable the post-commit hook:

```bash
chmod -x .git/hooks/post-commit  # Unix/Linux/macOS
# or
del .git\hooks\post-commit       # Windows (to remove)
```

To re-enable:

```bash
chmod +x .git/hooks/post-commit  # Unix/Linux/macOS
# or
# Copy from .claude/hooks/post-commit.template  # Windows
```

## Customization

Edit the scripts to customize:
- Display format
- Colors and symbols
- Information shown
- Error handling
