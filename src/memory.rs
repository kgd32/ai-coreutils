//! Memory access layer
//!
//! Provides safe memory access with pointer operations for large files.

use crate::error::{AiCoreutilsError, Result};
use memmap2::Mmap;
use std::fs::File;
use std::path::Path;

/// Safe memory access handler for files
pub struct SafeMemoryAccess {
    mmap: Mmap,
    size: usize,
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
            .map_err(|e| AiCoreutilsError::Io(e))?;

        let metadata = file.metadata()
            .map_err(|e| AiCoreutilsError::Io(e))?;

        let size = metadata.len() as usize;

        let mmap = unsafe {
            Mmap::map(&file)
                .map_err(|e| AiCoreutilsError::MemoryAccess(format!("Failed to map file: {}", e)))?
        };

        Ok(Self { mmap, size })
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

    /// Search for a pattern in the memory-mapped region
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

        let mut matches = Vec::new();
        let mut search_offset = 0;

        while search_offset + pattern.len() <= self.size {
            if let Some(window) = self.get(search_offset, pattern.len()) {
                if window == pattern {
                    matches.push(search_offset);
                }
            }
            search_offset += 1;
        }

        matches
    }

    /// Count occurrences of a byte in the memory-mapped region
    pub fn count_byte(&self, byte: u8) -> usize {
        self.mmap.iter().filter(|&&b| b == byte).count()
    }

    /// Create a SafeMemoryAccess from a vector (for testing)
    #[cfg(test)]
    pub fn from_vec(data: Vec<u8>) -> Result<Self> {
        use std::io::Cursor;
        // Create a temporary file-like structure
        let mmap = Mmap::map(&data.as_slice())
            .map_err(|e| AiCoreutilsError::MemoryAccess(format!("Failed to create mmap from vec: {}", e)))?;

        Ok(Self {
            size: data.len(),
            mmap,
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
}
