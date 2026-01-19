# AI-Coreutils Node.js Bindings

Native Node.js bindings for AI-Coreutils using NAPI-RS.

## Installation

```bash
npm install ai-coreutils
```

## Usage

### Memory Access

```javascript
const { MemoryAccess } = require('ai-coreutils');

// Memory map a file
const mem = new MemoryAccess('large_file.txt');

// Get file size
console.log(`File size: ${mem.size} bytes`);

// Read data at offset
const data = mem.get(0, 100);
if (data) {
    console.log(`First 100 bytes: ${data}`);
}

// Count byte occurrences
const newlines = mem.countByte('\n'.charCodeAt(0));
console.log(`Lines: ${newlines}`);

// Count text metrics
const metrics = mem.countTextMetrics();
console.log(`Lines: ${metrics.lines}, Words: ${metrics.words}, Bytes: ${metrics.bytes}`);

// Search for patterns
const matches = mem.findPattern(Buffer.from('hello'));
console.log(`Found pattern at offsets: ${matches}`);
```

### Text Processing

```javascript
const { TextProcessor } = require('ai-coreutils');

const processor = new TextProcessor();

// Analyze text
const data = Buffer.from('Hello world\nThis is a test\n');
const metrics = processor.analyze(data);

console.log(`Lines: ${metrics.lines}`);
console.log(`Words: ${metrics.words}`);
console.log(`Bytes: ${metrics.bytes}`);

// Count lines
const lines = processor.countLines(data);
console.log(`Lines: ${lines}`);

// Count words
const words = processor.countWords(data);
console.log(`Words: ${words}`);
```

### Pattern Detection

```javascript
const { PatternDetectorWrapper } = require('ai-coreutils');

const detector = new PatternDetectorWrapper();

// Detect patterns in text
const text = `
Contact us at support@example.com or admin@test.org
Visit https://example.com for more info
Server IP: 192.168.1.1
`;

const matches = detector.detectPatterns(text);
matches.forEach(match => {
    console.log(`Found ${match.patternType} at ${match.start}-${match.end}: ${match.matchedText}`);
    console.log(`  Confidence: ${match.confidence}`);
});

// Full analysis with statistics
const analysis = detector.analyzeContent(text, 'example.txt');
console.log(`Total patterns: ${analysis.totalPatterns}`);
console.log(`Statistics:`, analysis.statistics);
console.log(`Issues:`, analysis.issues);
```

### File Classification

```javascript
const { FileClassifierWrapper } = require('ai-coreutils');

const classifier = new FileClassifierWrapper();

// Read file content
const fs = require('fs');
const content = fs.readFileSync('example.rs');

// Classify the file
const classification = classifier.classify('example.rs', content);
console.log(`File type: ${classification.fileType}`);
console.log(`Language: ${classification.language}`);
console.log(`MIME type: ${classification.mimeType}`);
console.log(`Is binary: ${classification.isBinary}`);
console.log(`Confidence: ${classification.confidence}`);
```

### Utility Functions

```javascript
const { Utils } = require('ai-coreutils');

// Count lines and words
const text = 'Hello world\nThis is a test\n';
const lines = Utils.countLines(text);
const words = Utils.countWords(text);

console.log(`Lines: ${lines}, Words: ${words}`);

// Check if content is binary
const isBinary = Utils.isBinary(Buffer.from([0, 1, 2, 0, 0]));
console.log(`Is binary: ${isBinary}`);
```

### SIMD Configuration

```javascript
const { SimdConfigWrapper } = require('ai-coreutils');

// Detect CPU SIMD capabilities
const config = SimdConfigWrapper.detect();
console.log(`SIMD enabled: ${config.enabled}`);
console.log(`Vector width: ${config.vectorWidth} bytes`);

// Create with explicit settings
const customConfig = SimdConfigWrapper.withOptions(true, 32);
```

## API Reference

### `MemoryAccess`

Memory-mapped file access with SIMD operations.

- `new MemoryAccess(path: string)`: Create a new memory-mapped file access
- `size: number` (getter): Get the size of the memory-mapped region
- `ptr: number` (getter): Get a raw pointer to the memory
- `get(offset: number, length: number): Uint8Array | null`: Bounds-checked read
- `getByte(offset: number): number | null`: Get a byte at offset
- `findPattern(pattern: Uint8Array): number[]`: Search for a pattern
- `countByte(byte: number): number`: Count byte occurrences
- `countTextMetrics(): TextMetrics`: Count lines, words, bytes

### `TextProcessor`

SIMD-accelerated text processing.

- `new TextProcessor()`: Create a new processor
- `analyze(data: Uint8Array): TextMetrics`: Analyze text data
- `countLines(data: Uint8Array): number`: Count lines
- `countWords(data: Uint8Array): number`: Count words

### `TextMetrics`

Text analysis results.

- `lines: number`: Number of lines
- `words: number`: Number of words
- `bytes: number`: Number of bytes

### `PatternDetectorWrapper`

AI-powered pattern detection.

- `new PatternDetectorWrapper()`: Create a new detector
- `detectPatterns(text: string): PatternMatch[]`: Detect all patterns
- `analyzeContent(text: string, path: string): ContentAnalysis`: Analyze with statistics

### `PatternMatch`

Pattern match result.

- `pattern: string`: The matched pattern
- `matchedText: string`: The text that matched
- `start: number`: Start position
- `end: number`: End position
- `confidence: number`: Confidence score (0.0-1.0)
- `patternType: string`: Type of pattern

### `ContentAnalysis`

Content analysis results.

- `path: string`: File path analyzed
- `totalPatterns: number`: Total patterns found
- `matches: PatternMatch[]`: List of pattern matches
- `statistics: TextStatistics`: Text statistics
- `issues: string[]`: List of detected issues

### `TextStatistics`

Text statistics.

- `characters: number`: Total characters
- `bytes: number`: Total bytes
- `lines: number`: Total lines
- `words: number`: Total words
- `avgLineLength: number`: Average line length
- `maxLineLength: number`: Maximum line length
- `whitespaceRatio: number`: Percentage of whitespace
- `entropy: number`: Shannon entropy (randomness)

### `FileClassification`

File classification result.

- `path: string`: File path
- `fileType: string`: Detected file type
- `confidence: number`: Classification confidence
- `encoding: string`: Detected encoding
- `mimeType: string`: MIME type
- `isBinary: boolean`: Whether file is binary
- `language: string | null`: Detected language (if text)

### `FileClassifierWrapper`

File type classifier.

- `new FileClassifierWrapper()`: Create a new classifier
- `classify(path: string, content: Uint8Array): FileClassification`: Classify a file

### `Utils`

Utility functions.

- `Utils.countLines(text: string): number`: Count lines in a string
- `Utils.countWords(text: string): number`: Count words in a string
- `Utils.isBinary(content: Uint8Array): boolean`: Check if content is binary

### `SimdConfigWrapper`

SIMD configuration and capabilities.

- `SimdConfigWrapper.detect(): SimdConfigWrapper`: Detect CPU capabilities
- `SimdConfigWrapper.withOptions(enabled: boolean, vectorWidth: number): SimdConfigWrapper`: Create with explicit settings
- `enabled: boolean`: Whether SIMD is enabled
- `vectorWidth: number`: Vector width in bytes

## TypeScript Support

This package includes TypeScript definitions out of the box. Just import and use:

```typescript
import { MemoryAccess, TextProcessor } from 'ai-coreutils';

const mem = new MemoryAccess('file.txt');
const size: number = mem.size;
```

## Performance Tips

1. **Use memory mapping for large files**: MemoryAccess is optimized for files > 1MB
2. **SIMD is automatic**: SIMD operations are auto-detected and used when available
3. **Buffer reuse**: Reuse buffers when processing multiple files

## Platform Support

Pre-built binaries are available for:
- Linux (x86_64, aarch64)
- macOS (x86_64, aarch64/Apple Silicon)
- Windows (x86_64)

## License

MIT OR Apache-2.0
