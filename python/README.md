# AI-Coreutils Python Bindings

Python bindings for AI-Coreutils using PyO3.

## Installation

### From source

```bash
# Install Rust first (required)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install the package
pip install ai-coreutils
```

### Build locally

```bash
# Build the Python extension
maturin develop --release

# Or build wheel
maturin build --release
```

## Usage

### Safe Memory Access

```python
from ai_coreutils import SafeMemoryAccess

# Memory map a file
mem = SafeMemoryAccess("large_file.txt")

# Get file size
print(f"File size: {mem.size()} bytes")

# Read data at offset
data = mem.get(0, 100)
if data:
    print(f"First 100 bytes: {data}")

# Count byte occurrences
newlines = mem.count_byte(ord('\n'))
print(f"Lines: {newlines}")

# Count text metrics
lines, words, bytes = mem.count_text_metrics()
print(f"Lines: {lines}, Words: {words}, Bytes: {bytes}")

# Search for patterns
pattern = b"hello"
matches = mem.find_pattern(pattern)
print(f"Found pattern at offsets: {matches}")
```

### SIMD Text Processing

```python
from ai_coreutils import SimdTextProcessor

processor = SimdTextProcessor()

# Analyze text
data = b"Hello world\nThis is a test\n"
metrics = processor.analyze(data)

print(f"Lines: {metrics.lines}")
print(f"Words: {metrics.words}")
print(f"Bytes: {metrics.bytes}")

# Convert to dictionary
metrics_dict = metrics.to_dict()
print(metrics_dict)
```

### Pattern Detection

```python
from ai_coreutils import PatternDetector

detector = PatternDetector()

# Detect patterns in text
text = """
Contact us at support@example.com or admin@test.org
Visit https://example.com for more info
Server IP: 192.168.1.1
"""

matches = detector.detect_patterns(text)
for match in matches:
    print(f"Found {match.pattern_type} at {match.start}-{match.end}: {match.matched_text}")
    print(f"  Confidence: {match.confidence}")

# Analyze content with statistics
analysis = detector.analyze_content(text, "example.txt")
print(f"Total patterns: {analysis.total_patterns}")
print(f"Statistics: {analysis.statistics.to_dict()}")
print(f"Issues: {analysis.issues}")
```

### File Classification

```python
from ai_coreutils import FileClassifier

classifier = FileClassifier()

# Classify a file
with open("example.rs", "rb") as f:
    content = f.read()

classification = classifier.classify("example.rs", content)
print(f"File type: {classification.file_type}")
print(f"Language: {classification.language}")
print(f"MIME type: {classification.mime_type}")
print(f"Is binary: {classification.is_binary}")
print(f"Confidence: {classification.confidence}")
```

## API Reference

### `SafeMemoryAccess`

Memory-mapped file access with SIMD-accelerated operations.

- `new(path: str) -> SafeMemoryAccess`: Create a new memory-mapped file access
- `size() -> int`: Get the size of the memory-mapped region
- `as_ptr() -> int`: Get a raw pointer to the memory
- `get(offset: int, length: int) -> Optional[bytes]`: Bounds-checked read
- `get_byte(offset: int) -> Optional[int]`: Get a byte at offset
- `find_pattern(pattern: bytes) -> List[int]`: Search for a pattern
- `count_byte(byte: int) -> int`: Count byte occurrences
- `count_text_metrics() -> Tuple[int, int, int]`: Count lines, words, bytes

### `SimdTextProcessor`

SIMD-accelerated text processing.

- `new() -> SimdTextProcessor`: Create a new processor
- `analyze(data: bytes) -> TextMetrics`: Analyze text data
- `count_lines(data: bytes) -> int`: Count lines
- `count_words(data: bytes) -> int`: Count words

### `TextMetrics`

Text analysis results.

- `lines`: Number of lines
- `words`: Number of words
- `bytes`: Number of bytes
- `to_dict()`: Convert to dictionary

### `PatternDetector`

AI-powered pattern detection.

- `new() -> PatternDetector`: Create a new detector
- `detect_patterns(text: str) -> List[PatternMatch]`: Detect all patterns
- `analyze_content(text: str, path: str) -> ContentAnalysis`: Analyze with statistics

### `PatternMatch`

Pattern match result.

- `pattern`: The matched pattern
- `matched_text`: The text that matched
- `start`: Start position
- `end`: End position
- `confidence`: Confidence score (0.0-1.0)
- `pattern_type`: Type of pattern

### `PatternType`

Pattern type enumeration.

- `PatternType.email()`: Email addresses
- `PatternType.url()`: URLs
- `PatternType.ip_address()`: IP addresses
- `PatternType.phone_number()`: Phone numbers
- `PatternType.credit_card()`: Credit card numbers
- `PatternType.ssn()`: Social Security Numbers
- `PatternType.date()`: Dates
- `PatternType.hex()`: Hexadecimal values
- `PatternType.base64()`: Base64 encoded data
- `PatternType.uuid()`: UUIDs
- `PatternType.file_path()`: File paths

### `ContentAnalysis`

Content analysis results.

- `path`: File path analyzed
- `total_patterns`: Total patterns found
- `matches`: List of PatternMatch objects
- `statistics`: TextStatistics object
- `issues`: List of detected issues

### `TextStatistics`

Text statistics.

- `characters`: Total characters
- `bytes`: Total bytes
- `lines`: Total lines
- `words`: Total words
- `avg_line_length`: Average line length
- `max_line_length`: Maximum line length
- `whitespace_ratio`: Percentage of whitespace
- `entropy`: Shannon entropy (randomness)

### `FileClassification`

File classification result.

- `path`: File path
- `file_type`: Detected file type
- `confidence`: Classification confidence
- `encoding`: Detected encoding
- `mime_type`: MIME type
- `is_binary`: Whether file is binary
- `language`: Detected language (if text)

## Performance Tips

1. **Use memory mapping for large files**: SafeMemoryAccess is optimized for files > 1MB
2. **SIMD is automatic**: SIMD operations are auto-detected and used when available
3. **Batch operations**: Process multiple files concurrently for better performance

## License

MIT OR Apache-2.0
