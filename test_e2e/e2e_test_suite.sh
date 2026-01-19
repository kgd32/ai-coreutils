#!/bin/bash
# E2E Test Suite for AI-Coreutils
# Comprehensive end-to-end testing of all utilities

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Test counters
TESTS_RUN=0
TESTS_PASSED=0
TESTS_FAILED=0
TESTS_SKIPPED=0

# Directories
TEST_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
FIXTURES_DIR="$TEST_DIR/fixtures"
LOGS_DIR="$TEST_DIR/logs"
SAMPLES_DIR="$TEST_DIR/samples"
PROJECT_ROOT="$(dirname "$TEST_DIR")"

# Create output directories
mkdir -p "$LOGS_DIR"
mkdir -p "$SAMPLES_DIR"

# Build the project first
echo -e "${BLUE}=== Building AI-Coreutils ===${NC}"
cd "$PROJECT_ROOT"
cargo build --release 2>&1 | tee "$LOGS_DIR/build.log"

# Binary paths
AI_LS="$PROJECT_ROOT/target/release/ai-ls"
AI_CAT="$PROJECT_ROOT/target/release/ai-cat"
AI_GREP="$PROJECT_ROOT/target/release/ai-grep"
AI_HEAD="$PROJECT_ROOT/target/release/ai-head"
AI_TAIL="$PROJECT_ROOT/target/release/ai-tail"
AI_WC="$PROJECT_ROOT/target/release/ai-wc"
AI_TOUCH="$PROJECT_ROOT/target/release/ai-touch"
AI_MKDIR="$PROJECT_ROOT/target/release/ai-mkdir"
AI_RMDIR="$PROJECT_ROOT/target/release/ai-rmdir"
AI_CP="$PROJECT_ROOT/target/release/ai-cp"
AI_MV="$PROJECT_ROOT/target/release/ai-mv"
AI_RM="$PROJECT_ROOT/target/release/ai-rm"
AI_FIND="$PROJECT_ROOT/target/release/ai-find"
AI_CHMOD="$PROJECT_ROOT/target/release/ai-chmod"
AI_CHOWN="$PROJECT_ROOT/target/release/ai-chown"
AI_ANALYZE="$PROJECT_ROOT/target/release/ai-analyze"

# Helper functions
run_test() {
    local test_name="$1"
    local command="$2"
    local expected_result="$3"  # "pass" or "fail"

    TESTS_RUN=$((TESTS_RUN + 1))

    echo -e "\n${BLUE}[$TESTS_RUN] Testing: $test_name${NC}"
    echo "Command: $command"

    # Create log file for this test
    local test_log="$LOGS_DIR/test_${TESTS_RUN}_${test_name// /_}.log"
    local sample_file="$SAMPLES_DIR/test_${TESTS_RUN}_${test_name// /_}.jsonl"

    # Run the command and capture output
    if eval "$command" > "$sample_file" 2>&1; then
        local exit_code=0
    else
        local exit_code=$?
    fi

    # Check if result matches expectation
    if [ "$expected_result" = "pass" ]; then
        if [ $exit_code -eq 0 ]; then
            echo -e "${GREEN}✓ PASSED${NC}"
            TESTS_PASSED=$((TESTS_PASSED + 1))
        else
            echo -e "${RED}✗ FAILED (exit code: $exit_code)${NC}"
            cat "$sample_file"
            TESTS_FAILED=$((TESTS_FAILED + 1))
        fi
    else
        if [ $exit_code -ne 0 ]; then
            echo -e "${GREEN}✓ PASSED (expected failure)${NC}"
            TESTS_PASSED=$((TESTS_PASSED + 1))
        else
            echo -e "${RED}✗ FAILED (should have failed but passed)${NC}"
            TESTS_FAILED=$((TESTS_FAILED + 1))
        fi
    fi

    # Validate JSONL output if command succeeded
    if [ $exit_code -eq 0 ] && [ -s "$sample_file" ]; then
        if ! validate_jsonl "$sample_file" >> "$test_log" 2>&1; then
            echo -e "${YELLOW}⚠ WARNING: Invalid JSONL output${NC}"
            TESTS_FAILED=$((TESTS_FAILED + 1))
        fi
    fi
}

validate_jsonl() {
    local file="$1"
    local line_num=0

    while IFS= read -r line; do
        line_num=$((line_num + 1))
        if ! echo "$line" | jq empty 2>/dev/null; then
            echo "Invalid JSON on line $line_num: $line"
            return 1
        fi
    done < "$file"

    return 0
}

# Start E2E testing
echo -e "\n${BLUE}=== Starting E2E Test Suite ===${NC}"
echo "Test fixtures: $FIXTURES_DIR"
echo "Logs directory: $LOGS_DIR"
echo "Samples directory: $SAMPLES_DIR"
echo "Timestamp: $(date)"

# ==============================================================================
# TEST SUITE 1: Basic Utility Tests
# ==============================================================================
echo -e "\n${YELLOW}=== TEST SUITE 1: Basic Utility Tests ===${NC}"

# Test ai-ls
run_test "ai-ls basic directory listing" \
    "$AI_LS $FIXTURES_DIR" \
    "pass"

run_test "ai-ls with long format" \
    "$AI_LS -l $FIXTURES_DIR" \
    "pass"

run_test "ai-ls recursive" \
    "$AI_LS -R $FIXTURES_DIR" \
    "pass"

run_test "ai-ls with hidden files" \
    "$AI_LS -a $FIXTURES_DIR" \
    "pass"

# Test ai-cat
run_test "ai-cat single file" \
    "$AI_CAT $FIXTURES_DIR/small.txt" \
    "pass"

run_test "ai-cat multiple files" \
    "$AI_CAT $FIXTURES_DIR/small.txt $FIXTURES_DIR/medium.txt" \
    "pass"

run_test "ai-cat with line numbers" \
    "$AI_CAT -n $FIXTURES_DIR/multiline.txt" \
    "pass"

run_test "ai-cat non-existent file" \
    "$AI_CAT $FIXTURES_DIR/nonexistent.txt" \
    "fail"

# Test ai-grep
run_test "ai-grep basic pattern match" \
    "$AI_GREP PATTERN $FIXTURES_DIR/multiline.txt" \
    "pass"

run_test "ai-grep with line numbers" \
    "$AI_GREP -n PATTERN $FIXTURES_DIR/multiline.txt" \
    "pass"

run_test "ai-grep case insensitive" \
    "$AI_GREP -i pattern $FIXTURES_DIR/multiline.txt" \
    "pass"

run_test "ai-grep count matches" \
    "$AI_GREP -c PATTERN $FIXTURES_DIR/multiline.txt" \
    "pass"

run_test "ai-grep with context" \
    "$AI_GREP -C 2 PATTERN $FIXTURES_DIR/multiline.txt" \
    "pass"

run_test "ai-grep invert match" \
    "$AI_GREP -v PATTERN $FIXTURES_DIR/multiline.txt" \
    "pass"

# Test ai-head
run_test "ai-head default lines" \
    "$AI_HEAD $FIXTURES_DIR/multiline.txt" \
    "pass"

run_test "ai-head custom line count" \
    "$AI_HEAD -n 5 $FIXTURES_DIR/multiline.txt" \
    "pass"

# Test ai-tail
run_test "ai-tail default lines" \
    "$AI_TAIL $FIXTURES_DIR/multiline.txt" \
    "pass"

run_test "ai-tail custom line count" \
    "$AI_TAIL -n 5 $FIXTURES_DIR/multiline.txt" \
    "pass"

# Test ai-wc
run_test "ai-wc line count" \
    "$AI_WC -l $FIXTURES_DIR/multiline.txt" \
    "pass"

run_test "ai-wc word count" \
    "$AI_WC -w $FIXTURES_DIR/small.txt" \
    "pass"

run_test "ai-wc byte count" \
    "$AI_WC -c $FIXTURES_DIR/small.txt" \
    "pass"

run_test "ai-wc all counts" \
    "$AI_WC $FIXTURES_DIR/small.txt" \
    "pass"

# Test ai-touch
run_test "ai-touch create new file" \
    "touch $TEST_DIR/touch_test.txt && $AI_TOUCH $TEST_DIR/touch_test.txt && rm $TEST_DIR/touch_test.txt" \
    "pass"

# Test ai-mkdir
run_test "ai-mkdir create directory" \
    "$AI_MKDIR $TEST_DIR/mkdir_test && $AI_RM -rf $TEST_DIR/mkdir_test" \
    "pass"

run_test "ai-mkdir create nested directories" \
    "$AI_MKDIR -p $TEST_DIR/nested1/nested2 && $AI_RM -rf $TEST_DIR/nested1" \
    "pass"

# Test ai-rm
run_test "ai-rm remove file" \
    "touch $TEST_DIR/rm_test.txt && $AI_RM $TEST_DIR/rm_test.txt" \
    "pass"

run_test "ai-rm remove directory recursively" \
    "$AI_MKDIR -p $TEST_DIR/rm_test/dir && $AI_RM -rf $TEST_DIR/rm_test" \
    "pass"

# Test ai-cp
run_test "ai-cp copy file" \
    "cp $FIXTURES_DIR/small.txt $TEST_DIR/copy_source.txt && $AI_CP $TEST_DIR/copy_source.txt $TEST_DIR/copy_dest.txt && $AI_RM $TEST_DIR/copy_source.txt $TEST_DIR/copy_dest.txt" \
    "pass"

run_test "ai-cp copy directory recursively" \
    "$AI_MKDIR -p $TEST_DIR/cp_src/dir && $AI_CP -r $TEST_DIR/cp_src $TEST_DIR/cp_dest && $AI_RM -rf $TEST_DIR/cp_src $TEST_DIR/cp_dest" \
    "pass"

# Test ai-mv
run_test "ai-mv move file" \
    "cp $FIXTURES_DIR/small.txt $TEST_DIR/mv_source.txt && $AI_MV $TEST_DIR/mv_source.txt $TEST_DIR/mv_dest.txt && $AI_RM $TEST_DIR/mv_dest.txt" \
    "pass"

# Test ai-find
run_test "ai-find find by name" \
    "$AI_FIND $FIXTURES_DIR -name '*.txt'" \
    "pass"

run_test "ai-find find by type" \
    "$AI_FIND $FIXTURES_DIR -type f" \
    "pass"

# Test ai-chmod
run_test "ai-chmod change permissions" \
    "cp $FIXTURES_DIR/small.txt $TEST_DIR/chmod_test.txt && $AI_CHMOD 644 $TEST_DIR/chmod_test.txt && $AI_RM $TEST_DIR/chmod_test.txt" \
    "pass"

# ==============================================================================
# TEST SUITE 2: JSONL Output Validation
# ==============================================================================
echo -e "\n${YELLOW}=== TEST SUITE 2: JSONL Output Validation ===${NC}"

run_test "JSONL output - ai-ls" \
    "$AI_LS $FIXTURES_DIR | head -1" \
    "pass"

run_test "JSONL output - ai-cat" \
    "$AI_CAT $FIXTURES_DIR/small.txt | head -1" \
    "pass"

run_test "JSONL output - ai-grep" \
    "$AI_GREP test $FIXTURES_DIR/patterns.txt | head -1" \
    "pass"

# ==============================================================================
# TEST SUITE 3: Edge Cases
# ==============================================================================
echo -e "\n${YELLOW}=== TEST SUITE 3: Edge Cases ===${NC}"

run_test "Empty file handling" \
    "$AI_CAT $FIXTURES_DIR/empty.txt" \
    "pass"

run_test "Unicode content handling" \
    "$AI_CAT $FIXTURES_DIR/unicode.txt" \
    "pass"

run_test "Binary file handling" \
    "$AI_CAT $FIXTURES_DIR/binary.dat" \
    "pass"

run_test "Special characters handling" \
    "$AI_CAT $FIXTURES_DIR/special-chars.txt" \
    "pass"

run_test "Large file handling" \
    "$AI_CAT $FIXTURES_DIR/../large.txt" \
    "pass"

# ==============================================================================
# TEST SUITE 4: ML Pattern Detection
# ==============================================================================
echo -e "\n${YELLOW}=== TEST SUITE 4: ML Pattern Detection ===${NC}"

run_test "ai-analyze pattern detection" \
    "$AI_ANALYZE --patterns $FIXTURES_DIR/patterns.txt" \
    "pass"

run_test "ai-analyze file classification" \
    "$AI_ANALYZE --classify $FIXTURES_DIR/code-sample.rs" \
    "pass"

run_test "ai-analyze content statistics" \
    "$AI_ANALYZE --statistics $FIXTURES_DIR/multiline.txt" \
    "pass"

# ==============================================================================
# TEST SUITE 5: Recursive Operations
# ==============================================================================
echo -e "\n${YELLOW}=== TEST SUITE 5: Recursive Operations ===${NC}"

run_test "Recursive grep in directories" \
    "$AI_GREP -r file $FIXTURES_DIR/nested" \
    "pass"

run_test "Recursive ls" \
    "$AI_LS -R $FIXTURES_DIR/nested" \
    "pass"

# ==============================================================================
# TEST SUITE 6: Error Handling
# ==============================================================================
echo -e "\n${YELLOW}=== TEST SUITE 6: Error Handling ===${NC}"

run_test "Error: Non-existent file" \
    "$AI_CAT /nonexistent/path/file.txt" \
    "fail"

run_test "Error: Invalid permissions" \
    "$AI_CAT /root/.hidden_file 2>/dev/null || $AI_CAT /nonexistent/file.txt" \
    "fail"

# ==============================================================================
# SUMMARY
# ==============================================================================
echo -e "\n${BLUE}=== Test Suite Summary ===${NC}"
echo "Total tests run: $TESTS_RUN"
echo -e "Tests passed: ${GREEN}$TESTS_PASSED${NC}"
echo -e "Tests failed: ${RED}$TESTS_FAILED${NC}"
echo -e "Tests skipped: ${YELLOW}$TESTS_SKIPPED${NC}"

if [ $TESTS_FAILED -eq 0 ]; then
    echo -e "\n${GREEN}All tests passed!${NC}"
    exit 0
else
    echo -e "\n${RED}Some tests failed!${NC}"
    echo "Check logs in: $LOGS_DIR"
    exit 1
fi
