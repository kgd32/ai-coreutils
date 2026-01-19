# ai-tail - Output Last Part of Files

Output the last part of files with structured JSONL output and memory-mapped access.

## Description

`ai-tail` is a modern implementation of the `tail` utility designed for AI agents. It outputs the last part of files using memory mapping for efficient access to large files, with follow mode support.

## Usage

```bash
ai-tail [OPTIONS] [FILE]...
```

## Options

| Option | Short | GNU Equivalent | Description |
|--------|-------|----------------|-------------|
| `--lines` | `-n` | `-n` | Number of lines to show (default: 10) |
| `--bytes` | `-c` | `-c` | Number of bytes to show |
| `--follow` | `-f` | `-f` | Follow file as it grows |
| `--quiet` | `-q` | `-q` | Don't print file headers |
| `--verbose` | `-v` | `-v` | Always print file headers |
| `--zero-terminated` | `-z` | `-z` | Line delimiter is NUL, not newline |

## AI Enhancements

- **Memory Mapping**: Efficient access to large files from the end
- **JSONL Output**: Structured progress and metadata
- **Follow Mode**: Monitor file changes in real-time

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

### Operation Info

```json
{
  "type": "info",
  "data": {
    "file": "file.txt",
    "operation": "tail",
    "unit": "lines",
    "count": 10,
    "bytes_read": 512
  }
}
```

## Examples

### Show last 10 lines

```bash
ai-tail file.txt
```

### Show last 20 lines

```bash
ai-tail -n 20 file.txt
```

### Show last 100 bytes

```bash
ai-tail -c 100 file.txt
```

### Follow file as it grows

```bash
ai-tail -f /var/log/app.log
```

### Multiple files with headers

```bash
ai-tail -v *.log
```

### Multiple files without headers

```bash
ai-tail -q *.log
```

### Show all lines (effectively cat with progress)

```bash
ai-tail -n +1 file.txt
```

### Zero-terminated lines

```bash
ai-tail -z -n 5 data.txt
```

## Use Cases

### Log Monitoring

```bash
# Monitor application logs in real-time
ai-tail -f /var/log/app.log

# Monitor multiple log files
ai-tail -f /var/log/*.log
```

### Check Recent Errors

```bash
# Show last 50 lines of error log
ai-tail -n 50 error.log

# Find recent patterns
ai-tail -n 100 app.log | ai-grep "ERROR"
```

### Data Processing

```bash
# Get last N records
ai-tail -n 1000 data.csv

# Process from end of file
ai-tail -c 1MB large_file.bin
```

## Performance Considerations

- **Memory Mapping**: Reads from end of file, no need to scan entire file
- **Large Files**: Efficient even with multi-gigabyte files
- **Follow Mode**: Polling based, minimal CPU usage

## GNU Compatibility

| Feature | Status |
|---------|--------|
| Line count | ✅ Full support |
| Byte count | ✅ Full support |
| Follow mode | ✅ Full support |
| Headers | ✅ Full support |
| Zero-terminated | ✅ Full support |

## Exit Codes

- `0`: Success
- `1`: Error occurred

## See Also

- [ai-head](ai-head.md) - Output first part of files
- [ai-cat](ai-cat.md) - Display entire files
- [ai-grep](ai-grep.md) - Search patterns in files
