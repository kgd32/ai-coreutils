# Cross-Language Bindings for AI-Coreutils

This document describes the Python and Node.js bindings for AI-Coreutils.

## Overview

AI-Coreutils provides native bindings for:
- **Python** (via PyO3) - `python/` directory
- **Node.js** (via NAPI-RS) - `nodejs/` directory

Both bindings expose the same core functionality:
- Safe memory access with SIMD operations
- Text processing and metrics
- AI-powered pattern detection
- File classification

## Python Bindings (PyO3)

### Installation

```bash
# From PyPI (once published)
pip install ai-coreutils

# Build from source
cd python
pip install maturin
maturin develop --release
```

### Quick Start

```python
from ai_coreutils import SafeMemoryAccess, PatternDetector, FileClassifier

# Memory access
mem = SafeMemoryAccess("file.txt")
lines, words, bytes = mem.count_text_metrics()

# Pattern detection
detector = PatternDetector()
matches = detector.detect_patterns("Contact test@example.com")
for match in matches:
    print(f"Found {match.pattern_type}: {match.matched_text}")

# File classification
classifier = FileClassifier()
with open("script.py", "rb") as f:
    result = classifier.classify("script.py", f.read())
print(f"Language: {result.language}")
```

### Directory Structure

```
python/
├── README.md           # Python-specific documentation
├── examples/
│   ├── memory_access.py
│   ├── pattern_detection.py
│   └── file_classification.py
```

## Node.js Bindings (NAPI-RS)

### Installation

```bash
npm install ai-coreutils
```

### Quick Start

```javascript
const { MemoryAccess, PatternDetectorWrapper, FileClassifierWrapper } = require('ai-coreutils');

// Memory access
const mem = new MemoryAccess('file.txt');
const metrics = mem.countTextMetrics();

// Pattern detection
const detector = new PatternDetectorWrapper();
const matches = detector.detectPatterns('Contact test@example.com');
matches.forEach(match => {
    console.log(`Found ${match.patternType}: ${match.matchedText}`);
});

// File classification
const classifier = new FileClassifierWrapper();
const fs = require('fs');
const content = fs.readFileSync('script.py');
const result = classifier.classify('script.py', content);
console.log(`Language: ${result.language}`);
```

### Directory Structure

```
nodejs/
├── package.json        # NPM package configuration
├── Cargo.toml          # Rust bindings configuration
├── src/
│   └── lib.rs          # NAPI-RS bindings implementation
├── examples/
│   ├── memory.js
│   ├── pattern-detection.js
│   └── file-classification.js
└── README.md           # Node.js-specific documentation
```

## Building Bindings

### Python

```bash
# Install maturin
pip install maturin

# Build and install in development mode
maturin develop --release

# Build wheel for distribution
maturin build --release

# Run tests
pytest tests/
```

### Node.js

```bash
# Install dependencies
cd nodejs
npm install

# Build native module
npm run build

# Run tests
npm test

# Build for release
npm run build:release
```

## API Comparison

### Memory Access

| Python | Node.js | Description |
|--------|---------|-------------|
| `SafeMemoryAccess(path)` | `new MemoryAccess(path)` | Create memory-mapped access |
| `mem.size()` | `mem.size` | Get file size |
| `mem.get(offset, len)` | `mem.get(offset, len)` | Read data |
| `mem.find_pattern(data)` | `mem.findPattern(data)` | Search pattern |
| `mem.count_text_metrics()` | `mem.countTextMetrics()` | Get metrics |

### Pattern Detection

| Python | Node.js | Description |
|--------|---------|-------------|
| `PatternDetector()` | `new PatternDetectorWrapper()` | Create detector |
| `detector.detect_patterns(text)` | `detector.detectPatterns(text)` | Find patterns |
| `detector.analyze_content(text, path)` | `detector.analyzeContent(text, path)` | Full analysis |

### File Classification

| Python | Node.js | Description |
|--------|---------|-------------|
| `FileClassifier()` | `new FileClassifierWrapper()` | Create classifier |
| `classifier.classify(path, content)` | `classifier.classify(path, content)` | Classify file |

## Platform Support

### Python
- Windows (x86_64)
- Linux (x86_64, aarch64)
- macOS (x86_64, aarch64)

### Node.js
Pre-built binaries via NAPI-RS for:
- Windows (x86_64)
- Linux (x86_64, aarch64)
- macOS (x86_64, aarch64)

## Type Safety

### Python
- Full type hints included
- Works with mypy and type checkers

### Node.js
- TypeScript definitions included
- Full IntelliSense support

## Performance Considerations

1. **Memory Mapping**: Both bindings use `memmap2` for efficient file access
2. **SIMD**: Automatic detection and use of AVX2/SSE2 on x86_64
3. **Zero-Copy**: Where possible, data is passed without copying

## Testing

### Python Tests
```bash
cd python
pytest tests/ -v
```

### Node.js Tests
```bash
cd nodejs
npm test
```

## Publishing

### Python (PyPI)
```bash
maturin publish --username __token__ --password <pypi-token>
```

### Node.js (NPM)
```bash
cd nodejs
npm publish
```

## Troubleshooting

### Python
- **Import Error**: Ensure Rust is installed and `maturin` is available
- **Build Failures**: Check Rust version (`rustc --version`)

### Node.js
- **Native Module Build Fails**: Ensure node-gyp dependencies are installed
- **Platform Not Supported**: May need to build from source

## Examples

See language-specific directories for complete examples:
- `python/examples/` - Python usage examples
- `nodejs/examples/` - Node.js usage examples

## Contributing

When modifying core functionality:
1. Update Rust implementation
2. Update both Python and Node.js bindings
3. Add tests for both languages
4. Update documentation

## License

MIT OR Apache-2.0
