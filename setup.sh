#!/bin/bash

# Ralph Loop Setup Script for Existing Projects
# Sets up RALPH_HOME to use central Ralph installation without copying files
# Interactive mode - prompts before creating any files

set -e

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
RED='\033[0;31m'
CYAN='\033[0;36m'
BOLD='\033[1m'
NC='\033[0m'

# RALPH_HOME is where the central Ralph package is located
# Default: parent directory of this script
RALPH_HOME_DEFAULT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo -e "${CYAN}${BOLD}╔══════════════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}${BOLD}║${NC} ${BOLD}Ralph Loop Setup for Existing Projects${NC}                    ${CYAN}${BOLD}║${NC}"
echo -e "${CYAN}${BOLD}╚══════════════════════════════════════════════════════════════╝${NC}"
echo ""

# Check if RALPH_HOME is already set
if [[ -n "$RALPH_HOME" ]]; then
    echo -e "${GREEN}✓ RALPH_HOME is already set:${NC} ${BOLD}$RALPH_HOME${NC}"
    RALPH_HOME_SOURCE="environment"
else
    echo -e "${YELLOW}⊙ RALPH_HOME not set${NC}"
    echo ""
    echo -e "${BLUE}Default RALPH_HOME (this package location):${NC}"
    echo "  $RALPH_HOME_DEFAULT"
    echo ""
    echo -n "Use this location? [Y/n] "
    read -r response
    if [[ "$response" =~ ^[Nn]$ ]]; then
        echo -n "Enter path to Ralph package: "
        read -r custom_path
        if [[ -d "$custom_path/.ralph" ]]; then
            RALPH_HOME_DEFAULT="$custom_path"
        else
            echo -e "${RED}Error: .ralph not found in: $custom_path${NC}"
            exit 1
        fi
    fi
    RALPH_HOME="$RALPH_HOME_DEFAULT"
    RALPH_HOME_SOURCE="default"
    export RALPH_HOME="$RALPH_HOME"
    echo -e "${GREEN}✓ RALPH_HOME set to: $RALPH_HOME${NC}"
    echo ""
fi

# Verify RALPH_HOME has required files
if [[ ! -d "$RALPH_HOME/.ralph" ]]; then
    echo -e "${RED}Error: RALPH_HOME/.ralph not found at: $RALPH_HOME${NC}"
    echo ""
    echo "Please set RALPH_HOME to point to the Ralph package directory:"
    echo "  export RALPH_HOME=/path/to/ralph-package"
    exit 1
fi

# Target directory (current project)
TARGET_DIR="$(pwd)"

echo -e "${BLUE}════════════════════════════════════════════════════════════════${NC}"
echo -e "${BLUE}Target Project:${NC} ${BOLD}$TARGET_DIR${NC}"
echo -e "${BLUE}════════════════════════════════════════════════════════════════${NC}"
echo ""

# Scan existing files
echo -e "${BLUE}Scanning for existing files...${NC}"
echo ""

# Track what needs to be created
declare -a FILES_TO_CREATE=()
declare -a FILES_TO_SKIP=()

# Check .ralph directory
if [[ -d "$TARGET_DIR/.ralph" ]]; then
    echo -e "${YELLOW}⊙ .ralph/ directory exists${NC}"
else
    echo -e "${BLUE}⊙ .ralph/ directory not found - will create${NC}"
    FILES_TO_CREATE+=(".ralph/")
fi

# Check state.json
if [[ -f "$TARGET_DIR/.ralph/state.json" ]]; then
    echo -e "${GREEN}✓ .ralph/state.json exists${NC}"
    FILES_TO_SKIP+=("state.json")
else
    echo -e "${BLUE}⊙ .ralph/state.json not found - will create${NC}"
    FILES_TO_CREATE+=("state.json")
fi

# Check session.md
if [[ -f "$TARGET_DIR/.ralph/session.md" ]]; then
    echo -e "${GREEN}✓ .ralph/session.md exists${NC}"
    FILES_TO_SKIP+=("session.md")
else
    echo -e "${BLUE}⊙ .ralph/session.md not found - will create${NC}"
    FILES_TO_CREATE+=("session.md")
fi

# Check scratchpad.md
if [[ -f "$TARGET_DIR/.ralph/scratchpad.md" ]]; then
    echo -e "${GREEN}✓ .ralph/scratchpad.md exists${NC}"
    FILES_TO_SKIP+=("scratchpad.md")
else
    echo -e "${BLUE}⊙ .ralph/scratchpad.md not found - will create${NC}"
    FILES_TO_CREATE+=("scratchpad.md")
fi

# Check ralph.yml
if [[ -f "$TARGET_DIR/.ralph/ralph.yml" ]]; then
    echo -e "${GREEN}✓ .ralph/ralph.yml exists${NC}"
    FILES_TO_SKIP+=("ralph.yml")
else
    echo -e "${YELLOW}⊙ .ralph/ralph.yml not found (required)${NC}"
    FILES_TO_CREATE+=("ralph.yml")
fi

# Check CLAUDE.md
if [[ -f "$TARGET_DIR/CLAUDE.md" ]]; then
    echo -e "${GREEN}✓ CLAUDE.md exists${NC}"
    FILES_TO_SKIP+=("CLAUDE.md")
else
    echo -e "${YELLOW}⊙ CLAUDE.md not found (recommended)${NC}"
    FILES_TO_CREATE+=("CLAUDE.md")
fi

# Check spec.md
if [[ -f "$TARGET_DIR/spec.md" ]]; then
    echo -e "${GREEN}✓ spec.md exists${NC}"
    FILES_TO_SKIP+=("spec.md")
else
    echo -e "${YELLOW}⊙ spec.md not found (recommended)${NC}"
    FILES_TO_CREATE+=("spec.md")
fi

# Check for existing wrapper scripts
WRAPPER_FILES=("ralph-loop" "monitor" "response-analyzer" "circuit-breaker")
for wrapper in "${WRAPPER_FILES[@]}"; do
    if [[ -f "$TARGET_DIR/.ralph/$wrapper" ]]; then
        echo -e "${GREEN}✓ .ralph/$wrapper wrapper exists${NC}"
        FILES_TO_SKIP+=("$wrapper wrapper")
    else
        FILES_TO_CREATE+=("$wrapper wrapper")
    fi
done

echo ""
echo -e "${BLUE}════════════════════════════════════════════════════════════════${NC}"
echo ""

# Summary
if [[ ${#FILES_TO_CREATE[@]} -eq 0 ]]; then
    echo -e "${GREEN}${BOLD}All files already exist! Nothing to create.${NC}"
    echo ""
    echo -e "${BLUE}Your project is ready to use Ralph Loop.${NC}"
    echo ""
    echo -e "${BLUE}Available commands:${NC}"
    echo "  .ralph/ralph-loop              # Start autonomous loop"
    echo "  .ralph/monitor                 # Visual dashboard"
    echo "  .ralph/monitor --json          # JSON streaming"
    echo "  .ralph/circuit-breaker status  # Circuit breaker status"
    echo ""
    exit 0
fi

# Show what will be created
echo -e "${YELLOW}${BOLD}Files to be created:${NC}"
for file in "${FILES_TO_CREATE[@]}"; do
    echo -e "${YELLOW}  • ${file}${NC}"
done
echo ""
echo -e "${GREEN}${BOLD}Existing files (will be preserved):${NC}"
for file in "${FILES_TO_SKIP[@]}"; do
    echo -e "${GREEN}  ✓ ${file}${NC}"
done
echo ""

# Confirmation
echo -n "Create these files? [Y/n] "
read -r confirm
if [[ "$confirm" =~ ^[Nn]$ ]]; then
    echo "Setup cancelled."
    exit 0
fi

echo ""
echo -e "${BLUE}Creating files...${NC}"
echo ""

# Create directories
if [[ " ${FILES_TO_CREATE[@]} " =~ " \.ralph/ " ]]; then
    mkdir -p "$TARGET_DIR/.ralph/checkpoints"
    mkdir -p "$TARGET_DIR/.ralph/history"
    mkdir -p "$TARGET_DIR/.ralph/sessions"
    echo -e "${GREEN}✓ Created .ralph/ with subdirectories${NC}"
fi

# Create state.json
if [[ " ${FILES_TO_CREATE[@]} " =~ " state.json " ]]; then
    cat > "$TARGET_DIR/.ralph/state.json" << EOF
{
  "version": "1.0.0",
  "loop": {
    "iteration": 0,
    "max_iterations": 1000,
    "state": "idle",
    "last_run": null,
    "last_status": null
  },
  "tasks": {
    "current_task_id": null,
    "completed_tasks": [],
    "blocked_tasks": [],
    "total_tasks": 0,
    "completion_percentage": 0
  },
  "session": {
    "session_id": null,
    "started_at": null,
    "last_activity": null,
    "expires_at": null
  },
  "circuit_breaker": {
    "state": "CLOSED",
    "consecutive_no_progress": 0,
    "consecutive_same_error": 0,
    "last_progress_loop": 0,
    "total_opens": 0
  },
  "rate_limit": {
    "calls_this_hour": 0,
    "max_calls_per_hour": 100,
    "hour_reset_timestamp": null
  },
  "exit_conditions": {
    "test_only_loops": 0,
    "done_signals": 0,
    "completion_indicators": 0
  },
  "metadata": {
    "created_at": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
    "updated_at": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
    "ralph_version": "1.0.0",
    "ralph_home": "$RALPH_HOME"
  }
}
EOF
    echo -e "${GREEN}✓ Created .ralph/state.json${NC}"
fi

# Create session.md
if [[ " ${FILES_TO_CREATE[@]} " =~ " session\.md " ]]; then
    cp "$RALPH_HOME/.ralph/session.md" "$TARGET_DIR/.ralph/"
    echo -e "${GREEN}✓ Created .ralph/session.md${NC}"
fi

# Create scratchpad.md
if [[ " ${FILES_TO_CREATE[@]} " =~ " scratchpad\.md " ]]; then
    cp "$RALPH_HOME/.ralph/scratchpad.md" "$TARGET_DIR/.ralph/"
    echo -e "${GREEN}✓ Created .ralph/scratchpad.md${NC}"
fi

# Create ralph.yml
if [[ " ${FILES_TO_CREATE[@]} " =~ " ralph\.yml " ]]; then
    cp "$RALPH_HOME/.ralph/ralph.yml.template" "$TARGET_DIR/.ralph/ralph.yml"
    echo -e "${YELLOW}⊙ Created .ralph/ralph.yml (template - edit with your tasks)${NC}"
fi

# Create CLAUDE.md
if [[ " ${FILES_TO_CREATE[@]} " =~ " CLAUDE\.md " ]]; then
    cp "$RALPH_HOME/CLAUDE.md" "$TARGET_DIR/"
    echo -e "${YELLOW}⊙ Created CLAUDE.md (template - update with your patterns)${NC}"
fi

# Create spec.md placeholder
if [[ " ${FILES_TO_CREATE[@]} " =~ " spec\.md " ]]; then
    cat > "$TARGET_DIR/spec.md" << EOF
# Project Specification

## Overview
Describe your project here.

## Requirements
1. Requirement 1
2. Requirement 2

## Features
- Feature 1
- Feature 2

## Notes
Edit this file with your project requirements.
EOF
    echo -e "${YELLOW}⊙ Created spec.md (template - add your requirements)${NC}"
fi

# Create wrapper scripts
for wrapper in "${WRAPPER_FILES[@]}"; do
    if [[ " ${FILES_TO_CREATE[@]} " =~ " $wrapper wrapper " ]]; then
        case $wrapper in
            "ralph-loop")
                cat > "$TARGET_DIR/.ralph/ralph-loop" << EOF
#!/bin/bash
# Wrapper script - uses RALPH_HOME
export RALPH_HOME="\${RALPH_HOME:-$RALPH_HOME}"
exec "\$RALPH_HOME/.ralph/ralph_loop.sh" "\$@"
EOF
                ;;
            "monitor")
                cat > "$TARGET_DIR/.ralph/monitor" << EOF
#!/bin/bash
# Wrapper script - uses RALPH_HOME
export RALPH_HOME="\${RALPH_HOME:-$RALPH_HOME}"
exec "\$RALPH_HOME/.ralph/monitor.sh" "\$@"
EOF
                ;;
            "response-analyzer")
                cat > "$TARGET_DIR/.ralph/response-analyzer" << EOF
#!/bin/bash
# Wrapper script - uses RALPH_HOME
export RALPH_HOME="\${RALPH_HOME:-$RALPH_HOME}"
exec "\$RALPH_HOME/.ralph/response_analyzer.sh" "\$@"
EOF
                ;;
            "circuit-breaker")
                cat > "$TARGET_DIR/.ralph/circuit-breaker" << EOF
#!/bin/bash
# Wrapper script - uses RALPH_HOME
export RALPH_HOME="\${RALPH_HOME:-$RALPH_HOME}"
exec "\$RALPH_HOME/.ralph/circuit_breaker.sh" "\$@"
EOF
                ;;
        esac
        chmod +x "$TARGET_DIR/.ralph/$wrapper"
        echo -e "${GREEN}✓ Created .ralph/${wrapper}${NC}"
    fi
done

echo ""
echo -e "${GREEN}${BOLD}Setup Complete!${NC}"
echo ""
echo -e "${BLUE}Configuration:${NC}"
echo "  RALPH_HOME: $RALPH_HOME"
echo "  Project:    $TARGET_DIR"
echo ""
echo -e "${BLUE}Local files created:${NC}"
for file in "${FILES_TO_CREATE[@]}"; do
    echo "  • $file"
done
echo ""
echo -e "${BLUE}Scripts use central installation at:${NC}"
echo "  $RALPH_HOME/.ralph/"
echo ""
echo -e "${BLUE}════════════════════════════════════════════════════════════════${NC}"
echo ""
echo -e "${BLUE}Next steps:${NC}"
echo "  1. Edit .ralph/ralph.yml with your project tasks"
echo "  2. Review .ralph/state.json for configuration"
echo "  3. Run: .ralph/ralph-loop"
echo ""
echo -e "${BLUE}Quick commands:${NC}"
echo "  .ralph/ralph-loop                       # Start autonomous loop"
echo "  .ralph/monitor                          # Visual dashboard"
echo "  .ralph/monitor --json                   # JSON streaming"
echo "  .ralph/circuit-breaker status           # Circuit breaker status"
echo ""
echo -e "${YELLOW}Note: To make RALPH_HOME persistent across sessions:${NC}"
if [[ "$RALPH_HOME_SOURCE" == "default" ]]; then
    echo "  Add to ~/.bashrc:"
    echo "    export RALPH_HOME=\"$RALPH_HOME\""
    echo "    export PATH=\"\$PATH:\$RALPH_HOME/.ralph\""
    echo ""
    echo "  Then run: source ~/.bashrc"
fi
echo ""
