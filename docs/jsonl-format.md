# JSONL Output Format

Complete specification for JSONL (JSON Lines) output in AI-Coreutils.

## Overview

All AI-Coreutils utilities output data in JSONL format - one JSON object per line. This format is ideal for AI agents and programmatic processing because:

- **Streamable**: Process line-by-line without loading entire output
- **Parseable**: Standard JSON parsing
- **Structured**: Consistent schema across utilities
- **Extensible**: Easy to add new fields

## Record Types

### Result Record

Standard successful operation result.

```json
{
  "type": "result",
  "timestamp": "2026-01-19T12:00:00Z",
  "data": {
    // Utility-specific data
  }
}
```

### Error Record

Error or failure information.

```json
{
  "type": "error",
  "code": "ERROR_CODE",
  "message": "Human-readable error message",
  "timestamp": "2026-01-19T12:00:00Z",
  "file": "/path/to/file"  // Optional
}
```

### Info Record

General informational message.

```json
{
  "type": "info",
  "timestamp": "2026-01-19T12:00:00Z",
  "data": {
    // Info-specific data
  }
}
```

### Progress Record

Operation progress update.

```json
{
  "type": "progress",
  "timestamp": "2026-01-19T12:00:00Z",
  "current": 5,
  "total": 10,
  "message": "Processing: file.txt"
}
```

### File Entry Record

File metadata from `ai-ls`.

```json
{
  "type": "file_entry",
  "timestamp": "2026-01-19T12:00:00Z",
  "path": "/path/to/file.txt",
  "size": 1024,
  "modified": "2026-01-19T10:30:00Z",
  "is_dir": false,
  "is_symlink": false,
  "permissions": "644"
}
```

### Match Record

Pattern match from `ai-grep`.

```json
{
  "type": "match_record",
  "timestamp": "2026-01-19T12:00:00Z",
  "file": "/path/to/file.txt",
  "line_number": 42,
  "line_content": "ERROR: Something went wrong",
  "match_start": 0,
  "match_end": 5
}
```

### Line Record

Line of text from `ai-cat`.

```json
{
  "type": "line",
  "timestamp": "2026-01-19T12:00:00Z",
  "file": "file.txt",
  "line_number": 1,
  "content": "Hello, world!"
}
```

## Timestamp Format

All timestamps use ISO 8601 format in UTC:

```
2026-01-19T12:00:00Z
```

Parse with:

```python
from datetime import datetime
dt = datetime.fromisoformat(timestamp.replace('Z', '+00:00'))
```

```javascript
const dt = new Date(timestamp);
```

```rust
use chrono::DateTime;
let dt: DateTime<Utc> = timestamp.parse()?;
```

## Error Codes

| Code | Description |
|------|-------------|
| `LS_ERROR` | Directory listing error |
| `CAT_ERROR` | File read error |
| `GREP_ERROR` | Pattern search error |
| `HEAD_ERROR` | Head operation error |
| `TAIL_ERROR` | Tail operation error |
| `WC_ERROR` | Word count error |
| `TOUCH_ERROR` | Touch operation error |
| `MKDIR_ERROR` | Directory creation error |
| `RMDIR_ERROR` | Directory removal error |
| `CP_ERROR` | Copy operation error |
| `MV_ERROR` | Move operation error |
| `RM_ERROR` | Removal operation error |
| `FIND_ERROR` | Find operation error |
| `CHMOD_ERROR` | Permission change error |
| `CHOWN_ERROR` | Ownership change error |
| `ANALYSIS_FAILED` | Analysis operation error |
| `FILE_NOT_FOUND` | File does not exist |
| `PATH_NOT_FOUND` | Path does not exist |
| `PERMISSION_DENIED` | Insufficient permissions |
| `INVALID_INPUT` | Invalid input parameters |
| `IO_ERROR` | Generic I/O error |
| `JSON_ERROR` | JSON serialization error |

## Parsing JSONL

### Python

```python
import json
from datetime import datetime

def parse_jsonl(file_or_stream):
    """Parse JSONL from file or stream."""
    results = []
    for line in file_or_stream:
        record = json.loads(line.strip())
        results.append(record)
    return results

# Filter by type
def filter_by_type(records, record_type):
    return [r for r in records if r.get('type') == record_type]

# Handle errors
def check_errors(records):
    return [r for r in records if r.get('type') == 'error']

# Extract data
def extract_data(records):
    return [r.get('data') for r in records if r.get('type') == 'result']
```

### Node.js

```javascript
async function* parseJSONL(stream) {
  const readline = require('readline');
  const rl = readline.createInterface({ input: stream });

  for await (const line of rl) {
    if (line.trim()) {
      yield JSON.parse(line);
    }
  }
}

// Filter by type
async function filterByType(stream, type) {
  const results = [];
  for await (const record of parseJSONL(stream)) {
    if (record.type === type) {
      results.push(record);
    }
  }
  return results;
}

// Check for errors
async function checkErrors(stream) {
  const errors = [];
  for await (const record of parseJSONL(stream)) {
    if (record.type === 'error') {
      errors.push(record);
    }
  }
  return errors;
}
```

### Rust

```rust
use serde_json::Value;
use std::io::{BufRead, BufReader};

fn parse_jsonl<R: BufRead>(reader: R) -> Vec<Value> {
    reader.lines()
        .filter_map(|line| line.ok())
        .filter_map(|line| serde_json::from_str::<Value>(&line).ok())
        .collect()
}

// Filter by type
fn filter_by_type(records: &[Value], record_type: &str) -> Vec<&Value> {
    records.iter()
        .filter(|r| r["type"] == record_type)
        .collect()
}

// Check for errors
fn check_errors(records: &[Value]) -> Vec<&Value> {
    filter_by_type(records, "error")
}
```

### Bash/JQ

```bash
# Count records
ai-ls | jq 'length'

# Filter by type
ai-ls | jq 'select(.type == "file_entry")'

# Extract specific fields
ai-ls | jq '.path, .size'

# Check for errors
ai-ls | jq 'select(.type == "error")'

# Sum file sizes
ai-ls | jq 'select(.type == "file_entry") | .size' | awk '{s+=$1} END {print s}'
```

## Utility-Specific Schemas

### ai-ls

```json
{
  "type": "file_entry",
  "timestamp": "ISO 8601",
  "path": "string",
  "size": "number (bytes)",
  "modified": "ISO 8601",
  "is_dir": "boolean",
  "is_symlink": "boolean",
  "is_hidden": "boolean",
  "permissions": "string (octal)"
}
```

### ai-cat

```json
{
  "type": "line",
  "timestamp": "ISO 8601",
  "file": "string",
  "line_number": "number",
  "content": "string"
}
```

### ai-grep

```json
{
  "type": "match_record",
  "timestamp": "ISO 8601",
  "file": "string",
  "line_number": "number",
  "line_content": "string",
  "match_start": "number",
  "match_end": "number"
}
```

### ai-wc

```json
{
  "type": "info",
  "data": {
    "file": "string",
    "operation": "wc",
    "lines": "number",
    "words": "number",
    "bytes": "number",
    "chars": "number",
    "max_line_length": "number"
  }
}
```

### ai-analyze

Classification:
```json
{
  "type": "result",
  "data": {
    "type": "classification",
    "file": "string",
    "file_type": "string",
    "mime_type": "string",
    "encoding": "string",
    "is_binary": "boolean",
    "language": "string or null",
    "confidence": "number (0-1)"
  }
}
```

Analysis:
```json
{
  "type": "result",
  "data": {
    "type": "analysis",
    "file": "string",
    "total_patterns": "number",
    "patterns_by_type": {
      "email": "number",
      "url": "number",
      "ip": "number"
    },
    "statistics": {
      "lines": "number",
      "words": "number",
      "characters": "number",
      "bytes": "number",
      "avg_line_length": "number",
      "max_line_length": "number",
      "whitespace_ratio": "number",
      "entropy": "number"
    },
    "issues": ["string"]
  }
}
```

## Streaming Patterns

### Process Line by Line

```python
import sys
import json

for line in sys.stdin:
    record = json.loads(line)
    if record['type'] == 'result':
        process(record['data'])
```

### Collect All Records

```python
records = [json.loads(line) for line in sys.stdin]
```

### Stream Filter

```python
import json
import sys

for line in sys.stdin:
    record = json.loads(line)
    if record.get('size', 0) > 1000:
        print(json.dumps(record))
```

## Best Practices

1. **Always check type field**: Different record types have different schemas
2. **Handle errors**: Check for error records even when expecting results
3. **Use timestamps**: All records include UTC timestamps
4. **Parse defensively**: Not all fields are present in all record types
5. **Stream for large outputs**: Process line-by-line instead of loading all records

## Binary Data

Binary data is encoded as base64 in JSONL:

```json
{
  "type": "result",
  "data": {
    "binary_data": "SGVsbG8sIHdvcmxkIQ=="
  }
}
```

Decode:

```python
import base64
decoded = base64.b64decode(record['data']['binary_data'])
```

```javascript
const decoded = Buffer.from(record.data.binary_data, 'base64');
```

```rust
use base64::Engine;
let decoded = base64::engine::general_purpose::STANDARD.decode(&data)?;
```

## Extended Data

Some utilities include extended data in the `data` field:

```json
{
  "type": "result",
  "timestamp": "2026-01-19T12:00:00Z",
  "data": {
    "custom_field": "value",
    "nested": {
      "array": [1, 2, 3]
    }
  }
}
```

Always check the utility documentation for specific `data` field schemas.
