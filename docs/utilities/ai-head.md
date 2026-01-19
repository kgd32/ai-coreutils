# ai-head - Output First Part of Files

Output the first part of files with structured JSONL output and memory-mapped access.

## Description

`ai-head` is a modern implementation of the `head` utility designed for AI agents. It outputs the first part of files using memory mapping for efficient access to large files.

## Usage

```bash
ai-head [OPTIONS] [FILE]...
```

## Options

| Option | Short | GNU Equivalent | Description |
|--------|-------|----------------|-------------|
| `--lines` | `-n` | `-n` | Number of lines to show (default: 10) |
| `--bytes` | `-c` | `-c` | Number of bytes to show |
| `--quiet` | `-q` | `-q` | Don't print file headers |
| `--verbose` | `-v` | `-v` | Always print file headers |
| `--zero-terminated` | `-z` | `-z` | Line delimiter is NUL, not newline |

## AI Enhancements

- **Memory Mapping**: Efficient access to large files
- **JSONL Output**: Structured progress and metadata
- **Batch Processing**: Handle multiple files with progress tracking

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
    "operation": "head",
    "unit": "lines",
    "count": 10,
    "bytes_read": 512
  }
}
```

### Error Output

```json
{
  "type": "error",
  "code": "HEAD_ERROR",
  "message": "Failed to read file: Permission denied",
  "file": "file.txt"
}
```

## Examples

### Show first 10 lines

```bash
ai-head file.txt
```

### Show first 20 lines

```bash
ai-head -n 20 file.txt
```

### Show first 100 bytes

```bash
ai-head -c 100 file.txt
```

### Show first 1KB

```bash
ai-head -c 1024 file.txt
```

### Multiple files with headers

```bash
ai-head -v file1.txt file2.txt file3.txt
```

### Multiple files without headers

```bash
ai-head -q *.log
```

### Zero-terminated lines

```bash
ai-head -z -n 5 data.txt
```

### From stdin

```bash
cat large_file.txt | ai-head -n 100
```

## Performance Considerations

- **Memory Mapping**: Automatic for files > 1MB
- **Large Files**: Zero-copy operations minimize memory usage
- **Multiple Files**: Sequential processing with progress tracking

## GNU Compatibility

| Feature | Status |
|---------|--------|
| Line count | ✅ Full support |
| Byte count | ✅ Full support |
| Headers | ✅ Full support |
| Zero-terminated | ✅ Full support |
| Stdin support | ✅ Full support |

## Exit Codes

- `0`: Success
- `1`: Error occurred

## See Also

- [ai-tail](ai-tail.md) - Output last part of files
- [ai-cat](ai-cat.md) - Display entire files
