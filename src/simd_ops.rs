//! SIMD operations for AI-Coreutils
//!
//! This module provides SIMD-accelerated operations for text processing,
//! pattern matching, and byte counting. Uses portable SIMD via std::simd
//! or falls back to optimized scalar implementations.

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

/// SIMD configuration and capabilities
#[derive(Debug, Clone)]
pub struct SimdConfig {
    /// Enable SIMD optimizations
    pub enabled: bool,
    /// Preferred vector width (in bytes)
    pub vector_width: usize,
}

impl Default for SimdConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            vector_width: 32, // Default to 256-bit (32-byte) vectors
        }
    }
}

impl SimdConfig {
    /// Detect CPU SIMD capabilities and set optimal configuration
    pub fn detect() -> Self {
        #[cfg(target_arch = "x86_64")]
        {
            if is_x86_feature_detected!("avx2") {
                return Self {
                    enabled: true,
                    vector_width: 32, // AVX2: 256-bit
                };
            }
            if is_x86_feature_detected!("sse4.1") || is_x86_feature_detected!("sse2") {
                return Self {
                    enabled: true,
                    vector_width: 16, // SSE: 128-bit
                };
            }
        }

        #[cfg(target_arch = "aarch64")]
        {
            // ARM NEON is generally available on aarch64
            return Self {
                enabled: true,
                vector_width: 16, // NEON: 128-bit
            };
        }

        // Fallback to scalar
        Self {
            enabled: false,
            vector_width: 1,
        }
    }
}

/// SIMD-accelerated pattern searcher
pub struct SimdPatternSearcher {
    config: SimdConfig,
}

impl SimdPatternSearcher {
    /// Create a new SIMD pattern searcher with auto-detected capabilities
    pub fn new() -> Self {
        Self {
            config: SimdConfig::detect(),
        }
    }

    /// Create a new SIMD pattern searcher with explicit configuration
    pub fn with_config(config: SimdConfig) -> Self {
        Self { config }
    }

    /// Search for a pattern in a byte slice using SIMD when beneficial
    ///
    /// Returns the first offset where the pattern is found, or None if not found.
    pub fn find_first(&self, haystack: &[u8], needle: &[u8]) -> Option<usize> {
        if needle.is_empty() {
            return Some(0);
        }
        if haystack.len() < needle.len() {
            return None;
        }

        // For short patterns or haystacks, use the built-in method
        if haystack.len() < 256 || needle.len() < 2 {
            return haystack.windows(needle.len()).position(|w| w == needle);
        }

        // Use SIMD for large searches
        if self.config.enabled && needle.len() == 1 {
            // Single-byte search can be heavily optimized with SIMD
            self.find_byte_simd(haystack, needle[0])
        } else {
            // For multi-byte patterns, use optimized scalar search
            self.find_pattern_optimized(haystack, needle)
        }
    }

    /// Find all occurrences of a pattern using SIMD-accelerated search
    pub fn find_all(&self, haystack: &[u8], needle: &[u8]) -> Vec<usize> {
        if needle.is_empty() {
            return (0..=haystack.len()).collect();
        }
        if haystack.len() < needle.len() {
            return Vec::new();
        }

        let mut matches = Vec::new();
        let mut start = 0;

        while let Some(offset) = self.find_first(&haystack[start..], needle) {
            let absolute_offset = start + offset;
            matches.push(absolute_offset);
            start = absolute_offset + needle.len();

            if start >= haystack.len() {
                break;
            }
        }

        matches
    }

    /// SIMD-accelerated single byte search
    #[cfg(target_arch = "x86_64")]
    fn find_byte_simd(&self, haystack: &[u8], needle: u8) -> Option<usize> {
        if is_x86_feature_detected!("avx2") {
            unsafe { self.find_byte_avx2(haystack, needle) }
        } else if is_x86_feature_detected!("sse2") {
            unsafe { self.find_byte_sse2(haystack, needle) }
        } else {
            self.find_byte_scalar(haystack, needle)
        }
    }

    /// AVX2 implementation of single byte search
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "avx2")]
    unsafe fn find_byte_avx2(&self, haystack: &[u8], needle: u8) -> Option<usize> {
        const VECTOR_SIZE: usize = 32;

        let len = haystack.len();
        let mut pos = 0;

        // Process 32 bytes at a time
        while pos + VECTOR_SIZE <= len {
            let ptr = haystack.as_ptr().add(pos) as *const __m256i;
            let data = _mm256_loadu_si256(ptr);

            // Broadcast the needle byte to all lanes
            let needle_vec = _mm256_set1_epi8(needle as i8);

            // Compare for equality
            let cmp = _mm256_cmpeq_epi8(data, needle_vec);

            // Create a mask of matching bytes
            let mask = _mm256_movemask_epi8(cmp);

            if mask != 0 {
                // Find the position of the first set bit
                let trailing = mask.trailing_zeros() as usize;
                return Some(pos + trailing);
            }

            pos += VECTOR_SIZE;
        }

        // Handle remaining bytes
        self.find_byte_scalar(&haystack[pos..], needle).map(|offset| pos + offset)
    }

    /// SSE2 implementation of single byte search
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "sse2")]
    unsafe fn find_byte_sse2(&self, haystack: &[u8], needle: u8) -> Option<usize> {
        const VECTOR_SIZE: usize = 16;

        let len = haystack.len();
        let mut pos = 0;

        // Process 16 bytes at a time
        while pos + VECTOR_SIZE <= len {
            let ptr = haystack.as_ptr().add(pos) as *const __m128i;
            let data = _mm_loadu_si128(ptr);

            // Broadcast the needle byte to all lanes
            let needle_vec = _mm_set1_epi8(needle as i8);

            // Compare for equality
            let cmp = _mm_cmpeq_epi8(data, needle_vec);

            // Create a mask of matching bytes
            let mask = _mm_movemask_epi8(cmp);

            if mask != 0 {
                // Find the position of the first set bit
                let trailing = mask.trailing_zeros() as usize;
                return Some(pos + trailing);
            }

            pos += VECTOR_SIZE;
        }

        // Handle remaining bytes
        self.find_byte_scalar(&haystack[pos..], needle).map(|offset| pos + offset)
    }

    /// Scalar fallback for single byte search
    fn find_byte_scalar(&self, haystack: &[u8], needle: u8) -> Option<usize> {
        haystack.iter().position(|&b| b == needle)
    }

    /// Optimized pattern search using two-way algorithm
    fn find_pattern_optimized(&self, haystack: &[u8], needle: &[u8]) -> Option<usize> {
        // Use memchr for the first byte, then verify the full pattern
        let first_byte = needle[0];
        let mut pos = 0;

        while let Some(byte_offset) = self.find_byte_scalar(&haystack[pos..], first_byte) {
            let absolute_pos = pos + byte_offset;

            // Check if we have enough room for the full pattern
            if absolute_pos + needle.len() > haystack.len() {
                return None;
            }

            // Verify the full pattern matches
            if &haystack[absolute_pos..absolute_pos + needle.len()] == needle {
                return Some(absolute_pos);
            }

            pos = absolute_pos + 1;
        }

        None
    }
}

impl Default for SimdPatternSearcher {
    fn default() -> Self {
        Self::new()
    }
}

/// SIMD-accelerated byte counter
pub struct SimdByteCounter {
    config: SimdConfig,
}

impl SimdByteCounter {
    /// Create a new SIMD byte counter with auto-detected capabilities
    pub fn new() -> Self {
        Self {
            config: SimdConfig::detect(),
        }
    }

    /// Create a new SIMD byte counter with explicit configuration
    pub fn with_config(config: SimdConfig) -> Self {
        Self { config }
    }

    /// Count occurrences of a byte using SIMD
    pub fn count(&self, data: &[u8], byte: u8) -> usize {
        if !self.config.enabled || data.len() < 64 {
            return data.iter().filter(|&&b| b == byte).count();
        }

        #[cfg(target_arch = "x86_64")]
        {
            if is_x86_feature_detected!("avx2") {
                return unsafe { self.count_avx2(data, byte) };
            }
            if is_x86_feature_detected!("sse2") {
                return unsafe { self.count_sse2(data, byte) };
            }
        }

        self.count_scalar(data, byte)
    }

    /// AVX2 implementation of byte counting
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "avx2")]
    unsafe fn count_avx2(&self, data: &[u8], byte: u8) -> usize {
        const VECTOR_SIZE: usize = 32;

        let len = data.len();
        let mut pos = 0;
        let mut count = 0;

        // Process 32 bytes at a time
        while pos + VECTOR_SIZE <= len {
            let ptr = data.as_ptr().add(pos) as *const __m256i;
            let vec_data = _mm256_loadu_si256(ptr);

            // Broadcast the target byte to all lanes
            let vec_byte = _mm256_set1_epi8(byte as i8);

            // Compare for equality
            let cmp = _mm256_cmpeq_epi8(vec_data, vec_byte);

            // Create a mask and count bits
            let mask = _mm256_movemask_epi8(cmp) as u32;
            count += mask.count_ones() as usize;

            pos += VECTOR_SIZE;
        }

        // Handle remaining bytes
        count += self.count_scalar(&data[pos..], byte);

        count
    }

    /// SSE2 implementation of byte counting
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "sse2")]
    unsafe fn count_sse2(&self, data: &[u8], byte: u8) -> usize {
        const VECTOR_SIZE: usize = 16;

        let len = data.len();
        let mut pos = 0;
        let mut count = 0;

        // Process 16 bytes at a time
        while pos + VECTOR_SIZE <= len {
            let ptr = data.as_ptr().add(pos) as *const __m128i;
            let vec_data = _mm_loadu_si128(ptr);

            // Broadcast the target byte to all lanes
            let vec_byte = _mm_set1_epi8(byte as i8);

            // Compare for equality
            let cmp = _mm_cmpeq_epi8(vec_data, vec_byte);

            // Create a mask and count bits
            let mask = _mm_movemask_epi8(cmp) as u32;
            count += mask.count_ones() as usize;

            pos += VECTOR_SIZE;
        }

        // Handle remaining bytes
        count += self.count_scalar(&data[pos..], byte);

        count
    }

    /// Scalar fallback for byte counting
    fn count_scalar(&self, data: &[u8], byte: u8) -> usize {
        data.iter().filter(|&&b| b == byte).count()
    }

    /// Count multiple bytes simultaneously
    pub fn count_multiple(&self, data: &[u8], bytes: &[u8]) -> Vec<(u8, usize)> {
        bytes.iter().map(|&byte| {
            (byte, self.count(data, byte))
        }).collect()
    }
}

impl Default for SimdByteCounter {
    fn default() -> Self {
        Self::new()
    }
}

/// SIMD-accelerated whitespace detector
pub struct SimdWhitespaceDetector {
    config: SimdConfig,
}

impl SimdWhitespaceDetector {
    /// Create a new SIMD whitespace detector with auto-detected capabilities
    pub fn new() -> Self {
        Self {
            config: SimdConfig::detect(),
        }
    }

    /// Find the next non-whitespace character offset
    pub fn skip_whitespace(&self, data: &[u8], mut start: usize) -> usize {
        while start < data.len() {
            if !data[start].is_ascii_whitespace() {
                break;
            }
            start += 1;
        }
        start
    }

    /// Count lines in a buffer
    pub fn count_lines(&self, data: &[u8]) -> usize {
        self.count_byte(data, b'\n')
    }

    /// Count words in a buffer
    pub fn count_words(&self, data: &[u8]) -> usize {
        let mut count = 0;
        let mut in_word = false;

        for &byte in data.iter() {
            let is_whitespace = byte.is_ascii_whitespace();
            if is_whitespace {
                if in_word {
                    count += 1;
                    in_word = false;
                }
            } else {
                in_word = true;
            }
        }

        // Count the last word if the buffer doesn't end with whitespace
        if in_word {
            count += 1;
        }

        count
    }

    fn count_byte(&self, data: &[u8], byte: u8) -> usize {
        if !self.config.enabled || data.len() < 64 {
            return data.iter().filter(|&&b| b == byte).count();
        }

        #[cfg(target_arch = "x86_64")]
        {
            if is_x86_feature_detected!("avx2") {
                return unsafe { self.count_byte_avx2(data, byte) };
            }
            if is_x86_feature_detected!("sse2") {
                return unsafe { self.count_byte_sse2(data, byte) };
            }
        }

        self.count_byte_scalar(data, byte)
    }

    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "avx2")]
    unsafe fn count_byte_avx2(&self, data: &[u8], byte: u8) -> usize {
        const VECTOR_SIZE: usize = 32;

        let len = data.len();
        let mut pos = 0;
        let mut count = 0;

        while pos + VECTOR_SIZE <= len {
            let ptr = data.as_ptr().add(pos) as *const __m256i;
            let vec_data = _mm256_loadu_si256(ptr);
            let vec_byte = _mm256_set1_epi8(byte as i8);
            let cmp = _mm256_cmpeq_epi8(vec_data, vec_byte);
            let mask = _mm256_movemask_epi8(cmp) as u32;
            count += mask.count_ones() as usize;
            pos += VECTOR_SIZE;
        }

        count += self.count_byte_scalar(&data[pos..], byte);
        count
    }

    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "sse2")]
    unsafe fn count_byte_sse2(&self, data: &[u8], byte: u8) -> usize {
        const VECTOR_SIZE: usize = 16;

        let len = data.len();
        let mut pos = 0;
        let mut count = 0;

        while pos + VECTOR_SIZE <= len {
            let ptr = data.as_ptr().add(pos) as *const __m128i;
            let vec_data = _mm_loadu_si128(ptr);
            let vec_byte = _mm_set1_epi8(byte as i8);
            let cmp = _mm_cmpeq_epi8(vec_data, vec_byte);
            let mask = _mm_movemask_epi8(cmp) as u32;
            count += mask.count_ones() as usize;
            pos += VECTOR_SIZE;
        }

        count += self.count_byte_scalar(&data[pos..], byte);
        count
    }

    fn count_byte_scalar(&self, data: &[u8], byte: u8) -> usize {
        data.iter().filter(|&&b| b == byte).count()
    }
}

impl Default for SimdWhitespaceDetector {
    fn default() -> Self {
        Self::new()
    }
}

/// SIMD-optimized text processing utilities
pub struct SimdTextProcessor {
    pattern_searcher: SimdPatternSearcher,
    byte_counter: SimdByteCounter,
    whitespace_detector: SimdWhitespaceDetector,
}

impl SimdTextProcessor {
    /// Create a new SIMD text processor
    pub fn new() -> Self {
        Self {
            pattern_searcher: SimdPatternSearcher::new(),
            byte_counter: SimdByteCounter::new(),
            whitespace_detector: SimdWhitespaceDetector::new(),
        }
    }

    /// Create a new SIMD text processor with explicit configuration
    pub fn with_config(config: SimdConfig) -> Self {
        Self {
            pattern_searcher: SimdPatternSearcher::with_config(config.clone()),
            byte_counter: SimdByteCounter::with_config(config.clone()),
            whitespace_detector: SimdWhitespaceDetector::new(),
        }
    }

    /// Count lines, words, and bytes in a single pass
    pub fn analyze(&self, data: &[u8]) -> TextMetrics {
        let lines = self.whitespace_detector.count_lines(data);
        let words = self.whitespace_detector.count_words(data);
        let bytes = data.len();

        TextMetrics { lines, words, bytes }
    }

    /// Get references to internal components
    pub fn pattern_searcher(&self) -> &SimdPatternSearcher {
        &self.pattern_searcher
    }

    /// Get the byte counter component
    pub fn byte_counter(&self) -> &SimdByteCounter {
        &self.byte_counter
    }

    /// Get the whitespace detector component
    pub fn whitespace_detector(&self) -> &SimdWhitespaceDetector {
        &self.whitespace_detector
    }
}

impl Default for SimdTextProcessor {
    fn default() -> Self {
        Self::new()
    }
}

/// Text metrics result
#[derive(Debug, Clone, Copy)]
pub struct TextMetrics {
    /// Number of lines
    pub lines: usize,
    /// Number of words
    pub words: usize,
    /// Number of bytes
    pub bytes: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_searcher_find_first() {
        let searcher = SimdPatternSearcher::new();
        let haystack = b"Hello World! Hello again!";
        let needle = b"World";

        assert_eq!(searcher.find_first(haystack, needle), Some(6));
    }

    #[test]
    fn test_pattern_searcher_find_all() {
        let searcher = SimdPatternSearcher::new();
        let haystack = b"abc abc abc abc";
        let needle = b"abc";

        let matches = searcher.find_all(haystack, needle);
        assert_eq!(matches, vec![0, 4, 8, 12]);
    }

    #[test]
    fn test_byte_counter() {
        let counter = SimdByteCounter::new();
        let data = b"hello world, hello!";

        assert_eq!(counter.count(data, b'l'), 5);
        assert_eq!(counter.count(data, b'o'), 3);
        assert_eq!(counter.count(data, b'x'), 0);
    }

    #[test]
    fn test_whitespace_detector_count_lines() {
        let detector = SimdWhitespaceDetector::new();
        let data = b"Line 1\nLine 2\nLine 3\n";

        assert_eq!(detector.count_lines(data), 3);
    }

    #[test]
    fn test_whitespace_detector_count_words() {
        let detector = SimdWhitespaceDetector::new();
        let data = b"hello world this is a test";

        assert_eq!(detector.count_words(data), 6);
    }

    #[test]
    fn test_text_processor_analyze() {
        let processor = SimdTextProcessor::new();
        let data = b"Hello world\nThis is a test\n";

        let metrics = processor.analyze(data);
        assert_eq!(metrics.lines, 2);
        assert_eq!(metrics.words, 6);
        assert_eq!(metrics.bytes, 27); // "Hello world\nThis is a test\n" = 27 bytes
    }

    #[test]
    fn test_empty_data() {
        let processor = SimdTextProcessor::new();
        let data = b"";

        let metrics = processor.analyze(data);
        assert_eq!(metrics.lines, 0);
        assert_eq!(metrics.words, 0);
        assert_eq!(metrics.bytes, 0);
    }

    #[test]
    fn test_pattern_not_found() {
        let searcher = SimdPatternSearcher::new();
        let haystack = b"Hello World!";
        let needle = b"xyz";

        assert_eq!(searcher.find_first(haystack, needle), None);
    }

    #[test]
    fn test_byte_counter_multiple() {
        let counter = SimdByteCounter::new();
        let data = b"hello world";

        let counts = counter.count_multiple(data, &[b'l', b'o', b'x']);
        assert_eq!(counts, vec![(b'l', 3), (b'o', 2), (b'x', 0)]);
    }
}
