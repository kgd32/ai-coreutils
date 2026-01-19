# ai-chmod - Change File Permissions

Change file access permissions with JSONL structured output.

## Description

`ai-chmod` is a modern implementation of the `chmod` utility designed for AI agents. It changes file permissions with batch operation support.

## Usage

```bash
ai-chmod [OPTIONS] <MODE> <FILE>...
```

## Options

| Option | Short | GNU Equivalent | Description |
|--------|-------|----------------|-------------|
| `--recursive` | `-R` | `-R` | Recursive permission change |
| `--verbose` | `-v` | `-v` | Verbose output |
| `--changes` | `-c` | `-c` | Report only when changes are made |

## AI Enhancements

- **Batch Operations**: Change permissions on multiple files
- **JSONL Output**: Structured operation results
- **Progress Tracking**: Real-time status updates

## JSONL Output Format

### Permission Changed

```json
{
  "type": "info",
  "data": {
    "file": "/path/to/file.txt",
    "operation": "permissions_changed",
    "old_mode": "644",
    "new_mode": "755"
  }
}
```

## Examples

### Make file executable

```bash
ai-chmod +x script.sh
```

### Set specific permissions

```bash
ai-chmod 755 script.sh
```

### Make directory readable/executable

```bash
ai-chmod u+rx directory/
```

### Recursive permission change

```bash
ai-chmod -R 644 src/
```

### Verbose output

```bash
ai-chmod -v 755 script.sh
```

### Remove write permission

```bash
ai-chmod -w file.txt
```

### Set owner permissions only

```bash
ai-chmod u=rwx,go=rx file.sh
```

## Permission Modes

### Symbolic Mode

- `u` - user (owner)
- `g` - group
- `o` - others
- `a` - all

Operators:
- `+` - add permission
- `-` - remove permission
- `=` - set exact permission

Permissions:
- `r` - read
- `w` - write
- `x` - execute

### Numeric Mode

- `0` - no permissions
- `1` - execute only
- `2` - write only
- `3` - write and execute
- `4` - read only
- `5` - read and execute
- `6` - read and write
- `7` - read, write, and execute

Format: `OGP` where O=owner, G=group, P=others

## Common Permissions

| Mode | Numeric | Description |
|------|---------|-------------|
| `rw-r--r--` | `644` | Standard file |
| `rwxr-xr-x` | `755` | Executable/script |
| `rwx------` | `700` | Private file |
| `rw-rw-rw-` | `666` | Shared file |
| `rwxrwxrwx` | `777` | World-writable (rarely used) |

## Use Cases

### Script Setup

```bash
# Make scripts executable
ai-chmod +x *.sh
ai-chmod +x scripts/*
```

### Web Files

```bash
# Standard web permissions
ai-chmod 644 *.html *.css *.js
ai-chmod 755 cgi-bin/*
```

### Private Data

```bash
# Restrictive permissions for sensitive files
ai-chmod 600 private_key.pem
ai-chmod 600 ~/.ssh/id_rsa
```

### Directory Trees

```bash
# Set permissions recursively
ai-chmod -R 755 public_html/
ai-chmod -R g+rw shared/
```

## GNU Compatibility

| Feature | Status |
|---------|--------|
| Symbolic mode | ✅ Full support |
| Numeric mode | ✅ Full support |
| Recursive | ✅ Full support |
| Verbose | ✅ Full support |
| Changes only | ✅ Full support |

## Platform Notes

- **Unix/Linux**: Full support for all permission modes
- **Windows**: Limited support (Windows uses ACLs, not Unix permissions)

## Exit Codes

- `0`: Success
- `1`: Error occurred

## See Also

- [ai-chown](ai-chown.md) - Change file owner
- [ai-ls](ai-ls.md) - List file permissions
