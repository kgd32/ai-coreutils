#!/bin/bash
# Ralph Loop Status Display Script
# Displays Ralph Loop status after git commits

RALPH_STATE_FILE=".ralph/state.json"
RALPH_SESSION_FILE=".ralph/session.md"
RALPH_SCRATCHPAD_FILE=".ralph/scratchpad.md"

# Check if .ralph folder exists
if [ ! -d ".ralph" ]; then
    echo ""
    echo "┌────────────────────────────────────────────────────────────────────┐"
    echo "│                    🚫 Ralph Loop Not Initialized                  │"
    echo "├────────────────────────────────────────────────────────────────────┤"
    echo "│                                                                    │"
    echo "│  No .ralph/ folder found.                                          │"
    echo "│                                                                    │"
    echo "│  To get started:                                                   │"
    echo "│    /ralph:init \"your project idea\"                                │"
    echo "│                                                                    │"
    echo "└────────────────────────────────────────────────────────────────────┘"
    echo ""
    exit 0
fi

# Check if state.json exists
if [ ! -f "$RALPH_STATE_FILE" ]; then
    echo ""
    echo "┌────────────────────────────────────────────────────────────────────┐"
    echo "│                    🚫 Ralph Loop Not Initialized                  │"
    echo "├────────────────────────────────────────────────────────────────────┤"
    echo "│                                                                    │"
    echo "│  .ralph/state.json not found.                                      │"
    echo "│                                                                    │"
    echo "│  Run /ralph:init to initialize.                                    │"
    echo "│                                                                    │"
    echo "└────────────────────────────────────────────────────────────────────┘"
    echo ""
    exit 0
fi

# Parse state.json using basic tools (no jq dependency)
if command -v jq >/dev/null 2>&1; then
    # Use jq if available
    ITERATION=$(jq -r '.iteration' "$RALPH_STATE_FILE" 2>/dev/null || echo "N/A")
    PHASE=$(jq -r '.phase' "$RALPH_STATE_FILE" 2>/dev/null || echo "N/A")
    CURRENT_TASK=$(jq -r '.current_task_id' "$RALPH_STATE_FILE" 2>/dev/null || echo "N/A")
    TASK_STATUS=$(jq -r '.current_task_status' "$RALPH_STATE_FILE" 2>/dev/null || echo "N/A")
    TOTAL_TASKS=$(jq -r '.tasks.total' "$RALPH_STATE_FILE" 2>/dev/null || echo "N/A")
    DONE_TASKS=$(jq -r '.tasks.done' "$RALPH_STATE_FILE" 2>/dev/null || echo "N/A")
    IN_PROGRESS_TASKS=$(jq -r '.tasks.in_progress' "$RALPH_STATE_FILE" 2>/dev/null || echo "N/A")
    TODO_TASKS=$(jq -r '.tasks.todo' "$RALPH_STATE_FILE" 2>/dev/null || echo "N/A")
    LAST_UPDATED=$(jq -r '.last_updated' "$RALPH_STATE_FILE" 2>/dev/null || echo "N/A")
    BLOCKED=$(jq -r '.blocked' "$RALPH_STATE_FILE" 2>/dev/null || echo "false")
    TECH_STACK=$(jq -r '.tech_stack.language' "$RALPH_STATE_FILE" 2>/dev/null || echo "Unknown")
else
    # Fallback: parse with grep/sed
    ITERATION=$(grep -o '"iteration": *[0-9]*' "$RALPH_STATE_FILE" | head -1 | grep -o '[0-9]*' || echo "N/A")
    PHASE=$(grep -o '"phase": *"[^"]*"' "$RALPH_STATE_FILE" | head -1 | grep -o '"[^"]*"$' | tr -d '"' || echo "N/A")
    CURRENT_TASK=$(grep -o '"current_task_id": *"[^"]*"' "$RALPH_STATE_FILE" | head -1 | grep -o '"[^"]*"$' | tr -d '"' || echo "N/A")
    TASK_STATUS=$(grep -o '"current_task_status": *"[^"]*"' "$RALPH_STATE_FILE" | head -1 | grep -o '"[^"]*"$' | tr -d '"' || echo "N/A")
    TOTAL_TASKS=$(grep -o '"total": *[0-9]*' "$RALPH_STATE_FILE" | head -1 | grep -o '[0-9]*' || echo "N/A")
    DONE_TASKS=$(grep -o '"done": *[0-9]*' "$RALPH_STATE_FILE" | head -1 | grep -o '[0-9]*' || echo "N/A")
    IN_PROGRESS_TASKS=$(grep -o '"in_progress": *[0-9]*' "$RALPH_STATE_FILE" | head -1 | grep -o '[0-9]*' || echo "N/A")
    TODO_TASKS=$(grep -o '"todo": *[0-9]*' "$RALPH_STATE_FILE" | head -1 | grep -o '[0-9]*' || echo "N/A")
    LAST_UPDATED=$(grep -o '"last_updated": *"[^"]*"' "$RALPH_STATE_FILE" | head -1 | grep -o '"[^"]*"$' | tr -d '"' || echo "N/A")
    BLOCKED=$(grep -o '"blocked": *[^,}]*' "$RALPH_STATE_FILE" | head -1 | grep -o 'true' || echo "false")
    TECH_STACK=$(grep -o '"language": *"[^"]*"' "$RALPH_STATE_FILE" | head -1 | grep -o '"[^"]*"$' | tr -d '"' || echo "Unknown")
fi

# Calculate progress percentage
if [ "$TOTAL_TASKS" != "N/A" ] && [ "$TOTAL_TASKS" -gt 0 ]; then
    PROGRESS=$((DONE_TASKS * 100 / TOTAL_TASKS))
    FILLED=$((DONE_TASKS * 20 / TOTAL_TASKS))
    EMPTY=$((20 - FILLED))
    PROGRESS_BAR=$(printf '█%.0s' $(seq 1 $FILLED))$(printf '░%.0s' $(seq 1 $EMPTY))
else
    PROGRESS="N/A"
    PROGRESS_BAR="░░░░░░░░░░░░░░░░░░░░"
fi

# Format timestamp
if [ "$LAST_UPDATED" != "N/A" ]; then
    LAST_UPDATED=$(echo "$LAST_UPDATED" | sed 's/T/ /' | sed 's/\.[0-9]*Z$/ UTC/')
fi

# Display status
echo ""
echo "┌────────────────────────────────────────────────────────────────────┐"
echo "│                         🤖 Ralph Loop Status                       │"
echo "├────────────────────────────────────────────────────────────────────┤"
echo "│                                                                    │"
echo "│  Session                                                           │"
echo "│  ┌────────────────────────────────────────────────────────────┐    │"
echo "│  │  Iteration: $ITERATION                                                  │    │"
echo "│  │  Phase: $PHASE                                                    │    │"
echo "│  │  Last Update: $LAST_UPDATED            │    │"
echo "│  └────────────────────────────────────────────────────────────┘    │"
echo "│                                                                    │"
echo "│  Current Task                                                      │"
echo "│  ┌────────────────────────────────────────────────────────────┐    │"

if [ "$BLOCKED" = "true" ]; then
    echo "│  │  🚧 $CURRENT_TASK ($TASK_STATUS - BLOCKED)                   │    │"
elif [ "$TASK_STATUS" = "in-progress" ] || [ "$TASK_STATUS" = "in_progress" ]; then
    echo "│  │  ⚑ $CURRENT_TASK ($TASK_STATUS)                             │    │"
elif [ "$TASK_STATUS" = "done" ]; then
    echo "│  │  ✓ $CURRENT_TASK ($TASK_STATUS)                              │    │"
else
    echo "│  │  ○ $CURRENT_TASK ($TASK_STATUS)                              │    │"
fi

echo "│  └────────────────────────────────────────────────────────────┘    │"
echo "│                                                                    │"
echo "│  Tasks Summary                                                     │"
echo "│  ┌────────────────────────────────────────────────────────────┐    │"
echo "│  │  Total: $TOTAL_TASKS  │  Done: $DONE_TASKS  │  In Progress: $IN_PROGRESS_TASKS  │  Todo: $TODO_TASKS       │    │"
echo "│  └────────────────────────────────────────────────────────────┘    │"
echo "│                                                                    │"
echo "│  Overall Progress:  $PROGRESS_BAR  $PROGRESS% ($DONE_TASKS/$TOTAL_TASKS tasks)          │"
echo "│                                                                    │"
echo "│  Tech Stack                                                        │"
echo "│  ┌────────────────────────────────────────────────────────────┐    │"
echo "│  │  Language: $TECH_STACK                                             │    │"
echo "│  └────────────────────────────────────────────────────────────┘    │"
echo "│                                                                    │"

if [ "$BLOCKED" = "true" ]; then
    echo "│  Blockers                                                          │"
    echo "│  ┌────────────────────────────────────────────────────────────┐    │"
    echo "│  │  🚧 Ralph Loop is BLOCKED - check .ralph/scratchpad.md     │    │"
    echo "│  └────────────────────────────────────────────────────────────┘    │"
    echo "│                                                                    │"
fi

echo "└────────────────────────────────────────────────────────────────────┘"
echo ""
