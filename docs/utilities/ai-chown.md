# ai-chown - Change File Owner

Change file owner and group with JSONL structured output.

## Description

`ai-chown` is a modern implementation of the `chown` utility designed for AI agents. It changes file owner and group with batch operation support.

## Usage

```bash
ai-chown [OPTIONS] <OWNER>[:<GROUP>] <FILE>...
```

## Options

| Option | Short | GNU Equivalent | Description |
|--------|-------|----------------|-------------|
| `--recursive` | `-R` | `-R` | Recursive owner change |
| `--verbose` | `-v` | `-v` | Verbose output |
| `--from <OWNER>` | `--from` | `--from` | Change only if current owner matches |

## AI Enhancements

- **Batch Operations**: Change ownership on multiple files
- **JSONL Output**: Structured operation results
- **Progress Tracking**: Real-time status updates

## JSONL Output Format

### Owner Changed

```json
{
  "type": "info",
  "data": {
    "file": "/path/to/file.txt",
    "operation": "owner_changed",
    "old_owner": "root",
    "new_owner": "user",
    "old_group": "root",
    "new_group": "users"
  }
}
```

## Examples

### Change owner

```bash
ai-chown user file.txt
```

### Change owner and group

```bash
ai-chown user:group file.txt
```

### Change group only

```bash
ai-chown :group file.txt
```

### Recursive ownership change

```bash
ai-chown -R user:group directory/
```

### Verbose output

```bash
ai-chown -v user file.txt
```

### Change only if owned by specific user

```bash
ai-chown --from=root newuser file.txt
```

## Owner/Group Syntax

### Owner only
```
ai-chown username file.txt
```

### Owner and group
```
ai-chown user:group file.txt
```

### Group only
```
ai-chown :groupname file.txt
```

### Numeric IDs
```
ai-chown 1000:1000 file.txt
ai-chown 1000:100 file.txt
```

## Use Cases

### System Administration

```bash
# Transfer ownership to user
ai-chown -R user:users /home/user/

# Set web server files
ai-chown -R www-data:www-data /var/www/
```

### Software Installation

```bash
# Set correct permissions for installed software
ai-chown -R root:root /usr/local/bin/myapp
```

### Shared Directories

```bash
# Set shared group ownership
ai-chown -R :team /shared/project/
ai-chmod -R g+rw /shared/project/
```

## Permissions Note

You typically need root privileges to change file ownership. Use `sudo`:

```bash
sudo ai-chown user file.txt
sudo ai-chown -R user:group /path/to/dir
```

## GNU Compatibility

| Feature | Status |
|---------|--------|
| Owner change | ✅ Full support |
| Group change | ✅ Full support |
| Recursive | ✅ Full support |
| Verbose | ✅ Full support |
| From option | ✅ Full support |

## Platform Notes

- **Unix/Linux**: Full support
- **Windows**: Limited support (Windows uses different ownership model)
- **macOS**: Full support

## Exit Codes

- `0`: Success
- `1`: Error occurred

## See Also

- [ai-chmod](ai-chmod.md) - Change file permissions
- [ai-ls](ai-ls.md) - List file ownership
