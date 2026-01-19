# AI-Coreutils Documentation

Welcome to the AI-Coreutils documentation. AI-Coreutils is a modern implementation of GNU core utilities specifically designed for AI agents, featuring structured JSONL output, safe memory pointer access, and AI-powered analysis capabilities.

## Quick Links

- [Getting Started](getting-started.md) - Installation and first steps
- [Architecture](architecture.md) - System design and architecture
- [API Reference](api-reference.md) - Library API documentation
- [Utilities](utilities/) - Individual utility documentation
- [Examples](examples/) - Practical examples and tutorials

## Key Features

### 1. Structured JSONL Output
All utilities output machine-readable JSONL format by default, making it easy for AI agents to parse and process results.

```json
{"type": "result", "data": {"file": "example.txt", "lines": 42}}
{"type": "info", "message": "Processing complete"}
```

### 2. Safe Memory Access
Direct memory pointer access with bounds checking for high-performance file operations.

```rust
use ai_coreutils::SafeMemoryAccess;

let mem = SafeMemoryAccess::new("large_file.bin")?;
let data = mem.get(1024, 256)?; // Safe bounds-checked access
```

### 3. SIMD Optimizations
Automatic SIMD acceleration for text processing operations (AVX2/SSE2 on x86_64).

### 4. AI-Powered Analysis
Pattern detection, file classification, and content analysis powered by machine learning.

### 5. Async Operations
Concurrent file processing with tokio for improved performance.

### 6. Cross-Language Bindings
Native Python and Node.js bindings for easy integration.

## Available Utilities

| Utility | Description | GNU Equivalent |
|---------|-------------|----------------|
| `ai-ls` | List directory contents | `ls` |
| `ai-cat` | Concatenate and print files | `cat` |
| `ai-grep` | Search text patterns | `grep` |
| `ai-find` | Search directory tree | `find` |
| `ai-cp` | Copy files/directories | `cp` |
| `ai-mv` | Move/rename files | `mv` |
| `ai-rm` | Remove files/directories | `rm` |
| `ai-touch` | Create empty files/update timestamps | `touch` |
| `ai-mkdir` | Create directories | `mkdir` |
| `ai-rmdir` | Remove empty directories | `rmdir` |
| `ai-head` | Output first part of files | `head` |
| `ai-tail` | Output last part of files | `tail` |
| `ai-wc` | Count lines, words, bytes | `wc` |
| `ai-chmod` | Change file permissions | `chmod` |
| `ai-chown` | Change file owner | `chown` |
| `ai-analyze` | AI-powered file analysis | *New* |

## Installation

### From Crates.io
```bash
cargo install ai-coreutils
```

### From Source
```bash
git clone https://github.com/your-org/ai-coreutils
cd ai-coreutils
cargo install --path .
```

### Python Bindings
```bash
pip install ai-coreutils
```

### Node.js Bindings
```bash
npm install ai-coreutils
```

## Quick Start

### Command Line
```bash
# List directory with JSONL output
ai-ls /path/to/directory

# Search for patterns with context
ai-grep -r "pattern" /path/to/search

# Analyze files for patterns
ai-analyze --patterns --classify /path/to/files
```

### Rust Library
```rust
use ai_coreutils::{SafeMemoryAccess, PatternDetector};

// Memory access
let mem = SafeMemoryAccess::new("file.txt")?;
let size = mem.size();

// Pattern detection
let detector = PatternDetector::new();
let matches = detector.detect_patterns("Contact test@example.com");
```

### Python
```python
from ai_coreutils import SafeMemoryAccess, PatternDetector

mem = SafeMemoryAccess("file.txt")
print(f"Size: {mem.size()}")

detector = PatternDetector()
matches = detector.detect_patterns("Contact test@example.com")
```

### Node.js
```javascript
const { MemoryAccess, PatternDetectorWrapper } = require('ai-coreutils');

const mem = new MemoryAccess('file.txt');
console.log(`Size: ${mem.size}`);

const detector = new PatternDetectorWrapper();
const matches = detector.detectPatterns('Contact test@example.com');
```

## Documentation Structure

### User Documentation
- [Getting Started](getting-started.md) - Installation and basic usage
- [Utilities](utilities/) - Detailed utility documentation
- [Examples](examples/) - Practical examples

### Developer Documentation
- [Architecture](architecture.md) - System design and modules
- [API Reference](api-reference.md) - Complete API documentation
- [JSONL Format](jsonl-format.md) - Output format specification
- [Development](development.md) - Contributing and building

### Advanced Topics
- [Memory Access](memory-access.md) - Memory pointer usage
- [Async Operations](async-operations.md) - Async/await patterns
- [SIMD Optimizations](simd-optimizations.md) - Performance tuning
- [ML Integration](ml-integration.md) - Pattern detection
- [Bindings](bindings.md) - Cross-language bindings
- [Performance](performance.md) - Benchmarking guide

## Performance Characteristics

- **Memory Mapping**: 10x faster for files > 10MB
- **SIMD Operations**: 2-4x faster text processing
- **Async I/O**: 3x improvement for concurrent operations
- **Zero-Copy**: Minimal memory allocations

## Platform Support

- **Linux**: x86_64, aarch64
- **macOS**: x86_64, aarch64 (Apple Silicon)
- **Windows**: x86_64

## License

MIT OR Apache-2.0

## Contributing

See [Development Guide](development.md) for contribution guidelines.

## Support

- GitHub Issues: https://github.com/your-org/ai-coreutils/issues
- Documentation: https://docs.ai-coreutils.dev
