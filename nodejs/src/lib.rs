//! Node.js bindings for AI-Coreutils using NAPI-RS
//!
//! This module provides JavaScript/TypeScript bindings for the core
//! functionality of AI-Coreutils.

use napi_derive::napi;
use std::path::PathBuf;
use std::str;

// Import from ai-coreutils library
use ai_coreutils::memory::SafeMemoryAccess;
use ai_coreutils::simd_ops::{SimdConfig, SimdPatternSearcher, SimdByteCounter, SimdTextProcessor, TextMetrics};
use ai_coreutils::ml_ops::{PatternDetector, MlConfig, FileClassifier};

/// Safe memory access for files with SIMD operations
#[napi(object)]
pub struct TextMetrics {
    pub lines: u32,
    pub words: u32,
    pub bytes: u32,
}

/// Pattern match result
#[napi(object)]
pub struct PatternMatch {
    pub pattern: String,
    pub matched_text: String,
    pub start: u32,
    pub end: u32,
    pub confidence: f64,
    pub pattern_type: String,
}

/// Text statistics
#[napi(object)]
pub struct TextStatistics {
    pub characters: u32,
    pub bytes: u32,
    pub lines: u32,
    pub words: u32,
    pub avg_line_length: f64,
    pub max_line_length: u32,
    pub whitespace_ratio: f64,
    pub entropy: f64,
}

/// Content analysis result
#[napi(object)]
pub struct ContentAnalysis {
    pub path: String,
    pub total_patterns: u32,
    pub matches: Vec<PatternMatch>,
    pub statistics: TextStatistics,
    pub issues: Vec<String>,
}

/// File classification result
#[napi(object)]
pub struct FileClassification {
    pub path: String,
    pub file_type: String,
    pub confidence: f64,
    pub encoding: String,
    pub mime_type: String,
    pub is_binary: bool,
    pub language: Option<String>,
}

/// Safe memory access wrapper
#[napi]
pub struct MemoryAccess {
    inner: SafeMemoryAccess,
}

#[napi]
impl MemoryAccess {
    /// Create a new memory-mapped file access
    #[napi(constructor)]
    pub fn new(path: String) -> napi::Result<Self> {
        SafeMemoryAccess::new(&path)
            .map(|inner| Self { inner })
            .map_err(|e| napi::Error::new(napi::Status::GenericFailure, e.to_string()))
    }

    /// Get the size of the memory-mapped region
    #[napi(getter)]
    pub fn size(&self) -> u32 {
        self.inner.size() as u32
    }

    /// Get a raw pointer to the memory (as number)
    #[napi(getter)]
    pub fn ptr(&self) -> u64 {
        self.inner.as_ptr() as u64
    }

    /// Bounds-checked access to a slice of memory
    #[napi]
    pub fn get(&self, offset: u32, len: u32) -> Option<Vec<u8>> {
        self.inner.get(offset as usize, len as usize)
            .map(|data| data.to_vec())
    }

    /// Get a byte at the given offset
    #[napi]
    pub fn get_byte(&self, offset: u32) -> Option<u32> {
        self.inner.get_byte(offset as usize).map(|b| b as u32)
    }

    /// Search for a pattern in the memory-mapped region
    #[napi]
    pub fn find_pattern(&self, pattern: Vec<u8>) -> Vec<u32> {
        self.inner.find_pattern(&pattern)
            .into_iter()
            .map(|offset| offset as u32)
            .collect()
    }

    /// Count occurrences of a byte
    #[napi]
    pub fn count_byte(&self, byte: u32) -> u32 {
        self.inner.count_byte(byte as u8) as u32
    }

    /// Count lines, words, and bytes
    #[napi]
    pub fn count_text_metrics(&self) -> TextMetrics {
        let (lines, words, bytes) = self.inner.count_text_metrics();
        TextMetrics {
            lines: lines as u32,
            words: words as u32,
            bytes: bytes as u32,
        }
    }
}

/// SIMD text processor
#[napi]
pub struct TextProcessor {
    inner: SimdTextProcessor,
}

#[napi]
impl TextProcessor {
    /// Create a new SIMD text processor
    #[napi(constructor)]
    pub fn new() -> Self {
        Self {
            inner: SimdTextProcessor::new(),
        }
    }

    /// Analyze text and return metrics
    #[napi]
    pub fn analyze(&self, data: Vec<u8>) -> TextMetrics {
        let metrics = self.inner.analyze(&data);
        TextMetrics {
            lines: metrics.lines as u32,
            words: metrics.words as u32,
            bytes: metrics.bytes as u32,
        }
    }

    /// Count lines in data
    #[napi]
    pub fn count_lines(&self, data: Vec<u8>) -> u32 {
        self.inner.analyze(&data).lines as u32
    }

    /// Count words in data
    #[napi]
    pub fn count_words(&self, data: Vec<u8>) -> u32 {
        self.inner.analyze(&data).words as u32
    }
}

/// Pattern detector
#[napi]
pub struct PatternDetectorWrapper {
    inner: PatternDetector,
}

#[napi]
impl PatternDetectorWrapper {
    /// Create a new pattern detector
    #[napi(constructor)]
    pub fn new() -> napi::Result<Self> {
        PatternDetector::new()
            .map(|inner| Self { inner })
            .map_err(|e| napi::Error::new(napi::Status::GenericFailure, e.to_string()))
    }

    /// Detect all patterns in the given text
    #[napi]
    pub fn detect_patterns(&self, text: String) -> Vec<PatternMatch> {
        let matches = self.inner.detect_patterns(&text);
        matches
            .into_iter()
            .map(|m| PatternMatch {
                pattern: m.pattern,
                matched_text: m.matched_text,
                start: m.start as u32,
                end: m.end as u32,
                confidence: m.confidence,
                pattern_type: format!("{:?}", m.pattern_type),
            })
            .collect()
    }

    /// Analyze content and return detailed results
    #[napi]
    pub fn analyze_content(&self, text: String, path: String) -> napi::Result<ContentAnalysis> {
        let path_buf = PathBuf::from(&path);
        let analysis = self.inner.analyze_content(&text, &path_buf)
            .map_err(|e| napi::Error::new(napi::Status::GenericFailure, e.to_string()))?;

        Ok(ContentAnalysis {
            path: analysis.path,
            total_patterns: analysis.total_patterns as u32,
            matches: analysis
                .matches
                .into_iter()
                .map(|m| PatternMatch {
                    pattern: m.pattern,
                    matched_text: m.matched_text,
                    start: m.start as u32,
                    end: m.end as u32,
                    confidence: m.confidence,
                    pattern_type: format!("{:?}", m.pattern_type),
                })
                .collect(),
            statistics: TextStatistics {
                characters: analysis.statistics.characters as u32,
                bytes: analysis.statistics.bytes as u32,
                lines: analysis.statistics.lines as u32,
                words: analysis.statistics.words as u32,
                avg_line_length: analysis.statistics.avg_line_length,
                max_line_length: analysis.statistics.max_line_length as u32,
                whitespace_ratio: analysis.statistics.whitespace_ratio,
                entropy: analysis.statistics.entropy,
            },
            issues: analysis.issues,
        })
    }
}

/// File classifier
#[napi]
pub struct FileClassifierWrapper;

#[napi]
impl FileClassifierWrapper {
    /// Create a new file classifier
    #[napi(constructor)]
    pub fn new() -> Self {
        Self
    }

    /// Classify a file based on its extension and content
    #[napi]
    pub fn classify(&self, path: String, content: Vec<u8>) -> napi::Result<FileClassification> {
        let path_buf = PathBuf::from(&path);
        let classification = FileClassifier::classify(&path_buf, &content)
            .map_err(|e| napi::Error::new(napi::Status::GenericFailure, e.to_string()))?;

        Ok(FileClassification {
            path: classification.path,
            file_type: classification.file_type,
            confidence: classification.confidence,
            encoding: classification.encoding,
            mime_type: classification.mime_type,
            is_binary: classification.is_binary,
            language: classification.language,
        })
    }
}

/// SIMD configuration
#[napi]
pub struct SimdConfigWrapper {
    pub enabled: bool,
    pub vector_width: u32,
}

#[napi]
impl SimdConfigWrapper {
    /// Detect CPU SIMD capabilities
    #[napi(factory)]
    pub fn detect() -> Self {
        let config = SimdConfig::detect();
        Self {
            enabled: config.enabled,
            vector_width: config.vector_width as u32,
        }
    }

    /// Create with explicit settings
    #[napi(factory)]
    pub fn with_options(enabled: bool, vector_width: u32) -> Self {
        Self {
            enabled,
            vector_width,
        }
    }
}

/// Utility functions for common operations
#[napi]
pub struct Utils;

#[napi]
impl Utils {
    /// Count lines in a string
    #[napi]
    pub fn count_lines(text: String) -> u32 {
        text.lines().count() as u32
    }

    /// Count words in a string
    #[napi]
    pub fn count_words(text: String) -> u32 {
        text.split_whitespace().count() as u32
    }

    /// Check if content appears to be binary
    #[napi]
    pub fn is_binary(content: Vec<u8>) -> bool {
        if content.is_empty() {
            return false;
        }

        let sample_size = 1000.min(content.len());
        let null_count = content[..sample_size].iter().filter(|&&b| b == 0).count();

        if null_count > sample_size / 100 {
            return true;
        }

        let non_printable = content[..sample_size]
            .iter()
            .filter(|&&b| b < 0x20 && b != b'\t' && b != b'\n' && b != b'\r')
            .count();

        non_printable > sample_size / 20
    }
}
