//! Memory access layer
//!
//! Provides safe memory access with pointer operations for large files.

use crate::error::{AiCoreutilsError, Result};
use crate::simd_ops::{SimdByteCounter, SimdPatternSearcher, SimdTextProcessor};
use memmap2::Mmap;
use std::fs::File;
use std::path::Path;

/// Safe memory access handler for files
pub struct SafeMemoryAccess {
    mmap: Mmap,
    size: usize,
    pattern_searcher: SimdPatternSearcher,
    byte_counter: SimdByteCounter,
    text_processor: SimdTextProcessor,
}

impl SafeMemoryAccess {
    /// Create a new memory-mapped file access
    ///
    /// # Arguments
    /// * `path` - Path to the file to memory map
    ///
    /// # Returns
    /// A `SafeMemoryAccess` instance if successful
    ///
    /// # Example
    /// ```no_run
    /// use ai_coreutils::memory::SafeMemoryAccess;
    ///
    /// let access = SafeMemoryAccess::new("/path/to/file").unwrap();
    /// let data = access.get(0, 100).unwrap();
    /// ```
    pub fn new(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let file = File::open(path)
            .map_err(AiCoreutilsError::Io)?;

        let metadata = file.metadata()
            .map_err(AiCoreutilsError::Io)?;

        let size = metadata.len() as usize;

        let mmap = unsafe {
            Mmap::map(&file)
                .map_err(|e| AiCoreutilsError::MemoryAccess(format!("Failed to map file: {}", e)))?
        };

        Ok(Self {
            mmap,
            size,
            pattern_searcher: SimdPatternSearcher::new(),
            byte_counter: SimdByteCounter::new(),
            text_processor: SimdTextProcessor::new(),
        })
    }

    /// Get the size of the memory-mapped region
    pub fn size(&self) -> usize {
        self.size
    }

    /// Get a raw pointer to the memory
    pub fn as_ptr(&self) -> *const u8 {
        self.mmap.as_ptr()
    }

    /// Get a mutable pointer to the memory (if writable)
    #[allow(clippy::mut_from_ref)]
    pub fn as_mut_ptr(&self) -> *mut u8 {
        self.mmap.as_ptr() as *mut u8
    }

    /// Bounds-checked access to a slice of memory
    ///
    /// # Arguments
    /// * `offset` - Starting offset in bytes
    /// * `len` - Length of the slice to read
    ///
    /// # Returns
    /// `Some(&[u8])` if the range is valid, `None` otherwise
    pub fn get(&self, offset: usize, len: usize) -> Option<&[u8]> {
        if offset.saturating_add(len) <= self.size {
            Some(&self.mmap[offset..offset + len])
        } else {
            None
        }
    }

    /// Get a byte at the given offset
    ///
    /// # Returns
    /// `Some(u8)` if the offset is valid, `None` otherwise
    pub fn get_byte(&self, offset: usize) -> Option<u8> {
        if offset < self.size {
            Some(self.mmap[offset])
        } else {
            None
        }
    }

    /// Search for a pattern in the memory-mapped region (SIMD-accelerated)
    ///
    /// # Arguments
    /// * `pattern` - Byte pattern to search for
    ///
    /// # Returns
    /// Vector of offsets where the pattern was found
    pub fn find_pattern(&self, pattern: &[u8]) -> Vec<usize> {
        if pattern.is_empty() || pattern.len() > self.size {
            return Vec::new();
        }

        // Use SIMD-accelerated pattern search
        self.pattern_searcher.find_all(&self.mmap, pattern)
    }

    /// Count occurrences of a byte in the memory-mapped region (SIMD-accelerated)
    pub fn count_byte(&self, byte: u8) -> usize {
        self.byte_counter.count(&self.mmap, byte)
    }

    /// Count lines, words, and bytes in the memory-mapped region (SIMD-accelerated)
    ///
    /// # Returns
    /// Tuple of (lines, words, bytes)
    pub fn count_text_metrics(&self) -> (usize, usize, usize) {
        let metrics = self.text_processor.analyze(&self.mmap);
        (metrics.lines, metrics.words, metrics.bytes)
    }

    /// Create a SafeMemoryAccess from a vector (for testing)
    #[cfg(test)]
    pub fn from_vec(data: Vec<u8>) -> Result<Self> {
        use std::io::Write;
        // Create a temporary file
        let mut temp_file = tempfile::NamedTempFile::new()?;
        temp_file.write_all(&data)?;
        temp_file.flush()?;

        // Create mmap from the file
        let mmap = unsafe {
            Mmap::map(&*temp_file.as_file())
                .map_err(|e| AiCoreutilsError::MemoryAccess(format!("Failed to create mmap from vec: {}", e)))?
        };

        Ok(Self {
            size: data.len(),
            mmap,
            pattern_searcher: SimdPatternSearcher::new(),
            byte_counter: SimdByteCounter::new(),
            text_processor: SimdTextProcessor::new(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_memory_access_size() {
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(b"Hello, World!").unwrap();

        let access = SafeMemoryAccess::new(temp_file.path()).unwrap();
        assert_eq!(access.size(), 13);
    }

    #[test]
    fn test_memory_access_get() {
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(b"Hello, World!").unwrap();

        let access = SafeMemoryAccess::new(temp_file.path()).unwrap();
        let data = access.get(0, 5).unwrap();
        assert_eq!(data, b"Hello");
    }

    #[test]
    fn test_memory_access_bounds_checking() {
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(b"Hello").unwrap();

        let access = SafeMemoryAccess::new(temp_file.path()).unwrap();

        // Valid access
        assert!(access.get(0, 5).is_some());

        // Out of bounds
        assert!(access.get(0, 10).is_none());
        assert!(access.get(10, 1).is_none());
    }

    #[test]
    fn test_memory_access_get_byte() {
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(b"ABC").unwrap();

        let access = SafeMemoryAccess::new(temp_file.path()).unwrap();

        assert_eq!(access.get_byte(0), Some(b'A'));
        assert_eq!(access.get_byte(1), Some(b'B'));
        assert_eq!(access.get_byte(10), None);
    }

    #[test]
    fn test_find_pattern() {
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(b"abc abc abc").unwrap();

        let access = SafeMemoryAccess::new(temp_file.path()).unwrap();
        let matches = access.find_pattern(b"abc");

        assert_eq!(matches, vec![0, 4, 8]);
    }

    #[test]
    fn test_count_byte() {
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(b"hello world").unwrap();

        let access = SafeMemoryAccess::new(temp_file.path()).unwrap();
        assert_eq!(access.count_byte(b'l'), 3);
        assert_eq!(access.count_byte(b'x'), 0);
    }

    #[test]
    fn test_count_text_metrics() {
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(b"Hello world\nThis is a test\n").unwrap();

        let access = SafeMemoryAccess::new(temp_file.path()).unwrap();
        let (lines, words, bytes) = access.count_text_metrics();

        assert_eq!(lines, 2);
        assert_eq!(words, 6);
        assert_eq!(bytes, 27);
    }
}
