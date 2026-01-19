#!/bin/bash

# Ralph Loop Initialization Script
# Sets up .ralph/ directory structure for a new project

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
BOLD='\033[1m'
NC='\033[0m'

# Script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
RALPH_DIR="$PROJECT_ROOT/.ralph"

echo -e "${BLUE}${BOLD}Ralph Loop Initialization${NC}"
echo -e "${BLUE}═══════════════════════════${NC}"
echo ""

# Check if .ralph already exists
if [[ -d "$RALPH_DIR" ]] && [[ -f "$RALPH_DIR/state.json" ]]; then
    echo -e "${YELLOW}Warning: .ralph directory already exists${NC}"
    echo -n "Do you want to reinitialize? [y/N] "
    read -r response
    if [[ ! "$response" =~ ^[Yy]$ ]]; then
        echo "Initialization cancelled"
        exit 0
    fi
    echo ""
fi

# Create directory structure
echo -e "${BLUE}Creating .ralph directory structure...${NC}"
mkdir -p "$RALPH_DIR/checkpoints"
mkdir -p "$RALPH_DIR/history"
mkdir -p "$RALPH_DIR/sessions"
echo -e "${GREEN}✓ Created directories${NC}"

# Check for existing files and preserve if needed
preserve_file() {
    local file=$1
    local default_content=$2

    if [[ -f "$file" ]]; then
        echo -e "${YELLOW}  Preserving existing: $(basename "$file")${NC}"
    else
        echo "$default_content" > "$file"
        echo -e "${GREEN}  Created: $(basename "$file")${NC}"
    fi
}

# Create or preserve state.json
if [[ -f "$RALPH_DIR/state.json" ]]; then
    echo -e "${YELLOW}  Preserving existing: state.json${NC}"
else
    cat > "$RALPH_DIR/state.json" << EOF
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
    "ralph_version": "1.0.0"
  }
}
EOF
    echo -e "${GREEN}  Created: state.json${NC}"
fi

# Create or preserve session.md
preserve_file "$RALPH_DIR/session.md" "# Ralph Session

## Session Information
- **Session ID**: Not started
- **Status**: Idle

## Current Task
- No active task

## Session Progress
- **Iteration**: 0 / 1000
- **Tasks Completed**: 0 / 0

## Notes
- Session not yet started
"

# Create or preserve scratchpad.md
preserve_file "$RALPH_DIR/scratchpad.md" "# Ralph Scratchpad

> This file is used for iteration notes and context transfer between agents.

---

## Current Iteration Notes

### What I'm Working On
- Task: Not started
- Status: Idle

### What I've Tried
- No attempts yet

---

## Next Agent Context

### Priority Tasks
1. Initialize project
2. Set up development environment
3. Begin first task

---

## Quick Reference

### Success Criteria
- [ ] Project initialized
- [ ] Environment set up
- [ ] First task ready
"

# Check for ralph.yml
if [[ ! -f "$RALPH_DIR/ralph.yml" ]]; then
    echo -e "${RED}  Missing: ralph.yml${NC}"
    echo -e "${YELLOW}  You need to create ralph.yml with your project tasks${NC}"
    echo ""

    # Ask if they want to create a template
    echo -n "Create a template ralph.yml? [y/N] "
    read -r create_template
    if [[ "$create_template" =~ ^[Yy]$ ]]; then
        cat > "$RALPH_DIR/ralph.yml" << EOF
project:
  name: "My Project"
  description: "Project description"
  version: "0.1.0"
  repository: ""

  metadata:
    tech_stack: "Your tech stack"
    target_platforms: "Your platforms"
    deployment: "Your deployment"

  objectives:
    - "Objective 1"
    - "Objective 2"

tasks:
  - id: "setup-project"
    title: "Project Setup"
    description: |
      Initialize the project structure and dependencies
    priority: "critical"
    status: "todo"
    dependencies: []
    acceptance_criteria:
      - "Project structure created"
      - "Dependencies installed"
      - "Initial tests passing"
    subtasks:
      - "Create directory structure"
      - "Install dependencies"
      - "Set up build system"
    estimated_hours: 2

  - id: "implement-feature-1"
    title: "Feature 1"
    description: |
      Implement the first feature
    priority: "high"
    status: "todo"
    dependencies: ["setup-project"]
    acceptance_criteria:
      - "Feature working"
      - "Tests passing"
      - "Documentation complete"
    subtasks: []
    estimated_hours: 4

workflows:
  - name: "development"
    description: "Standard development workflow"
    steps:
      - "Make changes"
      - "Run tests"
      - "Commit changes"

dependencies:
  - "dependency1"

testing:
  framework: "Your test framework"
  coverage: "Coverage tool"

deployment:
  - "Deployment method"

monitoring:
  - "Metrics to track"

community:
  contribution_guide: "CONTRIBUTING.md"
EOF
        echo -e "${GREEN}  Created: ralph.yml (template)${NC}"
    fi
else
    echo -e "${GREEN}  Found: ralph.yml${NC}"
fi

# Check for CLAUDE.md in project root
if [[ ! -f "$PROJECT_ROOT/CLAUDE.md" ]]; then
    echo -e "${YELLOW}  Warning: CLAUDE.md not found in project root${NC}"
    echo -e "${YELLOW}  It's recommended to have a CLAUDE.md for agent knowledge${NC}"

    echo -n "Create a template CLAUDE.md? [y/N] "
    read -r create_claude
    if [[ "$create_claude" =~ ^[Yy]$ ]]; then
        cat > "$PROJECT_ROOT/CLAUDE.md" << EOF
# CLAUDE.md - Agent Knowledge Base

## Purpose
This document accumulates knowledge about what works and what doesn't in this project. Each agent should update this with their findings.

## How to Use This Document
1. READ THIS FIRST before starting any task
2. Update with your findings after each task
3. Be specific about what works and what fails
4. Include code examples and error messages

---

## Project Context

### Tech Stack Realities
**What Works:**
- Template for agents to add successes

**What Doesn't Work:**
- Template for agents to add failures

## Language/Framework Specifics

### Tool Calls That Work
\`\`\`
// Examples
\`\`\`

### Tool Calls That Don't Work
- Template for agents to add failures

## Failed Approaches
*Template for agents to add failures*

## Working Patterns
*Template for agents to add successes*

## Tool & Package Issues
*Template for agents to add findings*

## Testing Insights
*Template for agents to add findings*

## Integration Challenges
*Template for agents to add findings*

## Performance Optimizations
*Template for agents to add findings*

## Security Considerations
*Template for agents to add findings*

## Iteration Efficiency Tips
*Template for agents to add findings*

## Self-Healing Notes
*Template for agents to add findings*

## Cumulative Learnings
1. Template for ranked discoveries

## Current State Assessment
- Project health: Good
- Phase: Initial
- Test coverage: Not established
- Blockers: None

## Agent-to-Agent Messages
*Communication between iterations*

## Update Log
- $(date +%Y-%m-%d): Initial setup
EOF
        echo -e "${GREEN}  Created: CLAUDE.md (template)${NC}"
    fi
else
    echo -e "${GREEN}  Found: CLAUDE.md${NC}"
fi

# Check for spec.md in project root
if [[ ! -f "$PROJECT_ROOT/spec.md" ]]; then
    echo -e "${YELLOW}  Warning: spec.md not found in project root${NC}"
    echo -e "${YELLOW}  It's recommended to have a spec.md with project requirements${NC}"
else
    echo -e "${GREEN}  Found: spec.md${NC}"
fi

# Make scripts executable
chmod +x "$RALPH_DIR/ralph_loop.sh" 2>/dev/null || true
chmod +x "$RALPH_DIR/monitor.sh" 2>/dev/null || true
chmod +x "$RALPH_DIR/response_analyzer.sh" 2>/dev/null || true
chmod +x "$RALPH_DIR/circuit_breaker.sh" 2>/dev/null || true

echo ""
echo -e "${GREEN}${BOLD}Initialization Complete!${NC}"
echo ""
echo -e "${BLUE}Next steps:${NC}"
echo "1. Edit $RALPH_DIR/ralph.yml with your project tasks"
echo "2. Review $PROJECT_ROOT/CLAUDE.md and update with project-specific knowledge"
echo "3. Ensure $PROJECT_ROOT/spec.md contains your project requirements"
echo "4. Start the Ralph Loop:"
echo "   ${BOLD}   $RALPH_DIR/ralph_loop.sh${NC}"
echo ""
echo "5. Or start with monitoring:"
echo "   ${BOLD}   $RALPH_DIR/ralph_loop.sh &  # Start loop in background"
echo "   ${BOLD}   $RALPH_DIR/monitor.sh       # Start monitor${NC}"
echo ""
echo -e "${BLUE}Available commands:${NC}"
echo "  $RALPH_DIR/ralph_loop.sh         - Start autonomous loop"
echo "  $RALPH_DIR/monitor.sh             - Show monitoring dashboard"
echo "  $RALPH_DIR/response_analyzer.sh   - Analyze Claude output"
echo "  $RALPH_DIR/circuit_breaker.sh     - Manage circuit breaker"
echo ""
