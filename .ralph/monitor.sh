#!/bin/bash

# Ralph Loop Monitor for .ralph/ Structure
# Real-time dashboard for tracking loop status
# Supports visual mode (default) and JSON streaming mode (--json)

set -e

# Script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

# .ralph directory structure
RALPH_DIR="$PROJECT_ROOT/.ralph"
STATE_FILE="$RALPH_DIR/state.json"
RALPH_YML="$RALPH_DIR/ralph.yml"
SESSION_FILE="$RALPH_DIR/session.md"
HISTORY_DIR="$RALPH_DIR/history"

# Configuration
REFRESH_RATE=2
OUTPUT_MODE="visual"  # visual | json

# Colors (only used in visual mode)
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
BOLD='\033[1m'
NC='\033[0m'

# Get state value
get_state() {
    local val=$(jq -r ".$1" "$STATE_FILE" 2>/dev/null | tr -d '\r')
    if [[ "$val" == "null" ]] || [[ -z "$val" ]]; then
        echo "0"
    else
        echo "$val"
    fi
}

# Format timestamp
format_timestamp() {
    local ts=$1
    if [[ "$ts" != "null" ]] && [[ -n "$ts" ]]; then
        date -d "$ts" '+%Y-%m-%d %H:%M:%S' 2>/dev/null || echo "$ts"
    else
        echo "N/A"
    fi
}

# Output JSONL line
output_json() {
    local data=$1
    echo "$data"
}

# Build and output JSON state (for JSON mode)
output_json_state() {
    #local iteration=$(get_state "loop.iteration")
    #local max_iterations=$(get_state "loop.max_iterations")
    local iteration=$((10#$(get_state "loop.iteration" | tr -d '\r' || echo 0)))
    local max_iterations=$((10#$(get_state "loop.max_iterations" | tr -d '\r' || echo 0)))
    local state=$(get_state "loop.state")
    local last_status=$(get_state "loop.last_status")
    local last_run=$(get_state "loop.last_run")
    local session_id=$(get_state "session.session_id")
    local current_task=$(get_state "tasks.current_task_id")
    local total_tasks=$(get_state "tasks.total_tasks")
    local completed_tasks=$(grep -c "status: done" "$RALPH_YML" 2>/dev/null || echo "0")
    local completion_pct=$(get_state "tasks.completion_percentage")
    local cb_state=$(get_state "circuit_breaker.state")
    local no_progress=$(get_state "circuit_breaker.consecutive_no_progress")
    local same_error=$(get_state "circuit_breaker.consecutive_same_error")
    #local calls_this_hour=$(get_state "rate_limit.calls_this_hour")
    local calls_this_hour=$((10#$(get_state "rate_limit.calls_this_hour" | tr -d '\r' || echo 0)))
    local max_calls=$((10#$(get_state "rate_limit.max_calls_per_hour" | tr -d '\r' || echo 0)))
    #local max_calls=$(get_state "rate_limit.max_calls_per_hour")
    local test_loops=$(get_state "exit_conditions.test_only_loops")
    local done_signals=$(get_state "exit_conditions.done_signals")
    local completion_indicators=$(get_state "exit_conditions.completion_indicators")


# Safe Progress Calculation
local loop_progress=0
if (( max_iterations > 0 )); then
    loop_progress=$(( iteration * 100 / max_iterations ))
fi

# Safe Rate Limit Calculation
local remaining_calls=0
if (( max_calls > 0 )); then
    remaining_calls=$(( max_calls - calls_this_hour ))
fi

    # Get project info
    local project_name=$(jq -r '.project.name // "Unknown"' "$RALPH_YML" 2>/dev/null || echo "Unknown")
    local project_version=$(jq -r '.project.version // "0.0.0"' "$RALPH_YML" 2>/dev/null || echo "0.0.0")

    # Get current task title
    local task_title="None"
    if [[ "$current_task" != "null" ]] && [[ -n "$current_task" ]]; then
        task_title=$(awk "/- id: \"$current_task\"/,/^  - id:/" "$RALPH_YML" 2>/dev/null | grep "title:" | cut -d':' -f2- | xargs || echo "Unknown")
    fi

    # Build JSON
    local timestamp=$(date -u +%Y-%m-%dT%H:%M:%SZ)

    cat << EOF
{
  "timestamp": "$timestamp",
  "project": {
    "name": "$project_name",
    "version": "$project_version"
  },
  "loop": {
    "iteration": $iteration,
    "max_iterations": $max_iterations,
    "state": "$state",
    "last_status": "$last_status",
    "last_run": "$last_run",
    "progress_percent": $((iteration * 100 / max_iterations))
  },
  "session": {
    "session_id": "$session_id",
    "current_task": "$current_task",
    "current_task_title": "$task_title"
  },
  "tasks": {
    "total": $total_tasks,
    "completed": $completed_tasks,
    "completion_percent": $completion_pct
  },
  "circuit_breaker": {
    "state": "$cb_state",
    "consecutive_no_progress": $no_progress,
    "consecutive_same_error": $same_error
  },
  "rate_limit": {
    "calls_this_hour": $calls_this_hour,
    "max_calls_per_hour": $max_calls,
    "remaining": $((max_calls - calls_this_hour))
  },
  "exit_conditions": {
    "test_only_loops": $test_loops,
    "done_signals": $done_signals,
    "completion_indicators": $completion_indicators
  }
}
EOF
}

# Draw header (visual mode only)
draw_header() {
    local project_name=$(jq -r '.project.name // "Unknown Project"' "$RALPH_YML" 2>/dev/null || echo "Unknown Project")
    local version=$(jq -r '.project.version // "0.1.0"' "$RALPH_YML" 2>/dev/null || echo "0.1.0")

    echo -e "${CYAN}${BOLD}╔════════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${CYAN}${BOLD}║${NC} ${BOLD}Ralph Loop Monitor${NC}                                              ${CYAN}${BOLD}║${NC}"
    echo -e "${CYAN}${BOLD}║${NC} Project: ${BOLD}$project_name${NC} v${version}                        ${CYAN}${BOLD}║${NC}"
    echo -e "${CYAN}${BOLD}╠════════════════════════════════════════════════════════════════╣${NC}"
}

# Draw footer (visual mode only)
draw_footer() {
    echo -e "${CYAN}${BOLD}╠════════════════════════════════════════════════════════════════╣${NC}"
    echo -e "${CYAN}${BOLD}║${NC} Press ${BOLD}Ctrl+C${NC} to exit monitor                                 ${CYAN}${BOLD}║${NC}"
    echo -e "${CYAN}${BOLD}╚════════════════════════════════════════════════════════════════╝${NC}"
}

# Draw loop status section (visual mode only)
draw_loop_status() {
    local iteration=$(get_state "loop.iteration")
    local max_iterations=$(get_state "loop.max_iterations")
    local state=$(get_state "loop.state")
    local last_status=$(get_state "loop.last_status")
    local last_run=$(get_state "loop.last_run")

    # Calculate progress bar
    local progress=$((iteration * 100 / max_iterations))
    local bar_width=40
    local filled=$((progress * bar_width / 100))
    local empty=$((bar_width - filled))
    local bar=""
    for ((i=0; i<filled; i++)); do bar+="█"; done
    for ((i=0; i<empty; i++)); do bar+="░"; done

    # State color
    local state_color=$GREEN
    if [[ "$state" == "error" ]] || [[ "$state" == "circuit_open" ]]; then
        state_color=$RED
    elif [[ "$state" == "running" ]]; then
        state_color=$YELLOW
    fi

    echo -e "${CYAN}${BOLD}║${NC} ${BOLD}Loop Status${NC}"
    echo -e "${CYAN}${BOLD}║${NC}   State: ${state_color}${BOLD}${state}${NC}"
    echo -e "${CYAN}${BOLD}║${NC}   Iteration: ${BOLD}${iteration}${NC} / ${max_iterations} (${progress}%)"
    echo -e "${CYAN}${BOLD}║${NC}   Progress: [${GREEN}${bar}${NC}]"
    echo -e "${CYAN}${BOLD}║${NC}   Last Status: ${last_status}"
    echo -e "${CYAN}${BOLD}║${NC}   Last Run: $(format_timestamp "$last_run")"
    echo -e "${CYAN}${BOLD}║${NC}"
}

# Draw session info section (visual mode only)
draw_session_info() {
    local session_id=$(get_state "session.session_id")
    local started=$(get_state "session.started_at")
    local last_activity=$(get_state "session.last_activity")
    local current_task=$(get_state "tasks.current_task_id")

    echo -e "${CYAN}${BOLD}║${NC} ${BOLD}Session Info${NC}"
    echo -e "${CYAN}${BOLD}║${NC}   Session ID: ${session_id:0:20}..."
    echo -e "${CYAN}${BOLD}║${NC}   Started: $(format_timestamp "$started")"
    echo -e "${CYAN}${BOLD}║${NC}   Last Activity: $(format_timestamp "$last_activity")"
    echo -e "${CYAN}${BOLD}║${NC}   Current Task: ${current_task}"
    echo -e "${CYAN}${BOLD}║${NC}"
}

# Draw task progress section (visual mode only)
draw_task_progress() {
    local total_tasks=$(get_state "tasks.total_tasks")
    local completed_tasks=$(grep -c "status: done" "$RALPH_YML" 2>/dev/null || echo "0")
    local completion_pct=$(get_state "tasks.completion_percentage")
    local todo_tasks=$(grep -c "status: todo" "$RALPH_YML" 2>/dev/null || echo "0")
    local in_progress_tasks=$(grep -c "status: in-progress" "$RALPH_YML" 2>/dev/null || echo "0")

    echo -e "${CYAN}${BOLD}║${NC} ${BOLD}Task Progress${NC}"
    echo -e "${CYAN}${BOLD}║${NC}   Completion: ${BOLD}${completed_tasks}${NC} / ${total_tasks} tasks (${completion_pct}%)"
    echo -e "${CYAN}${BOLD}║${NC}   To Do: ${YELLOW}${todo_tasks}${NC} | In Progress: ${YELLOW}${in_progress_tasks}${NC} | Done: ${GREEN}${completed_tasks}${NC}"
    echo -e "${CYAN}${BOLD}║${NC}"
}

# Draw circuit breaker section (visual mode only)
draw_circuit_breaker() {
    local cb_state=$(get_state "circuit_breaker.state")
    local no_progress=$(get_state "circuit_breaker.consecutive_no_progress")
    local same_error=$(get_state "circuit_breaker.consecutive_same_error")
    local total_opens=$(get_state "circuit_breaker.total_opens")

    local cb_color=$GREEN
    if [[ "$cb_state" == "OPEN" ]]; then
        cb_color=$RED
    elif [[ "$cb_state" == "HALF_OPEN" ]]; then
        cb_color=$YELLOW
    fi

    echo -e "${CYAN}${BOLD}║${NC} ${BOLD}Circuit Breaker${NC}"
    echo -e "${CYAN}${BOLD}║${NC}   State: ${cb_color}${BOLD}${cb_state}${NC}"
    echo -e "${CYAN}${BOLD}║${NC}   No Progress: ${no_progress} | Same Error: ${same_error}"
    echo -e "${CYAN}${BOLD}║${NC}   Total Opens: ${total_opens}"
    echo -e "${CYAN}${BOLD}║${NC}"
}

# Draw rate limit section (visual mode only)
draw_rate_limit() {
    local calls_this_hour=$((10#$(get_state "rate_limit.calls_this_hour" | tr -d '\r' || echo 0)))
    local max_calls=$((10#$(get_state "rate_limit.max_calls_per_hour" | tr -d '\r' || echo 0)))
    
    local pct=0
    local remaining=0
    
    if (( max_calls > 0 )); then
        pct=$(( (calls_this_hour * 100) / max_calls ))
        remaining=$(( max_calls - calls_this_hour ))
    fi
    # ... rest of the function
}

# Draw exit conditions section (visual mode only)
draw_exit_conditions() {
    local test_loops=$(get_state "exit_conditions.test_only_loops")
    local done_signals=$(get_state "exit_conditions.done_signals")
    local completion_indicators=$(get_state "exit_conditions.completion_indicators")

    echo -e "${CYAN}${BOLD}║${NC} ${BOLD}Exit Conditions${NC}"
    echo -e "${CYAN}${BOLD}║${NC}   Test-Only Loops: ${test_loops} / 3"
    echo -e "${CYAN}${BOLD}║${NC}   Done Signals: ${done_signals} / 2"
    echo -e "${CYAN}${BOLD}║${NC}   Completion Indicators: ${completion_indicators} / 2"
    echo -e "${CYAN}${BOLD}║${NC}"
}

# Draw current task section (visual mode only)
draw_current_task() {
    local current_task=$(get_state "tasks.current_task_id")

    echo -e "${CYAN}${BOLD}║${NC} ${BOLD}Current Task${NC}"

    if [[ "$current_task" != "null" ]] && [[ -n "$current_task" ]]; then
        # Extract task info from ralph.yml
        local task_info=$(awk "/- id: \"$current_task\"/,/^  - id:/" "$RALPH_YML" | head -10)
        local task_title=$(echo "$task_info" | grep "title:" | cut -d':' -f2- | xargs || echo "Unknown")
        local task_status=$(echo "$task_info" | grep "status:" | cut -d':' -f2- | xargs || echo "todo")

        local status_color=$YELLOW
        [[ "$task_status" == "done" ]] && status_color=$GREEN
        [[ "$task_status" == "blocked" ]] && status_color=$RED

        echo -e "${CYAN}${BOLD}║${NC}   ID: ${current_task}"
        echo -e "${CYAN}${BOLD}║${NC}   Title: ${task_title}"
        echo -e "${CYAN}${BOLD}║${NC}   Status: ${status_color}${task_status}${NC}"
    else
        echo -e "${CYAN}${BOLD}║${NC}   No active task"
    fi
    echo -e "${CYAN}${BOLD}║${NC}"
}

# Draw recent activity section (visual mode only)
draw_recent_activity() {
    echo -e "${CYAN}${BOLD}║${NC} ${BOLD}Recent Activity (Last 5)${NC}"

    local iteration=$(get_state "loop.iteration")
    for ((i=$((iteration - 5)); i<=iteration; i++)); do
        if [[ $i -gt 0 ]]; then
            local output_file="$HISTORY_DIR/claude_output_${i}.json"
            if [[ -f "$output_file" ]]; then
                local status=$(jq -r '.status // "UNKNOWN"' "$output_file" 2>/dev/null || echo "UNKNOWN")
                local work_type=$(jq -r '.work_type // "UNKNOWN"' "$output_file" 2>/dev/null || echo "UNKNOWN")
                local files_modified=$(jq -r '.files_modified // 0' "$output_file" 2>/dev/null || echo "0")

                local status_color=$GREEN
                [[ "$status" == "ERROR" ]] && status_color=$RED
                [[ "$status" == "IN_PROGRESS" ]] && status_color=$YELLOW

                echo -e "${CYAN}${BOLD}║${NC}   [${i}] ${status_color}${status}${NC} | ${work_type} | ${files_modified} files"
            fi
        fi
    done
    echo -e "${CYAN}${BOLD}║${NC}"
}

# Main monitoring loop - JSON mode
main_json() {
    # Output initial state
    output_json_state

    # Watch for state changes and output
    local last_iteration=$(get_state "loop.iteration")

    while true; do
        sleep "$REFRESH_RATE"

        local current_iteration=$(get_state "loop.iteration")

        # Output on change or every 5 iterations
        if [[ $current_iteration -ne $last_iteration ]] || [[ $((current_iteration % 5)) -eq 0 ]]; then
            output_json_state
            last_iteration=$current_iteration
        fi
    done
}

# Main monitoring loop - Visual mode
main_visual() {
    # Setup visual mode
    clear
    tput civis 2>/dev/null || true
    trap 'tput cnorm 2>/dev/null || true; echo' EXIT

    while true; do
        # Clear screen
        clear 2>/dev/null || printf "\033c"

        # Draw all sections
        draw_header
        draw_loop_status
        draw_session_info
        draw_task_progress
        draw_circuit_breaker
        draw_rate_limit
        draw_exit_conditions
        draw_current_task
        draw_recent_activity
        draw_footer

        # Wait before refresh
        sleep "$REFRESH_RATE"
    done
}

# Main monitoring loop
main() {
    if [[ "$OUTPUT_MODE" == "json" ]]; then
        main_json
    else
        main_visual
    fi
}

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --refresh|-r)
            REFRESH_RATE="$2"
            shift 2
            ;;
        --json)
            OUTPUT_MODE="json"
            shift
            ;;
        --visual)
            OUTPUT_MODE="visual"
            shift
            ;;
        --help|-h)
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Ralph Loop Monitor - Track loop status in real-time"
            echo ""
            echo "Options:"
            echo "  --refresh, -r SECONDS   Set refresh rate (default: 2)"
            echo "  --json                  Output JSON Lines (JSONL) for piping/automation"
            echo "  --visual                Visual terminal dashboard (default)"
            echo "  --help, -h             Show this help"
            echo ""
            echo "Examples:"
            echo "  $0                      # Visual dashboard (default)"
            echo "  $0 --json              # JSON streaming output"
            echo "  $0 --json --refresh 1   # JSON output, 1 second refresh"
            echo "  $0 --refresh 5         # Visual dashboard, 5 second refresh"
            echo ""
            echo "JSON streaming can be piped to other tools:"
            echo "  $0 --json | jq '.loop.iteration'"
            echo "  $0 --json > monitor.log"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

# Run main
main
