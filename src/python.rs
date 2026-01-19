//! Python bindings for AI-Coreutils
//!
//! This module provides Python bindings using PyO3, exposing the core
//! functionality of AI-Coreutils to Python code.

#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg(feature = "python")]
use pyo3::types::{PyBytes, PyDict};
#[cfg(feature = "python")]
use std::path::PathBuf;

#[cfg(feature = "python")]
use crate::memory::SafeMemoryAccess;
#[cfg(feature = "python")]
use crate::simd_ops::{SimdConfig, SimdTextProcessor};
#[cfg(feature = "python")]
use crate::ml_ops::{PatternDetector, FileClassifier};

/// Python wrapper for SafeMemoryAccess
#[cfg(feature = "python")]
#[pyclass(name = "SafeMemoryAccess")]
pub struct PySafeMemoryAccess {
    inner: SafeMemoryAccess,
}

#[cfg(feature = "python")]
#[pymethods]
impl PySafeMemoryAccess {
    /// Create a new memory-mapped file access
    #[new]
    #[pyo3(signature = (path))]
    pub fn new(path: &str) -> PyResult<Self> {
        let access = SafeMemoryAccess::new(path)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;
        Ok(Self { inner: access })
    }

    /// Get the size of the memory-mapped region
    pub fn size(&self) -> usize {
        self.inner.size()
    }

    /// Get a raw pointer to the memory
    pub fn as_ptr(&self) -> usize {
        self.inner.as_ptr() as usize
    }

    /// Bounds-checked access to a slice of memory
    pub fn get(&self, offset: usize, len: usize) -> PyResult<Option<Py<PyBytes>>> {
        Python::with_gil(|py| {
            Ok(self.inner.get(offset, len).map(|data| {
                PyBytes::new_bound(py, data).into()
            }))
        })
    }

    /// Get a byte at the given offset
    pub fn get_byte(&self, offset: usize) -> Option<u8> {
        self.inner.get_byte(offset)
    }

    /// Search for a pattern in the memory-mapped region
    pub fn find_pattern(&self, pattern: &[u8]) -> Vec<usize> {
        self.inner.find_pattern(pattern)
    }

    /// Count occurrences of a byte in the memory-mapped region
    pub fn count_byte(&self, byte: u8) -> usize {
        self.inner.count_byte(byte)
    }

    /// Count lines, words, and bytes in the memory-mapped region
    pub fn count_text_metrics(&self) -> (usize, usize, usize) {
        self.inner.count_text_metrics()
    }

    /// Get a string representation
    pub fn __repr__(&self) -> String {
        format!("SafeMemoryAccess(size={})", self.inner.size())
    }
}

/// Python wrapper for SimdConfig
#[cfg(feature = "python")]
#[pyclass(name = "SimdConfig")]
#[derive(Clone)]
pub struct PySimdConfig {
    inner: SimdConfig,
}

#[cfg(feature = "python")]
#[pymethods]
impl PySimdConfig {
    /// Create a new SIMD config with auto-detected capabilities
    #[staticmethod]
    pub fn detect() -> Self {
        Self {
            inner: SimdConfig::detect(),
        }
    }

    /// Create a new SIMD config with explicit settings
    #[new]
    #[pyo3(signature = (enabled=true, vector_width=32))]
    pub fn new(enabled: bool, vector_width: usize) -> Self {
        Self {
            inner: SimdConfig {
                enabled,
                vector_width,
            },
        }
    }

    /// Check if SIMD is enabled
    pub fn enabled(&self) -> bool {
        self.inner.enabled
    }

    /// Get the vector width
    pub fn vector_width(&self) -> usize {
        self.inner.vector_width
    }

    /// Get a string representation
    pub fn __repr__(&self) -> String {
        format!("SimdConfig(enabled={}, vector_width={})", self.inner.enabled, self.inner.vector_width)
    }
}

/// Python wrapper for TextMetrics
#[cfg(feature = "python")]
#[pyclass(name = "TextMetrics")]
#[derive(Clone)]
pub struct PyTextMetrics {
    /// Number of lines
    pub lines: usize,
    /// Number of words
    pub words: usize,
    /// Number of bytes
    pub bytes: usize,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyTextMetrics {
    /// Create new TextMetrics
    #[new]
    pub fn new(lines: usize, words: usize, bytes: usize) -> Self {
        Self { lines, words, bytes }
    }

    /// Get lines count
    pub fn lines(&self) -> usize {
        self.lines
    }

    /// Get words count
    pub fn words(&self) -> usize {
        self.words
    }

    /// Get bytes count
    pub fn bytes(&self) -> usize {
        self.bytes
    }

    /// Get a dictionary representation
    pub fn to_dict(&self) -> Py<PyDict> {
        Python::with_gil(|py| {
            let dict = PyDict::new_bound(py);
            dict.set_item("lines", self.lines).unwrap();
            dict.set_item("words", self.words).unwrap();
            dict.set_item("bytes", self.bytes).unwrap();
            dict.into()
        })
    }

    /// Get a string representation
    pub fn __repr__(&self) -> String {
        format!("TextMetrics(lines={}, words={}, bytes={})", self.lines, self.words, self.bytes)
    }
}

/// Python wrapper for SimdTextProcessor
#[cfg(feature = "python")]
#[pyclass(name = "SimdTextProcessor")]
pub struct PySimdTextProcessor {
    inner: SimdTextProcessor,
}

#[cfg(feature = "python")]
#[pymethods]
impl PySimdTextProcessor {
    /// Create a new SIMD text processor
    #[new]
    pub fn new() -> Self {
        Self {
            inner: SimdTextProcessor::new(),
        }
    }

    /// Analyze text and return metrics
    pub fn analyze(&self, data: &[u8]) -> PyTextMetrics {
        let metrics = self.inner.analyze(data);
        PyTextMetrics {
            lines: metrics.lines,
            words: metrics.words,
            bytes: metrics.bytes,
        }
    }

    /// Count lines in data
    pub fn count_lines(&self, data: &[u8]) -> usize {
        self.inner.analyze(data).lines
    }

    /// Count words in data
    pub fn count_words(&self, data: &[u8]) -> usize {
        self.inner.analyze(data).words
    }

    /// Get a string representation
    pub fn __repr__(&self) -> String {
        "SimdTextProcessor()".to_string()
    }
}

/// Python wrapper for PatternType
#[cfg(feature = "python")]
#[pyclass(name = "PatternType")]
#[derive(Clone)]
pub struct PyPatternType {
    /// Pattern type name
    pub name: String,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyPatternType {
    /// Email pattern type
    #[staticmethod]
    pub fn email() -> Self {
        Self { name: "Email".to_string() }
    }

    /// URL pattern type
    #[staticmethod]
    pub fn url() -> Self {
        Self { name: "Url".to_string() }
    }

    /// IP address pattern type
    #[staticmethod]
    pub fn ip_address() -> Self {
        Self { name: "IpAddress".to_string() }
    }

    /// Phone number pattern type
    #[staticmethod]
    pub fn phone_number() -> Self {
        Self { name: "PhoneNumber".to_string() }
    }

    /// Credit card pattern type
    #[staticmethod]
    pub fn credit_card() -> Self {
        Self { name: "CreditCard".to_string() }
    }

    /// SSN pattern type
    #[staticmethod]
    pub fn ssn() -> Self {
        Self { name: "Ssn".to_string() }
    }

    /// Date pattern type
    #[staticmethod]
    pub fn date() -> Self {
        Self { name: "Date".to_string() }
    }

    /// Hex pattern type
    #[staticmethod]
    pub fn hex() -> Self {
        Self { name: "Hex".to_string() }
    }

    /// Base64 pattern type
    #[staticmethod]
    pub fn base64() -> Self {
        Self { name: "Base64".to_string() }
    }

    /// UUID pattern type
    #[staticmethod]
    pub fn uuid() -> Self {
        Self { name: "Uuid".to_string() }
    }

    /// File path pattern type
    #[staticmethod]
    pub fn file_path() -> Self {
        Self { name: "FilePath".to_string() }
    }

    /// Get a string representation
    pub fn __repr__(&self) -> String {
        format!("PatternType({})", self.name)
    }

    /// Get a string representation
    pub fn __str__(&self) -> String {
        self.name.clone()
    }
}

/// Python wrapper for PatternMatch
#[cfg(feature = "python")]
#[pyclass(name = "PatternMatch")]
#[derive(Clone)]
pub struct PyPatternMatch {
    /// The regex pattern used
    pub pattern: String,
    /// The text that matched the pattern
    pub matched_text: String,
    /// Start position of the match
    pub start: usize,
    /// End position of the match
    pub end: usize,
    /// Confidence score (0.0-1.0)
    pub confidence: f64,
    /// Type of pattern
    pub pattern_type: PyPatternType,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyPatternMatch {
    /// Create a new PatternMatch
    #[new]
    pub fn new(
        pattern: String,
        matched_text: String,
        start: usize,
        end: usize,
        confidence: f64,
        pattern_type: PyPatternType,
    ) -> Self {
        Self {
            pattern,
            matched_text,
            start,
            end,
            confidence,
            pattern_type,
        }
    }

    /// Get a dictionary representation
    pub fn to_dict(&self) -> Py<PyDict> {
        Python::with_gil(|py| {
            let dict = PyDict::new_bound(py);
            dict.set_item("pattern", &self.pattern).unwrap();
            dict.set_item("matched_text", &self.matched_text).unwrap();
            dict.set_item("start", self.start).unwrap();
            dict.set_item("end", self.end).unwrap();
            dict.set_item("confidence", self.confidence).unwrap();
            dict.set_item("pattern_type", self.pattern_type.name.clone()).unwrap();
            dict.into()
        })
    }

    /// Get a string representation
    pub fn __repr__(&self) -> String {
        format!(
            "PatternMatch(pattern={}, matched_text={}, confidence={})",
            self.pattern, self.matched_text, self.confidence
        )
    }
}

/// Python wrapper for TextStatistics
#[cfg(feature = "python")]
#[pyclass(name = "TextStatistics")]
#[derive(Clone)]
pub struct PyTextStatistics {
    /// Total character count
    pub characters: usize,
    /// Total byte count
    pub bytes: usize,
    /// Total line count
    pub lines: usize,
    /// Total word count
    pub words: usize,
    /// Average line length
    pub avg_line_length: f64,
    /// Maximum line length
    pub max_line_length: usize,
    /// Ratio of whitespace characters
    pub whitespace_ratio: f64,
    /// Shannon entropy score
    pub entropy: f64,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyTextStatistics {
    /// Create a new TextStatistics
    #[new]
    pub fn new(
        characters: usize,
        bytes: usize,
        lines: usize,
        words: usize,
        avg_line_length: f64,
        max_line_length: usize,
        whitespace_ratio: f64,
        entropy: f64,
    ) -> Self {
        Self {
            characters,
            bytes,
            lines,
            words,
            avg_line_length,
            max_line_length,
            whitespace_ratio,
            entropy,
        }
    }

    /// Get a dictionary representation
    pub fn to_dict(&self) -> Py<PyDict> {
        Python::with_gil(|py| {
            let dict = PyDict::new_bound(py);
            dict.set_item("characters", self.characters).unwrap();
            dict.set_item("bytes", self.bytes).unwrap();
            dict.set_item("lines", self.lines).unwrap();
            dict.set_item("words", self.words).unwrap();
            dict.set_item("avg_line_length", self.avg_line_length).unwrap();
            dict.set_item("max_line_length", self.max_line_length).unwrap();
            dict.set_item("whitespace_ratio", self.whitespace_ratio).unwrap();
            dict.set_item("entropy", self.entropy).unwrap();
            dict.into()
        })
    }

    /// Get a string representation
    pub fn __repr__(&self) -> String {
        format!(
            "TextStatistics(lines={}, words={}, bytes={}, entropy={})",
            self.lines, self.words, self.bytes, self.entropy
        )
    }
}

/// Python wrapper for ContentAnalysis
#[cfg(feature = "python")]
#[pyclass(name = "ContentAnalysis")]
pub struct PyContentAnalysis {
    /// File path analyzed
    pub path: String,
    /// Total number of patterns found
    pub total_patterns: usize,
    /// All pattern matches
    pub matches: Vec<PyPatternMatch>,
    /// Text statistics
    pub statistics: PyTextStatistics,
    /// Detected issues (e.g., high entropy, sensitive data)
    pub issues: Vec<String>,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyContentAnalysis {
    /// Get total patterns found
    pub fn total_patterns(&self) -> usize {
        self.total_patterns
    }

    /// Get all pattern matches
    pub fn matches(&self) -> Vec<PyPatternMatch> {
        self.matches.clone()
    }

    /// Get statistics
    pub fn statistics(&self) -> PyTextStatistics {
        self.statistics.clone()
    }

    /// Get detected issues
    pub fn issues(&self) -> Vec<String> {
        self.issues.clone()
    }

    /// Get a dictionary representation
    pub fn to_dict(&self) -> Py<PyDict> {
        Python::with_gil(|py| {
            let dict = PyDict::new_bound(py);
            dict.set_item("path", &self.path).unwrap();
            dict.set_item("total_patterns", self.total_patterns).unwrap();
            dict.set_item("matches", self.matches.iter().map(|m| m.to_dict()).collect::<Vec<_>>()).unwrap();
            dict.set_item("statistics", self.statistics.to_dict()).unwrap();
            dict.set_item("issues", &self.issues).unwrap();
            dict.into()
        })
    }

    /// Get a string representation
    pub fn __repr__(&self) -> String {
        format!(
            "ContentAnalysis(path={}, total_patterns={})",
            self.path, self.total_patterns
        )
    }
}

/// Python wrapper for FileClassification
#[cfg(feature = "python")]
#[pyclass(name = "FileClassification")]
pub struct PyFileClassification {
    /// File path classified
    pub path: String,
    /// Detected file type
    pub file_type: String,
    /// Confidence score (0.0-1.0)
    pub confidence: f64,
    /// Text encoding
    pub encoding: String,
    /// MIME type
    pub mime_type: String,
    /// Whether file is binary
    pub is_binary: bool,
    /// Detected programming language (if applicable)
    pub language: Option<String>,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyFileClassification {
    /// Get a dictionary representation
    pub fn to_dict(&self) -> Py<PyDict> {
        Python::with_gil(|py| {
            let dict = PyDict::new_bound(py);
            dict.set_item("path", &self.path).unwrap();
            dict.set_item("file_type", &self.file_type).unwrap();
            dict.set_item("confidence", self.confidence).unwrap();
            dict.set_item("encoding", &self.encoding).unwrap();
            dict.set_item("mime_type", &self.mime_type).unwrap();
            dict.set_item("is_binary", self.is_binary).unwrap();
            dict.set_item("language", &self.language).unwrap();
            dict.into()
        })
    }

    /// Get a string representation
    pub fn __repr__(&self) -> String {
        format!(
            "FileClassification(path={}, file_type={}, confidence={})",
            self.path, self.file_type, self.confidence
        )
    }
}

/// Python wrapper for PatternDetector
#[cfg(feature = "python")]
#[pyclass(name = "PatternDetector")]
pub struct PyPatternDetector {
    inner: PatternDetector,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyPatternDetector {
    /// Create a new pattern detector with default settings
    #[new]
    pub fn new() -> PyResult<Self> {
        let detector = PatternDetector::new()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        Ok(Self { inner: detector })
    }

    /// Detect all patterns in the given text
    pub fn detect_patterns(&self, text: &str) -> Vec<PyPatternMatch> {
        let matches = self.inner.detect_patterns(text);
        matches
            .into_iter()
            .map(|m| {
                let pattern_type_name = format!("{:?}", m.pattern_type);
                PyPatternMatch {
                    pattern: m.pattern,
                    matched_text: m.matched_text,
                    start: m.start,
                    end: m.end,
                    confidence: m.confidence,
                    pattern_type: PyPatternType { name: pattern_type_name },
                }
            })
            .collect()
    }

    /// Analyze content and return detailed results
    pub fn analyze_content(&self, text: &str, path: &str) -> PyResult<PyContentAnalysis> {
        let path = PathBuf::from(path);
        let analysis = self.inner.analyze_content(text, &path)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;

        Ok(PyContentAnalysis {
            path: analysis.path,
            total_patterns: analysis.total_patterns,
            matches: analysis
                .matches
                .into_iter()
                .map(|m| {
                    let pattern_type_name = format!("{:?}", m.pattern_type);
                    PyPatternMatch {
                        pattern: m.pattern,
                        matched_text: m.matched_text,
                        start: m.start,
                        end: m.end,
                        confidence: m.confidence,
                        pattern_type: PyPatternType { name: pattern_type_name },
                    }
                })
                .collect(),
            statistics: PyTextStatistics {
                characters: analysis.statistics.characters,
                bytes: analysis.statistics.bytes,
                lines: analysis.statistics.lines,
                words: analysis.statistics.words,
                avg_line_length: analysis.statistics.avg_line_length,
                max_line_length: analysis.statistics.max_line_length,
                whitespace_ratio: analysis.statistics.whitespace_ratio,
                entropy: analysis.statistics.entropy,
            },
            issues: analysis.issues,
        })
    }

    /// Get a string representation
    pub fn __repr__(&self) -> String {
        "PatternDetector()".to_string()
    }
}

/// Python wrapper for FileClassifier
#[cfg(feature = "python")]
#[pyclass(name = "FileClassifier")]
pub struct PyFileClassifier;

#[cfg(feature = "python")]
#[pymethods]
impl PyFileClassifier {
    /// Create a new file classifier
    #[new]
    pub fn new() -> Self {
        Self
    }

    /// Classify a file based on its extension and content
    pub fn classify(&self, path: &str, content: &[u8]) -> PyResult<PyFileClassification> {
        let path = PathBuf::from(path);
        let classification = FileClassifier::classify(&path, content)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))?;

        Ok(PyFileClassification {
            path: classification.path,
            file_type: classification.file_type,
            confidence: classification.confidence,
            encoding: classification.encoding,
            mime_type: classification.mime_type,
            is_binary: classification.is_binary,
            language: classification.language,
        })
    }

    /// Get a string representation
    pub fn __repr__(&self) -> String {
        "FileClassifier()".to_string()
    }
}

/// Python module definition
#[cfg(feature = "python")]
#[pymodule]
fn ai_coreutils(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PySafeMemoryAccess>()?;
    m.add_class::<PySimdConfig>()?;
    m.add_class::<PySimdTextProcessor>()?;
    m.add_class::<PyTextMetrics>()?;
    m.add_class::<PyPatternType>()?;
    m.add_class::<PyPatternMatch>()?;
    m.add_class::<PyTextStatistics>()?;
    m.add_class::<PyContentAnalysis>()?;
    m.add_class::<PyFileClassification>()?;
    m.add_class::<PyPatternDetector>()?;
    m.add_class::<PyFileClassifier>()?;
    Ok(())
}
