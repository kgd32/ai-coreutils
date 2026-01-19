# ai-ls - List Directory Contents

List directory contents with structured JSONL output and AI-friendly metadata.

## Description

`ai-ls` is a modern implementation of the `ls` utility designed for AI agents. It outputs directory listings in JSONL format with rich metadata including file sizes, permissions, timestamps, and more.

## Usage

```bash
ai-ls [OPTIONS] [PATHS]...
```

## Options

| Option | Short | GNU Equivalent | Description |
|--------|-------|----------------|-------------|
| `--all` | `-a` | `-a` | Show all files (including hidden files starting with `.`) |
| `--long` | `-l` | `-l` | Long format with detailed metadata |
| `--human-readable` | `-h` | `-h` | Show sizes in human-readable format (K, M, G) |
| `--recursive` | `-R` | `-R` | List subdirectories recursively |
| `--sort-time` | `-t` | `-t` | Sort by modification time (newest first) |
| `--sort-size` | `-S` | `-S` | Sort by file size (largest first) |
| `--reverse` | `-r` | `-r` | Reverse sort order |
| `--json` | `-j` | *New* | Output JSONL format (default: true) |

## AI Enhancements

- **Structured Output**: All data in JSONL format for easy parsing
- **Rich Metadata**: Includes timestamps in ISO 8601 format, permissions, and type flags
- **Memory Efficient**: Uses walkdir for efficient directory traversal
- **Cross-Platform**: Works on Linux, macOS, and Windows

## JSONL Output Format

### Basic Output

```json
{"type":"file_entry","timestamp":"2026-01-19T12:00:00Z","path":"/path/to/file.txt","size":1024,"modified":"2026-01-19T10:30:00Z","is_dir":false,"is_symlink":false,"permissions":"644"}
```

### Long Format Output

```json
{
  "type": "result",
  "timestamp": "2026-01-19T12:00:00Z",
  "data": {
    "type": "file",
    "path": "/path/to/file.txt",
    "name": "file.txt",
    "size": 1024,
    "size_human": "1.0K",
    "modified": "2026-01-19T10:30:00Z",
    "is_dir": false,
    "is_symlink": false,
    "is_hidden": false,
    "permissions": "644"
  }
}
```

### Error Output

```json
{"type":"error","code":"LS_ERROR","message":"Failed to list directory: Permission denied","timestamp":"2026-01-19T12:00:00Z"}
```

## Examples

### List current directory

```bash
ai-ls
```

### List with details

```bash
ai-ls -l
```

### List all files including hidden

```bash
ai-ls -a
```

### Recursive directory listing

```bash
ai-ls -R /path/to/directory
```

### Sort by size, largest first

```bash
ai-ls -S
```

### Sort by modification time, oldest first

```bash
ai-ls -t -r
```

### Human-readable sizes with long format

```bash
ai-ls -lh
```

### Multiple directories

```bash
ai-ls /path/to/dir1 /path/to/dir2
```

## Performance Considerations

- **Memory**: Uses streaming directory traversal with walkdir
- **Large Directories**: Efficiently handles directories with thousands of files
- **Network Drives**: May be slower due to metadata fetching

## GNU Compatibility

`ai-ls` maintains compatibility with GNU `ls` core options:

| Feature | Status |
|---------|--------|
| Basic listing | ✅ Full support |
| Long format | ✅ Full support |
| Hidden files | ✅ Full support |
| Recursive | ✅ Full support |
| Sorting options | ✅ Full support |
| Human-readable sizes | ✅ Full support |
| Color output | ❌ Not applicable (JSONL) |
| Inode numbers | ❌ Not applicable |

## Exit Codes

- `0`: Success
- `1`: General error
- `2`: Misuse of shell commands

## See Also

- [ai-cat](ai-cat.md) - Display file contents
- [ai-find](ai-find.md) - Search for files in directory hierarchy
- [JSONL Format](../jsonl-format.md) - Understanding JSONL output
