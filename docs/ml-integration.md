# ML Integration Guide

Guide to AI-powered pattern detection and content analysis in AI-Coreutils.

## Overview

AI-Coreutils includes ML-powered features for:

- **Pattern detection** (emails, URLs, IPs, etc.)
- **File classification** (type, encoding, language)
- **Content analysis** (statistics, entropy)
- **Confidence scoring** for all detections

## Pattern Detection

### Supported Patterns

| Pattern | Example | Confidence |
|---------|---------|------------|
| Email | `user@example.com` | 95% |
| URL | `https://example.com` | 98% |
| IPv4 | `192.168.1.1` | 90% |
| IPv6 | `2001:0db8::1` | 85% |
| Phone | `(555) 123-4567` | 80% |
| SSN | `123-45-6789` | 95% |
| Credit Card | `4532-1234-5678-9010` | 90% |
| UUID | `550e8400-e29b-41d4-a716-446655440000` | 100% |
| Date | `2026-01-19` | 75% |
| Hex | `0x1a2b3c` | 85% |
| Base64 | `SGVsbG8gV29ybGQ=` | 90% |
| File Path | `/path/to/file.txt` | 70% |

### Using Pattern Detection

#### CLI

```bash
# Detect all patterns
ai-analyze --patterns document.txt

# Detect specific patterns
ai-analyze -t email,url contacts.txt

# Recursive analysis
ai-analyze -r --patterns ./src

# Verbose output with individual matches
ai-analyze -v --patterns file.txt
```

#### Rust API

```rust
use ai_coreutils::ml_ops::PatternDetector;

// Create detector
let detector = PatternDetector::new()?;

// Detect patterns in text
let text = "Contact test@example.com or visit https://example.com";
let matches = detector.detect_patterns(text);

for m in matches {
    println!("{}: {} (confidence: {})",
        m.pattern_type, m.matched_text, m.confidence);
}
```

#### Python API

```python
from ai_coreutils import PatternDetector

detector = PatternDetector()
matches = detector.detect_patterns("Contact test@example.com")

for match in matches:
    print(f"{match.pattern_type}: {match.matched_text}")
```

#### Node.js API

```javascript
const { PatternDetectorWrapper } = require('ai-coreutils');

const detector = new PatternDetectorWrapper();
const matches = detector.detectPatterns('Contact test@example.com');

matches.forEach(match => {
    console.log(`${match.patternType}: ${match.matchedText}`);
});
```

### Pattern Types

```rust
pub enum PatternType {
    Email,
    Url,
    IpAddress,
    PhoneNumber,
    Ssn,
    CreditCard,
    Uuid,
    Date,
    Hex,
    Base64,
    FilePath,
}
```

### Pattern Match Structure

```rust
pub struct PatternMatch {
    pub pattern: String,           // Regex pattern used
    pub matched_text: String,       // The actual match
    pub start: usize,               // Start position
    pub end: usize,                 // End position
    pub confidence: f64,            // 0.0 to 1.0
    pub pattern_type: PatternType,  // Type of pattern
}
```

## File Classification

### Classification Types

| Type | Description | Extensions |
|------|-------------|------------|
| Text | Plain text files | .txt, .md, .csv |
| Code | Source code | .rs, .py, .js, etc. |
| Data | Structured data | .json, .yaml, .xml |
| Markup | Markup languages | .html, .xml |
| Config | Configuration files | .conf, .ini, .yaml |
| Binary | Binary files | .bin, .exe |
| Archive | Compressed files | .zip, .tar, .gz |

### Using File Classification

#### CLI

```bash
# Classify files
ai-analyze --classify unknown_file.bin

# Recursive classification
ai-analyze -r -c ./unsorted

# Classification with patterns
ai-analyze -c -p document.txt
```

#### Rust API

```rust
use ai_coreutils::ml_ops::FileClassifier;
use std::fs;

let content = fs::read("unknown_file")?;
let classification = FileClassifier::classify(
    Path::new("unknown_file"),
    &content
)?;

println!("Type: {}", classification.file_type);
println!("MIME: {}", classification.mime_type);
println!("Binary: {}", classification.is_binary);
println!("Confidence: {}", classification.confidence);
```

### Classification Structure

```rust
pub struct FileClassification {
    pub path: String,              // File path
    pub file_type: String,         // Type description
    pub mime_type: String,         // MIME type
    pub encoding: String,          // Character encoding
    pub is_binary: bool,           // Binary detection
    pub language: Option<String>,  // Programming language
    pub confidence: f64,           // 0.0 to 1.0
}
```

## Content Analysis

### Statistics Calculated

| Metric | Description |
|--------|-------------|
| Lines | Number of lines |
| Words | Number of words |
| Characters | Character count |
| Bytes | Byte count |
| Avg Line Length | Average characters per line |
| Max Line Length | Longest line |
| Whitespace Ratio | Ratio of whitespace to content |
| Entropy | Shannon entropy (randomness) |

### Using Content Analysis

#### CLI

```bash
# Show statistics
ai-analyze --statistics document.txt

# Full analysis (patterns + statistics)
ai-analyze -p -s document.txt

# Entropy analysis for encryption detection
ai-analyze -s suspicious_file.bin
```

#### Rust API

```rust
use ai_coreutils::ml_ops::PatternDetector;

let detector = PatternDetector::new()?;
let text = std::fs::read_to_string("file.txt")?;
let analysis = detector.analyze_content(&text, Path::new("file.txt"))?;

println!("Total patterns: {}", analysis.total_patterns);
println!("Lines: {}", analysis.statistics.lines);
println!("Entropy: {}", analysis.statistics.entropy);
```

### Analysis Structure

```rust
pub struct ContentAnalysis {
    pub total_patterns: usize,
    pub patterns_by_type: HashMap<String, usize>,
    pub matches: Vec<PatternMatch>,
    pub statistics: ContentStatistics,
    pub issues: Vec<String>,
}

pub struct ContentStatistics {
    pub lines: usize,
    pub words: usize,
    pub characters: usize,
    pub bytes: usize,
    pub avg_line_length: f64,
    pub max_line_length: usize,
    pub whitespace_ratio: f64,
    pub entropy: f64,
}
```

## Entropy Analysis

### Interpreting Entropy

| Range | Interpretation | Example |
|-------|----------------|---------|
| 0.0 - 3.0 | Highly structured | Code, configs |
| 3.0 - 5.0 | Natural language | Text, prose |
| 5.0 - 7.0 | Mixed/Compressed | Mixed data |
| > 7.8 | Encrypted/Random | Encrypted files |

### Use Cases

#### Detect Encrypted Files

```rust
let analysis = detector.analyze_content(&text, path)?;

if analysis.statistics.entropy > 7.8 {
    println!("Warning: High entropy - may be encrypted");
}
```

#### Detect Compression

```rust
if analysis.statistics.entropy > 6.0 {
    println!("High entropy detected - possible compression");
}
```

## Confidence Thresholds

### Setting Thresholds

```bash
# High confidence only
ai-analyze -m 0.9 --patterns file.txt

# Low threshold for more matches
ai-analyze -m 0.3 --patterns file.txt
```

### API Usage

```rust
use ai_coreutils::ml_ops::{PatternDetector, MlConfig};

let config = MlConfig {
    detect_patterns: true,
    analyze_entropy: true,
    min_confidence: 0.9,  // 90% threshold
    max_samples: 10000,
};

let detector = PatternDetector::with_config(config)?;
```

## Use Cases

### Security Auditing

```bash
# Find exposed credentials
ai-analyze -t email,ssn,creditcard -r ./src

# Find API keys (Base64 with high entropy)
ai-analyze -t base64 -s ./config
```

### Data Validation

```bash
# Verify email formats
ai-analyze -t email -m 0.9 contacts.csv

# Find valid URLs
ai-analyze -t url -m 0.95 links.txt
```

### Content Type Detection

```bash
# Classify unknown files
ai-analyze -c ./unsorted/*

# Detect binary files
ai-analyze -c -s ./download/*
```

### Code Analysis

```bash
# Find TODO comments
ai-analyze -r ./src | grep "TODO"

# Find URLs in code
ai-analyze -t url -r ./src
```

## Performance Considerations

- **Pattern Detection**: O(n) where n is text length
- **File Classification**: O(1) for extension, O(k) for content (k = sample size)
- **Entropy Calculation**: O(n) where n is text length
- **Optimization**: Samples first 10KB for classification

## Best Practices

1. **Use specific patterns**: More accurate than all patterns
2. **Set appropriate thresholds**: Balance precision vs recall
3. **Combine with statistics**: Better understanding of content
4. **Use recursive mode**: Analyze entire directories
5. **Filter by confidence**: Ignore low-confidence matches

## Troubleshooting

### Too Many False Positives

```bash
# Increase confidence threshold
ai-analyze -m 0.9 --patterns file.txt
```

### Missing Patterns

```bash
# Decrease confidence threshold
ai-analyze -m 0.3 --patterns file.txt
```

### Slow Analysis

```bash
# Disable statistics for pattern-only
ai-analyze -p -s false file.txt

# Limit pattern types
ai-analyze -t email,url file.txt
```

## Pattern Regex Reference

### Email
```
[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}
```

### URL
```
https?://[^\s]+
```

### IPv4
```
\b(?:\d{1,3}\.){3}\d{1,3}\b
```

### UUID
```
[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}
```

### Base64
```
(?:[A-Za-z0-9+/]{4})*(?:[A-Za-z0-9+/]{2}==|[A-Za-z0-9+/]{3}=)?
```
