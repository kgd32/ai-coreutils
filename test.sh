# 1. Define your command (assuming 'claude' is in your PATH)
CLAUDE_CMD="/c/Users/Kimpa/AppData/Roaming/npm/claude.cmd"

# 2. Define your arguments in an array (safer for spaces)
# Use the '-p' flag if you want it to run once and exit (headless)
claude_args=("Write a hello world in our chat in javascript. no files")

echo "--- Starting Claude Test ---"

# 3. Simple execution
# If you are in Git Bash/MinGW, adding 'winpty' often fixes "stuck" commands
winpty "$CLAUDE_CMD" "${claude_args[@]}"

#timeout $((CLAUDE_TIMEOUT_MINUTES * 60)) $CLAUDE_CMD "${claude_args[@]}" > "$output_file" 2>&1


echo "--- Test Finished with Exit Code: $? ---"