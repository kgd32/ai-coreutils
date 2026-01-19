# ai-analyze - AI-Powered File Analysis

Intelligent file analysis with pattern detection, file classification, and content analysis.

## Description

`ai-analyze` is a unique AI-enhanced utility that performs pattern detection, file classification, and content analysis. It detects common data patterns (emails, URLs, IPs, etc.) and classifies files by type with confidence scoring.

## Usage

```bash
ai-analyze [OPTIONS] <FILES>...
```

## Options

| Option | Short | Description |
|--------|-------|-------------|
| `--patterns` | `-p` | Enable pattern detection (default: true) |
| `--classify` | `-c` | Enable file classification (default: true) |
| `--statistics` | `-s` | Show detailed statistics (lines, words, entropy) |
| `--pattern-types` | `-t` | Detect specific pattern types (comma-separated) |
| `--min-confidence` | `-m` | Minimum confidence threshold (0.0-1.0, default: 0.5) |
| `--recursive` | `-r` | Recursive directory analysis |
| `--jsonl` | `-j` | Output in JSONL format (default: true) |
| `--verbose` | `-v` | Show verbose output with individual matches |

## Pattern Types

Available pattern types for `-t` option:
- `email` - Email addresses
- `url` - HTTP/HTTPS URLs
- `ip` - IP addresses (IPv4 and IPv6)
- `phone` - Phone numbers (multiple formats)
- `ssn` - Social Security Numbers (US)
- `creditcard` - Credit card numbers
- `uuid` - UUID identifiers
- `date` - Date strings
- `hex` - Hexadecimal values
- `base64` - Base64 encoded data
- `filepath` - File paths

## AI Enhancements

- **Pattern Detection**: Regex-based detection with confidence scoring
- **File Classification**: Extension + content-based type detection
- **Content Analysis**: Text statistics and entropy calculation
- **Confidence Scoring**: Probabilistic confidence for all matches

## JSONL Output Format

### Classification Result

```json
{
  "type": "result",
  "data": {
    "type": "classification",
    "file": "/path/to/file.txt",
    "file_type": "text",
    "mime_type": "text/plain",
    "encoding": "utf-8",
    "is_binary": false,
    "language": "Unknown",
    "confidence": 0.95
  }
}
```

### Analysis Result

```json
{
  "type": "result",
  "data": {
    "type": "analysis",
    "file": "/path/to/file.txt",
    "total_patterns": 15,
    "patterns_by_type": {
      "email": 5,
      "url": 3,
      "ip": 7
    },
    "statistics": {
      "lines": 100,
      "words": 500,
      "characters": 3500,
      "bytes": 3500,
      "avg_line_length": 35.0,
      "max_line_length": 120,
      "whitespace_ratio": 0.15,
      "entropy": 4.52
    },
    "issues": []
  }
}
```

### Pattern Match (verbose)

```json
{
  "type": "result",
  "data": {
    "type": "pattern_match",
    "file": "/path/to/file.txt",
    "pattern_type": "Email",
    "matched_text": "user@example.com",
    "position": {
      "start": 10,
      "end": 26
    },
    "confidence": 0.98
  }
}
```

## Examples

### Analyze a file

```bash
ai-analyze document.txt
```

### Detect only emails and URLs

```bash
ai-analyze -t email,url contacts.txt
```

### Show detailed statistics

```bash
ai-analyze --statistics log.txt
```

### Recursive directory analysis

```bash
ai-analyze -r ./src
```

### High confidence threshold

```bash
ai-analyze -m 0.9 data.txt
```

### Verbose output with individual matches

```bash
ai-analyze -v file.txt
```

### Classification only (no patterns)

```bash
ai-analyze -p false -c true *.bin
```

### Patterns only (no classification)

```bash
ai-analyze -p true -c false emails.txt
```

## Content Statistics

When `--statistics` is enabled, the following metrics are calculated:

| Metric | Description |
|--------|-------------|
| `lines` | Number of lines |
| `words` | Number of words |
| `characters` | Character count |
| `bytes` | Byte count |
| `avg_line_length` | Average line length |
| `max_line_length` | Maximum line length |
| `whitespace_ratio` | Ratio of whitespace |
| `entropy` | Shannon entropy (0-8) |

### Entropy Interpretation

- **< 3.0**: Highly structured data (e.g., code, configs)
- **3.0 - 5.0**: Natural language (e.g., prose, documentation)
- **5.0 - 7.0**: Mixed or compressed data
- **> 7.0**: Encrypted or highly random data

## Pattern Detection

### Email Addresses
```
user@example.com
john.doe+tag@co.uk
```

### URLs
```
https://example.com/path?query=value
http://localhost:8080/api
```

### IP Addresses
```
192.168.1.1
2001:0db8:85a3::8a2e:0370:7334
```

### UUIDs
```
550e8400-e29b-41d4-a716-446655440000
```

### Phone Numbers
```
(555) 123-4567
+1-555-123-4567
555.123.4567
```

### Credit Cards
```
4532-1234-5678-9010
4532 1234 5678 9010
```

### SSNs
```
123-45-6789
```

## File Classification

Files are classified based on:

1. **Extension**: Known file extensions
2. **Content Analysis**: Binary vs text detection
3. **Language Detection**: Programming language heuristics

Supported types:
- Text files (txt, md, csv, etc.)
- Code files (rs, py, js, etc.)
- Data files (json, yaml, xml, etc.)
- Binary files (exe, bin, etc.)

## Performance Considerations

- **Pattern Detection**: Optimized regex with pre-compiled patterns
- **Entropy Calculation**: O(n) complexity, streaming
- **Large Files**: Samples first 10KB for classification
- **Recursive Mode**: Parallel directory traversal

## Use Cases

### Security Auditing
```bash
# Find exposed credentials in code
ai-analyze -r --pattern-types email,ssn,creditcard,api-key ./src
```

### Data Validation
```bash
# Verify email format in contact list
ai-analyze -t email -m 0.9 contacts.csv
```

### Code Analysis
```bash
# Find TODO comments and URLs in codebase
ai-analyze -r --pattern-types url,filepath ./src
```

### Log Analysis
```bash
# Analyze log file patterns and statistics
ai-analyze --statistics -t ip,url,datetime app.log
```

### Content Type Detection
```bash
# Classify unknown files
ai-analyze -c true -p false -r ./unsorted
```

## Exit Codes

- `0`: Success
- `1`: Error occurred
- `2`: Invalid arguments

## See Also

- [ai-grep](ai-grep.md) - Pattern searching
- [ML Integration](../ml-integration.md) - Advanced ML features
- [Pattern Detection API](../api-reference.md) - Programmatic access
