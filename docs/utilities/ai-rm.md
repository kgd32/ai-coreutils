# ai-rm - Remove Files and Directories

Remove files and directories with JSONL structured output.

## Description

`ai-rm` is a modern implementation of the `rm` utility designed for AI agents. It removes files and directories with batch operation support and progress tracking.

## Usage

```bash
ai-rm [OPTIONS] <FILE>...
```

## Options

| Option | Short | GNU Equivalent | Description |
|--------|-------|----------------|-------------|
| `--recursive` | `-r` / `-R` | `-r` | Remove directories recursively |
| `--force` | `-f` | `-f` | Ignore nonexistent files, never prompt |
| `--interactive` | `-i` | `-i` | Prompt before removal |
| `--verbose` | `-v` | `-v` | Verbose output |
| `--dir` | `-d` | `-d` | Remove empty directories |

## AI Enhancements

- **Batch Operations**: Remove multiple files efficiently
- **JSONL Output**: Structured operation results
- **Progress Tracking**: Real-time status updates

## JSONL Output Format

### Removal Info

```json
{
  "type": "info",
  "data": {
    "operation": "removed",
    "path": "/path/to/file.txt",
    "type": "file"
  }
}
```

### Error Output

```json
{
  "type": "error",
  "code": "RM_ERROR",
  "message": "Failed to remove: Permission denied",
  "path": "/path/to/protected.txt"
}
```

## Examples

### Remove single file

```bash
ai-rm file.txt
```

### Remove multiple files

```bash
ai-rm file1.txt file2.txt file3.txt
```

### Remove directory recursively

```bash
ai-rm -r directory/
```

### Force remove (no prompts)

```bash
ai-rm -f file.txt
```

### Interactive removal

```bash
ai-rm -i important.txt
```

### Verbose output

```bash
ai-rm -v *.log
```

### Remove empty directory

```bash
ai-rm -d empty_dir/
```

### Force recursive directory removal

```bash
ai-rm -rf build/
```

## Use Cases

### Cleanup Build Artifacts

```bash
# Remove build directory
ai-rm -rf build/

# Remove object files
ai-rm *.o
```

### Log Rotation

```bash
# Remove old logs
ai-rm *.log.2023

# Remove log directory
ai-rm -rf logs/2023/
```

### Temporary Files

```bash
# Remove temp files
ai-rm -rf /tmp/myapp_cache/

# Remove cache files
ai-rm cache/*.*
```

### Safe Cleanup

```bash
# Interactive for important files
ai-rm -i important_docs/

# Verbose to see what's being removed
ai-rm -v old_data/*
```

## Safety Considerations

- **Double-check paths**: Verify paths before running `ai-rm -rf`
- **Use interactive mode**: `-i` flag for important deletions
- **Test first**: Use `ai-ls` to verify files before removal

## GNU Compatibility

| Feature | Status |
|---------|--------|
| Basic removal | ✅ Full support |
| Recursive | ✅ Full support |
| Force | ✅ Full support |
| Interactive | ✅ Full support |
| Verbose | ✅ Full support |
| Empty directory | ✅ Full support |

## Exit Codes

- `0`: Success
- `1`: Error occurred

## See Also

- [ai-cp](ai-cp.md) - Copy files
- [ai-mv](ai-mv.md) - Move files
