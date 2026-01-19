#!/bin/bash

# Circuit Breaker for .ralph/ Structure
# Prevents runaway loops by detecting stagnation and errors

set -e

# Script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

# .ralph directory structure
RALPH_DIR="$PROJECT_ROOT/.ralph"
STATE_FILE="$RALPH_DIR/state.json"
HISTORY_DIR="$RALPH_DIR/history"
RALPH_YML="$RALPH_DIR/ralph.yml"

# Circuit breaker states
CB_STATE_CLOSED="CLOSED"        # Normal operation, progress detected
CB_STATE_HALF_OPEN="HALF_OPEN"  # Monitoring mode, checking for recovery
CB_STATE_OPEN="OPEN"            # Failure detected, execution halted

# Thresholds
CB_NO_PROGRESS_THRESHOLD=3      # Open circuit after N loops with no progress
CB_SAME_ERROR_THRESHOLD=5       # Open circuit after N loops with same error
CB_OUTPUT_DECLINE_THRESHOLD=70  # Open circuit if output declines by >70%

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

# Initialize circuit breaker
init_circuit_breaker() {
    # Check if circuit breaker state exists in state.json
    local current_state=$(get_state "circuit_breaker.state")

    if [[ "$current_state" == "null" ]] || [[ -z "$current_state" ]]; then
        update_state "circuit_breaker.state" "\"$CB_STATE_CLOSED\""
        update_state "circuit_breaker.consecutive_no_progress" "0"
        update_state "circuit_breaker.consecutive_same_error" "0"
        update_state "circuit_breaker.last_progress_loop" "0"
        update_state "circuit_breaker.total_opens" "0"
        log_status "INFO" "Circuit breaker initialized to CLOSED"
    fi
}

# Get current circuit breaker state
get_circuit_state() {
    get_state "circuit_breaker.state"
}

# Check if circuit breaker allows execution
can_execute() {
    local state=$(get_circuit_state)

    if [[ "$state" == "$CB_STATE_OPEN" ]]; then
        log_status "ERROR" "Circuit breaker is OPEN - execution blocked"
        return 1
    else
        return 0
    fi
}

# Record loop result and update circuit breaker state
record_loop_result() {
    local loop_number=$1
    local files_changed=$2
    local has_errors=$3
    local output_length=$4

    local state=$(get_circuit_state)
    local no_progress_count=$(get_state "circuit_breaker.consecutive_no_progress")
    local error_count=$(get_state "circuit_breaker.consecutive_same_error")
    local last_progress_loop=$(get_state "circuit_breaker.last_progress_loop")

    log_status "INFO" "Recording loop result: files=$files_changed, errors=$has_errors, output_len=$output_length"

    case $state in
        "$CB_STATE_CLOSED")
            handle_closed_state "$files_changed" "$has_errors" "$output_length" "$no_progress_count" "$error_count"
            ;;
        "$CB_STATE_HALF_OPEN")
            handle_half_open_state "$files_changed" "$has_errors"
            ;;
        "$CB_STATE_OPEN")
            log_status "INFO" "Circuit breaker is OPEN - use 'reset' to recover"
            ;;
    esac
}

# Handle CLOSED state logic
handle_closed_state() {
    local files_changed=$1
    local has_errors=$2
    local output_length=$3
    local no_progress_count=$4
    local error_count=$5

    local should_open=false
    local reason=""

    # Check for no progress
    if [[ $files_changed -eq 0 ]]; then
        no_progress_count=$((no_progress_count + 1))
        update_state "circuit_breaker.consecutive_no_progress" "$no_progress_count"

        if [[ $no_progress_count -ge $CB_NO_PROGRESS_THRESHOLD ]]; then
            should_open=true
            reason="No progress for $no_progress_count consecutive loops"
        fi
    else
        # Progress detected - reset no progress counter
        update_state "circuit_breaker.consecutive_no_progress" "0"
        update_state "circuit_breaker.last_progress_loop" "$(get_state "loop.iteration")"
    fi

    # Check for repeated errors
    if [[ "$has_errors" == "true" ]]; then
        error_count=$((error_count + 1))
        update_state "circuit_breaker.consecutive_same_error" "$error_count"

        if [[ $error_count -ge $CB_SAME_ERROR_THRESHOLD ]]; then
            should_open=true
            reason="Repeated errors for $error_count consecutive loops"
        fi
    else
        # No errors - reset error counter
        update_state "circuit_breaker.consecutive_same_error" "0"
    fi

    # Check for output decline
    local previous_output=$(get_state "circuit_breaker.previous_output_length" || echo "0")
    if [[ $previous_output -gt 0 ]]; then
        local decline_pct=$((100 - (output_length * 100 / previous_output)))
        if [[ $decline_pct -gt $CB_OUTPUT_DECLINE_THRESHOLD ]]; then
            should_open=true
            reason="Output declined by ${decline_pct}%"
        fi
    fi
    update_state "circuit_breaker.previous_output_length" "$output_length"

    # Open circuit if needed
    if [[ "$should_open" == "true" ]]; then
        open_circuit "$reason"
    fi
}

# Handle HALF_OPEN state logic
handle_half_open_state() {
    local files_changed=$1
    local has_errors=$2

    if [[ $files_changed -gt 0 ]] && [[ "$has_errors" != "true" ]]; then
        # Recovery detected - close circuit
        log_status "SUCCESS" "Recovery detected - closing circuit breaker"
        update_state "circuit_breaker.state" "\"$CB_STATE_CLOSED\""
        update_state "circuit_breaker.consecutive_no_progress" "0"
        update_state "circuit_breaker.consecutive_same_error" "0"
    else
        # Recovery failed - open circuit again
        open_circuit "Recovery attempt failed in HALF_OPEN state"
    fi
}

# Open the circuit breaker
open_circuit() {
    local reason=$1

    local total_opens=$(($(get_state "circuit_breaker.total_opens") + 1))

    update_state "circuit_breaker.state" "\"$CB_STATE_OPEN\""
    update_state "circuit_breaker.total_opens" "$total_opens"
    update_state "circuit_breaker.open_reason" "\"$reason\""
    update_state "circuit_breaker.opened_at" "\"$(date -u +%Y-%m-%dT%H:%M:%SZ)\""

    log_status "ERROR" "=== CIRCUIT BREAKER OPENED ==="
    log_status "ERROR" "Reason: $reason"
    log_status "ERROR" "Total opens: $total_opens"
    log_status "ERROR" "Use: .ralph/circuit_breaker.sh reset to recover"

    # Create alert file
    cat > "$RALPH_DIR/circuit_breaker_alert.txt" << EOF
CIRCUIT BREAKER OPENED
======================
Time: $(date -u +%Y-%m-%dT%H:%M:%SZ)
Reason: $reason
Total Opens: $total_opens

To recover, run:
  .ralph/circuit_breaker.sh reset
EOF
}

# Reset the circuit breaker
reset_circuit_breaker() {
    log_status "INFO" "Resetting circuit breaker..."

    update_state "circuit_breaker.state" "\"$CB_STATE_HALF_OPEN\""
    update_state "circuit_breaker.consecutive_no_progress" "0"
    update_state "circuit_breaker.consecutive_same_error" "0"

    rm -f "$RALPH_DIR/circuit_breaker_alert.txt"

    log_status "SUCCESS" "Circuit breaker reset to HALF_OPEN (monitoring mode)"
}

# Show circuit breaker status
show_circuit_status() {
    local state=$(get_circuit_state)
    local no_progress=$(get_state "circuit_breaker.consecutive_no_progress")
    local same_error=$(get_state "circuit_breaker.consecutive_same_error")
    local total_opens=$(get_state "circuit_breaker.total_opens")
    local open_reason=$(get_state "circuit_breaker.open_reason" || echo "N/A")
    local opened_at=$(get_state "circuit_breaker.opened_at" || echo "N/A")

    echo ""
    echo "=== Circuit Breaker Status ==="
    echo "State: $state"
    echo "Consecutive No Progress: $no_progress / $CB_NO_PROGRESS_THRESHOLD"
    echo "Consecutive Same Error: $same_error / $CB_SAME_ERROR_THRESHOLD"
    echo "Total Opens: $total_opens"
    echo ""
    if [[ "$state" == "$CB_STATE_OPEN" ]]; then
        echo "Open Reason: $open_reason"
        echo "Opened At: $opened_at"
    fi
    echo "=============================="
    echo ""
}

# Check for stuck loops by analyzing recent history
check_stuck_loop() {
    local recent_loops=${1:-5}
    local iteration=$(get_state "loop.iteration")

    log_status "INFO" "Checking for stuck loops (last $recent_loops iterations)..."

    # Collect error lines from recent output files
    declare -a error_lines=()
    declare -a error_files=()

    for ((i=$((iteration - recent_loops)); i<=iteration; i++)); do
        local output_file="$HISTORY_DIR/claude_output_${i}.json"
        if [[ -f "$output_file" ]]; then
            # Filter out JSON field false positives and get real errors
            local errors=$(grep -v '"[^"]*error[^"]*":' "$output_file" 2>/dev/null | \
                          grep -E '(^Error:|^ERROR:|^error:|\]: error|Link: error|Error occurred|failed with error|Exception|Fatal|FATAL)' || true)

            if [[ -n "$errors" ]]; then
                while IFS= read -r line; do
                    error_lines+=("$line")
                    error_files+=("$output_file")
                done <<< "$errors"
            fi
        fi
    done

    # Check if all errors appear in all recent files (stuck pattern)
    if [[ ${#error_lines[@]} -gt 0 ]]; then
        local unique_errors=($(printf "%s\n" "${error_lines[@]}" | sort -u))
        local stuck=true

        for error in "${unique_errors[@]}"; do
            local file_count=0
            for file in "${error_files[@]}"; do
                if grep -qF "$error" "$file" 2>/dev/null; then
                    file_count=$((file_count + 1))
                fi
            done

            # If error doesn't appear in all files, not stuck
            if [[ $file_count -lt ${#error_files[@]} ]]; then
                stuck=false
                break
            fi
        done

        if [[ "$stuck" == "true" ]] && [[ ${#unique_errors[@]} -gt 0 ]]; then
            log_status "WARN" "Stuck loop detected: ${#unique_errors[@]} unique errors repeating"
            return 0
        fi
    fi

    log_status "INFO" "No stuck loop detected"
    return 1
}

# CLI interface
case "${1:-}" in
    "init")
        init_circuit_breaker
        ;;
    "status")
        show_circuit_status
        ;;
    "reset")
        reset_circuit_breaker
        ;;
    "check")
        can_execute
        ;;
    "record")
        init_circuit_breaker
        record_loop_result "${2:-1}" "${3:-0}" "${4:-false}" "${5:-0}"
        ;;
    "stuck")
        check_stuck_loop "${2:-5}"
        ;;
    *)
        echo "Usage: $0 {init|status|reset|check|record|stuck} [args...]"
        echo ""
        echo "Commands:"
        echo "  init                       Initialize circuit breaker"
        echo "  status                     Show circuit breaker status"
        echo "  reset                      Reset circuit breaker to HALF_OPEN"
        echo "  check                      Check if execution is allowed"
        echo "  record <loop> <files> <errors> <output_len>  Record loop result"
        echo "  stuck [recent_loops]       Check for stuck loops"
        exit 1
        ;;
esac
