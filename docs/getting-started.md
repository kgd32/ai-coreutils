# Getting Started with AI-Coreutils

Welcome to AI-Coreutils! This guide will help you install, configure, and start using AI-Coreutils.

## Table of Contents

1. [Installation](#installation)
2. [Quick Start](#quick-start)
3. [Basic Usage](#basic-usage)
4. [JSONL Output](#jsonl-output)
5. [Common Patterns](#common-patterns)
6. [Next Steps](#next-steps)

## Installation

### From Crates.io (Recommended)

```bash
cargo install ai-coreutils
```

This installs all utilities as standalone binaries:
- `ai-ls`, `ai-cat`, `ai-grep`
- `ai-head`, `ai-tail`, `ai-wc`
- `ai-cp`, `ai-mv`, `ai-rm`
- `ai-touch`, `ai-mkdir`, `ai-rmdir`
- `ai-find`, `ai-chmod`, `ai-chown`
- `ai-analyze`

### From Source

```bash
git clone https://github.com/your-org/ai-coreutils
cd ai-coreutils
cargo install --path .
```

### Python Bindings

```bash
pip install ai-coreutils
```

### Node.js Bindings

```bash
npm install ai-coreutils
```

## Quick Start

### Your First Command

List directory contents with AI-friendly output:

```bash
ai-ls
```

Output (JSONL format):
```json
{"type":"file_entry","timestamp":"2026-01-19T12:00:00Z","path":"file.txt","size":1024,"modified":"2026-01-19T10:30:00Z","is_dir":false,"is_symlink":false,"permissions":"644"}
```

### Search for Patterns

```bash
ai-grep "TODO" ./src
```

### Analyze Files

```bash
ai-analyze --patterns document.txt
```

## Basic Usage

### List Files

```bash
# Basic listing
ai-ls

# Long format with details
ai-ls -l

# Human-readable sizes
ai-ls -lh

# Recursive directory listing
ai-ls -R
```

### View Files

```bash
# View file contents
ai-cat file.txt

# Number lines
ai-cat -n file.txt

# View with async processing
ai-cat --async *.log
```

### Search Files

```bash
# Search for pattern
ai-grep "error" app.log

# Recursive search
ai-grep -r "TODO" ./src

# Case insensitive
ai-grep -i "error" *.log

# With context
ai-grep -C 3 "function" src/
```

### File Operations

```bash
# Copy files
ai-cp source.txt dest.txt

# Move files
ai-mv old.txt new.txt

# Remove files
ai-rm unwanted.txt

# Create directories
ai-mkdir -p new/dir/path

# Remove directories
ai-rm -r directory/
```

## JSONL Output

All AI-Coreutils output JSONL (JSON Lines) format - one JSON object per line. This makes it easy for AI agents to parse results.

### Basic Structure

```json
{"type":"result","timestamp":"2026-01-19T12:00:00Z","data":{...}}
{"type":"error","code":"ERROR_CODE","message":"Error message","timestamp":"2026-01-19T12:00:00Z"}
{"type":"info","data":{...}}
{"type":"progress","current":5,"total":10,"message":"Processing..."}
```

### Parsing JSONL Output

#### Python

```python
import json

for line in output:
    record = json.loads(line)
    if record['type'] == 'result':
        print(record['data'])
    elif record['type'] == 'error':
        print(f"Error: {record['message']}")
```

#### Node.js

```javascript
const readline = require('readline');

async function parseJSONL(stream) {
  const rl = readline.createInterface({ input: stream });

  for await (const line of rl) {
    const record = JSON.parse(line);
    if (record.type === 'result') {
      console.log(record.data);
    }
  }
}
```

#### Rust

```rust
use serde_json::Value;

for line in output.lines() {
    let record: Value = serde_json::from_str(&line)?;
    match record["type"].as_str() {
        Some("result") => println!("Result: {}", record["data"]),
        Some("error") => eprintln!("Error: {}", record["message"]),
        _ => {}
    }
}
```

## Common Patterns

### Pipeline Processing

```bash
# List files, filter, and process
ai-ls -R ./src | jq 'select(.size > 1000)'
ai-grep "TODO" ./src | jq 'select(.line_number > 100)'
```

### Batch Operations

```bash
# Process multiple files concurrently
ai-cat --async --max-concurrent 20 *.log

# Recursive analysis
ai-analyze -r --patterns ./src
```

### Error Handling

```bash
# Check for errors in output
ai-grep "pattern" file.txt | jq 'select(.type == "error")'

# Count successful operations
ai-ls | jq 'select(.type == "result")' | wc -l
```

## Library Usage

### Rust

```rust
use ai_coreutils::{SafeMemoryAccess, PatternDetector};

// Memory access
let mem = SafeMemoryAccess::new("file.txt")?;
let data = mem.get(0, 1024)?;

// Pattern detection
let detector = PatternDetector::new();
let matches = detector.detect_patterns("Contact test@example.com");
```

### Python

```python
from ai_coreutils import SafeMemoryAccess, PatternDetector

# Memory access
mem = SafeMemoryAccess("file.txt")
data = mem.get(0, 1024)

# Pattern detection
detector = PatternDetector()
matches = detector.detect_patterns("Contact test@example.com")
```

### Node.js

```javascript
const { MemoryAccess, PatternDetectorWrapper } = require('ai-coreutils');

// Memory access
const mem = new MemoryAccess('file.txt');
const data = mem.get(0, 1024);

// Pattern detection
const detector = new PatternDetectorWrapper();
const matches = detector.detectPatterns('Contact test@example.com');
```

## Performance Tips

1. **Use Async Mode**: For multiple files, enable async mode
   ```bash
   ai-cat --async *.log
   ai-grep --async -r "pattern" /large/dir
   ```

2. **Memory Mapping**: Large files automatically use memory mapping (10x faster)

3. **SIMD Operations**: Text processing is automatically accelerated when available

4. **Batch Operations**: Process multiple files in one command
   ```bash
   ai-wc *.rs
   ai-ls dir1/ dir2/ dir3/
   ```

## Next Steps

- [Architecture](architecture.md) - Understanding the system design
- [API Reference](api-reference.md) - Complete library API
- [JSONL Format](jsonl-format.md) - Output format specification
- [Examples](examples/) - Practical use cases

## Getting Help

- GitHub Issues: https://github.com/your-org/ai-coreutils/issues
- Documentation: https://docs.ai-coreutils.dev
- CLI Help: `ai-<utility> --help`

## Troubleshooting

### Command Not Found

```bash
# Check installation
cargo install --list | grep ai-coreutils

# Reinstall if needed
cargo install ai-coreutils --force
```

### Permission Errors

```bash
# Use sudo for system-wide installation
sudo cargo install ai-coreutils
```

### Python Import Errors

```bash
# Ensure Python bindings are installed
pip install ai-coreutils --upgrade
```
