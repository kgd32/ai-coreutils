# ai-mv - Move/Rename Files

Move or rename files and directories with JSONL structured output.

## Description

`ai-mv` is a modern implementation of the `mv` utility designed for AI agents. It moves and renames files with batch operation support and progress tracking.

## Usage

```bash
ai-mv [OPTIONS] <SOURCE>... <DESTINATION>
```

## Options

| Option | Short | GNU Equivalent | Description |
|--------|-------|----------------|-------------|
| `--force` | `-f` | `-f` | Force overwrite without prompt |
| `--interactive` | `-i` | `-i` | Interactive prompt before overwrite |
| `--no-clobber` | `-n` | `-n` | Don't overwrite existing files |
| `--verbose` | `-v` | `-v` | Verbose output |
| `--backup` | `-b` | `-b` | Create backup before overwrite |
| `--suffix` | `-S` | `-S` | Override backup suffix |

## AI Enhancements

- **Batch Operations**: Move multiple files efficiently
- **JSONL Output**: Structured operation results
- **Progress Tracking**: Real-time status updates

## JSONL Output Format

### Move Info

```json
{
  "type": "info",
  "data": {
    "operation": "moved",
    "source": "/path/to/source.txt",
    "destination": "/path/to/dest.txt"
  }
}
```

### Error Output

```json
{
  "type": "error",
  "code": "MV_ERROR",
  "message": "Failed to move file: Permission denied",
  "source": "/path/to/source.txt"
}
```

## Examples

### Rename file

```bash
ai-mv old_name.txt new_name.txt
```

### Move file to directory

```bash
ai-mv file.txt /path/to/directory/
```

### Move multiple files

```bash
ai-mv file1.txt file2.txt file3.txt /destination/
```

### Force overwrite

```bash
ai-mv -f source.txt dest.txt
```

### Interactive move

```bash
ai-mv -i source.txt dest.txt
```

### Don't overwrite

```bash
ai-mv -n source.txt dest.txt
```

### Create backup

```bash
ai-mv -b source.txt dest.txt
```

### Verbose output

```bash
ai-mv -v source.txt dest.txt
```

## Use Cases

### File Organization

```bash
# Move files to organized folders
ai-mv *.txt documents/
ai-mv *.jpg photos/
ai-mv *.mp3 music/
```

### Renaming

```bash
# Rename files
ai-mv old_config.yaml new_config.yaml

# Add suffix
ai-mv file.txt file_backup.txt
```

### Safe Operations

```bash
# Interactive for important files
ai-mv -i important.txt backup/

# Create backups
ai-mv -b production.json backup.json
```

### Batch Moving

```bash
# Move by pattern
ai-mv log_*.txt old_logs/

# Move directories
ai-mv old_project/ archive/
```

## GNU Compatibility

| Feature | Status |
|---------|--------|
| Basic move | ✅ Full support |
| Force | ✅ Full support |
| Interactive | ✅ Full support |
| No clobber | ✅ Full support |
| Verbose | ✅ Full support |
| Backup | ✅ Full support |

## Cross-Filesystem Moves

When moving files across filesystems, `ai-mv` will:
1. Copy the file to the destination
2. Verify the copy succeeded
3. Remove the source file

## Exit Codes

- `0`: Success
- `1`: Error occurred

## See Also

- [ai-cp](ai-cp.md) - Copy files
- [ai-rm](ai-rm.md) - Remove files
