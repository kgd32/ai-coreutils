# ai-grep - Search Text Patterns

Search for patterns in files with structured JSONL output, context support, and async concurrent processing.

## Description

`ai-grep` is a modern implementation of the `grep` utility designed for AI agents. It searches text patterns in files with memory-mapped access, context handling, and concurrent processing capabilities.

## Usage

```bash
ai-grep [OPTIONS] <PATTERN> <PATHS>...
```

## Options

| Option | Short | GNU Equivalent | Description |
|--------|-------|----------------|-------------|
| `--recursive` | `-r` | `-r` | Recursive directory search |
| `--async` | `-a` | *New* | Enable async concurrent processing |
| `--max-concurrent` | `-j` | *New* | Max concurrent operations (default: 10) |
| `--line-number` | `-n` | `-n` | Show line numbers |
| `--count` | `-c` | `-c` | Show count of matches |
| `--ignore-case` | `-i` | `-i` | Case insensitive search |
| `--invert-match` | `-v` | `-v` | Show non-matching lines |
| `--files-with-matches` | `-l` | `-l` | List matching files only |
| `--files-without-match` | `-L` | `-L` | List non-matching files only |
| `--only-matching` | `-o` | `-o` | Show only matching part |
| `--fixed-strings` | `-F` | `-F` | Fixed strings (not regex) |
| `--extended-regex` | `-E` | `-E` | Extended regex |
| `--after-context` | `-A` | `-A` | Show NUM lines after match |
| `--before-context` | `-B` | `-B` | Show NUM lines before match |
| `--context` | `-C` | `-C` | Show NUM lines around match |

## AI Enhancements

- **Memory Mapping**: Fast pattern search in large files
- **Async Processing**: Concurrent file processing with tokio
- **Context Handling**: Structured context around matches
- **Multiple Output Modes**: Count, files, matches, or combinations

## JSONL Output Format

### Match Record

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

### Count Output

```json
{
  "type": "result",
  "data": {
    "file": "/path/to/file.txt",
    "match_count": 15
  }
}
```

### Files With Matches

```json
{
  "type": "result",
  "data": {
    "file": "/path/to/file.txt"
  }
}
```

### Error Output

```json
{
  "type": "error",
  "code": "GREP_ERROR",
  "message": "Failed to search file: Permission denied",
  "timestamp": "2026-01-19T12:00:00Z"
}
```

## Examples

### Search for pattern in files

```bash
ai-grep "pattern" file.txt
```

### Recursive directory search

```bash
ai-grep -r "TODO" ./src
```

### Case insensitive search with line numbers

```bash
ai-grep -i -n "error" *.log
```

### Show context around matches

```bash
ai-grep -C 3 "function" src/
```

### Count matches per file

```bash
ai-grep -c "import" *.py
```

### List files containing pattern

```bash
ai-grep -l "TODO" ./src
```

### Show only matching parts

```bash
ai-grep -o "\b[A-Z]{2,}\b" text.txt
```

### Invert match (show non-matching)

```bash
ai-grep -v "comment" code.py
```

### Async concurrent processing

```bash
ai-grep --async --max-concurrent 20 -r "error" /var/log
```

### Before and after context

```bash
ai-grep -A 5 -B 2 "exception" app.log
```

## Performance Considerations

### Sync vs Async Mode

```bash
# Single file or directory - sync is sufficient
ai-grep "pattern" file.txt

# Many files - use async for 3x improvement
ai-grep --async -r "pattern" /large/directory

# Network storage - increase concurrency
ai-grep --async --max-concurrent 50 -r "pattern" /network/drive
```

### Memory Mapping

- Files > 10MB: Automatically uses memory mapping (10x faster)
- Pattern search: SIMD-accelerated when available
- Large result sets: Streaming output to minimize memory

## GNU Compatibility

`ai-grep` maintains compatibility with GNU `grep` core options:

| Feature | Status |
|---------|--------|
| Basic pattern search | ✅ Full support |
| Recursive search | ✅ Full support |
| Context (-A, -B, -C) | ✅ Full support |
| Count mode (-c) | ✅ Full support |
| Invert match (-v) | ✅ Full support |
| Files with matches (-l) | ✅ Full support |
| Only matching (-o) | ✅ Full support |
| Case insensitive (-i) | ✅ Full support |
| Fixed strings (-F) | ✅ Full support |
| Extended regex (-E) | ✅ Full support |
| Async mode | ✅ New feature |

## Pattern Examples

### Email addresses
```bash
ai-grep -o "[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}" emails.txt
```

### IP addresses
```bash
ai-grep -o "\b(?:\d{1,3}\.){3}\d{1,3}\b" logs.txt
```

### Phone numbers (US)
```bash
ai-grep -o "\b\d{3}-\d{3}-\d{4}\b" contacts.txt
```

### UUIDs
```bash
ai-grep -o "[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}" data.txt
```

## Exit Codes

- `0`: Matches found
- `1`: No matches found
- `2`: Error occurred

## See Also

- [ai-analyze](ai-analyze.md) - AI-powered pattern detection
- [ai-find](ai-find.md) - Search directory tree
- [ML Integration](../ml-integration.md) - Advanced pattern detection
