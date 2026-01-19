# Ralph Loop - Complete Guide

## What is Ralph Loop?

Ralph Loop is an autonomous AI development system for Claude Code. It continuously iterates on your project, working through tasks until completion, with intelligent safeguards to prevent infinite loops.

## Table of Contents

- [Quick Start](#quick-start)
- [Installation](#installation)
- [Setup](#setup)
- [Usage](#usage)
- [Configuration](#configuration)
- [Project Structure](#project-structure)
- [Advanced](#advanced)
- [Troubleshooting](#troubleshooting)

---

## Quick Start

### For New Projects

```bash
# Navigate to the Ralph package
cd ralph-package

# Install to a new project
./install.sh /path/to/new-project

# Or install to current directory
./install.sh
```

### For Existing Projects

```bash
# Navigate to your existing project
cd /path/to/existing/project

# Run setup from the Ralph package
/path/to/ralph-package/setup.sh
```

### Run the Loop

```bash
# Start autonomous development
.ralph/ralph-loop

# With monitoring (recommended)
.ralph/ralph-loop &
.ralph/monitor

# JSON streaming mode
.ralph/monitor --json
```

---

## Installation

### Prerequisites

1. **Node.js** (18+) - for npm
2. **Claude Code CLI** - the AI that powers Ralph
3. **jq** - JSON processing
4. **bash** - Git Bash, WSL, or Linux/Mac terminal

### Install Claude Code CLI

```bash
# Install globally
npm install -g @anthropic-ai/claude-code

# Authenticate
claude auth
```

### Install Ralph Loop

#### Option 1: Install to New Project

```bash
cd ralph-package
./install.sh /path/to/new-project
```

This copies all Ralph Loop files to the project.

#### Option 2: Setup Existing Project (Recommended)

```bash
cd /path/to/existing/project
/path/to/ralph-package/setup.sh
```

This:
- Sets `RALPH_HOME` environment variable
- Creates minimal local files (state.json, wrappers)
- Uses central Ralph scripts (no copying)
- Preserves existing project files

#### Option 3: Manual Installation

```bash
# Copy .ralph folder to your project
cp -r ralph-package/.ralph /path/to/your/project/
cp ralph-package/CLAUDE.md /path/to/your/project/
```

---

## Setup

### 1. Initialize (if needed)

```bash
.ralph/init.sh
```

Creates templates if missing.

### 2. Configure Tasks

Edit `.ralph/ralph.yml`:

```yaml
project:
  name: "My Project"
  description: "What it does"
  version: "0.1.0"

tasks:
  - id: "task-1"
    title: "First Task"
    description: "What needs to be done"
    priority: "critical"
    status: "todo"
    dependencies: []
    acceptance_criteria:
      - "Criterion 1"
      - "Criterion 2"
```

### 3. Verify Required Files

Ensure your project has:
- `.ralph/ralph.yml` - task configuration
- `CLAUDE.md` - knowledge base (in root)
- `spec.md` - project requirements (in root, recommended)

---

## Usage

### Basic Commands

```bash
# Start autonomous loop
.ralph/ralph-loop

# Monitor with visual dashboard
.ralph/monitor

# Monitor with JSON streaming
.ralph/monitor --json

# Monitor with custom refresh rate
.ralph/monitor --refresh 5

# Check circuit breaker status
.ralph/circuit-breaker status

# Reset circuit breaker
.ralph/circuit-breaker reset

# Parse Claude output
.ralph/response-analyzer parse <output_file>

# Update task status
.ralph/response-analyzer update-task <task-id> done
```

### With Environment Variables

```bash
# Skip tool permissions (bypass confirmation)
RALPH_SKIP_PERMISSIONS=true .ralph/ralph-loop

# With monitoring
RALPH_SKIP_PERMISSIONS=true .ralph/ralph-loop &
.ralph/monitor
```

### JSON Streaming Examples

```bash
# Stream JSON to file
.ralph/monitor --json > monitor.log

# Watch specific field
.ralph/monitor --json | jq '.loop.iteration'

# Filter for completion
.ralph/monitor --json | jq 'select(.tasks.completion_percent == 100)'

# Format output
.ralph/monitor --json | jq '{iteration: .loop.iteration, status: .loop.state}'
```

---

## Configuration

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `RALPH_HOME` | Central Ralph package location (for setup.sh) | Auto-detected |
| `RALPH_SKIP_PERMISSIONS` | Skip Claude tool confirmation | `false` |

### .ralph/state.json

Main configuration file:

```json
{
  "config": {
    "skip_permissions": false,
    "claude_timeout_minutes": 15,
    "max_calls_per_hour": 100
  }
}
```

### Command Line Flags

```bash
# ralph-loop (no flags yet, uses env vars)
.ralph/ralph-loop

# monitor
.ralph/monitor --refresh 5    # Refresh rate in seconds
.ralph/monitor --json         # JSON streaming mode
.ralph/monitor --visual       # Visual dashboard (default)
```

### ralph.yml Task Configuration

```yaml
tasks:
  - id: "unique-id"
    title: "Task Title"
    description: "What to do"
    priority: "critical|high|medium|low"
    status: "todo|in-progress|done|blocked"
    dependencies: ["other-task-id"]
    acceptance_criteria:
      - "Must have this"
    subtasks:
      - "Subtask 1"
    estimated_hours: 4
```

---

## Project Structure

### After Setup (install.sh - New Project)

```
project/
├── .ralph/                      # All Ralph files copied
│   ├── ralph_loop.sh          # Main loop
│   ├── monitor.sh              # Monitor
│   ├── response_analyzer.sh    # Analyzer
│   ├── circuit_breaker.sh      # Circuit breaker
│   ├── init.sh                 # Init script
│   ├── state.json              # State
│   ├── session.md              # Session info
│   ├── scratchpad.md           # Notes
│   ├── prompt.md               # Instructions
│   ├── ralph.yml               # Your tasks
│   ├── checkpoints/            # Checkpoints
│   ├── history/                # Logs
│   └── sessions/               # Sessions
├── CLAUDE.md                   # Knowledge base
└── spec.md                     # Requirements
```

### After Setup (setup.sh - Existing Project)

```
project/
├── .ralph/                      # Minimal local files
│   ├── state.json              # Local state
│   ├── session.md              # Local session
│   ├── scratchpad.md           # Local notes
│   ├── ralph.yml               # Your tasks
│   ├── ralph-loop              # Wrapper to central
│   ├── monitor                 # Wrapper to central
│   ├── response-analyzer       # Wrapper to central
│   └── circuit-breaker         # Wrapper to central
├── CLAUDE.md                   # Knowledge base
└── spec.md                     # Requirements

# Scripts use: $RALPH_HOME/.ralph/
```

---

## Advanced

### Using RALPH_HOME

Set `RALPH_HOME` to point to the central Ralph package:

```bash
# In ~/.bashrc
export RALPH_HOME="/path/to/ralph-package"
export PATH="$PATH:$RALPH_HOME/.ralph"
```

Then projects can just run `setup.sh` to create wrappers.

### Skip Permissions Mode

Auto-approve all tool confirmations in Claude Code:

```bash
# Set env var
RALPH_SKIP_PERMISSIONS=true .ralph/ralph-loop

# Or export for session
export RALPH_SKIP_PERMISSIONS=true
.ralph/ralph-loop
```

**Warning**: This bypasses Claude's tool confirmation. Only use in trusted environments.

### Circuit Breaker

Prevents runaway loops by detecting:
- No progress (no file changes)
- Repeated errors
- Output decline

```bash
# Check status
.ralph/circuit-breaker status

# Reset after opening
.ralph/circuit-breaker reset

# Check if execution allowed
.ralph/circuit-breaker check
```

### Exit Conditions

Ralph stops when:
- All tasks marked `done`
- Test-only threshold reached (feature complete)
- Consecutive done signals
- EXIT_SIGNAL: true in RALPH_STATUS
- Circuit breaker opens

### RALPH_STATUS Block

Agents must include this at the end of responses:

```markdown
---RALPH_STATUS---
STATUS: IN_PROGRESS | COMPLETE | BLOCKED
CURRENT_TASK: task-id
TASKS_COMPLETED_THIS_LOOP: 2
FILES_MODIFIED: 5
TESTS_STATUS: PASSING | FAILING | NOT_RUN
WORK_TYPE: IMPLEMENTATION | TESTING | DOCUMENTATION
EXIT_SIGNAL: false | true
RECOMMENDATION: Continue with next task
---END_RALPH_STATUS---
```

---

## Troubleshooting

### Claude Code Not Found

```bash
# Install Claude Code CLI
npm install -g @anthropic-ai/claude-code
claude auth
```

### Permission Denied

```bash
# Make scripts executable
chmod +x .ralph/*.sh
```

### Circuit Breaker Open

```bash
# Reset circuit breaker
.ralph/circuit-breaker reset

# Check what happened
cat .ralph/circuit_breaker_alert.txt
```

### Start Fresh

```bash
# Reset state (preserves ralph.yml)
rm .ralph/state.json
.ralph/init.sh
```

### Complete Reset

```bash
# Remove all Ralph files
rm -rf .ralph

# Re-run setup
/path/to/ralph-package/setup.sh
```

### Debug Mode

Check logs:

```bash
# View daily log
cat .ralph/history/ralph-$(date +%Y%m%d).log

# View specific iteration output
cat .ralph/history/claude_output_5.json

# Check current state
cat .ralph/state.json | jq '.'
```

### Windows Specifics

**PowerShell:**
```powershell
# Use bash to run scripts
bash .ralph/ralph-loop

# Or set PATH
$env:PATH += ";C:\path\to\ralph-package\.ralph"
```

**Git Bash:**
```bash
# Works directly
.ralph/ralph-loop
```

---

## File Reference

### Scripts

| Script | Purpose |
|--------|---------|
| `ralph-loop` / `ralph_loop.sh` | Main autonomous loop |
| `monitor` / `monitor.sh` | Monitoring dashboard |
| `circuit-breaker` / `circuit_breaker.sh` | Circuit breaker management |
| `response-analyzer` / `response_analyzer.sh` | Output analysis |
| `init.sh` | Initialize templates |

### State Files

| File | Purpose |
|------|---------|
| `state.json` | Loop state, metrics, configuration |
| `session.md` | Current session info |
| `scratchpad.md` | Agent notes and context |
| `ralph.yml` | Task configuration (you create) |
| `prompt.md` | Agent instructions template |

### Documentation

| File | Purpose |
|------|---------|
| `CLAUDE.md` | Agent knowledge base |
| `spec.md` | Project requirements |
| `RALPH.md` | This file |

---

## Version

**Ralph Loop Package**: v1.0.0
**Compatible with**: Claude Code CLI v2.0.76+
