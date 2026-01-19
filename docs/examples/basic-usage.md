# Basic Usage Examples

Practical examples of using AI-Coreutils for common tasks.

## Table of Contents

1. [File Operations](#file-operations)
2. [Text Processing](#text-processing)
3. [Search and Find](#search-and-find)
4. [Batch Operations](#batch-operations)
5. [Working with JSONL Output](#working-with-jsonl-output)

## File Operations

### List Files with Details

```bash
# List files with long format
ai-ls -l

# List with human-readable sizes
ai-ls -lh

# List all files including hidden
ai-ls -la

# List recursively
ai-ls -R ./src
```

### View File Contents

```bash
# View file
ai-cat file.txt

# Number lines
ai-cat -n file.txt

# View multiple files
ai-cat file1.txt file2.txt file3.txt
```

### Copy, Move, Delete

```bash
# Copy file
ai-cp source.txt dest.txt

# Copy directory
ai-cp -R src/ dest/

# Move file
ai-mv old.txt new.txt

# Delete file
ai-rm unwanted.txt

# Delete directory
ai-rm -r directory/
```

### File Information

```bash
# Count lines, words, bytes
ai-wc file.txt

# Show first 10 lines
ai-head file.txt

# Show last 10 lines
ai-tail file.txt

# Show last 20 lines
ai-tail -n 20 file.txt
```

## Text Processing

### Pattern Matching

```bash
# Search for pattern
ai-grep "TODO" ./src

# Case insensitive
ai-grep -i "error" *.log

# With context
ai-grep -C 3 "function" main.rs

# Count matches
ai-grep -c "import" *.py
```

### Pattern Detection

```bash
# Find all emails
ai-analyze -t email contacts.txt

# Find URLs and IPs
ai-analyze -t url,ip server_logs.txt

# Detect all patterns
ai-analyze -p document.txt
```

### File Classification

```bash
# Classify file
ai-analyze -c unknown_file.bin

# Classify directory
ai-analyze -r -c ./unsorted

# Full analysis
ai-analyze -p -c -s file.txt
```

## Search and Find

### Find Files

```bash
# Find by name
ai-find ./src -name "*.rs"

# Find directories
ai-find . -type d

# Find large files
ai-find . -size +100M

# Find recent files
ai-find . -mtime -7
```

### Search in Files

```bash
# Search recursively
ai-grep -r "pattern" ./src

# Search specific files
ai-grep "pattern" *.txt

# Show filenames only
ai-grep -l "TODO" ./src

# Invert match
ai-grep -v "comment" code.py
```

## Batch Operations

### Process Multiple Files

```bash
# Count lines in multiple files
ai-wc *.txt

# Search in multiple files
ai-grep "error" *.log

# Copy multiple files
ai-cp file*.txt backup/
```

### Async Processing

```bash
# Process files concurrently
ai-cat --async *.log

# Concurrent search
ai-grep --async -r "pattern" /large/dir

# Custom concurrency
ai-cat --async --max-concurrent 20 *.log
```

### Directory Operations

```bash
# Create nested directories
ai-mkdir -p path/to/nested/dir

# Remove directory recursively
ai-rm -r build/

# Copy directory tree
ai-cp -R src/ dest/
```

## Working with JSONL Output

### Parse JSONL with jq

```bash
# Extract file paths
ai-ls | jq -r '.path'

# Filter large files
ai-ls | jq 'select(.size > 1000)'

# Sum file sizes
ai-ls | jq '[.size | tonumber] | add'

# Count errors
ai-grep "error" log.txt | jq 'select(.type == "error")' | wc -l
```

### Parse JSONL with Python

```python
import json
import subprocess

# Run command and parse output
result = subprocess.run(['ai-ls', '-l'], capture_output=True, text=True)

for line in result.stdout.splitlines():
    record = json.loads(line)
    if record.get('type') == 'file_entry':
        print(f"{record['path']}: {record['size']} bytes")
```

### Parse JSONL with Node.js

```javascript
const { exec } = require('child_process');

exec('ai-ls -l', (error, stdout, stderr) => {
    if (error) {
        console.error(`Error: ${error}`);
        return;
    }

    const lines = stdout.split('\n');
    lines.forEach(line => {
        if (line.trim()) {
            const record = JSON.parse(line);
            if (record.type === 'file_entry') {
                console.log(`${record.path}: ${record.size} bytes`);
            }
        }
    });
});
```

## Common Workflows

### Log Analysis

```bash
# Find recent errors
ai-tail -n 100 app.log | ai-grep "ERROR"

# Count error types
ai-grep "ERROR" app.log | jq -r '.line_content' | sort | uniq -c

# Monitor log in real-time
ai-tail -f app.log | ai-grep "ERROR"
```

### Code Statistics

```bash
# Count lines of code
ai-wc -l src/**/*.rs

# Find TODO comments
ai-grep -r "TODO" ./src

# List source files
ai-find ./src -name "*.rs"
```

### Backup Operations

```bash
# Copy with progress
ai-cp -v -R src/ backup/

# Archive mode
ai-cp -a important_data/ backup/

# Verify copy
ai-wc src/*.txt backup/*.txt
```

### Data Processing

```bash
# Process CSV files
for file in data/*.csv; do
    ai-grep "pattern" "$file" | jq -r '.line_content'
done

# Convert JSONL to CSV
ai-cat data.jsonl | jq -r '[.field1, .field2] | @csv'
```

### System Administration

```bash
# Find large files
ai-find /var/log -size +100M

# Find old files
ai-find /tmp -mtime +30

# Set permissions
ai-chmod -R 755 public_html/

# Change ownership
ai-chown -R user:group /home/user/
```

## Tips and Tricks

### Combining Utilities

```bash
# List files, filter large ones, copy them
ai-ls -R | jq 'select(.size > 1000000) | .path' | xargs ai-cp -t large_files/

# Search for patterns and analyze
ai-grep -r "email" ./src | jq -r '.line_content' | ai-analyze -t email -
```

### Performance Optimization

```bash
# Use async for many files
ai-cat --async --max-concurrent 20 *.log

# Use SIMD automatically enabled
ai-wc large_file.txt  # SIMD-accelerated
```

### Error Handling

```bash
# Check for errors in output
ai-ls | jq 'select(.type == "error")'

# Continue on errors
ai-cp --no-clobber source/* dest/  # Skip existing files
```

## Real-World Examples

### Web Server Log Analysis

```bash
# Find 404 errors
ai-grep " 404 " access.log | jq -r '.line_content'

# Count unique IPs
ai-grep " 200 " access.log | jq -r '.line_content' | awk '{print $1}' | sort | uniq

# Find slow requests
ai-grep " [5-9][0-9][0-9] " access.log | jq -r '.line_content'
```

### Backup Verification

```bash
# Compare file counts
original=$(ai-ls -R original/ | jq 'length')
backup=$(ai-ls -R backup/ | jq 'length')
echo "Original: $original, Backup: $backup"

# Compare sizes
ai-wc -c original/* | ai-grep -v "total"
ai-wc -c backup/* | ai-grep -v "total"
```

### Code Quality Check

```bash
# Find long lines
ai-wc -L src/**/*.rs | jq 'select(.max_line_length > 100)'

# Count TODO/FIXME comments
ai-grep -r "TODO\|FIXME" ./src | jq 'length'

# Find unused imports (Rust specific)
ai-grep "^use " src/*.rs | sort | uniq -c | sort -rn
```
