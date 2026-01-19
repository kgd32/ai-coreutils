# Ralph Loop Package

Autonomous AI development loop for Claude Code. Copy to any project to add Ralph Loop capabilities.

## Quick Start

```bash
# For new projects (copies files)
./install.sh /path/to/project

# For existing projects (uses RALPH_HOME, no copying)
/path/to/ralph-package/setup.sh
```

## Documentation

See **[RALPH.md](RALPH.md)** for complete documentation on:
- Installation
- Setup
- Usage
- Configuration
- Project Structure
- Advanced Features
- Troubleshooting

## Quick Reference

```bash
# After setup in your project:
.ralph/ralph-loop              # Start autonomous loop
.ralph/monitor                 # Visual dashboard
.ralph/monitor --json          # JSON streaming
RALPH_SKIP_PERMISSIONS=true .ralph/ralph-loop  # Skip confirmations
```

## Requirements

- Claude Code CLI: `npm install -g @anthropic-ai/claude-code && claude auth`
- jq (for JSON processing)
- bash (Git Bash, WSL, Linux, macOS)

## Package Contents

```
ralph-package/
├── install.sh                 # Install to new projects (copies files)
├── setup.sh                   # Setup existing projects (uses RALPH_HOME)
├── COPY.bat                    # Windows copy script
├── RALPH.md                    # Complete documentation
├── README.md                   # This file
├── CLAUDE.md                   # Knowledge base template
└── .ralph/
    ├── ralph_loop.sh          # Main loop
    ├── monitor.sh              # Monitor
    ├── response_analyzer.sh    # Analyzer
    ├── circuit_breaker.sh      # Circuit breaker
    ├── init.sh                 # Init script
    ├── state.json              # State template
    ├── session.md              # Session template
    ├── scratchpad.md           # Notes template
    ├── prompt.md               # Instructions template
    ├── ralph.yml.template      # Task template
    ├── checkpoints/            # Directory
    ├── history/                # Directory
    └── sessions/               # Directory
```

## Two Installation Modes

### 1. install.sh - For New Projects

Copies all Ralph files to the project:

```bash
cd ralph-package
./install.sh /path/to/new-project
```

### 2. setup.sh - For Existing Projects (Recommended)

Sets `RALPH_HOME`, creates minimal local files:

```bash
cd /path/to/existing/project
/path/to/ralph-package/setup.sh
```

Benefits:
- No file copying
- Uses central Ralph scripts
- Easy updates (update once, all projects benefit)
- Preserves existing project structure

## Environment Variables

```bash
# Skip tool confirmations (auto-approve)
RALPH_SKIP_PERMISSIONS=true

# Point to central Ralph package (for setup.sh)
RALPH_HOME=/path/to/ralph-package
```

## License

MIT License - Use in any project.
