# ai-rmdir - Remove Empty Directories

Remove empty directories with JSONL structured output.

## Description

`ai-rmdir` is a modern implementation of the `rmdir` utility designed for AI agents. It removes empty directories with batch operation support.

## Usage

```bash
ai-rmdir [OPTIONS] <DIRECTORY>...
```

## Options

| Option | Short | GNU Equivalent | Description |
|--------|-------|----------------|-------------|
| `--ignore-fail-on-non-empty` | | `--ignore-fail-on-non-empty` | Ignore failures for non-empty directories |
| `--parents` | `-p` | `-p` | Remove parent directories if they become empty |
| `--verbose` | `-v` | `-v` | Verbose output |

## AI Enhancements

- **Batch Operations**: Remove multiple directories
- **JSONL Output**: Structured operation results
- **Parent Removal**: Clean up empty parent directories

## JSONL Output Format

### Operation Info

```json
{
  "type": "info",
  "data": {
    "directory": "/path/to/empty_dir",
    "operation": "removed"
  }
}
```

### Error Output

```json
{
  "type": "error",
  "code": "RMDIR_ERROR",
  "message": "Directory not empty: /path/to/dir",
  "directory": "/path/to/dir"
}
```

## Examples

### Remove single empty directory

```bash
ai-rmdir empty_directory
```

### Remove multiple empty directories

```bash
ai-rmdir dir1 dir2 dir3
```

### Remove with parent cleanup

```bash
ai-rmdir -p parent/child/grandchild
```

### Ignore non-empty errors

```bash
ai-rmdir --ignore-fail-on-non-empty *
```

### Verbose output

```bash
ai-rmdir -v empty_dir
```

## Use Cases

### Cleanup Build Artifacts

```bash
# Remove empty build directories
ai-rmdir build/debug
ai-rmdir build/release
```

### Cleanup Empty Logs

```bash
# Remove empty log directories
ai-rmdir logs/old/*
```

### Parent Directory Cleanup

```bash
# Remove entire empty hierarchy
ai-rmdir -p a/b/c/d/e
```

## GNU Compatibility

| Feature | Status |
|---------|--------|
| Basic removal | ✅ Full support |
| Parent removal (-p) | ✅ Full support |
| Non-empty ignore | ✅ Full support |
| Verbose output | ✅ Full support |

## Exit Codes

- `0`: Success
- `1`: Error occurred

## See Also

- [ai-mkdir](ai-mkdir.md) - Create directories
- [ai-rm](ai-rm.md) - Remove files and directories
