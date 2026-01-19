#!/bin/bash

# Ralph Loop for .ralph/ Structure
# Autonomous AI development loop with intelligent exit detection

set -e

# Script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

# .ralph directory structure
RALPH_DIR="$PROJECT_ROOT/.ralph"
STATE_FILE="$RALPH_DIR/state.json"
RALPH_YML="$RALPH_DIR/ralph.yml"
SESSION_FILE="$RALPH_DIR/session.md"
SCRATCHPAD_FILE="$RALPH_DIR/scratchpad.md"
PROMPT_FILE="$RALPH_DIR/prompt.md"
CLAUDE_MD="$PROJECT_ROOT/CLAUDE.md"
SPEC_MD="$PROJECT_ROOT/spec.md"

# Subdirectories
CHECKPOINTS_DIR="$RALPH_DIR/checkpoints"
HISTORY_DIR="$RALPH_DIR/history"
SESSIONS_DIR="$RALPH_DIR/sessions"

# Claude Code CLI configuration
#CLAUDE_CMD="claude"
CLAUDE_CMD="/c/Users/Kimpa/AppData/Roaming/npm/claude.cmd"
CLAUDE_OUTPUT_FORMAT="json"
CLAUDE_TIMEOUT_MINUTES=15
MAX_CALLS_PER_HOUR=100

# Permission handling (env: RALPH_SKIP_PERMISSIONS=true)
# When true, adds --dangerously-skip-permissions flag (bypass tool confirmation)
#RALPH_SKIP_PERMISSIONS="${RALPH_SKIP_PERMISSIONS:-false}"
RALPH_SKIP_PERMISSIONS=true

# Exit detection configuration
MAX_CONSECUTIVE_TEST_LOOPS=3
MAX_CONSECUTIVE_DONE_SIGNALS=2
TEST_PERCENTAGE_THRESHOLD=30

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
NC='\033[0m'

# Log function
log_status() {
    local level=$1
    local message=$2
    local timestamp=$(date '+%Y-%m-%d %H:%M:%S')
    local color=""

    case $level in
        "INFO")  color=$BLUE ;;
        "WARN")  color=$YELLOW ;;
        "ERROR") color=$RED ;;
        "SUCCESS") color=$GREEN ;;
        "LOOP") color=$PURPLE ;;
    esac

    echo -e "${color}[$timestamp] [$level] $message${NC}"

    # Log to history
    local log_file="$HISTORY_DIR/ralph-$(date +%Y%m%d).log"
    echo "[$timestamp] [$level] $message" >> "$log_file"
}

# Initialize .ralph directory structure
init_ralph_structure() {
    log_status "INFO" "Initializing .ralph directory structure..."

    mkdir -p "$CHECKPOINTS_DIR"
    mkdir -p "$HISTORY_DIR"
    mkdir -p "$SESSIONS_DIR"

    # Initialize state.json if it doesn't exist
    if [[ ! -f "$STATE_FILE" ]]; then
        cat > "$STATE_FILE" << EOF
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
    "max_calls_per_hour": $MAX_CALLS_PER_HOUR,
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
        log_status "SUCCESS" "Created state.json"
    fi

    # Check for required files
    if [[ ! -f "$RALPH_YML" ]]; then
        log_status "ERROR" "ralph.yml not found in $RALPH_DIR"
        log_status "INFO" "Please create ralph.yml with your project tasks"
        exit 1
    fi

    if [[ ! -f "$CLAUDE_MD" ]]; then
        log_status "WARN" "CLAUDE.md not found in project root"
    fi

    if [[ ! -f "$SPEC_MD" ]]; then
        log_status "WARN" "spec.md not found in project root"
    fi
}

# Update state.json field
update_state() {
    local field=$1
    local value=$2

    # Handle nested keys like "loop.iteration"
    local jq_query=".$field = $value"

    jq "$jq_query" "$STATE_FILE" > "${STATE_FILE}.tmp"
    mv "${STATE_FILE}.tmp" "$STATE_FILE"
}

# Get state value
get_state() {
    local field=$1
    jq -r ".$field" "$STATE_FILE" 2>/dev/null || echo "null"
}

# Start new session
start_session() {
    local session_id="ralph_$(date +%s)_$RANDOM"
    local now=$(date -u +%Y-%m-%dT%H:%M:%SZ)

    update_state "session.session_id" "\"$session_id\""
    update_state "session.started_at" "\"$now\""
    update_state "session.last_activity" "\"$now\""

    # Calculate expiration (24 hours from now)
    local expires=$(date -d "+24 hours" -u +%Y-%m-%dT%H:%M:%SZ 2>/dev/null || date -u -v+24H +%Y-%m-%dT%H:%M:%SZ)
    update_state "session.expires_at" "\"$expires\""

    log_status "SUCCESS" "Started session: $session_id"

    # Create session log file
    local session_log="$SESSIONS_DIR/${session_id}.md"
    cat > "$session_log" << EOF
# Ralph Session: $session_id

**Started**: $now
**Status**: active

---

## Session Log

EOF

    echo "$session_id"
}

# Update session activity
update_session_activity() {
    local now=$(date -u +%Y-%m-%dT%H:%M:%SZ)
    update_state "session.last_activity" "\"$now\""
}

# Parse RALPH_STATUS block from output
parse_ralph_status() {
    local output_file=$1
    local status_file="${2:-.ralph_status_result}"

    # Extract RALPH_STATUS block
    sed -n '/---RALPH_STATUS---/,/--END_RALPH_STATUS---/p' "$output_file" > "$status_file.tmp"

    # Parse fields
    local status=$(grep "^STATUS:" "$status_file.tmp" | cut -d':' -f2- | xargs)
    local current_task=$(grep "^CURRENT_TASK:" "$status_file.tmp" | cut -d':' -f2- | xargs)
    local tasks_completed=$(grep "^TASKS_COMPLETED_THIS_LOOP:" "$status_file.tmp" | cut -d':' -f2- | xargs || echo "0")
    local files_modified=$(grep "^FILES_MODIFIED:" "$status_file.tmp" | cut -d':' -f2- | xargs || echo "0")
    local tests_status=$(grep "^TESTS_STATUS:" "$status_file.tmp" | cut -d':' -f2- | xargs)
    local work_type=$(grep "^WORK_TYPE:" "$status_file.tmp" | cut -d':' -f2- | xargs)
    local exit_signal=$(grep "^EXIT_SIGNAL:" "$status_file.tmp" | cut -d':' -f2- | xargs || echo "false")
    local recommendation=$(grep "^RECOMMENDATION:" "$status_file.tmp" | cut -d':' -f2- | xargs)

    # Create JSON result
    cat > "$status_file" << EOF
{
  "status": "$status",
  "current_task": "$current_task",
  "tasks_completed": $tasks_completed,
  "files_modified": $files_modified,
  "tests_status": "$tests_status",
  "work_type": "$work_type",
  "exit_signal": $exit_signal,
  "recommendation": "$recommendation"
}
EOF

    rm -f "$status_file.tmp"

    # Return exit_signal as boolean
    [[ "$exit_signal" == "true" ]] && return 0 || return 1
}

# Check exit conditions
check_exit_conditions() {
    local iteration=$(get_state "loop.iteration")
    local exit_signal_count=$(get_state "exit_conditions.completion_indicators")
    local test_only_loops=$(get_state "exit_conditions.test_only_loops")
    local done_signals=$(get_state "exit_conditions.done_signals")

    # Check for explicit exit signal
    if [[ $exit_signal_count -ge 2 ]]; then
        log_status "SUCCESS" "Exit signal threshold reached"
        return 0
    fi

    # Check for too many test-only loops
    if [[ $test_only_loops -ge $MAX_CONSECUTIVE_TEST_LOOPS ]]; then
        log_status "SUCCESS" "All features implemented (test-only threshold reached)"
        return 0
    fi

    # Check for consecutive done signals
    if [[ $done_signals -ge $MAX_CONSECUTIVE_DONE_SIGNALS ]]; then
        log_status "SUCCESS" "Consecutive done signals threshold reached"
        return 0
    fi

    return 1
}

# Build prompt for Claude
build_prompt() {
    local iteration=$1
    local session_id=$2

    # 1. Get raw counts and strip any whitespace/non-numeric characters
    local total_tasks=$(grep -c "^  - id:" "$RALPH_YML" 2>/dev/null | tr -d '\r' || echo "0")
    local completed_tasks=$(grep -c "status: done" "$RALPH_YML" 2>/dev/null | tr -d '\r' || echo "0")

    # 2. Ensure they are treated as base-10 numbers (fixes '08' octal or empty string issues)
    total_tasks=$((10#${total_tasks:-0}))
    completed_tasks=$((10#${completed_tasks:-0}))

    # 3. Safe Math: Check for zero before dividing
    local completion_pct=0
    if (( total_tasks > 0 )); then
        completion_pct=$(( (completed_tasks * 100) / total_tasks ))
    fi

    # 4. Update state (ensure the JSON gets a number, not a string)
    update_state "tasks.total_tasks" "$total_tasks"
    update_state "tasks.completion_percentage" "$completion_pct"
    # 5. Substitute variables safely
    # Note: Using 'sed' for the project name because jq cannot read YAML files directly
    local project_name=$(grep "name:" "$RALPH_YML" | head -1 | cut -d':' -f2 | xargs || echo "Project")
    
    prompt_content="${prompt_content//\{\{PROJECT_NAME\}\}/$project_name}"
    prompt_content="${prompt_content//\{\{ITERATION\}\}/$iteration}"
    prompt_content="${prompt_content//\{\{SESSION_ID\}\}/$session_id}"
    prompt_content="${prompt_content//\{\{COMPLETION_PERCENTAGE\}\}/$completion_pct}"

    # 6. Output the final prompt
    cat << EOF
$prompt_content

---

## Current Loop Context

- **Iteration**: ${iteration:-0}
- **Session**: ${session_id:-none}
- **Completion**: $completed_tasks/$total_tasks tasks ($completion_pct%)
- **Current Task**:
$current_task

---

## Previous Scratchpad Notes

$( [ -f "$SCRATCHPAD_FILE" ] && tail -50 "$SCRATCHPAD_FILE" || echo "No previous notes" )

---

Begin your work on the highest priority task from ralph.yml.
EOF
}

# Execute Claude Code
execute_claude() {
    local prompt_file=$1
    local iteration=$2

    log_status "INFO" "Executing Claude Code (iteration $iteration)..."

    local output_file="$HISTORY_DIR/claude_output_${iteration}.json"
    
    # Read the prompt into a variable first to ensure it's clean
    local prompt_text=$(cat "$prompt_file")

    # Build Claude command args
    local claude_args=()
    #claude_args+=("code") # Ensure the 'code' subcommand is present
    claude_args+=("--dangerously-skip-permissions")
    claude_args+=("--output-format json")
    claude_args+=("-p" "$prompt_text")
    # Add skip permissions flag if env is set
    #if [[ "$RALPH_SKIP_PERMISSIONS" == "true" ]]; then
    #    claude_args+=("--dangerously-skip-permissions")
    #fi

    # EXECUTION:
    # 1. We use winpty because it's a Windows .cmd/binary
    # 2. We use 'stdbuf' or just direct execution to avoid "empty file" buffering
    # 3. We wrap in parentheses to capture output correctly
    
    #winpty "$CLAUDE_CMD" "${claude_args[@]}" > "$output_file" 2>&1
    cat "$prompt_file" | "$CLAUDE_CMD" --dangerously-skip-permissions -p "Follow instructions and start working!" > "$output_file" 2>&1

    #log_status "claude: $CLAUDE_CMD"
    #log_status "args: ${claude_args[@]}"
    #log_status "output file: $output_file"
    local exit_code=$?

    # Check for errors
    if [[ $exit_code -ne 0 ]]; then
        log_status "ERROR" "Claude Code failed with exit code $exit_code"
        # If the output file is empty, it might be a winpty redirection issue
        if [[ ! -s "$output_file" ]]; then
             log_status "WARN" "Output file is empty. Check if winpty is blocking the stream."
        fi
        return 1
    fi

    # Update rate limit
    local calls_this_hour=$(($(get_state "rate_limit.calls_this_hour") + 1))
    update_state "rate_limit.calls_this_hour" "$calls_this_hour"

    return 0
}

# Main loop
main() {
    log_status "LOOP" "=== Ralph Loop Starting ==="
    log_status "INFO" "Project root: $PROJECT_ROOT"
    log_status "INFO" ".ralph directory: $RALPH_DIR"

    # Initialize structure
    #init_ralph_structure

    # Start session
    local session_id=$(start_session)

    # Get current iteration
    local iteration=$(get_state "loop.iteration")
    local max_iterations=$(get_state "loop.max_iterations")

    log_status "INFO" "Starting from iteration: $iteration"
    log_status "INFO" "Max iterations: $max_iterations"

    # Main loop
    while [[ $iteration -lt $max_iterations ]]; do
        iteration=$((iteration + 1))
        update_state "loop.iteration" "$iteration"
        update_state "loop.state" "\"running\""

        log_status "LOOP" "--- Iteration $iteration ---"

        # Check rate limit
        local calls_this_hour=$(get_state "rate_limit.calls_this_hour")
        if [[ $calls_this_hour -ge $MAX_CALLS_PER_HOUR ]]; then
            log_status "WARN" "Rate limit reached ($calls_this_hour/$MAX_CALLS_PER_HOUR)"
            log_status "INFO" "Waiting for hour reset..."
            sleep 3600
        fi

        # Build prompt
        local prompt_file="$HISTORY_DIR/prompt_${iteration}.md"
        build_prompt "$iteration" "$session_id" > "$prompt_file"

        # Execute Claude
        if ! execute_claude "$prompt_file" "$iteration"; then
            log_status "ERROR" "Claude execution failed"
            update_state "loop.last_status" "\"error\""
            break
        fi

        # Parse output
        local output_file="$HISTORY_DIR/claude_output_${iteration}.json"
        if parse_ralph_status "$output_file"; then
            # Exit signal is true
            local count=$(($(get_state "exit_conditions.completion_indicators") + 1))
            update_state "exit_conditions.completion_indicators" "$count"
            log_status "INFO" "Exit signal received ($count/2)"
        fi

        # Update session activity
        update_session_activity

        # Check exit conditions
        if check_exit_conditions; then
            log_status "SUCCESS" "Exit conditions met, stopping loop"
            update_state "loop.state" "\"complete\""
            update_state "loop.last_status" "\"success\""
            break
        fi

        # Check circuit breaker
        local cb_state=$(get_state "circuit_breaker.state")
        if [[ "$cb_state" == "OPEN" ]]; then
            log_status "ERROR" "Circuit breaker is OPEN, stopping loop"
            update_state "loop.last_status" "\"circuit_open\""
            break
        fi

        # Small delay between iterations
        sleep 2
    done

    # Finalize
    log_status "LOOP" "=== Ralph Loop Complete ==="
    log_status "INFO" "Total iterations: $iteration"

    # Create checkpoint
    local checkpoint_file="$CHECKPOINTS_DIR/checkpoint_$(date +%Y%m%d_%H%M%S).txt"
    cat > "$checkpoint_file" << EOF
Ralph Loop Checkpoint
=====================
Time: $(date -u +%Y-%m-%dT%H:%M:%SZ)
Iteration: $iteration
Session: $session_id
State: $(get_state "loop.state")

Project Status:
- Tasks Completed: $(get_state "tasks.completed_tasks | length")
- Completion: $(get_state "tasks.completion_percentage")%
- Files Modified: [count from git]

Next Steps:
$(tail -20 "$SCRATCHPAD_FILE" 2>/dev/null || echo "See scratchpad.md")
EOF

    log_status "SUCCESS" "Checkpoint created: $checkpoint_file"
}

# Run main
main "$@"
