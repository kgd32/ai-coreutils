# ai-mkdir - Create Directories

Create directories with batch operation support and JSONL structured output.

## Description

`ai-mkdir` is a modern implementation of the `mkdir` utility designed for AI agents. It creates directories with parent directory creation support and progress tracking.

## Usage

```bash
ai-mkdir [OPTIONS] <DIRECTORY>...
```

## Options

| Option | Short | GNU Equivalent | Description |
|--------|-------|----------------|-------------|
| `--parents` | `-p` | `-p` | Create parent directories as needed |
| `--mode` | `-m` | `-m` | Set file mode (permissions) |
| `--verbose` | `-v` | `-v` | Verbose output |

## AI Enhancements

- **Batch Operations**: Create multiple directories efficiently
- **JSONL Output**: Structured operation results
- **Progress Tracking**: Real-time progress updates

## JSONL Output Format

### Progress Update

```json
{
  "type": "progress",
  "current": 1,
  "total": 5,
  "message": "Creating: /path/to/dir"
}
```

### Verbose Info

```json
{
  "type": "info",
  "data": {
    "directory": "/path/to/dir",
    "operation": "created",
    "path": "/path/to/dir",
    "is_dir": true
  }
}
```

### Summary

```json
{
  "type": "info",
  "data": {
    "operation": "mkdir_summary",
    "total_directories": 5,
    "successful": 5,
    "errors": 0
  }
}
```

## Examples

### Create single directory

```bash
ai-mkdir new_directory
```

### Create nested directories

```bash
ai-mkdir -p path/to/nested/directory
```

### Create multiple directories

```bash
ai-mkdir dir1 dir2 dir3
```

### Verbose output

```bash
ai-mkdir -v new_dir
```

### Create directories with specific mode (Unix)

```bash
ai-mkdir -m 755 public_dir
```

### Batch create

```bash
ai-mkdir -p data/{2020..2025}/{01..12}
```

## Use Cases

### Project Structure

```bash
# Create project directories
ai-mkdir -p src/{bin,lib,tests}
ai-mkdir -p docs examples

# Create data directories
ai-mkdir -p data/{raw,processed,archive}
```

### Build Directories

```bash
# Build output directories
ai-mkdir -p build/{debug,release}
```

### Log Directories

```bash
# Create log directories
ai-mkdir -p logs/{app,error,debug}
```

## GNU Compatibility

| Feature | Status |
|---------|--------|
| Basic creation | ✅ Full support |
| Parent creation (-p) | ✅ Full support |
| Mode setting (-m) | ✅ Unix support |
| Verbose output | ✅ Full support |

## Exit Codes

- `0`: Success
- `1`: Error occurred

## See Also

- [ai-rmdir](ai-rmdir.md) - Remove directories
- [ai-touch](ai-touch.md) - Create files
