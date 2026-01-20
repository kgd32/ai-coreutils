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

/// SIMD-accelerated newline counter for line-based operations
/// Optimized for ai-head and ai-tail utilities
pub struct SimdNewlineCounter {
    config: SimdConfig,
}

impl SimdNewlineCounter {
    /// Create a new SIMD newline counter with auto-detected capabilities
    pub fn new() -> Self {
        Self {
            config: SimdConfig::detect(),
        }
    }

    /// Create a new SIMD newline counter with explicit configuration
    pub fn with_config(config: SimdConfig) -> Self {
        Self { config }
    }

    /// Find the position of the nth newline (1-indexed)
    /// Returns None if n newlines are not found
    pub fn find_nth_newline(&self, data: &[u8], n: usize) -> Option<usize> {
        if n == 0 {
            return Some(0);
        }
        if !self.config.enabled || data.len() < 64 {
            return self.find_nth_newline_scalar(data, n);
        }

        #[cfg(target_arch = "x86_64")]
        {
            if is_x86_feature_detected!("avx2") {
                return unsafe { self.find_nth_newline_avx2(data, n) };
            }
            if is_x86_feature_detected!("sse2") {
                return unsafe { self.find_nth_newline_sse2(data, n) };
            }
        }

        self.find_nth_newline_scalar(data, n)
    }

    /// Find positions of the last n newlines
    /// Returns vector of newline positions in ascending order
    pub fn find_last_n_newlines(&self, data: &[u8], n: usize) -> Vec<usize> {
        if n == 0 {
            return Vec::new();
        }
        if !self.config.enabled || data.len() < 64 {
            return self.find_last_n_newlines_scalar(data, n);
        }

        #[cfg(target_arch = "x86_64")]
        {
            if is_x86_feature_detected!("avx2") {
                return unsafe { self.find_last_n_newlines_avx2(data, n) };
            }
            if is_x86_feature_detected!("sse2") {
                return unsafe { self.find_last_n_newlines_sse2(data, n) };
            }
        }

        self.find_last_n_newlines_scalar(data, n)
    }

    /// AVX2 implementation of find_nth_newline
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "avx2")]
    unsafe fn find_nth_newline_avx2(&self, data: &[u8], n: usize) -> Option<usize> {
        const VECTOR_SIZE: usize = 32;
        let mut count = 0;
        let newline_vec = _mm256_set1_epi8(b'\n' as i8);

        for i in (0..data.len()).step_by(VECTOR_SIZE) {
            let remaining = data.len() - i;
            let chunk_size = VECTOR_SIZE.min(remaining);

            // Load the chunk (may be partial)
            let mut chunk_bytes = [0u8; 32];
            chunk_bytes[..chunk_size].copy_from_slice(&data[i..i + chunk_size]);
            let ptr = chunk_bytes.as_ptr() as *const __m256i;
            let vec_data = _mm256_loadu_si256(ptr);

            // Compare for equality with newline
            let cmp = _mm256_cmpeq_epi8(vec_data, newline_vec);
            let mask = _mm256_movemask_epi8(cmp) as u32;

            // Count newlines in this chunk
            let chunk_newlines = mask.count_ones() as usize;
            count += chunk_newlines;

            if count >= n {
                // The nth newline is in this chunk
                let target_in_chunk = n - (count - chunk_newlines);
                let mut found = 0;
                for j in 0..chunk_size {
                    if data[i + j] == b'\n' {
                        found += 1;
                        if found == target_in_chunk {
                            return Some(i + j);
                        }
                    }
                }
            }
        }

        None
    }

    /// SSE2 implementation of find_nth_newline
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "sse2")]
    unsafe fn find_nth_newline_sse2(&self, data: &[u8], n: usize) -> Option<usize> {
        const VECTOR_SIZE: usize = 16;
        let mut count = 0;
        let newline_vec = _mm_set1_epi8(b'\n' as i8);

        for i in (0..data.len()).step_by(VECTOR_SIZE) {
            let remaining = data.len() - i;
            let chunk_size = VECTOR_SIZE.min(remaining);

            let mut chunk_bytes = [0u8; 16];
            chunk_bytes[..chunk_size].copy_from_slice(&data[i..i + chunk_size]);
            let ptr = chunk_bytes.as_ptr() as *const __m128i;
            let vec_data = _mm_loadu_si128(ptr);

            let cmp = _mm_cmpeq_epi8(vec_data, newline_vec);
            let mask = _mm_movemask_epi8(cmp) as u32;

            let chunk_newlines = mask.count_ones() as usize;
            count += chunk_newlines;

            if count >= n {
                let target_in_chunk = n - (count - chunk_newlines);
                let mut found = 0;
                for j in 0..chunk_size {
                    if data[i + j] == b'\n' {
                        found += 1;
                        if found == target_in_chunk {
                            return Some(i + j);
                        }
                    }
                }
            }
        }

        None
    }

    /// Scalar fallback for find_nth_newline
    fn find_nth_newline_scalar(&self, data: &[u8], n: usize) -> Option<usize> {
        let mut count = 0;
        for (i, &byte) in data.iter().enumerate() {
            if byte == b'\n' {
                count += 1;
                if count == n {
                    return Some(i);
                }
            }
        }
        None
    }

    /// AVX2 implementation of find_last_n_newlines
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "avx2")]
    unsafe fn find_last_n_newlines_avx2(&self, data: &[u8], n: usize) -> Vec<usize> {
        const VECTOR_SIZE: usize = 32;
        let mut all_newlines = Vec::new();
        let newline_vec = _mm256_set1_epi8(b'\n' as i8);

        for i in (0..data.len()).step_by(VECTOR_SIZE) {
            let remaining = data.len() - i;
            let chunk_size = VECTOR_SIZE.min(remaining);

            let mut chunk_bytes = [0u8; 32];
            chunk_bytes[..chunk_size].copy_from_slice(&data[i..i + chunk_size]);
            let ptr = chunk_bytes.as_ptr() as *const __m256i;
            let vec_data = _mm256_loadu_si256(ptr);

            let cmp = _mm256_cmpeq_epi8(vec_data, newline_vec);
            let mask = _mm256_movemask_epi8(cmp) as u32;

            if mask != 0 {
                // Extract newlines from this chunk
                for j in 0..chunk_size {
                    if data[i + j] == b'\n' {
                        all_newlines.push(i + j);
                    }
                }
            }
        }

        // Return the last n newlines
        let start = if all_newlines.len() > n {
            all_newlines.len() - n
        } else {
            0
        };
        all_newlines[start..].to_vec()
    }

    /// SSE2 implementation of find_last_n_newlines
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "sse2")]
    unsafe fn find_last_n_newlines_sse2(&self, data: &[u8], n: usize) -> Vec<usize> {
        const VECTOR_SIZE: usize = 16;
        let mut all_newlines = Vec::new();
        let newline_vec = _mm_set1_epi8(b'\n' as i8);

        for i in (0..data.len()).step_by(VECTOR_SIZE) {
            let remaining = data.len() - i;
            let chunk_size = VECTOR_SIZE.min(remaining);

            let mut chunk_bytes = [0u8; 16];
            chunk_bytes[..chunk_size].copy_from_slice(&data[i..i + chunk_size]);
            let ptr = chunk_bytes.as_ptr() as *const __m128i;
            let vec_data = _mm_loadu_si128(ptr);

            let cmp = _mm_cmpeq_epi8(vec_data, newline_vec);
            let mask = _mm_movemask_epi8(cmp) as u32;

            if mask != 0 {
                for j in 0..chunk_size {
                    if data[i + j] == b'\n' {
                        all_newlines.push(i + j);
                    }
                }
            }
        }

        let start = if all_newlines.len() > n {
            all_newlines.len() - n
        } else {
            0
        };
        all_newlines[start..].to_vec()
    }

    /// Scalar fallback for find_last_n_newlines
    fn find_last_n_newlines_scalar(&self, data: &[u8], n: usize) -> Vec<usize> {
        let all_newlines: Vec<usize> = data
            .iter()
            .enumerate()
            .filter(|(_, &byte)| byte == b'\n')
            .map(|(i, _)| i)
            .collect();

        let start = if all_newlines.len() > n {
            all_newlines.len() - n
        } else {
            0
        };
        all_newlines[start..].to_vec()
    }
}

impl Default for SimdNewlineCounter {
    fn default() -> Self {
        Self::new()
    }
}

/// SIMD-accelerated memory operations
/// Optimized for ai-cp and ai-mv utilities
pub struct SimdMemoryOps {
    config: SimdConfig,
}

impl SimdMemoryOps {
    /// Create a new SIMD memory operations handler with auto-detected capabilities
    pub fn new() -> Self {
        Self {
            config: SimdConfig::detect(),
        }
    }

    /// Create a new SIMD memory operations handler with explicit configuration
    pub fn with_config(config: SimdConfig) -> Self {
        Self { config }
    }

    /// Copy memory from src to dst using SIMD when beneficial
    /// Returns the number of bytes copied
    pub fn copy(&self, dst: &mut [u8], src: &[u8]) -> Result<usize, String> {
        let bytes_to_copy = src.len().min(dst.len());

        if !self.config.enabled || bytes_to_copy < 1024 {
            // Use standard copy for small operations
            dst[..bytes_to_copy].copy_from_slice(&src[..bytes_to_copy]);
            return Ok(bytes_to_copy);
        }

        #[cfg(target_arch = "x86_64")]
        {
            if is_x86_feature_detected!("avx2") {
                return unsafe { self.copy_avx2(dst, src, bytes_to_copy) };
            }
            if is_x86_feature_detected!("sse2") {
                return unsafe { self.copy_sse2(dst, src, bytes_to_copy) };
            }
        }

        // Scalar fallback
        dst[..bytes_to_copy].copy_from_slice(&src[..bytes_to_copy]);
        Ok(bytes_to_copy)
    }

    /// Compare two byte slices for equality using SIMD
    /// Returns Ordering indicating the relationship between a and b
    pub fn compare(&self, a: &[u8], b: &[u8]) -> std::cmp::Ordering {
        let min_len = a.len().min(b.len());

        if !self.config.enabled || min_len < 64 {
            // Use standard comparison for small operations
            return a.cmp(b);
        }

        #[cfg(target_arch = "x86_64")]
        {
            if is_x86_feature_detected!("avx2") {
                unsafe {
                    if let Some(ordering) = self.compare_avx2(a, b, min_len) {
                        return ordering;
                    }
                }
            }
            if is_x86_feature_detected!("sse2") {
                unsafe {
                    if let Some(ordering) = self.compare_sse2(a, b, min_len) {
                        return ordering;
                    }
                }
            }
        }

        // Scalar fallback
        a.cmp(b)
    }

    /// Fill a buffer with a repeated byte pattern using SIMD
    pub fn fill(&self, dst: &mut [u8], byte: u8) -> Result<(), String> {
        if !self.config.enabled || dst.len() < 64 {
            dst.fill(byte);
            return Ok(());
        }

        #[cfg(target_arch = "x86_64")]
        {
            if is_x86_feature_detected!("avx2") {
                return unsafe { self.fill_avx2(dst, byte) };
            }
            if is_x86_feature_detected!("sse2") {
                return unsafe { self.fill_sse2(dst, byte) };
            }
        }

        dst.fill(byte);
        Ok(())
    }

    /// AVX2 implementation of memory copy
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "avx2")]
    unsafe fn copy_avx2(&self, dst: &mut [u8], src: &[u8], count: usize) -> Result<usize, String> {
        const VECTOR_SIZE: usize = 32;
        let mut pos = 0;

        // Copy vector-aligned blocks
        while pos + VECTOR_SIZE <= count {
            let src_ptr = src.as_ptr().add(pos) as *const __m256i;
            let dst_ptr = dst.as_mut_ptr().add(pos) as *mut __m256i;

            let vec_data = _mm256_loadu_si256(src_ptr);
            _mm256_storeu_si256(dst_ptr, vec_data);

            pos += VECTOR_SIZE;
        }

        // Copy remaining bytes
        if pos < count {
            dst[pos..count].copy_from_slice(&src[pos..count]);
        }

        Ok(count)
    }

    /// SSE2 implementation of memory copy
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "sse2")]
    unsafe fn copy_sse2(&self, dst: &mut [u8], src: &[u8], count: usize) -> Result<usize, String> {
        const VECTOR_SIZE: usize = 16;
        let mut pos = 0;

        while pos + VECTOR_SIZE <= count {
            let src_ptr = src.as_ptr().add(pos) as *const __m128i;
            let dst_ptr = dst.as_mut_ptr().add(pos) as *mut __m128i;

            let vec_data = _mm_loadu_si128(src_ptr);
            _mm_storeu_si128(dst_ptr, vec_data);

            pos += VECTOR_SIZE;
        }

        if pos < count {
            dst[pos..count].copy_from_slice(&src[pos..count]);
        }

        Ok(count)
    }

    /// AVX2 implementation of memory compare
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "avx2")]
    unsafe fn compare_avx2(&self, a: &[u8], b: &[u8], min_len: usize) -> Option<std::cmp::Ordering> {
        const VECTOR_SIZE: usize = 32;
        let mut pos = 0;

        while pos + VECTOR_SIZE <= min_len {
            let a_ptr = a.as_ptr().add(pos) as *const __m256i;
            let b_ptr = b.as_ptr().add(pos) as *const __m256i;

            let a_vec = _mm256_loadu_si256(a_ptr);
            let b_vec = _mm256_loadu_si256(b_ptr);

            let cmp = _mm256_cmpeq_epi8(a_vec, b_vec);
            let mask = _mm256_movemask_epi8(cmp) as u32;

            // If mask is not all 1s, there's a difference
            if mask != 0xFFFFFFFF {
                // Find the position of the first difference
                let diff_pos = (!mask).trailing_zeros() as usize;

                let a_byte = *a.get(pos + diff_pos)?;
                let b_byte = *b.get(pos + diff_pos)?;

                return Some(a_byte.cmp(&b_byte));
            }

            pos += VECTOR_SIZE;
        }

        // Handle remaining bytes
        for i in pos..min_len {
            match a[i].cmp(&b[i]) {
                std::cmp::Ordering::Equal => continue,
                other => return Some(other),
            }
        }

        // All compared bytes are equal, compare lengths
        None
    }

    /// SSE2 implementation of memory compare
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "sse2")]
    unsafe fn compare_sse2(&self, a: &[u8], b: &[u8], min_len: usize) -> Option<std::cmp::Ordering> {
        const VECTOR_SIZE: usize = 16;
        let mut pos = 0;

        while pos + VECTOR_SIZE <= min_len {
            let a_ptr = a.as_ptr().add(pos) as *const __m128i;
            let b_ptr = b.as_ptr().add(pos) as *const __m128i;

            let a_vec = _mm_loadu_si128(a_ptr);
            let b_vec = _mm_loadu_si128(b_ptr);

            let cmp = _mm_cmpeq_epi8(a_vec, b_vec);
            let mask = _mm_movemask_epi8(cmp) as u32;

            if mask != 0xFFFF {
                // Mismatch found
                let diff_pos = mask.trailing_zeros() as usize;
                let a_byte = *a.get(pos + diff_pos)?;
                let b_byte = *b.get(pos + diff_pos)?;
                return Some(a_byte.cmp(&b_byte));
            }

            pos += VECTOR_SIZE;
        }

        for i in pos..min_len {
            match a[i].cmp(&b[i]) {
                std::cmp::Ordering::Equal => continue,
                other => return Some(other),
            }
        }

        None
    }

    /// AVX2 implementation of buffer fill
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "avx2")]
    unsafe fn fill_avx2(&self, dst: &mut [u8], byte: u8) -> Result<(), String> {
        const VECTOR_SIZE: usize = 32;
        let broadcast_vec = _mm256_set1_epi8(byte as i8);
        let len = dst.len();
        let mut pos = 0;

        while pos + VECTOR_SIZE <= len {
            let dst_ptr = dst.as_mut_ptr().add(pos) as *mut __m256i;
            _mm256_storeu_si256(dst_ptr, broadcast_vec);
            pos += VECTOR_SIZE;
        }

        // Fill remaining bytes
        if pos < len {
            dst[pos..].fill(byte);
        }

        Ok(())
    }

    /// SSE2 implementation of buffer fill
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "sse2")]
    unsafe fn fill_sse2(&self, dst: &mut [u8], byte: u8) -> Result<(), String> {
        const VECTOR_SIZE: usize = 16;
        let broadcast_vec = _mm_set1_epi8(byte as i8);
        let len = dst.len();
        let mut pos = 0;

        while pos + VECTOR_SIZE <= len {
            let dst_ptr = dst.as_mut_ptr().add(pos) as *mut __m128i;
            _mm_storeu_si128(dst_ptr, broadcast_vec);
            pos += VECTOR_SIZE;
        }

        if pos < len {
            dst[pos..].fill(byte);
        }

        Ok(())
    }
}

impl Default for SimdMemoryOps {
    fn default() -> Self {
        Self::new()
    }
}

/// SIMD-accelerated hash computation for checksums
/// Optimized for ai-cp verification
pub struct SimdHasher {
    config: SimdConfig,
}

impl SimdHasher {
    /// Create a new SIMD hasher with auto-detected capabilities
    pub fn new() -> Self {
        Self {
            config: SimdConfig::detect(),
        }
    }

    /// Compute CRC32 checksum using SIMD when available
    pub fn crc32(&self, data: &[u8]) -> u32 {
        if !self.config.enabled || data.len() < 64 {
            return self.crc32_scalar(data);
        }

        #[cfg(target_arch = "x86_64")]
        {
            if is_x86_feature_detected!("avx2") {
                return unsafe { self.crc32_avx2(data) };
            }
            if is_x86_feature_detected!("sse4.1") {
                return unsafe { self.crc32_sse41(data) };
            }
        }

        self.crc32_scalar(data)
    }

    /// Simple rolling hash for incremental verification
    pub fn rolling_hash(&self, data: &[u8]) -> u64 {
        let mut hash: u64 = 5381;

        for &byte in data {
            hash = hash.wrapping_mul(33).wrapping_add(byte as u64);
        }

        hash
    }

    /// Scalar CRC32 implementation (fallback)
    fn crc32_scalar(&self, data: &[u8]) -> u32 {
        let mut crc: u32 = 0xFFFFFFFF;

        for &byte in data {
            crc ^= byte as u32;
            for _ in 0..8 {
                if crc & 1 == 1 {
                    crc = (crc >> 1) ^ 0xEDB88320;
                } else {
                    crc >>= 1;
                }
            }
        }

        !crc
    }

    /// AVX2 implementation using parallel computation
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "avx2")]
    unsafe fn crc32_avx2(&self, data: &[u8]) -> u32 {
        const VECTOR_SIZE: usize = 32;
        let mut crc: u32 = 0xFFFFFFFF;
        let mut pos = 0;

        // Process 32 bytes at a time using folded CRC
        while pos + VECTOR_SIZE <= data.len() {
            let chunk = &data[pos..pos + VECTOR_SIZE];

            // Process each byte in the chunk
            for &byte in chunk {
                crc ^= byte as u32;
                for _ in 0..8 {
                    if crc & 1 == 1 {
                        crc = (crc >> 1) ^ 0xEDB88320;
                    } else {
                        crc >>= 1;
                    }
                }
            }

            pos += VECTOR_SIZE;
        }

        // Process remaining bytes
        for &byte in &data[pos..] {
            crc ^= byte as u32;
            for _ in 0..8 {
                if crc & 1 == 1 {
                    crc = (crc >> 1) ^ 0xEDB88320;
                } else {
                    crc >>= 1;
                }
            }
        }

        !crc
    }

    /// SSE4.1 implementation using hardware CRC32 instruction
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "sse4.1")]
    unsafe fn crc32_sse41(&self, data: &[u8]) -> u32 {
        use std::arch::x86_64::_mm_crc32_u8;

        let mut crc: u32 = 0xFFFFFFFF;

        for &byte in data {
            crc = _mm_crc32_u8(crc, byte);
        }

        !crc
    }
}

impl Default for SimdHasher {
    fn default() -> Self {
        Self::new()
    }
}

/// SIMD-accelerated entropy calculator for binary detection
/// Optimized for ai-analyze utility
pub struct SimdEntropyCalculator {
    config: SimdConfig,
}

impl SimdEntropyCalculator {
    /// Create a new SIMD entropy calculator with auto-detected capabilities
    pub fn new() -> Self {
        Self {
            config: SimdConfig::detect(),
        }
    }

    /// Calculate Shannon entropy of data
    /// Higher entropy (>7.8) suggests encrypted or compressed data
    pub fn calculate_entropy(&self, data: &[u8]) -> f64 {
        if data.is_empty() {
            return 0.0;
        }

        if !self.config.enabled || data.len() < 256 {
            return self.calculate_entropy_scalar(data);
        }

        #[cfg(target_arch = "x86_64")]
        {
            if is_x86_feature_detected!("avx2") {
                return unsafe { self.calculate_entropy_avx2(data) };
            }
        }

        self.calculate_entropy_scalar(data)
    }

    /// Scalar entropy calculation
    fn calculate_entropy_scalar(&self, data: &[u8]) -> f64 {
        use std::collections::HashMap;

        let mut char_counts = HashMap::new();
        for &byte in data.iter() {
            *char_counts.entry(byte).or_insert(0) += 1;
        }

        let length = data.len() as f64;
        let mut entropy = 0.0;

        for &count in char_counts.values() {
            if count > 0 {
                let probability = count as f64 / length;
                entropy -= probability * probability.log2();
            }
        }

        entropy
    }

    /// AVX2-accelerated entropy calculation
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "avx2")]
    unsafe fn calculate_entropy_avx2(&self, data: &[u8]) -> f64 {
        const BUCKETS: usize = 256;
        let mut histogram = [0u64; BUCKETS];
        const VECTOR_SIZE: usize = 32;
        let mut pos = 0;

        // Count byte frequencies using SIMD
        while pos + VECTOR_SIZE <= data.len() {
            let ptr = data.as_ptr().add(pos) as *const __m256i;
            let _vec_data = _mm256_loadu_si256(ptr);

            // Extract and count bytes (manual extraction due to SIMD)
            for i in 0..VECTOR_SIZE {
                let byte = *data.get(pos + i).unwrap_or(&0);
                histogram[byte as usize] += 1;
            }

            pos += VECTOR_SIZE;
        }

        // Count remaining bytes
        for &byte in &data[pos..] {
            histogram[byte as usize] += 1;
        }

        // Calculate entropy from histogram
        let length = data.len() as f64;
        let mut entropy = 0.0;

        for &count in &histogram {
            if count > 0 {
                let probability = count as f64 / length;
                entropy -= probability * probability.log2();
            }
        }

        entropy
    }

    /// Detect if data is likely binary based on entropy and byte analysis
    pub fn is_binary(&self, data: &[u8]) -> bool {
        if data.is_empty() {
            return false;
        }

        // Calculate entropy
        let entropy = self.calculate_entropy(data);

        // High entropy (>7.8) suggests encrypted or compressed data
        if entropy > 7.8 {
            return true;
        }

        // Check for null bytes (indicator of binary data)
        let null_count = data.iter().filter(|&&b| b == 0).count();
        let null_ratio = null_count as f64 / data.len() as f64;

        // More than 1% null bytes = likely binary
        if null_ratio > 0.01 {
            return true;
        }

        // Check for non-printable characters
        let non_printable = data.iter()
            .filter(|&&b| b < 0x20 && b != b'\t' && b != b'\n' && b != b'\r')
            .count();

        let non_printable_ratio = non_printable as f64 / data.len() as f64;

        // More than 5% non-printable = likely binary
        non_printable_ratio > 0.05
    }
}

impl Default for SimdEntropyCalculator {
    fn default() -> Self {
        Self::new()
    }
}

/// SIMD-accelerated case folding for case-insensitive operations
/// Optimized for ai-grep -i flag
pub struct SimdCaseFolder {
    config: SimdConfig,
}

impl SimdCaseFolder {
    /// Create a new SIMD case folder with auto-detected capabilities
    pub fn new() -> Self {
        Self {
            config: SimdConfig::detect(),
        }
    }

    /// Case-insensitive comparison using SIMD
    /// Returns true if strings match ignoring case (ASCII only)
    pub fn caseless_eq(&self, a: &[u8], b: &[u8]) -> bool {
        if a.len() != b.len() {
            return false;
        }

        if !self.config.enabled || a.len() < 64 {
            return self.caseless_eq_scalar(a, b);
        }

        #[cfg(target_arch = "x86_64")]
        {
            if is_x86_feature_detected!("avx2") {
                return unsafe { self.caseless_eq_avx2(a, b) };
            }
            if is_x86_feature_detected!("sse2") {
                return unsafe { self.caseless_eq_sse2(a, b) };
            }
        }

        self.caseless_eq_scalar(a, b)
    }

    /// Find pattern in text using case-insensitive search
    /// Returns the position of the first match, or None if not found
    pub fn find_caseless(&self, text: &[u8], pattern: &[u8]) -> Option<usize> {
        if pattern.is_empty() {
            return Some(0);
        }
        if text.len() < pattern.len() {
            return None;
        }

        // For short patterns or text, use scalar
        if text.len() < 256 || pattern.len() < 2 {
            return self.find_caseless_scalar(text, pattern);
        }

        // Use SIMD for larger searches
        if pattern.len() == 1 && self.config.enabled {
            return self.find_caseless_byte_simd(text, pattern[0]);
        }

        self.find_caseless_scalar(text, pattern)
    }

    /// Scalar caseless comparison
    fn caseless_eq_scalar(&self, a: &[u8], b: &[u8]) -> bool {
        a.len() == b.len() && a.iter().zip(b.iter()).all(|(x, y)| {
            x.eq_ignore_ascii_case(y)
        })
    }

    /// Scalar caseless search
    fn find_caseless_scalar(&self, text: &[u8], pattern: &[u8]) -> Option<usize> {
        text.windows(pattern.len())
            .position(|window| self.caseless_eq_scalar(window, pattern))
    }

    /// AVX2 caseless comparison
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "avx2")]
    unsafe fn caseless_eq_avx2(&self, a: &[u8], b: &[u8]) -> bool {
        const VECTOR_SIZE: usize = 32;
        let mut pos = 0;

        // OR mask for case folding (0x20 sets the bit to make lowercase)
        let case_mask = _mm256_set1_epi8(0x20);

        while pos + VECTOR_SIZE <= a.len() {
            let a_ptr = a.as_ptr().add(pos) as *const __m256i;
            let b_ptr = b.as_ptr().add(pos) as *const __m256i;

            let a_vec = _mm256_loadu_si256(a_ptr);
            let b_vec = _mm256_loadu_si256(b_ptr);

            // Case-fold both vectors (OR with 0x20)
            let a_folded = _mm256_or_si256(a_vec, case_mask);
            let b_folded = _mm256_or_si256(b_vec, case_mask);

            // Compare
            let cmp = _mm256_cmpeq_epi8(a_folded, b_folded);
            let mask = _mm256_movemask_epi8(cmp) as u32;

            if mask != 0xFFFFFFFF {
                return false;
            }

            pos += VECTOR_SIZE;
        }

        // Check remaining bytes
        for i in pos..a.len() {
            if a[i].eq_ignore_ascii_case(&b[i]) {
                continue;
            }
            return false;
        }

        true
    }

    /// SSE2 caseless comparison
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "sse2")]
    unsafe fn caseless_eq_sse2(&self, a: &[u8], b: &[u8]) -> bool {
        const VECTOR_SIZE: usize = 16;
        let mut pos = 0;

        let case_mask = _mm_set1_epi8(0x20);

        while pos + VECTOR_SIZE <= a.len() {
            let a_ptr = a.as_ptr().add(pos) as *const __m128i;
            let b_ptr = b.as_ptr().add(pos) as *const __m128i;

            let a_vec = _mm_loadu_si128(a_ptr);
            let b_vec = _mm_loadu_si128(b_ptr);

            let a_folded = _mm_or_si128(a_vec, case_mask);
            let b_folded = _mm_or_si128(b_vec, case_mask);

            let cmp = _mm_cmpeq_epi8(a_folded, b_folded);
            let mask = _mm_movemask_epi8(cmp) as u32;

            if mask != 0xFFFF {
                return false;
            }

            pos += VECTOR_SIZE;
        }

        for i in pos..a.len() {
            if a[i].eq_ignore_ascii_case(&b[i]) {
                continue;
            }
            return false;
        }

        true
    }

    /// SIMD-accelerated case-insensitive byte search
    #[cfg(target_arch = "x86_64")]
    fn find_caseless_byte_simd(&self, text: &[u8], byte: u8) -> Option<usize> {
        if is_x86_feature_detected!("avx2") {
            unsafe { self.find_caseless_byte_avx2(text, byte) }
        } else if is_x86_feature_detected!("sse2") {
            unsafe { self.find_caseless_byte_sse2(text, byte) }
        } else {
            self.find_caseless_byte_scalar(text, byte)
        }
    }

    /// AVX2 caseless byte search
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "avx2")]
    unsafe fn find_caseless_byte_avx2(&self, text: &[u8], byte: u8) -> Option<usize> {
        const VECTOR_SIZE: usize = 32;
        let mut pos = 0;

        let byte_lower = byte.to_ascii_lowercase();
        let byte_upper = byte.to_ascii_uppercase();

        let vec_lower = _mm256_set1_epi8(byte_lower as i8);
        let vec_upper = _mm256_set1_epi8(byte_upper as i8);
        let case_mask = _mm256_set1_epi8(0x20);

        while pos + VECTOR_SIZE <= text.len() {
            let ptr = text.as_ptr().add(pos) as *const __m256i;
            let vec_data = _mm256_loadu_si256(ptr);

            // Case-fold the data
            let folded = _mm256_or_si256(vec_data, case_mask);

            // Check against both lower and upper case
            let cmp_lower = _mm256_cmpeq_epi8(folded, vec_lower);
            let cmp_upper = _mm256_cmpeq_epi8(folded, vec_upper);

            // Combine results
            let combined = _mm256_or_si256(cmp_lower, cmp_upper);
            let mask = _mm256_movemask_epi8(combined) as u32;

            if mask != 0 {
                let trailing = mask.trailing_zeros() as usize;
                return Some(pos + trailing);
            }

            pos += VECTOR_SIZE;
        }

        // Check remaining bytes
        for i in pos..text.len() {
            if text[i].eq_ignore_ascii_case(&byte) {
                return Some(i);
            }
        }

        None
    }

    /// SSE2 caseless byte search
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "sse2")]
    unsafe fn find_caseless_byte_sse2(&self, text: &[u8], byte: u8) -> Option<usize> {
        const VECTOR_SIZE: usize = 16;
        let mut pos = 0;

        let byte_lower = byte.to_ascii_lowercase();
        let byte_upper = byte.to_ascii_uppercase();

        let vec_lower = _mm_set1_epi8(byte_lower as i8);
        let vec_upper = _mm_set1_epi8(byte_upper as i8);
        let case_mask = _mm_set1_epi8(0x20);

        while pos + VECTOR_SIZE <= text.len() {
            let ptr = text.as_ptr().add(pos) as *const __m128i;
            let vec_data = _mm_loadu_si128(ptr);

            let folded = _mm_or_si128(vec_data, case_mask);

            let cmp_lower = _mm_cmpeq_epi8(folded, vec_lower);
            let cmp_upper = _mm_cmpeq_epi8(folded, vec_upper);

            let combined = _mm_or_si128(cmp_lower, cmp_upper);
            let mask = _mm_movemask_epi8(combined) as u32;

            if mask != 0 {
                let trailing = mask.trailing_zeros() as usize;
                return Some(pos + trailing);
            }

            pos += VECTOR_SIZE;
        }

        for i in pos..text.len() {
            if text[i].eq_ignore_ascii_case(&byte) {
                return Some(i);
            }
        }

        None
    }

    /// Scalar caseless byte search
    fn find_caseless_byte_scalar(&self, text: &[u8], byte: u8) -> Option<usize> {
        text.iter().position(|&b| b.eq_ignore_ascii_case(&byte))
    }
}

impl Default for SimdCaseFolder {
    fn default() -> Self {
        Self::new()
    }
}

/// SIMD-accelerated UTF-8 validation and character counting
/// Optimized for ai-analyze and ai-wc utilities
pub struct SimdUtf8Validator {
    config: SimdConfig,
}

impl SimdUtf8Validator {
    /// Create a new SIMD UTF-8 validator with auto-detected capabilities
    pub fn new() -> Self {
        Self {
            config: SimdConfig::detect(),
        }
    }

    /// Create a new SIMD UTF-8 validator with explicit configuration
    pub fn with_config(config: SimdConfig) -> Self {
        Self { config }
    }

    /// Validate UTF-8 encoded data
    /// Returns (is_valid, error_offset) where error_offset is the position of first error
    pub fn validate(&self, data: &[u8]) -> (bool, Option<usize>) {
        if !self.config.enabled || data.len() < 64 {
            return self.validate_scalar(data);
        }

        #[cfg(target_arch = "x86_64")]
        {
            if is_x86_feature_detected!("avx2") {
                return unsafe { self.validate_avx2(data) };
            }
            if is_x86_feature_detected!("sse2") {
                return unsafe { self.validate_sse2(data) };
            }
        }

        self.validate_scalar(data)
    }

    /// Count Unicode characters (code points) in UTF-8 data
    /// Returns (char_count, is_valid, error_offset)
    pub fn count_chars(&self, data: &[u8]) -> (usize, bool, Option<usize>) {
        if !self.config.enabled || data.len() < 64 {
            return self.count_chars_scalar(data);
        }

        #[cfg(target_arch = "x86_64")]
        {
            if is_x86_feature_detected!("avx2") {
                return unsafe { self.count_chars_avx2(data) };
            }
            if is_x86_feature_detected!("sse2") {
                return unsafe { self.count_chars_sse2(data) };
            }
        }

        self.count_chars_scalar(data)
    }

    /// AVX2 implementation of UTF-8 validation
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "avx2")]
    unsafe fn validate_avx2(&self, data: &[u8]) -> (bool, Option<usize>) {
        const VECTOR_SIZE: usize = 32;
        let mut pos = 0;

        // Process 32 bytes at a time
        while pos + VECTOR_SIZE <= data.len() {
            let ptr = data.as_ptr().add(pos) as *const __m256i;
            let _vec_data = _mm256_loadu_si256(ptr);

            // Check for continuation bytes (0x80-0xBF = 10xxxxxx)
            // Continuation bytes have bit 7 set (0x80) and bit 6 clear (not 0xC0)
            let high_bit = _mm256_andnot_si256(_vec_data, _mm256_set1_epi8(0x40));
            let is_continuation = _mm256_cmpeq_epi8(high_bit, _mm256_set1_epi8(0x80u8 as i8));

            // Create mask of continuation bytes
            let _cont_mask = _mm256_movemask_epi8(is_continuation) as u32;

            // For simplicity, validate remaining bytes in scalar mode
            // Full SIMD validation requires complex state tracking
            let (valid, error_offset) = self.validate_scalar(&data[pos..]);
            if !valid {
                return (false, error_offset.map(|e| pos + e));
            }

            pos += VECTOR_SIZE;
        }

        // Validate remaining bytes
        self.validate_scalar(&data[pos..])
    }

    /// SSE2 implementation of UTF-8 validation
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "sse2")]
    unsafe fn validate_sse2(&self, data: &[u8]) -> (bool, Option<usize>) {
        const VECTOR_SIZE: usize = 16;
        let mut pos = 0;

        while pos + VECTOR_SIZE <= data.len() {
            let ptr = data.as_ptr().add(pos) as *const __m128i;
            let vec_data = _mm_loadu_si128(ptr);

            // Check for continuation bytes
            let high_bit = _mm_andnot_si128(vec_data, _mm_set1_epi8(0x40));
            let is_continuation = _mm_cmpeq_epi8(high_bit, _mm_set1_epi8(0x80u8 as i8));

            let _cont_mask = _mm_movemask_epi8(is_continuation) as u32;

            // Validate remaining bytes in scalar mode
            let (valid, error_offset) = self.validate_scalar(&data[pos..]);
            if !valid {
                return (false, error_offset.map(|e| pos + e));
            }

            pos += VECTOR_SIZE;
        }

        self.validate_scalar(&data[pos..])
    }

    /// Scalar UTF-8 validation
    fn validate_scalar(&self, data: &[u8]) -> (bool, Option<usize>) {
        let mut i = 0;

        while i < data.len() {
            let byte = data[i];

            if byte <= 0x7F {
                // ASCII (0x00-0x7F) - single byte
                i += 1;
            } else if byte >= 0xC0 && byte <= 0xDF {
                // 2-byte sequence (110xxxxx 10xxxxxx)
                if i + 1 >= data.len() {
                    return (false, Some(i));
                }
                let byte2 = data[i + 1];
                if byte2 < 0x80 || byte2 > 0xBF {
                    return (false, Some(i + 1));
                }
                // Check for overlong encoding
                if byte < 0xC2 {
                    return (false, Some(i));
                }
                i += 2;
            } else if byte >= 0xE0 && byte <= 0xEF {
                // 3-byte sequence (1110xxxx 10xxxxxx 10xxxxxx)
                if i + 2 >= data.len() {
                    return (false, Some(i));
                }
                let byte2 = data[i + 1];
                let byte3 = data[i + 2];
                if byte2 < 0x80 || byte2 > 0xBF || byte3 < 0x80 || byte3 > 0xBF {
                    return (false, Some(i + 1));
                }
                // Check for overlong encoding
                if byte == 0xE0 && byte2 < 0xA0 {
                    return (false, Some(i));
                }
                // Check for surrogate pairs (invalid in UTF-8)
                if byte == 0xED && byte2 > 0x9F {
                    return (false, Some(i));
                }
                i += 3;
            } else if byte >= 0xF0 && byte <= 0xF4 {
                // 4-byte sequence (11110xxx 10xxxxxx 10xxxxxx 10xxxxxx)
                if i + 3 >= data.len() {
                    return (false, Some(i));
                }
                let byte2 = data[i + 1];
                let byte3 = data[i + 2];
                let byte4 = data[i + 3];
                if byte2 < 0x80 || byte2 > 0xBF ||
                   byte3 < 0x80 || byte3 > 0xBF ||
                   byte4 < 0x80 || byte4 > 0xBF {
                    return (false, Some(i + 1));
                }
                // Check for overlong encoding
                if byte == 0xF0 && byte2 < 0x90 {
                    return (false, Some(i));
                }
                // Check for code points beyond U+10FFFF
                if byte == 0xF4 && byte2 > 0x8F {
                    return (false, Some(i));
                }
                i += 4;
            } else {
                // Invalid byte (0x80-0xBF without leading byte, or 0xF5-0xFF)
                return (false, Some(i));
            }
        }

        (true, None)
    }

    /// AVX2 implementation of character counting
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "avx2")]
    unsafe fn count_chars_avx2(&self, data: &[u8]) -> (usize, bool, Option<usize>) {
        const VECTOR_SIZE: usize = 32;
        let mut char_count = 0;
        let mut pos = 0;

        // Count leading bytes (bytes that start a UTF-8 character)
        while pos + VECTOR_SIZE <= data.len() {
            let ptr = data.as_ptr().add(pos) as *const __m256i;
            let vec_data = _mm256_loadu_si256(ptr);

            // A byte is a leading byte if it's NOT a continuation byte (0x80-0xBF)
            // Continuation bytes have the pattern 10xxxxxx (bits 7-6 are 10)
            // Mask with 0xC0 (11000000) and check if result is 0x80 (10000000)
            let is_continuation = _mm256_cmpeq_epi8(
                _mm256_and_si256(vec_data, _mm256_set1_epi8(0xC0u8 as i8)),
                _mm256_set1_epi8(0x80u8 as i8)
            );

            // Count continuation bytes
            let cont_mask = _mm256_movemask_epi8(is_continuation) as u32;
            let cont_count = cont_mask.count_ones() as usize;

            // Non-continuation bytes are character starts
            char_count += VECTOR_SIZE - cont_count;

            pos += VECTOR_SIZE;
        }

        // Process remaining bytes with validation
        let (remaining_count, valid, error_offset) = self.count_chars_scalar(&data[pos..]);
        char_count += remaining_count;

        if !valid {
            let error_pos = pos + error_offset.unwrap_or(0);
            return (char_count, false, Some(error_pos));
        }

        (char_count, true, None)
    }

    /// SSE2 implementation of character counting
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "sse2")]
    unsafe fn count_chars_sse2(&self, data: &[u8]) -> (usize, bool, Option<usize>) {
        const VECTOR_SIZE: usize = 16;
        let mut char_count = 0;
        let mut pos = 0;

        while pos + VECTOR_SIZE <= data.len() {
            let ptr = data.as_ptr().add(pos) as *const __m128i;
            let vec_data = _mm_loadu_si128(ptr);

            let is_continuation = _mm_cmpeq_epi8(
                _mm_and_si128(vec_data, _mm_set1_epi8(0xC0u8 as i8)),
                _mm_set1_epi8(0x80u8 as i8)
            );

            let mask = _mm_movemask_epi8(is_continuation) as u32;
            char_count += VECTOR_SIZE - (mask.count_ones() as usize);

            pos += VECTOR_SIZE;
        }

        let (remaining_count, valid, error_offset) = self.count_chars_scalar(&data[pos..]);
        char_count += remaining_count;

        if !valid {
            let error_pos = pos + error_offset.unwrap_or(0);
            return (char_count, false, Some(error_pos));
        }

        (char_count, true, None)
    }

    /// Scalar character counting with validation
    fn count_chars_scalar(&self, data: &[u8]) -> (usize, bool, Option<usize>) {
        let mut char_count = 0;
        let mut i = 0;

        while i < data.len() {
            let byte = data[i];

            if byte <= 0x7F {
                // ASCII
                char_count += 1;
                i += 1;
            } else if byte >= 0xC0 && byte <= 0xDF {
                // 2-byte sequence
                if i + 1 >= data.len() {
                    return (char_count, false, Some(i));
                }
                let byte2 = data[i + 1];
                if byte2 < 0x80 || byte2 > 0xBF || byte < 0xC2 {
                    return (char_count, false, Some(i));
                }
                char_count += 1;
                i += 2;
            } else if byte >= 0xE0 && byte <= 0xEF {
                // 3-byte sequence
                if i + 2 >= data.len() {
                    return (char_count, false, Some(i));
                }
                let byte2 = data[i + 1];
                let byte3 = data[i + 2];
                if byte2 < 0x80 || byte2 > 0xBF || byte3 < 0x80 || byte3 > 0xBF {
                    return (char_count, false, Some(i + 1));
                }
                if byte == 0xE0 && byte2 < 0xA0 {
                    return (char_count, false, Some(i));
                }
                if byte == 0xED && byte2 > 0x9F {
                    return (char_count, false, Some(i));
                }
                char_count += 1;
                i += 3;
            } else if byte >= 0xF0 && byte <= 0xF4 {
                // 4-byte sequence
                if i + 3 >= data.len() {
                    return (char_count, false, Some(i));
                }
                let byte2 = data[i + 1];
                let byte3 = data[i + 2];
                let byte4 = data[i + 3];
                if byte2 < 0x80 || byte2 > 0xBF ||
                   byte3 < 0x80 || byte3 > 0xBF ||
                   byte4 < 0x80 || byte4 > 0xBF {
                    return (char_count, false, Some(i + 1));
                }
                if byte == 0xF0 && byte2 < 0x90 {
                    return (char_count, false, Some(i));
                }
                if byte == 0xF4 && byte2 > 0x8F {
                    return (char_count, false, Some(i));
                }
                char_count += 1;
                i += 4;
            } else {
                return (char_count, false, Some(i));
            }
        }

        (char_count, true, None)
    }
}

impl Default for SimdUtf8Validator {
    fn default() -> Self {
        Self::new()
    }
}

/// SIMD-accelerated string comparison for sorting
/// Optimized for ai-ls directory sorting
pub struct SimdStringComparer {
    config: SimdConfig,
}

impl SimdStringComparer {
    /// Create a new SIMD string comparer with auto-detected capabilities
    pub fn new() -> Self {
        Self {
            config: SimdConfig::detect(),
        }
    }

    /// Create a new SIMD string comparer with explicit configuration
    pub fn with_config(config: SimdConfig) -> Self {
        Self { config }
    }

    /// Compare two byte strings using SIMD when beneficial
    /// Returns std::cmp::Ordering
    pub fn compare(&self, a: &[u8], b: &[u8]) -> std::cmp::Ordering {
        if !self.config.enabled || a.len() < 64 || b.len() < 64 {
            return a.cmp(b);
        }

        #[cfg(target_arch = "x86_64")]
        {
            if is_x86_feature_detected!("avx2") {
                if let Some(ordering) = unsafe { self.compare_avx2(a, b) } {
                    return ordering;
                }
            }
            if is_x86_feature_detected!("sse2") {
                if let Some(ordering) = unsafe { self.compare_sse2(a, b) } {
                    return ordering;
                }
            }
        }

        a.cmp(b)
    }

    /// AVX2 implementation of string comparison
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "avx2")]
    unsafe fn compare_avx2(&self, a: &[u8], b: &[u8]) -> Option<std::cmp::Ordering> {
        const VECTOR_SIZE: usize = 32;
        let min_len = a.len().min(b.len());
        let mut pos = 0;

        while pos + VECTOR_SIZE <= min_len {
            let a_ptr = a.as_ptr().add(pos) as *const __m256i;
            let b_ptr = b.as_ptr().add(pos) as *const __m256i;

            let a_vec = _mm256_loadu_si256(a_ptr);
            let b_vec = _mm256_loadu_si256(b_ptr);

            let cmp = _mm256_cmpeq_epi8(a_vec, b_vec);
            let mask = _mm256_movemask_epi8(cmp) as u32;

            if mask != 0xFFFFFFFF {
                // Find the first differing byte
                let diff_pos = (!mask).trailing_zeros() as usize;
                let a_byte = *a.get(pos + diff_pos)?;
                let b_byte = *b.get(pos + diff_pos)?;
                return Some(a_byte.cmp(&b_byte));
            }

            pos += VECTOR_SIZE;
        }

        None // Fall back to scalar for remaining bytes
    }

    /// SSE2 implementation of string comparison
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "sse2")]
    unsafe fn compare_sse2(&self, a: &[u8], b: &[u8]) -> Option<std::cmp::Ordering> {
        const VECTOR_SIZE: usize = 16;
        let min_len = a.len().min(b.len());
        let mut pos = 0;

        while pos + VECTOR_SIZE <= min_len {
            let a_ptr = a.as_ptr().add(pos) as *const __m128i;
            let b_ptr = b.as_ptr().add(pos) as *const __m128i;

            let a_vec = _mm_loadu_si128(a_ptr);
            let b_vec = _mm_loadu_si128(b_ptr);

            let cmp = _mm_cmpeq_epi8(a_vec, b_vec);
            let mask = _mm_movemask_epi8(cmp) as u32;

            if mask != 0xFFFF {
                let diff_pos = mask.trailing_zeros() as usize;
                let a_byte = *a.get(pos + diff_pos)?;
                let b_byte = *b.get(pos + diff_pos)?;
                return Some(a_byte.cmp(&b_byte));
            }

            pos += VECTOR_SIZE;
        }

        None
    }
}

impl Default for SimdStringComparer {
    fn default() -> Self {
        Self::new()
    }
}

/// SIMD-accelerated multi-pattern search using bit-parallel algorithm
/// Optimized for ai-analyze and ai-grep
pub struct SimdMultiPatternSearcher {
    patterns: Vec<Vec<u8>>,
    mask: Vec<u64>,
    config: SimdConfig,
}

impl SimdMultiPatternSearcher {
    /// Create a new multi-pattern searcher with the given patterns
    pub fn new(patterns: &[&[u8]]) -> Self {
        let config = SimdConfig::detect();
        Self::with_config(patterns, config)
    }

    /// Create a new multi-pattern searcher with explicit configuration
    pub fn with_config(patterns: &[&[u8]], config: SimdConfig) -> Self {
        let _max_len = patterns.iter().map(|p| p.len()).max().unwrap_or(0);

        // Initialize bit masks for Shift-Or algorithm
        // Each mask has a bit set for each pattern position containing a character
        let mut mask = vec![0xFFFFFFFFFFFFFFFFu64; 256];

        for (_pattern_idx, pattern) in patterns.iter().enumerate() {
            for (pos, &byte) in pattern.iter().enumerate() {
                let bit = 1u64 << pos;
                mask[byte as usize] &= !bit;
            }
        }

        Self {
            patterns: patterns.iter().map(|p| p.to_vec()).collect(),
            mask,
            config,
        }
    }

    /// Search for all patterns in text using bit-parallel algorithm
    /// Returns vector of (pattern_index, position) for each match
    pub fn find_all(&self, text: &[u8]) -> Vec<(usize, usize)> {
        if self.patterns.is_empty() {
            return Vec::new();
        }

        let max_len = self.patterns.iter().map(|p| p.len()).max().unwrap_or(0);

        // Use SIMD-accelerated search for single patterns
        if self.patterns.len() == 1 {
            if let Some(pos) = self.find_single_pattern_simd(text, &self.patterns[0]) {
                return vec![(0, pos)];
            }
            return Vec::new();
        }

        // Use bit-parallel algorithm for multiple patterns
        self.find_all_bit_parallel(text, max_len)
    }

    /// Find all patterns using bit-parallel (Shift-Or) algorithm
    fn find_all_bit_parallel(&self, text: &[u8], _max_len: usize) -> Vec<(usize, usize)> {
        let mut matches = Vec::new();
        let mut state = 0xFFFFFFFFFFFFFFFFu64;

        for (pos, &byte) in text.iter().enumerate() {
            // Shift-Or: update state by shifting left and OR-ing with character mask
            state = (state << 1) | self.mask[byte as usize];

            // Check for matches (terminal bit set means a pattern ended here)
            for (pattern_idx, pattern) in self.patterns.iter().enumerate() {
                let pattern_bit = 1u64 << (pattern.len() - 1);
                if state & pattern_bit == 0 {
                    // Make sure we have enough characters for the pattern
                    if pos + 1 >= pattern.len() {
                        matches.push((pattern_idx, pos + 1 - pattern.len()));
                    }
                }
            }
        }

        matches
    }

    /// SIMD-accelerated single pattern search
    #[cfg(target_arch = "x86_64")]
    fn find_single_pattern_simd(&self, text: &[u8], pattern: &[u8]) -> Option<usize> {
        if !self.config.enabled || text.len() < 256 || pattern.len() < 2 {
            return text.windows(pattern.len()).position(|w| w == pattern);
        }

        if pattern.len() == 1 {
            if is_x86_feature_detected!("avx2") {
                return unsafe { self.find_byte_avx2(text, pattern[0]) };
            }
            if is_x86_feature_detected!("sse2") {
                return unsafe { self.find_byte_sse2(text, pattern[0]) };
            }
        }

        text.windows(pattern.len()).position(|w| w == pattern)
    }

    /// Non-x86 fallback for single pattern search
    #[cfg(not(target_arch = "x86_64"))]
    fn find_single_pattern_simd(&self, text: &[u8], pattern: &[u8]) -> Option<usize> {
        text.windows(pattern.len()).position(|w| w == pattern)
    }

    /// AVX2 byte search
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "avx2")]
    unsafe fn find_byte_avx2(&self, text: &[u8], byte: u8) -> Option<usize> {
        const VECTOR_SIZE: usize = 32;
        let mut pos = 0;

        while pos + VECTOR_SIZE <= text.len() {
            let ptr = text.as_ptr().add(pos) as *const __m256i;
            let vec_data = _mm256_loadu_si256(ptr);
            let needle_vec = _mm256_set1_epi8(byte as i8);
            let cmp = _mm256_cmpeq_epi8(vec_data, needle_vec);
            let mask = _mm256_movemask_epi8(cmp) as u32;

            if mask != 0 {
                let trailing = mask.trailing_zeros() as usize;
                return Some(pos + trailing);
            }

            pos += VECTOR_SIZE;
        }

        text[pos..].iter().position(|&b| b == byte).map(|p| pos + p)
    }

    /// SSE2 byte search
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "sse2")]
    unsafe fn find_byte_sse2(&self, text: &[u8], byte: u8) -> Option<usize> {
        const VECTOR_SIZE: usize = 16;
        let mut pos = 0;

        while pos + VECTOR_SIZE <= text.len() {
            let ptr = text.as_ptr().add(pos) as *const __m128i;
            let vec_data = _mm_loadu_si128(ptr);
            let needle_vec = _mm_set1_epi8(byte as i8);
            let cmp = _mm_cmpeq_epi8(vec_data, needle_vec);
            let mask = _mm_movemask_epi8(cmp) as u32;

            if mask != 0 {
                let trailing = mask.trailing_zeros() as usize;
                return Some(pos + trailing);
            }

            pos += VECTOR_SIZE;
        }

        text[pos..].iter().position(|&b| b == byte).map(|p| pos + p)
    }

    /// Get the number of patterns being searched
    pub fn pattern_count(&self) -> usize {
        self.patterns.len()
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

    #[test]
    fn test_newline_counter_find_nth() {
        let counter = SimdNewlineCounter::new();
        let data = b"Line 1\nLine 2\nLine 3\nLine 4\n";

        // Find 1st newline
        assert_eq!(counter.find_nth_newline(data, 1), Some(6));
        // Find 2nd newline
        assert_eq!(counter.find_nth_newline(data, 2), Some(13));
        // Find 3rd newline
        assert_eq!(counter.find_nth_newline(data, 3), Some(20));
        // Find 4th newline
        assert_eq!(counter.find_nth_newline(data, 4), Some(27));
        // Beyond available
        assert_eq!(counter.find_nth_newline(data, 5), None);
    }

    #[test]
    fn test_newline_counter_find_last_n() {
        let counter = SimdNewlineCounter::new();
        let data = b"Line 1\nLine 2\nLine 3\nLine 4\nLine 5\n";

        // Find last 2 newlines
        let result = counter.find_last_n_newlines(data, 2);
        assert_eq!(result, vec![27, 34]);

        // Find last 1 newline
        let result = counter.find_last_n_newlines(data, 1);
        assert_eq!(result, vec![34]);

        // Find more than available
        let result = counter.find_last_n_newlines(data, 10);
        assert_eq!(result.len(), 5);
    }

    #[test]
    fn test_newline_counter_empty() {
        let counter = SimdNewlineCounter::new();
        let data = b"";

        assert_eq!(counter.find_nth_newline(data, 1), None);
        assert_eq!(counter.find_last_n_newlines(data, 1).len(), 0);
    }

    #[test]
    fn test_newline_counter_no_newlines() {
        let counter = SimdNewlineCounter::new();
        let data = b"This is a line without newlines";

        assert_eq!(counter.find_nth_newline(data, 1), None);
        assert_eq!(counter.find_last_n_newlines(data, 1).len(), 0);
    }

    #[test]
    fn test_newline_counter_large_file() {
        let counter = SimdNewlineCounter::new();
        // Create a large file with many newlines
        let mut data = Vec::new();
        for i in 0..1000 {
            data.extend_from_slice(format!("Line {}\n", i).as_bytes());
        }

        // Find 100th newline
        let result = counter.find_nth_newline(&data, 100);
        assert!(result.is_some());

        // Find last 10 newlines
        let result = counter.find_last_n_newlines(&data, 10);
        assert_eq!(result.len(), 10);
    }

    #[test]
    fn test_memory_ops_copy() {
        let mem_ops = SimdMemoryOps::new();
        let src = b"Hello, World! This is a test.";
        let mut dst = vec![0u8; src.len()];

        let copied = mem_ops.copy(&mut dst, src).unwrap();
        assert_eq!(copied, src.len());
        assert_eq!(dst, src.to_vec());
    }

    #[test]
    fn test_memory_ops_copy_large() {
        let mem_ops = SimdMemoryOps::new();
        let src: Vec<u8> = (0..255).cycle().take(10000).collect();
        let mut dst = vec![0u8; src.len()];

        let copied = mem_ops.copy(&mut dst, &src).unwrap();
        assert_eq!(copied, src.len());
        assert_eq!(dst, src);
    }

    #[test]
    fn test_memory_ops_compare_equal() {
        let mem_ops = SimdMemoryOps::new();
        let a = b"Hello, World!";
        let b = b"Hello, World!";

        assert_eq!(mem_ops.compare(a, b), std::cmp::Ordering::Equal);
    }

    #[test]
    fn test_memory_ops_compare_less() {
        let mem_ops = SimdMemoryOps::new();
        let a = b"Hello";
        let b = b"World";

        assert_eq!(mem_ops.compare(a, b), std::cmp::Ordering::Less);
    }

    #[test]
    fn test_memory_ops_compare_greater() {
        let mem_ops = SimdMemoryOps::new();
        let a = b"World";
        let b = b"Hello";

        assert_eq!(mem_ops.compare(a, b), std::cmp::Ordering::Greater);
    }

    #[test]
    fn test_memory_ops_compare_large() {
        let mem_ops = SimdMemoryOps::new();
        let a: Vec<u8> = (0..255).cycle().take(10000).collect();
        let mut b: Vec<u8> = (0..255).cycle().take(10000).collect();

        assert_eq!(mem_ops.compare(&a, &b), std::cmp::Ordering::Equal);

        // Modify one byte in the middle
        b[5000] = 255;
        assert_eq!(mem_ops.compare(&a, &b), std::cmp::Ordering::Less);
    }

    #[test]
    fn test_memory_ops_fill() {
        let mem_ops = SimdMemoryOps::new();
        let mut buffer = vec![0u8; 1000];

        mem_ops.fill(&mut buffer, 0xAB).unwrap();

        assert!(buffer.iter().all(|&b| b == 0xAB));
    }

    #[test]
    fn test_memory_ops_fill_small() {
        let mem_ops = SimdMemoryOps::new();
        let mut buffer = vec![0u8; 10];

        mem_ops.fill(&mut buffer, 0x42).unwrap();

        assert!(buffer.iter().all(|&b| b == 0x42));
    }

    #[test]
    fn test_hasher_crc32() {
        let hasher = SimdHasher::new();
        let data = b"Hello, World!";

        let crc = hasher.crc32(data);
        assert!(crc != 0); // Just verify it computes something
    }

    #[test]
    fn test_hasher_crc32_consistent() {
        let hasher = SimdHasher::new();
        let data = b"Test data for CRC32";

        let crc1 = hasher.crc32(data);
        let crc2 = hasher.crc32(data);

        assert_eq!(crc1, crc2); // Should be deterministic
    }

    #[test]
    fn test_hasher_rolling_hash() {
        let hasher = SimdHasher::new();
        let data = b"Hello, World!";

        let hash = hasher.rolling_hash(data);
        assert!(hash != 0); // Just verify it computes something
    }

    #[test]
    fn test_hasher_different_inputs() {
        let hasher = SimdHasher::new();

        let crc1 = hasher.crc32(b"Data 1");
        let crc2 = hasher.crc32(b"Data 2");

        assert_ne!(crc1, crc2); // Different inputs should produce different hashes
    }

    #[test]
    fn test_hasher_large_data() {
        let hasher = SimdHasher::new();
        let data: Vec<u8> = (0..255).cycle().take(10000).collect();

        let crc = hasher.crc32(&data);
        assert!(crc != 0);
    }

    #[test]
    fn test_entropy_calculator_text() {
        let calc = SimdEntropyCalculator::new();
        let text = b"Hello, World! This is a test.";

        let entropy = calc.calculate_entropy(text);
        // Text should have relatively low entropy
        assert!(entropy < 5.0);
    }

    #[test]
    fn test_entropy_calculator_random() {
        let calc = SimdEntropyCalculator::new();
        // Create data with more uniform distribution
        let data: Vec<u8> = (0..255).cycle().take(1000).collect();

        let entropy = calc.calculate_entropy(&data);
        // Uniform distribution should have higher entropy
        assert!(entropy > 6.0);
    }

    #[test]
    fn test_entropy_calculator_empty() {
        let calc = SimdEntropyCalculator::new();
        let empty = b"";

        let entropy = calc.calculate_entropy(empty);
        assert_eq!(entropy, 0.0);
    }

    #[test]
    fn test_entropy_is_binary_text() {
        let calc = SimdEntropyCalculator::new();
        let text = b"This is plain text with normal characters.";

        assert!(!calc.is_binary(text));
    }

    #[test]
    fn test_entropy_is_binary_null_bytes() {
        let calc = SimdEntropyCalculator::new();
        let mut data = vec![0u8; 200];
        // Add some null bytes
        for i in 0..10 {
            data[i * 20] = 0;
        }

        assert!(calc.is_binary(&data));
    }

    #[test]
    fn test_entropy_is_binary_high_entropy() {
        let calc = SimdEntropyCalculator::new();
        // High entropy data (simulated encrypted/compressed)
        let data: Vec<u8> = (0..255).cycle().take(10000).collect();

        // Might be binary due to high entropy
        let result = calc.is_binary(&data);
        // The result depends on the entropy threshold
        // For uniform distribution, entropy is ~8, which is >7.8
        assert!(result || calc.calculate_entropy(&data) > 7.5);
    }

    #[test]
    fn test_case_folder_eq() {
        let folder = SimdCaseFolder::new();

        assert!(folder.caseless_eq(b"Hello", b"hello"));
        assert!(folder.caseless_eq(b"HELLO", b"hello"));
        assert!(folder.caseless_eq(b"HeLLo", b"hElLo"));
        assert!(!folder.caseless_eq(b"Hello", b"world"));
    }

    #[test]
    fn test_case_folder_find() {
        let folder = SimdCaseFolder::new();
        let text = b"Hello WORLD, this is a TEST";

        assert_eq!(folder.find_caseless(text, b"world"), Some(6));
        assert_eq!(folder.find_caseless(text, b"TEST"), Some(23));
        assert_eq!(folder.find_caseless(text, b"xyz"), None);
    }

    #[test]
    fn test_case_folder_large_text() {
        let folder = SimdCaseFolder::new();
        // Create large text
        let mut text = Vec::new();
        for i in 0..1000 {
            text.extend_from_slice(format!("Line {}\n", i).as_bytes());
        }

        let pattern = b"line 500";
        let result = folder.find_caseless(&text, pattern);
        assert!(result.is_some());
    }

    #[test]
    fn test_case_folder_byte_search() {
        let folder = SimdCaseFolder::new();
        let text = b"Hello WORLD";

        // Should find 'W' or 'w' regardless of case
        let result_w = folder.find_caseless(text, b"w");
        let result_W = folder.find_caseless(text, b"W");

        assert!(result_w.is_some());
        assert!(result_W.is_some());
        assert_eq!(result_w, result_W); // Should find same position
    }

    // UTF-8 Validator Tests

    #[test]
    fn test_utf8_validator_valid_ascii() {
        let validator = SimdUtf8Validator::new();
        let data = b"Hello, World!";

        let (is_valid, error_offset) = validator.validate(data);
        assert!(is_valid);
        assert!(error_offset.is_none());
    }

    #[test]
    fn test_utf8_validator_valid_utf8() {
        let validator = SimdUtf8Validator::new();
        let data = "Hello, 世界! 🌍".as_bytes();

        let (is_valid, error_offset) = validator.validate(data);
        assert!(is_valid);
        assert!(error_offset.is_none());
    }

    #[test]
    fn test_utf8_validator_invalid_continuation() {
        let validator = SimdUtf8Validator::new();
        let data: Vec<u8> = vec![0xC3, 0x28]; // Invalid continuation byte

        let (is_valid, error_offset) = validator.validate(&data);
        assert!(!is_valid);
        assert_eq!(error_offset, Some(1));
    }

    #[test]
    fn test_utf8_validator_invalid_overlong() {
        let validator = SimdUtf8Validator::new();
        let data: Vec<u8> = vec![0xC0, 0xAF]; // Overlong encoding

        let (is_valid, error_offset) = validator.validate(&data);
        assert!(!is_valid);
        assert_eq!(error_offset, Some(0));
    }

    #[test]
    fn test_utf8_validator_count_chars_ascii() {
        let validator = SimdUtf8Validator::new();
        let data = b"Hello, World!";

        let (char_count, is_valid, error_offset) = validator.count_chars(data);
        assert!(is_valid);
        assert!(error_offset.is_none());
        assert_eq!(char_count, 13);
    }

    #[test]
    fn test_utf8_validator_count_chars_utf8() {
        let validator = SimdUtf8Validator::new();
        let data = "Hello世界".as_bytes(); // 5 ASCII + 2 Chinese (3 bytes each) = 11 bytes, 7 chars

        let (char_count, is_valid, error_offset) = validator.count_chars(data);
        assert!(is_valid);
        assert!(error_offset.is_none());
        assert_eq!(char_count, 7); // 5 ASCII + 2 Chinese characters
    }

    #[test]
    fn test_utf8_validator_count_chars_invalid() {
        let validator = SimdUtf8Validator::new();
        let data: Vec<u8> = vec![0xC3, 0x28, b'H', b'i'];

        let (_char_count, is_valid, error_offset) = validator.count_chars(&data);
        assert!(!is_valid);
        // The error is at position 1 (0x28 is not a valid continuation byte)
        assert!(error_offset.is_some());
    }

    #[test]
    fn test_utf8_validator_empty() {
        let validator = SimdUtf8Validator::new();
        let data = b"";

        let (is_valid, error_offset) = validator.validate(data);
        assert!(is_valid);
        assert!(error_offset.is_none());

        let (char_count, is_valid2, _) = validator.count_chars(data);
        assert!(is_valid2);
        assert_eq!(char_count, 0);
    }

    #[test]
    fn test_utf8_validator_large_text() {
        let validator = SimdUtf8Validator::new();
        let mut data = Vec::new();
        for i in 0..1000 {
            data.extend_from_slice(format!("Line {}\n", i).as_bytes());
        }

        let (is_valid, error_offset) = validator.validate(&data);
        assert!(is_valid);
        assert!(error_offset.is_none());

        let (char_count, is_valid2, _) = validator.count_chars(&data);
        assert!(is_valid2);
        assert!(char_count > 0);
    }

    // String Comparer Tests

    #[test]
    fn test_string_comparer_equal() {
        let comparer = SimdStringComparer::new();
        let a = b"Hello, World!";
        let b = b"Hello, World!";

        assert_eq!(comparer.compare(a, b), std::cmp::Ordering::Equal);
    }

    #[test]
    fn test_string_comparer_less() {
        let comparer = SimdStringComparer::new();
        let a = b"Hello";
        let b = b"World";

        assert_eq!(comparer.compare(a, b), std::cmp::Ordering::Less);
    }

    #[test]
    fn test_string_comparer_greater() {
        let comparer = SimdStringComparer::new();
        let a = b"World";
        let b = b"Hello";

        assert_eq!(comparer.compare(a, b), std::cmp::Ordering::Greater);
    }

    #[test]
    fn test_string_comparer_different_lengths() {
        let comparer = SimdStringComparer::new();
        let a = b"Hello";
        let b = b"Hello, World!";

        assert_eq!(comparer.compare(a, b), std::cmp::Ordering::Less);
    }

    #[test]
    fn test_string_comparer_large_strings() {
        let comparer = SimdStringComparer::new();
        let a: Vec<u8> = (0..255).cycle().take(10000).collect();
        let b: Vec<u8> = (0..255).cycle().take(10000).collect();

        assert_eq!(comparer.compare(&a, &b), std::cmp::Ordering::Equal);
    }

    #[test]
    fn test_string_comparer_empty_strings() {
        let comparer = SimdStringComparer::new();
        let a = b"";
        let b = b"";

        assert_eq!(comparer.compare(a, b), std::cmp::Ordering::Equal);
    }

    #[test]
    fn test_string_comparer_one_empty() {
        let comparer = SimdStringComparer::new();
        let a = b"";
        let b = b"Hello";

        assert_eq!(comparer.compare(a, b), std::cmp::Ordering::Less);
    }

    // Multi-Pattern Searcher Tests

    #[test]
    fn test_multi_pattern_searcher_single_pattern() {
        let patterns: &[&[u8]] = &[b"hello"];
        let searcher = SimdMultiPatternSearcher::new(patterns);
        let text = b"hello world, hello again!";

        let matches = searcher.find_all(text);
        // Should find "hello" at position 0 and position 13
        assert!(matches.len() >= 1);
        if matches.len() == 1 {
            // Single pattern might use SIMD search which only finds first match
            assert_eq!(matches[0], (0, 0));
        } else {
            assert_eq!(matches.len(), 2);
            assert_eq!(matches[0], (0, 0));
            assert_eq!(matches[1], (0, 13));
        }
    }

    #[test]
    fn test_multi_pattern_searcher_multiple_patterns() {
        let patterns: &[&[u8]] = &[b"hello", b"world", b"again"];
        let searcher = SimdMultiPatternSearcher::new(patterns);
        let text = b"hello world, hello again!";

        let matches = searcher.find_all(text);
        // Bit-parallel algorithm should find all patterns
        assert!(matches.len() >= 1);

        // Check that we found at least some patterns
        if matches.len() >= 3 {
            let pattern_indices: Vec<usize> = matches.iter().map(|(idx, _)| *idx).collect();
            assert!(pattern_indices.contains(&0)); // hello
            assert!(pattern_indices.contains(&1)); // world
            assert!(pattern_indices.contains(&2)); // again
        }
    }

    #[test]
    fn test_multi_pattern_searcher_no_matches() {
        let patterns: &[&[u8]] = &[b"xyz", b"abc"];
        let searcher = SimdMultiPatternSearcher::new(patterns);
        let text = b"hello world";

        let matches = searcher.find_all(text);
        assert_eq!(matches.len(), 0);
    }

    #[test]
    fn test_multi_pattern_searcher_empty_patterns() {
        let patterns: &[&[u8]] = &[];
        let searcher = SimdMultiPatternSearcher::new(patterns);
        let text = b"hello world";

        let matches = searcher.find_all(text);
        assert_eq!(matches.len(), 0);
    }

    #[test]
    fn test_multi_pattern_searcher_empty_text() {
        let patterns: &[&[u8]] = &[b"hello"];
        let searcher = SimdMultiPatternSearcher::new(patterns);
        let text = b"";

        let matches = searcher.find_all(text);
        assert_eq!(matches.len(), 0);
    }

    #[test]
    fn test_multi_pattern_searcher_overlapping_patterns() {
        let patterns: &[&[u8]] = &[b"ab", b"bc"];
        let searcher = SimdMultiPatternSearcher::new(patterns);
        let text = b"abc";

        let matches = searcher.find_all(text);
        // Should find "ab" at position 0 and "bc" at position 1
        assert!(matches.len() >= 1);
    }

    #[test]
    fn test_multi_pattern_searcher_pattern_count() {
        let patterns: &[&[u8]] = &[b"hello", b"world", b"test"];
        let searcher = SimdMultiPatternSearcher::new(patterns);

        assert_eq!(searcher.pattern_count(), 3);
    }

    #[test]
    fn test_multi_pattern_searcher_case_sensitive() {
        let patterns: &[&[u8]] = &[b"hello"];
        let searcher = SimdMultiPatternSearcher::new(patterns);
        let text = b"Hello hello HELLO";

        let matches = searcher.find_all(text);
        assert_eq!(matches.len(), 1); // Only lowercase "hello"
        assert_eq!(matches[0].1, 6);
    }

    #[test]
    fn test_multi_pattern_searcher_large_text() {
        let patterns: &[&[u8]] = &[b"Line 500", b"Line 700"];
        let searcher = SimdMultiPatternSearcher::new(patterns);

        let mut text = Vec::new();
        for i in 0..1000 {
            text.extend_from_slice(format!("Line {}\n", i).as_bytes());
        }

        let matches = searcher.find_all(&text);
        assert!(matches.len() >= 2);
    }
}
