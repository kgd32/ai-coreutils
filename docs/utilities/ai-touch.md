# ai-touch - Create Files or Update Timestamps

Update file access and modification times, or create files if they don't exist.

## Description

`ai-touch` is a modern implementation of the `touch` utility designed for AI agents. It creates empty files or updates timestamps with batch operation support and progress tracking.

## Usage

```bash
ai-touch [OPTIONS] <FILE>...
```

## Options

| Option | Short | GNU Equivalent | Description |
|--------|-------|----------------|-------------|
| `--no-create` | | `-c` | Don't create files if they don't exist |
| `--access-only` | `-a` | `-a` | Change only the access time |
| `--modification-only` | `-m` | `-m` | Change only the modification time |
| `--reference` | `-r` | `-r` | Use reference file's times |
| `--date` | `-d` | `-d` | Set time to specified value |
| `--verbose` | `-v` | `-v` | Verbose output |

## AI Enhancements

- **Batch Operations**: Process multiple files with progress tracking
- **JSONL Output**: Structured operation results
- **Error Recovery**: Continue on errors with detailed reporting

## JSONL Output Format

### Progress Update

```json
{
  "type": "progress",
  "current": 1,
  "total": 5,
  "message": "Processing: file.txt"
}
```

### Verbose Info

```json
{
  "type": "info",
  "data": {
    "file": "new_file.txt",
    "operation": "created"
  }
}
```

### Summary

```json
{
  "type": "info",
  "data": {
    "operation": "touch_summary",
    "total_files": 5,
    "successful": 4,
    "errors": 1
  }
}
```

### Error Output

```json
{
  "type": "error",
  "code": "TOUCH_ERROR",
  "message": "Failed to touch file: Permission denied",
  "file": "/protected/file.txt"
}
```

## Examples

### Create empty file

```bash
ai-touch new_file.txt
```

### Update timestamp

```bash
ai-touch existing_file.txt
```

### Create multiple files

```bash
ai-touch file1.txt file2.txt file3.txt
```

### Don't create if doesn't exist

```bash
ai-touch -c maybe_exists.txt
```

### Verbose output

```bash
ai-touch -v *.txt
```

### Use reference file timestamp

```bash
ai-touch -r reference.txt target.txt
```

### Create files in batch

```bash
ai-touch file_{1..100}.txt
```

## Use Cases

### File Creation

```bash
# Create placeholder files
ai-touch placeholder1.txt placeholder2.txt

# Create log files
ai-touch app.log error.log debug.log

# Batch create
ai-touch data_{1..10}.csv
```

### Timestamp Management

```bash
# Update to current time
ai-touch old_file.dat

# Mark files as processed
ai-touch processed_*.dat
```

### Build Systems

```bash
# Create marker files
ai-touch .build_complete

# Touch dependencies
ai-touch -r header.c dependency.o
```

## GNU Compatibility

| Feature | Status |
|---------|--------|
| Create files | ✅ Full support |
| Update timestamps | ✅ Full support |
| No-create option | ✅ Full support |
| Access time only | ✅ Full support |
| Modification time only | ✅ Full support |
| Reference file | ✅ Full support |
| Date string | ✅ Full support |

## Exit Codes

- `0`: Success (all files processed)
- `1`: One or more files failed

## See Also

- [ai-mkdir](ai-mkdir.md) - Create directories
- [ai-rm](ai-rm.md) - Remove files
