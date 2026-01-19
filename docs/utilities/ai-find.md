# ai-find - Search Directory Tree

Search for files in a directory hierarchy with JSONL structured output.

## Description

`ai-find` is a modern implementation of the `find` utility designed for AI agents. It searches for files in directory hierarchies with various filtering options.

## Usage

```bash
ai-find [OPTIONS] [PATH]... [EXPRESSIONS]
```

## Options

| Option | Description |
|--------|-------------|
| `--name <PATTERN>` | Match files by name pattern |
| `--type <TYPE>` | Filter by file type (f=file, d=dir, l=symlink) |
| `--size <SIZE>` | Filter by file size (+N=N or greater, -N=less than N) |
| `--mtime <DAYS>` | Modified time in days (+N=more than N days ago) |
| `--perm <MODE>` | Match by permissions (octal) |
| `--user <USER>` | Match by owner user |
| `--group <GROUP>` | Match by owner group |
| `--empty` | Match empty files/directories |
| `--executable` | Match executable files |

## AI Enhancements

- **JSONL Output**: Structured file listings with metadata
- **Batch Operations**: Efficient directory traversal
- **Rich Metadata**: Detailed file information in results

## JSONL Output Format

### File Found

```json
{
  "type": "result",
  "data": {
    "path": "/path/to/file.txt",
    "type": "file",
    "size": 1024,
    "modified": "2026-01-19T10:30:00Z",
    "permissions": "644"
  }
}
```

## Examples

### Find all files in directory

```bash
ai-find /path/to/dir
```

### Find by name pattern

```bash
ai-find /path -name "*.txt"
```

### Find directories only

```bash
ai-find /path -type d
```

### Find large files

```bash
ai-find /path -size +100M
```

### Find files modified recently

```bash
ai-find /path -mtime -7
```

### Find executable files

```bash
ai-find /path -type f -executable
```

### Find empty files

```bash
ai-find /path -type f -empty
```

### Multiple conditions

```bash
ai-find /path -name "*.log" -size +1M
```

## Use Cases

### File Discovery

```bash
# Find all Python files
ai-find . -name "*.py"

# Find configuration files
ai-find /etc -name "*.conf"
```

### Disk Usage Analysis

```bash
# Find large files
ai-find / -size +1G

# Find old files
ai-find /data -mtime +365
```

### Cleanup

```bash
# Find old logs
ai-find /var/log -name "*.log" -mtime +30

# Find temp files
ai-find /tmp -name "*~"
```

### System Administration

```bash
# Find executable files
ai-find /bin -type f -executable

# Find world-writable files
ai-find / -perm -o+w
```

## Size Formats

- `b` - 512-byte blocks
- `c` - bytes
- `k` - kilobytes
- `M` - megabytes
- `G` - gigabytes

Examples:
- `--size +100M` - files larger than 100MB
- `--size -1k` - files smaller than 1KB
- `--size 100` - files exactly 100 512-byte blocks

## GNU Compatibility

| Feature | Status |
|---------|--------|
| Name matching | ✅ Full support |
| Type filtering | ✅ Full support |
| Size filtering | ✅ Full support |
| Time filtering | ✅ Full support |
| Permissions | ✅ Full support |
| User/group | ✅ Unix support |

## Exit Codes

- `0`: Success (all paths processed)
- `1`: Error occurred

## See Also

- [ai-ls](ai-ls.md) - List directory contents
- [ai-grep](ai-grep.md) - Search file contents
