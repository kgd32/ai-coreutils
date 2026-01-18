# GNU Coreutils Specifications - Reverse Engineered

## Overview
GNU coreutils is a collection of essential file, shell, and text manipulation utilities. This document reverse engineers their specifications to inform the design of AI-Coreutils.

## Utility Categories

### 1. File System Operations
#### `ls` - List directory contents
**Purpose**: Display directory contents
**Key Options**:
- `-l`: Long format (permissions, owner, size, date)
- `-a`: Show all files (including hidden)
- `-h`: Human-readable sizes
- `-R`: Recursive listing
- `-t`: Sort by modification time
- `-S`: Sort by size
- `-X`: Sort by extension

**Input**: Directory path(s) or current directory
**Output**: Tabulated list (default) or detailed info
**Exit Codes**: 0 (success), 1 (minor issues), 2 (serious errors)

**Performance**: O(n) where n = number of directory entries
**Edge Cases**: 
- Permission denied directories
- Symbolic link loops
- Very large directories (>1M entries)
- Network filesystems

---

#### `cp` - Copy files and directories
**Purpose**: Duplicate files/directories
**Key Options**:
- `-r`, `-R`: Recursive copy
- `-a`: Archive mode (preserves all attributes)
- `-p`: Preserve permissions, timestamps
- `-v`: Verbose output
- `-i`: Interactive prompt
- `-u`: Update only newer files
- `-l`: Create hard links instead of copying
- `-s`: Create symbolic links

**Input**: Source file(s), destination
**Output**: None (except with -v)
**Exit Codes**: 0 (success), non-zero (errors)

**Performance**: O(file_size) + metadata operations
**Edge Cases**:
- Copying to same directory
- Permission issues
- Disk space exhaustion
- Sparse files
- Extended attributes

---

#### `mv` - Move/rename files
**Purpose**: Relocate files within filesystem
**Key Options**:
- `-i`: Interactive prompt
- `-v`: Verbose output
- `-u`: Update only newer files
- `-n`: No clobber
- `-T`: Treat destination as normal file

**Input**: Source(s), destination
**Output**: None (except with -v)
**Exit Codes**: 0 (success), non-zero (errors)

**Performance**: O(1) for same filesystem (metadata update), O(file_size) for cross-filesystem
**Edge Cases**:
- Moving across filesystems
- Moving to self
- Directory conflicts
- Permission issues

---

#### `rm` - Remove files
**Purpose**: Delete files/directories
**Key Options**:
- `-r`, `-R`: Recursive removal
- `-f`: Force (ignore nonexistent files, never prompt)
- `-i`: Interactive prompt
- `-v`: Verbose output
- `-I`: Prompt once for >3 files

**Input**: File path(s)
**Output**: None (except with -v)
**Exit Codes**: 0 (success), non-zero (errors)

**Performance**: O(1) for unlink, O(n) for directory traversal
**Edge Cases**:
- Write-protected files
- Non-empty directories
- Very deep directory trees
- Open file handles

---

#### `mkdir` - Create directories
**Purpose**: Make new directories
**Key Options**:
- `-p`: Create parent directories as needed
- `-m`: Set permissions (mode)
- `-v`: Verbose output
- `-Z`: Set SELinux context

**Input**: Directory path(s)
**Output**: None (except with -v)
**Exit Codes**: 0 (success), non-zero (errors)

**Performance**: O(1) per directory
**Edge Cases**:
- Parent directories don't exist
- Permission denied
- Directory already exists
- Nested path creation

---

#### `rmdir` - Remove empty directories
**Purpose**: Delete empty directories
**Key Options**:
- `-p`: Remove parent directories if empty
- `-v`: Verbose output
- `--ignore-fail-on-non-empty`

**Input**: Directory path(s)
**Output**: None (except with -v)
**Exit Codes**: 0 (success), non-zero (errors)

**Performance**: O(1) per directory
**Edge Cases**:
- Non-empty directories
- Permission denied
- Symbolic links to directories

---

### 2. File Content Operations
#### `cat` - Concatenate and display files
**Purpose**: Display file contents sequentially
**Key Options**:
- `-n`: Number all output lines
- `-b`: Number non-blank lines
- `-A`: Show all characters (including non-printing)
- `-E`: Show end of lines ($)
- `-T`: Show tabs as ^I
- `-s`: Squeeze multiple blank lines

**Input**: File path(s) or stdin
**Output**: File contents to stdout
**Exit Codes**: 0 (success), non-zero (errors)

**Performance**: O(total_file_size)
**Edge Cases**:
- Very large files
- Binary files
- Files that disappear while reading
- Special files (devices, pipes)

---

#### `tac` - Concatenate and display files in reverse
**Purpose**: Display files line-by-line in reverse order
**Key Options**:
- `-b`: Before separator
- `-r`: Use regex as separator
- `-s`: Use specified separator

**Input**: File path(s) or stdin
**Output**: Reversed file contents
**Exit Codes**: 0 (success), non-zero (errors)

**Performance**: O(file_size) + memory for buffering
**Edge Cases**:
- Very large files (memory intensive)
- Different line endings
- Files without final newline

---

#### `head` - Output first part of files
**Purpose**: Display first lines of files
**Key Options**:
- `-n NUM`: Number of lines (default 10)
- `-c BYTES`: Number of bytes
- `-q`: Quiet mode (no headers)
- `-v`: Verbose mode (always headers)

**Input**: File path(s) or stdin
**Output**: First N lines/bytes
**Exit Codes**: 0 (success), non-zero (errors)

**Performance**: O(N) where N = requested bytes/lines
**Edge Cases**:
- Files smaller than requested size
- Binary files
- Multiple files with headers

---

#### `tail` - Output last part of files
**Purpose**: Display last lines of files
**Key Options**:
- `-n NUM`: Number of lines (default 10)
- `-f`: Follow file (output appended data)
- `-F`: Follow by name (reopens if deleted)
- `-c BYTES`: Number of bytes
- `--pid=PID`: Exit when process dies

**Input**: File path(s) or stdin
**Output**: Last N lines/bytes
**Exit Codes**: 0 (success), non-zero (errors)

**Performance**: O(N) for initial read, O(1) for follow
**Edge Cases**:
- File rotation during -f
- Files that shrink
- Multiple file following
- Network latency

---

#### `wc` - Word, line, character count
**Purpose**: Count lines, words, characters in files
**Key Options**:
- `-l`: Count lines only
- `-w`: Count words only
- `-c`: Count bytes only
- `-m`: Count characters only
- `-L`: Maximum line length

**Input**: File path(s) or stdin
**Output**: Counts to stdout
**Exit Codes**: 0 (success), non-zero (errors)

**Performance**: O(file_size)
**Edge Cases**:
- Unicode characters
- Very long lines
- Binary files
- Multiple files with totals

---

### 3. Text Processing
#### `grep` - Pattern searching
**Purpose**: Search for patterns in text
**Key Options**:
- `-i`: Case insensitive
- `-v`: Invert match
- `-r`, `-R`: Recursive search
- `-n`: Show line numbers
- `-c`: Count matching lines
- `-l`: List matching files
- `-L`: List non-matching files
- `-E`: Extended regex
- `-P`: Perl regex
- `-F`: Fixed strings
- `-o`: Show only matching part
- `-A NUM`: After context
- `-B NUM`: Before context
- `-C NUM`: Context

**Input**: Pattern, file path(s) or stdin
**Output**: Matching lines
**Exit Codes**: 0 (matches found), 1 (no matches), 2 (errors)

**Performance**: O(file_size * pattern_complexity)
**Edge Cases**:
- Complex regex patterns
- Very large files
- Binary files
- Encoding issues

---

#### `sed` - Stream editor
**Purpose**: Text transformation
**Key Options**:
- `-e`: Execute script
- `-f`: Script file
- `-i`: Edit files in-place
- `-n`: Suppress default output
- `-r`: Extended regex

**Input**: Script commands, file path(s) or stdin
**Output**: Transformed text
**Exit Codes**: 0 (success), non-zero (errors)

**Performance**: O(file_size * operations)
**Edge Cases**:
- Complex scripts
- In-place editing
- Large files
- Pattern space overflow

---

#### `awk` - Pattern scanning and processing
**Purpose**: Text processing with programming language
**Key Options**:
- `-F`: Field separator
- `-v`: Variable assignment
- `-f`: Program file

**Input**: Program, file path(s) or stdin
**Output**: Processed text
**Exit Codes**: 0 (success), non-zero (errors)

**Performance**: O(file_size * program_complexity)
**Edge Cases**:
- Complex programs
- Large datasets
- Memory limits
- Field parsing

---

#### `sort` - Sort lines of text
**Purpose**: Sort text lines
**Key Options**:
- `-n`: Numeric sort
- `-r`: Reverse sort
- `-f`: Case insensitive
- `-k`: Sort key
- `-t`: Field separator
- `-u`: Unique lines
- `-o`: Output file
- `-T`: Temporary directory
- `-S`: Buffer size

**Input**: File path(s) or stdin
**Output**: Sorted lines
**Exit Codes**: 0 (success), non-zero (errors)

**Performance**: O(n log n) where n = lines
**Edge Cases**:
- Very large files (disk-based sort)
- Complex keys
- Memory exhaustion
- Temporary file space

---

#### `uniq` - Remove duplicate lines
**Purpose**: Filter adjacent duplicate lines
**Key Options**:
- `-c`: Count occurrences
- `-d`: Only duplicates
- `-u`: Only unique
- `-i`: Case insensitive
- `-f`: Skip fields
- `-s`: Skip characters

**Input**: File path(s) or stdin
**Output**: Filtered lines
**Exit Codes**: 0 (success), non-zero (errors)

**Performance**: O(file_size)
**Edge Cases**:
- Unsorted input
- Large files
- Complex skipping rules

---

### 4. System Information
#### `df` - Disk free space
**Purpose**: Report filesystem disk space usage
**Key Options**:
- `-h`: Human-readable
- `-i`: Inode usage
- `-T`: Filesystem type
- `-x`: Exclude type
- `-t`: Include type

**Input**: Filesystem path(s) or all
**Output**: Disk usage table
**Exit Codes**: 0 (success), non-zero (errors)

**Performance**: O(filesystems)
**Edge Cases**:
- Network filesystems
- Mount points during access
- Very large filesystems

---

#### `du` - Disk usage
**Purpose**: Summarize file space usage
**Key Options**:
- `-h`: Human-readable
- `-s`: Summary only
- `-a`: All files
- `-c`: Grand total
- `-d`: Depth
- `--exclude`: Exclude pattern
- `-X`: Exclude from file

**Input**: Directory path(s)
**Output**: Usage statistics
**Exit Codes**: 0 (success), non-zero (errors)

**Performance**: O(total_files)
**Edge Cases**:
- Very deep directory trees
- Hard links (counted multiple times)
- Sparse files
- Network filesystems

---

#### `ps` - Process status
**Purpose**: Report process status
**Key Options**:
- `-a`: All users
- `-u`: User-oriented format
- `-x`: Include processes without tty
- `-e`: All processes
- `-f`: Full format
- `-o`: Custom format
- `-p`: Specific process
- `-C`: Command name

**Input**: None (reads /proc)
**Output**: Process table
**Exit Codes**: 0 (success), non-zero (errors)

**Performance**: O(processes)
**Edge Cases**:
- Processes ending during execution
- Zombie processes
- Permission denied for some processes

---

### 5. File Comparison
#### `diff` - Compare files line by line
**Purpose**: Find differences between files
**Key Options**:
- `-u`: Unified format
- `-c`: Context format
- `-y`: Side by side
- `-w`: Ignore all whitespace
- `-i`: Ignore case
- `-r`: Recursive
- `-q`: Brief output

**Input**: File/directory paths
**Output**: Differences
**Exit Codes**: 0 (identical), 1 (different), 2 (errors)

**Performance**: O(file_size)
**Edge Cases**:
- Very large files
- Binary files
- Different encodings
- Directory comparisons

---

#### `cmp` - Compare two files byte by byte
**Purpose**: Byte-level file comparison
**Key Options**:
- `-l`: Byte-by-byte differences
- `-s`: Silent output
- `-i`: Skip bytes

**Input**: Two file paths
**Output**: First difference location
**Exit Codes**: 0 (identical), 1 (different), 2 (errors)

**Performance**: O(min(file1_size, file2_size))
**Edge Cases**:
- Very large files
- Binary files
- Files that disappear

---

### 6. File Type Operations
#### `file` - Determine file type
**Purpose**: Identify file type
**Key Options**:
- `-b`: Brief mode
- `-i`: MIME type
- `-L`: Follow symlinks
- `-z`: Look inside compressed files

**Input**: File path(s)
**Output**: File type description
**Exit Codes**: 0 (success), non-zero (errors)

**Performance**: O(file_header_size)
**Edge Cases**:
- Unknown file types
- Corrupted files
- Files without magic numbers

---

#### `find` - Search for files
**Purpose**: Search directory tree
**Key Options**:
- `-name`: Filename pattern
- `-type`: File type
- `-size`: File size
- `-mtime`: Modification time
- `-exec`: Execute command
- `-print`: Print path
- `-delete`: Delete found files

**Input**: Starting path(s), expressions
**Output**: Matching file paths
**Exit Codes**: 0 (success), non-zero (errors)

**Performance**: O(total_files_scanned)
**Edge Cases**:
- Very deep directory trees
- Network filesystems
- Complex expressions
- Permission denied directories

---

### 7. Archive Operations
#### `tar` - Tape archiver
**Purpose**: Archive files
**Key Options**:
- `-c`: Create archive
- `-x`: Extract archive
- `-t`: List contents
- `-v`: Verbose
- `-f`: Archive file
- `-z`: gzip compression
- `-j`: bzip2 compression
- `-J`: xz compression

**Input**: Operation, archive file, paths
**Output**: Archive or extracted files
**Exit Codes**: 0 (success), non-zero (errors)

**Performance**: O(total_file_size)
**Edge Cases**:
- Very large archives
- Compression errors
- Disk space
- Special files (devices, sockets)

---

## Common Patterns Across Coreutils

### 1. Option Parsing
- Short options: `-a`, `-b`, `-c`
- Long options: `--all`, `--backup`, `--binary`
- Combined short options: `-la` = `-l -a`
- Option arguments: `-n 10` or `-n10`

### 2. Input Handling
- Multiple file arguments
- Default to stdin if no files
- `-` represents stdin
- Recursive operations with `-r`/`-R`

### 3. Output Formats
- Human-readable with `-h`
- Quiet mode with `-q`
- Verbose mode with `-v`
- Null terminators with `-0` (some tools)

### 4. Exit Codes
- 0: Success
- 1: Minor issues/no matches
- 2: Serious errors

### 5. Error Handling
- Continue on non-fatal errors
- Report errors to stderr
- Exit with highest error code

### 6. Performance Characteristics
- Streaming for large files
- Memory efficient (usually)
- Disk-based operations for very large data

## AI-Optimization Opportunities

### 1. Structured Output
- All utilities can output JSONL
- Include metadata (timestamps, permissions, checksums)
- Machine-readable error messages

### 2. Memory Access
- Memory mapping for large files
- Direct pointer access for performance
- Zero-copy operations where possible

### 3. Parallel Processing
- Multi-file operations in parallel
- SIMD for text processing
- Async I/O for network operations

### 4. AI Enhancements
- Pattern detection in `grep`
- Intelligent sorting in `sort`
- Predictive file operations
- Anomaly detection

### 5. Extended Metadata
- File hashes
- Content type detection
- Access patterns
- Relationship mapping

## Implementation Priority for AI-Coreutils

### Phase 1 (MVP)
1. `ls` - Most common, demonstrates structured output
2. `cat` - Shows memory mapping capabilities
3. `grep` - AI pattern detection opportunity

### Phase 2
4. `find` - Recursive operations
5. `cp`/`mv` - File operations with progress
6. `head`/`tail` - Streaming operations

### Phase 3
7. `sort` - Parallel processing
8. `diff` - Intelligent comparison
9. `tar` - Archive operations with AI insights

This specification serves as the foundation for implementing AI-Coreutils with enhanced capabilities while maintaining compatibility with GNU coreutils behavior.