#!/bin/bash

# Response Analyzer for .ralph/ Structure
# Analyzes Claude Code output and updates state accordingly

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
CLAUDE_MD="$PROJECT_ROOT/CLAUDE.md"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
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
    esac

    echo -e "${color}[$timestamp] [$level] $message${NC}"
}

# Get state value
get_state() {
    local field=$1
    jq -r ".$field" "$STATE_FILE" 2>/dev/null || echo "null"
}

# Update state.json field
update_state() {
    local field=$1
    local value=$2
    local jq_query=".$field = $value"

    jq "$jq_query" "$STATE_FILE" > "${STATE_FILE}.tmp"
    mv "${STATE_FILE}.tmp" "$STATE_FILE"
}

# Parse RALPH_STATUS block from Claude output
parse_ralph_status() {
    local output_file=$1
    local result_file="${2:-.ralph_analysis_result}"

    # Check if file exists
    if [[ ! -f "$output_file" ]]; then
        log_status "ERROR" "Output file not found: $output_file"
        return 1
    fi

    # Extract RALPH_STATUS block
    sed -n '/---RALPH_STATUS---/,/--END_RALPH_STATUS---/p' "$output_file" > "$result_file.tmp" 2>/dev/null

    # Check if we found the block
    if [[ ! -s "$result_file.tmp" ]]; then
        log_status "WARN" "No RALPH_STATUS block found in output"
        rm -f "$result_file.tmp"
        return 1
    fi

    # Parse fields from the status block
    local status=$(grep "^STATUS:" "$result_file.tmp" | cut -d':' -f2- | xargs || echo "UNKNOWN")
    local current_task=$(grep "^CURRENT_TASK:" "$result_file.tmp" | cut -d':' -f2- | xargs || echo "none")
    local tasks_completed=$(grep "^TASKS_COMPLETED_THIS_LOOP:" "$result_file.tmp" | cut -d':' -f2- | xargs || echo "0")
    local files_modified=$(grep "^FILES_MODIFIED:" "$result_file.tmp" | cut -d':' -f2- | xargs || echo "0")
    local tests_status=$(grep "^TESTS_STATUS:" "$result_file.tmp" | cut -d':' -f2- | xargs || echo "NOT_RUN")
    local work_type=$(grep "^WORK_TYPE:" "$result_file.tmp" | cut -d':' -f2- | xargs || echo "UNKNOWN")
    local exit_signal=$(grep "^EXIT_SIGNAL:" "$result_file.tmp" | cut -d':' -f2- | xargs || echo "false")
    local recommendation=$(grep "^RECOMMENDATION:" "$result_file.tmp" | cut -d':' -f2- | xargs || echo "No recommendation")

    # Create JSON result
    cat > "$result_file" << EOF
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

    rm -f "$result_file.tmp"

    log_status "INFO" "Parsed RALPH_STATUS: $status | Work: $work_type | Files: $files_modified | Exit: $exit_signal"

    # Return exit signal as exit code
    [[ "$exit_signal" == "true" ]] && return 0 || return 1
}

# Analyze Claude output for patterns
analyze_output_patterns() {
    local output_file=$1

    # Detect test-only loop
    local has_test_commands=$(grep -cE "(npm test|cargo test|pytest|bats|jest|go test)" "$output_file" 2>/dev/null || echo "0")
    local has_file_ops=$(grep -cE "(Write|Edit|Read)" "$output_file" 2>/dev/null || echo "0")

    if [[ $has_test_commands -gt 0 ]] && [[ $has_file_ops -eq 0 ]]; then
        local count=$(($(get_state "exit_conditions.test_only_loops") + 1))
        update_state "exit_conditions.test_only_loops" "$count"
        log_status "INFO" "Test-only loop detected ($count/$MAX_CONSECUTIVE_TEST_LOOPS)"
    else
        # Reset counter if not test-only
        update_state "exit_conditions.test_only_loops" "0"
    fi

    # Detect completion indicators
    local completion_keywords=("done" "complete" "finished" "all tasks complete" "project complete" "ready for review")
    local completion_count=0

    for keyword in "${completion_keywords[@]}"; do
        if grep -qi "$keyword" "$output_file" 2>/dev/null; then
            completion_count=$((completion_count + 1))
        fi
    done

    if [[ $completion_count -ge 2 ]]; then
        local current=$(get_state "exit_conditions.completion_indicators")
        update_state "exit_conditions.completion_indicators" "$((current + 1))"
        log_status "INFO" "Completion indicators found ($completion_count keywords)"
    fi

    # Detect done signals
    if grep -qiE "(STATUS: COMPLETE|EXIT_SIGNAL: true)" "$output_file" 2>/dev/null; then
        local count=$(($(get_state "exit_conditions.done_signals") + 1))
        update_state "exit_conditions.done_signals" "$count"
        log_status "INFO" "Done signal detected ($count/$MAX_CONSECUTIVE_DONE_SIGNALS)"
    fi
}

# Check for errors in output
check_for_errors() {
    local output_file=$1

    # Filter out JSON field false positives
    local error_count=$(grep -v '"[^"]*error[^"]*":' "$output_file" 2>/dev/null | \
                        grep -cE '(^Error:|^ERROR:|^error:|\]: error|Link: error|Error occurred|failed with error|Exception|Fatal|FATAL)' || echo "0")

    if [[ $error_count -gt 0 ]]; then
        log_status "WARN" "Errors detected in output: $error_count errors"
        # Track for circuit breaker
        return 1
    fi

    return 0
}

# Update ralph.yml task status
update_task_status() {
    local task_id=$1
    local new_status=$2

    if [[ -z "$task_id" ]] || [[ "$task_id" == "none" ]]; then
        return
    fi

    # Use sed to update the task status in ralph.yml
    # This handles multiline YAML properly
    local temp_file="${RALPH_YML}.tmp"

    awk -v task_id="$task_id" -v new_status="$new_status" '
    BEGIN { in_task = 0; found = 0 }
    /^  - id:/ {
        if ($0 == "  - id: \"" task_id "\"") {
            in_task = 1
        } else {
            in_task = 0
        }
    }
    in_task && /^    status:/ {
        if (!found) {
            print "    status: \"" new_status "\""
            found = 1
            next
        }
    }
    { print }
    ' "$RALPH_YML" > "$temp_file"

    if [[ -f "$temp_file" ]]; then
        mv "$temp_file" "$RALPH_YML"
        log_status "INFO" "Updated task $task_id status to $new_status"
    fi
}

# Update session.md with current activity
update_session_file() {
    local iteration=$1
    local status=$2
    local current_task=$3
    local recommendation=$4

    local session_id=$(get_state "session.session_id")
    local now=$(date -u +%Y-%m-%dT%H:%M:%SZ)
    local total_tasks=$(get_state "tasks.total_tasks")
    local completed_tasks=$(grep -c "status: done" "$RALPH_YML" 2>/dev/null || echo "0")
    local completion_pct=$((completed_tasks * 100 / total_tasks))

    # Update session.md
    cat > "$SESSION_FILE" << EOF
# Ralph Session

## Session Information
- **Session ID**: $session_id
- **Started**: $(get_state "session.started_at")
- **Status**: $status

## Current Task
- **Task ID**: $current_task
- **Status**: $status

## Session Progress
- **Iteration**: $iteration / $(get_state "loop.max_iterations")
- **Tasks Completed**: $completed_tasks / $total_tasks
- **Completion**: ${completion_pct}%

## Last Activity
Iteration $iteration completed at $now
Status: $status
Work Type: $(jq -r '.work_type // "UNKNOWN"' .ralph_analysis_result 2>/dev/null || echo "UNKNOWN")

## Next Steps
1. $recommendation
2. Continue with next highest priority task from ralph.yml
3. Check CLAUDE.md for relevant patterns

## Blockers
$(if [[ "$status" == "BLOCKED" ]]; then
    echo "- Task blocked - see scratchpad.md for details"
else
    echo "- None identified"
fi)

## Notes
- Session active and progressing
- Last update: $now
EOF

    log_status "INFO" "Updated session.md"
}

# Update scratchpad.md with iteration notes
update_scratchpad() {
    local iteration=$1
    local status=$2
    local recommendation=$3

    local now=$(date '+%Y-%m-%d %H:%M:%S')

    # Add to scratchpad
    cat >> "$SCRATCHPAD_FILE" << EOF

---

## Iteration $iteration Notes ($now)

**Status**: $status
**Recommendation**: $recommendation

### What Was Done
$(if [[ -f ".ralph_analysis_result" ]]; then
    jq -r '.recommendation' .ralph_analysis_result 2>/dev/null || echo "See analysis"
else
    echo "No analysis available"
fi)

### Files Modified
$(if [[ -f ".ralph_analysis_result" ]]; then
    local files=$(jq -r '.files_modified' .ralph_analysis_result 2>/dev/null || echo "0")
    echo "$files files modified"
else
    echo "Unknown"
fi)

### Test Status
$(if [[ -f ".ralph_analysis_result" ]]; then
    jq -r '.tests_status' .ralph_analysis_result 2>/dev/null || echo "UNKNOWN"
else
    echo "UNKNOWN"
fi)

EOF

    log_status "INFO" "Updated scratchpad.md"
}

# Main analysis function
analyze_response() {
    local output_file=$1
    local iteration=$2

    log_status "INFO" "Analyzing Claude response..."

    # Parse RALPH_STATUS block
    if parse_ralph_status "$output_file"; then
        # Exit signal is true
        log_status "SUCCESS" "EXIT_SIGNAL: true - project may be complete"
    fi

    # Read analysis result
    if [[ -f ".ralph_analysis_result" ]]; then
        local status=$(jq -r '.status' .ralph_analysis_result)
        local current_task=$(jq -r '.current_task' .ralph_analysis_result)
        local work_type=$(jq -r '.work_type' .ralph_analysis_result)
        local recommendation=$(jq -r '.recommendation' .ralph_analysis_result)
        local tasks_completed=$(jq -r '.tasks_completed' .ralph_analysis_result)

        # Update task status if work was done
        if [[ "$status" == "COMPLETE" ]] && [[ "$current_task" != "none" ]]; then
            update_task_status "$current_task" "done"
            # Add to completed tasks list
            local completed_list=$(get_state "tasks.completed_tasks")
            if [[ "$completed_list" != "null" ]]; then
                completed_list="$completed_list, \"$current_task\""
            else
                completed_list="[\"$current_task\"]"
            fi
            update_state "tasks.completed_tasks" "$completed_list"
        fi

        # Update session file
        update_session_file "$iteration" "$status" "$current_task" "$recommendation"

        # Update scratchpad
        update_scratchpad "$iteration" "$status" "$recommendation"

        # Update state
        update_state "tasks.current_task_id" "\"$current_task\""
        update_state "loop.last_status" "\"$status\""
    fi

    # Analyze output patterns
    analyze_output_patterns "$output_file"

    # Check for errors
    if check_for_errors "$output_file"; then
        # No errors - reset circuit breaker consecutive error counter
        update_state "circuit_breaker.consecutive_same_error" "0"
    else
        # Errors found - increment counter
        local count=$(($(get_state "circuit_breaker.consecutive_same_error") + 1))
        update_state "circuit_breaker.consecutive_same_error" "$count"
        log_status "WARN" "Circuit breaker error count: $count"
    fi

    # Update completion percentage
    local total_tasks=$(grep -c "^  - id:" "$RALPH_YML" 2>/dev/null || echo "0")
    local completed=$(grep -c "status: done" "$RALPH_YML" 2>/dev/null || echo "0")
    local completion_pct=$((completed * 100 / total_tasks))
    update_state "tasks.completion_percentage" "$completion_pct"

    log_status "SUCCESS" "Response analysis complete"
}

# CLI interface
case "${1:-}" in
    "parse")
        parse_ralph_status "$2" "${3:-.ralph_analysis_result}"
        ;;
    "analyze")
        analyze_response "$2" "${3:-1}"
        ;;
    "update-task")
        update_task_status "$2" "$3"
        ;;
    *)
        echo "Usage: $0 {parse|analyze|update-task} [args...]"
        echo ""
        echo "Commands:"
        echo "  parse <output_file> [result_file]     Parse RALPH_STATUS block from output"
        echo "  analyze <output_file> [iteration]     Full analysis of Claude output"
        echo "  update-task <task_id> <status>        Update task status in ralph.yml"
        exit 1
        ;;
esac
