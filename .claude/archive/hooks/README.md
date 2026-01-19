# Ralph Loop Hooks

This directory contains hook scripts for the Ralph Loop autonomous development agent.

## Important: Windows Git Hooks Limitation

**On Windows**, git hooks may not run automatically due to file system limitations with Unix execute permissions.

### Workaround Options:

1. **Manual Status Check:** Use `/ralph-status` command after commits
2. **Git Alias:** Add an alias to run status manually:
   ```bash
   git config alias.ralph-status '!powershell.exe -ExecutionPolicy Bypass -File .claude/hooks/ralph-status.ps1'
   ```
   Then use: `git ralph-status`

3. **Enable Hooks (if supported):**
   - Run: `git config core.hooksPath .git/hooks`
   - Ensure your file system supports execute permissions

## Status Script

### `ralph-status.ps1` (Windows PowerShell)
Displays Ralph Loop status with:
- Current iteration and phase
- Current task and status
- Tasks summary (total, done, in-progress, todo)
- Overall progress bar
- Tech stack information
- Blockers (if any)

### `ralph-status.sh` (Unix/Linux/macOS)
Same functionality for Unix-based systems.

## Manual Usage

Show Ralph Loop status anytime:

```bash
# Windows
powershell.exe -ExecutionPolicy Bypass -File .claude/hooks/ralph-status.ps1

# Unix/Linux/macOS
bash .claude/hooks/ralph-status.sh

# Via Claude Code command
/ralph-status
```

## Output Example

```
+====================================================================+
|                         🤖 Ralph Loop Status                       |
+--------------------------------------------------------------------+
|                                                                    |
|  Session                                                           |
|  +----------------------------------------------------------------+ |
|  |  Iteration: 2                                                    |
|  |  Phase: mvp                                                      |
|  |  Last Update: 2026-01-19 13:30:00 UTC                           |
|  +----------------------------------------------------------------+ |
|                                                                    |
|  Current Task                                                      |
|  +----------------------------------------------------------------+ |
|  |  [TODO] implement-remaining-utils (todo)                        |
|  +----------------------------------------------------------------+ |
|                                                                    |
|  Tasks Summary                                                     |
|  +----------------------------------------------------------------+ |
|  |  Total: 10  |  Done: 6  |  In Progress: 0  |  Todo: 4            |
|  +----------------------------------------------------------------+ |
|                                                                    |
|  Overall Progress:  ############........  60% (6/10 tasks)        |
|                                                                    |
|  Tech Stack                                                        |
|  +----------------------------------------------------------------+ |
|  |  Language: Rust                                                  |
|  +----------------------------------------------------------------+ |
|                                                                    |
+====================================================================+
```

## Symbols

- `[BLOCKED]` - Task is blocked
- `[IN-PROGRESS]` - Task is currently being worked on
- `[DONE]` - Task is complete
- `[TODO]` - Task is pending

## Configuration

Hook behavior is documented in `.claude/hooks/hooks.json`.
