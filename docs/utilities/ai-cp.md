# ai-cp - Copy Files and Directories

Copy files and directories with progress tracking and JSONL structured output.

## Description

`ai-cp` is a modern implementation of the `cp` utility designed for AI agents. It copies files and directories with progress tracking, attribute preservation, and batch operations.

## Usage

```bash
ai-cp [OPTIONS] <SOURCE>... <DESTINATION>
```

## Options

| Option | Short | GNU Equivalent | Description |
|--------|-------|----------------|-------------|
| `--recursive` | `-R` | `-r`/`-R` | Recursive copy (for directories) |
| `--archive` | `-a` | `-a` | Archive mode (preserve all attributes) |
| `--preserve` | `-p` | `-p` | Preserve permissions, timestamps |
| `--verbose` | `-v` | `-v` | Verbose output |
| `--interactive` | `-i` | `-i` | Interactive prompt before overwrite |
| `--update` | `-u` | `-u` | Copy only when source is newer |
| `--link` | `-l` | `-l` | Create hard links instead of copying |
| `--symbolic-link` | `-s` | `-s` | Create symbolic links |
| `--no-clobber` | `-n` | `-n` | Don't overwrite existing files |

## AI Enhancements

- **Progress Tracking**: Real-time copy progress with JSONL updates
- **Batch Operations**: Efficiently copy multiple files
- **Structured Output**: Detailed metadata in JSONL format

## JSONL Output Format

### Progress Update

```json
{
  "type": "progress",
  "current": 524288,
  "total": 1048576,
  "message": "Copying source.txt"
}
```

### File Copied

```json
{
  "type": "info",
  "data": {
    "type": "file_copied",
    "source": "/path/to/source.txt",
    "dest": "/path/to/dest.txt",
    "size": 1024
  }
}
```

### Copy Summary

```json
{
  "type": "result",
  "data": {
    "type": "copy_summary",
    "files_copied": 10,
    "bytes_copied": 1048576,
    "dirs_created": 2,
    "errors": 0
  }
}
```

## Examples

### Copy single file

```bash
ai-cp source.txt dest.txt
```

### Copy file to directory

```bash
ai-cp file.txt /path/to/directory/
```

### Copy multiple files to directory

```bash
ai-cp file1.txt file2.txt file3.txt /destination/
```

### Recursive directory copy

```bash
ai-cp -R source_dir/ dest_dir/
```

### Archive mode (preserve all)

```bash
ai-cp -a source/ dest/
```

### Preserve permissions and timestamps

```bash
ai-cp -p important_file.txt backup/
```

### Interactive copy

```bash
ai-cp -i source.txt dest.txt
```

### Copy only if source is newer

```bash
ai-cp -u source.txt backup.txt
```

### Create hard links instead of copying

```bash
ai-cp -l original.txt link.txt
```

### Create symbolic links

```bash
ai-cp -s original.txt symlink.txt
```

### Don't overwrite existing files

```bash
ai-cp -n source.txt dest.txt
```

### Verbose copy with details

```bash
ai-cp -v -R source/ dest/
```

## Use Cases

### Backup Files

```bash
# Create backup with attributes preserved
ai-cp -a important_data/ backup/

# Copy only modified files
ai-cp -u src/ backup/
```

### Project Templates

```bash
# Copy template to new project
ai-cp -R template/ new_project/

# Preserve all attributes
ai-cp -a project_template/ my_project/
```

### Data Organization

```bash
# Copy files by date
ai-cp *.txt 2024/01/

# Copy images to organized folders
ai-cp *.jpg photos/
```

### Link Creation

```bash
# Create hard links for deduplication
ai-cp -l original.mp3 duplicate.mp3

# Create symbolic links for convenience
ai-cp -s /long/path/to/file.txt ./shortcut.txt
```

## Performance Considerations

- **Large Files**: Progress updates every 1MB for files > 1MB
- **Batch Operations**: Multiple files processed efficiently
- **Memory Usage**: 8KB buffer for file copying (low memory footprint)

## GNU Compatibility

| Feature | Status |
|---------|--------|
| Basic copy | ✅ Full support |
| Recursive | ✅ Full support |
| Archive mode | ✅ Full support |
| Preserve | ✅ Full support |
| Interactive | ✅ Full support |
| Update only | ✅ Full support |
| Hard links | ✅ Full support |
| Symbolic links | ✅ Full support |
| No clobber | ✅ Full support |

## Exit Codes

- `0`: Success
- `1`: Error occurred

## See Also

- [ai-mv](ai-mv.md) - Move/rename files
- [ai-rm](ai-rm.md) - Remove files
