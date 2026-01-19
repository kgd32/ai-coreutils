# Pipeline Processing Examples

Examples of building data processing pipelines with AI-Coreutils.

## Table of Contents

1. [Basic Pipelines](#basic-pipelines)
2. [JSONL Processing](#jsonl-processing)
3. [Complex Workflows](#complex-workflows)
4. [Performance Optimization](#performance-optimization)

## Basic Pipelines

### File Discovery and Processing

```bash
# Find all Rust files and count lines
ai-find ./src -name "*.rs" | xargs ai-wc -l

# Find large log files and analyze
ai-find /var/log -size +100M | xargs ai-analyze -s
```

### Filter and Transform

```bash
# List files, filter large ones, show details
ai-ls -R | \
  jq 'select(.size > 1000000)' | \
  jq '{path, size: (.size | tonumber)}'

# Find recent files and copy
ai-find . -mtime -7 -type f | \
  jq -r '.path' | \
  xargs -I {} ai-cp {} recent_files/
```

### Count and Aggregate

```bash
# Count files by extension
ai-ls -R | \
  jq -r '.path' | \
  grep -o '\.[^.]*$' | \
  sort | uniq -c

# Sum directory sizes
ai-ls -R | \
  jq 'select(.is_dir == false)' | \
  jq '[.size | tonumber] | add'
```

## JSONL Processing

### Streaming JSONL Processing

```python
import json
import sys

def process_jsonl(input_stream):
    for line in input_stream:
        record = json.loads(line)
        if record.get('type') == 'result':
            yield record['data']

# Example: Filter large files
import subprocess

proc = subprocess.Popen(['ai-ls', '-R'], stdout=subprocess.PIPE)
for data in process_jsonl(proc.stdout):
    if data.get('size', 0) > 1000000:
        print(f"Large file: {data['path']}")
```

### Multi-Stage Processing

```bash
# Stage 1: List files
# Stage 2: Filter by size
# Stage 3: Extract paths
# Stage 4: Process files
ai-ls -R | \
  jq 'select(.size > 1000 and .size < 1000000)' | \
  jq -r '.path' | \
  xargs ai-grep "pattern"
```

### Aggregation Pipeline

```bash
# Collect all JSONL records, aggregate, and summarize
ai-ls -R | \
  jq '{type: .type, size: .size}' | \
  jq -s 'group_by(.type) | map({type: .[0].type, count: length})'
```

## Complex Workflows

### Log Processing Pipeline

```bash
#!/bin/bash

# 1. Extract log entries
ai-grep "ERROR" app.log | \
  # 2. Parse timestamps
  jq -r '.timestamp' | \
  # 3. Extract hour
  cut -d'T' -f2 | cut -d':' -f1 | \
  # 4. Count per hour
  sort | uniq -c | \
  # 5. Sort by frequency
  sort -rn
```

### Backup Verification Pipeline

```bash
#!/bin/bash

# Compare source and backup
echo "Source files:"
ai-ls -R source/ | jq -r '.path' | sort > /tmp/source.txt

echo "Backup files:"
ai-ls -R backup/ | jq -r '.path' | sort > /tmp/backup.txt

# Find differences
echo "Files in source but not in backup:"
comm -23 /tmp/source.txt /tmp/backup.txt
```

### Code Analysis Pipeline

```bash
#!/bin/bash

# 1. Find all source files
ai-find ./src -name "*.rs" | \
  # 2. Extract paths
  jq -r '.path' | \
  # 3. Count lines in each
  xargs ai-wc -l | \
  # 4. Parse results
  jq -r 'select(.type == "info") | .data' | \
  # 5. Calculate statistics
  jq '[.lines | tonumber] | {total: add, avg: (add / length), max: max}'
```

### Data Cleaning Pipeline

```bash
#!/bin/bash

# 1. List all data files
ai-find ./data -name "*.csv" | \
  # 2. Filter small files (likely corrupted)
  jq 'select(.size > 100)' | \
  # 3. Extract paths
  jq -r '.path' | \
  # 4. Process each file
  xargs -I {} sh -c '
    echo "Processing: {}"
    # Remove empty lines
    ai-grep -v "^\s*$" {} | jq -r ".line_content" > {}.clean
  '
```

## Performance Optimization

### Parallel Processing

```bash
# Process files in parallel using xargs
ai-find ./data -name "*.txt" | \
  jq -r '.path' | \
  xargs -P 8 -I {} ai-wc {}

# Use GNU parallel for more control
ai-find ./logs -name "*.log" | \
  jq -r '.path' | \
  parallel -j 4 ai-grep "ERROR" {}
```

### Batching

```bash
# Process files in batches
ai-ls -R | \
  jq -r '.path' | \
  split -l 100 - batch_ --filter='
    cat $FILE | xargs ai-cat --async | ai-grep "pattern" > ${FILE}.results
  '
```

### Memory-Efficient Processing

```bash
# Process line by line without loading entire file
ai-cat large_file.txt | \
  while IFS= read -r line; do
    # Process line
    echo "$line" | grep "pattern"
  done
```

## Real-World Pipelines

### ETL Pipeline

```bash
#!/bin/bash

# Extract: List files
ai-find /incoming -name "*.json" | jq -r '.path' > /tmp/files.txt

# Transform: Process each file
while read -r file; do
  ai-cat "$file" | \
    jq '.data | select(.status == "active")' | \
    ai-grep "pattern" | \
    jq -c '{id, timestamp, value}' > "/processed/$(basename $file)"
done < /tmp/files.txt

# Load: Verify processed files
ai-find /processed -name "*.json" | jq 'length'
```

### Log Analytics Pipeline

```bash
#!/bin/bash

LOG_DIR="/var/log/app"
OUTPUT_DIR="/analytics/$(date +%Y%m%d)"

mkdir -p "$OUTPUT_DIR"

# Extract unique IPs
ai-grep "GET\|POST" "$LOG_DIR"/*.log | \
  jq -r '.line_content' | \
  grep -oE '[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}' | \
  sort | uniq -c | sort -rn > "$OUTPUT_DIR/ips.txt"

# Extract status codes
ai-grep " HTTP/" "$LOG_DIR"/*.log | \
  jq -r '.line_content' | \
  grep -oE ' [0-9]{3} ' | \
  sort | uniq -c | sort -rn > "$OUTPUT_DIR/status_codes.txt"

# Calculate statistics
echo "Analysis complete: $OUTPUT_DIR"
ai-wc -l "$OUTPUT_DIR"/*
```

### Backup Rotation Pipeline

```bash
#!/bin/bash

SOURCE_DIR="/data"
BACKUP_DIR="/backup"
RETENTION_DAYS=30

# Find old backups
ai-find "$BACKUP_DIR" -mtime +$RETENTION_DAYS | \
  jq -r '.path' | \
  # Create archive list
  tee >(/tmp/delete_list.txt) | \
  # Count files to delete
  wc -l

# Verify before deleting
echo "Files to delete:"
cat /tmp/delete_list.txt

# Delete old backups
cat /tmp/delete_list.txt | xargs ai-rm

# Create new backup
ai-cp -a "$SOURCE_DIR" "$BACKUP_DIR/$(date +%Y%m%d)/"

echo "Backup rotation complete"
```

### Continuous Monitoring Pipeline

```bash
#!/bin/bash

LOG_FILE="/var/log/monitor.log"
ALERT_THRESHOLD=100

while true; do
  # Count errors in last minute
  ERROR_COUNT=$(ai-tail -n 1000 "$LOG_FILE" | ai-grep "ERROR" | jq 'length')

  if [ "$ERROR_COUNT" -gt "$ALERT_THRESHOLD" ]; then
    # Alert!
    echo "ALERT: $ERROR_COUNT errors detected" | \
      ai-grep "pattern" | \
      jq -c '{timestamp: now(), errors: $ERROR_COUNT}' >> alerts.jsonl
  fi

  # Summary statistics
  ai-tail -n 1000 "$LOG_FILE" | \
    ai-grep "ERROR\|WARN" | \
    jq -r '.line_content' | \
    sort | uniq -c | sort -rn > /tmp/stats.txt

  sleep 60
done
```

### Data Validation Pipeline

```bash
#!/bin/bash

VALIDATE_DIR="/data/incoming"
REJECT_DIR="/data/rejected"

ai-find "$VALIDATE_DIR" -name "*.json" | jq -r '.path' | while read -r file; do
  # Validate JSON structure
  if ai-cat "$file" | jq empty 2>/dev/null; then
    # Check required fields
    if ai-cat "$file" | jq -e '.id and .timestamp and .data' > /dev/null; then
      echo "Valid: $file"
      mv "$file" "/data/valid/"
    else
      echo "Missing required fields: $file"
      mv "$file" "$REJECT_DIR/"
    fi
  else
    echo "Invalid JSON: $file"
    mv "$file" "$REJECT_DIR/"
  fi
done
```

### Multi-Source Aggregation Pipeline

```bash
#!/bin/bash

# Aggregate data from multiple sources
SOURCES=("/data/source1" "/data/source2" "/data/source3")
OUTPUT="/aggregated/$(date +%Y%m%d).jsonl"

for source in "${SOURCES[@]}"; do
  # Find all data files
  ai-find "$source" -name "*.json" | jq -r '.path' | while read -r file; do
    # Normalize and append to output
    ai-cat "$file" | \
      jq '{source: "'"$source"'", data: .}' | \
      tee -a "$OUTPUT"
  done
done

# Sort and deduplicate
ai-cat "$OUTPUT" | \
  jq -s 'unique_by(.data.id)' | \
  jq -r '.[]' > "${OUTPUT}.dedup"

echo "Aggregation complete: ${OUTPUT}.dedup"
```
