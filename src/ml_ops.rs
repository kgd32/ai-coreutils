//! Machine Learning Operations Module
//!
//! This module provides AI-powered pattern detection, file classification,
//! and content analysis capabilities using heuristic algorithms and statistical methods.

use crate::error::{AiCoreutilsError, Result};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// Pattern match result with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternMatch {
    /// The pattern that was matched
    pub pattern: String,
    /// The matched text
    pub matched_text: String,
    /// Start position in the text
    pub start: usize,
    /// End position in the text
    pub end: usize,
    /// Confidence score (0.0 to 1.0)
    pub confidence: f64,
    /// Pattern type/category
    pub pattern_type: PatternType,
}

/// Types of patterns that can be detected
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PatternType {
    /// Email addresses
    Email,
    /// URLs/URIs
    Url,
    /// IP addresses (IPv4)
    IpAddress,
    /// Phone numbers
    PhoneNumber,
    /// Credit card numbers
    CreditCard,
    /// Social Security Numbers
    Ssn,
    /// Dates and timestamps
    Date,
    /// Hexadecimal values
    Hex,
    /// Base64 encoded data
    Base64,
    /// JSON data
    Json,
    /// UUIDs
    Uuid,
    /// File paths
    FilePath,
    /// Code snippets
    Code,
    /// Custom pattern
    Custom(String),
}

/// File classification result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileClassification {
    /// File path
    pub path: String,
    /// Detected file type
    pub file_type: String,
    /// Confidence in classification
    pub confidence: f64,
    /// Detected encoding
    pub encoding: String,
    /// MIME type
    pub mime_type: String,
    /// Whether file appears to be binary
    pub is_binary: bool,
    /// Detected language (if text)
    pub language: Option<String>,
}

/// Content analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentAnalysis {
    /// File path analyzed
    pub path: String,
    /// Total patterns found
    pub total_patterns: usize,
    /// Patterns grouped by type
    pub patterns_by_type: HashMap<String, usize>,
    /// All pattern matches
    pub matches: Vec<PatternMatch>,
    /// Text statistics
    pub statistics: TextStatistics,
    /// Detected issues/anomalies
    pub issues: Vec<String>,
}

/// Text statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextStatistics {
    /// Total characters
    pub characters: usize,
    /// Total bytes
    pub bytes: usize,
    /// Total lines
    pub lines: usize,
    /// Total words
    pub words: usize,
    /// Average line length
    pub avg_line_length: f64,
    /// Maximum line length
    pub max_line_length: usize,
    /// Percentage of whitespace
    pub whitespace_ratio: f64,
    /// Entropy (randomness indicator)
    pub entropy: f64,
}

/// ML operations configuration
#[derive(Debug, Clone)]
pub struct MlConfig {
    /// Enable entropy analysis
    pub analyze_entropy: bool,
    /// Enable pattern detection
    pub detect_patterns: bool,
    /// Minimum confidence threshold
    pub min_confidence: f64,
    /// Maximum samples to analyze
    pub max_samples: usize,
}

impl Default for MlConfig {
    fn default() -> Self {
        Self {
            analyze_entropy: true,
            detect_patterns: true,
            min_confidence: 0.5,
            max_samples: 10000,
        }
    }
}

/// Pattern detector for various common patterns
pub struct PatternDetector {
    config: MlConfig,
    patterns: Vec<(PatternType, Regex)>,
}

impl PatternDetector {
    /// Create a new pattern detector with default patterns
    pub fn new() -> Result<Self> {
        Self::with_config(MlConfig::default())
    }

    /// Create a new pattern detector with custom configuration
    pub fn with_config(config: MlConfig) -> Result<Self> {
        let mut detector = Self {
            config: config.clone(),
            patterns: Vec::new(),
        };

        // Initialize built-in patterns
        detector.init_patterns()?;

        Ok(detector)
    }

    /// Initialize built-in regex patterns
    fn init_patterns(&mut self) -> Result<()> {
        // Email pattern
        self.patterns.push((
            PatternType::Email,
            Regex::new(
                r"(?i)\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b"
            ).map_err(|e| AiCoreutilsError::InvalidInput(format!("Invalid email regex: {}", e)))?,
        ));

        // URL pattern
        self.patterns.push((
            PatternType::Url,
            Regex::new(
                r"(?i)\b(https?://|www\.)[^\s/$.?#].[^\s]*\b"
            ).map_err(|e| AiCoreutilsError::InvalidInput(format!("Invalid URL regex: {}", e)))?,
        ));

        // IPv4 address pattern
        self.patterns.push((
            PatternType::IpAddress,
            Regex::new(
                r"\b(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\b"
            ).map_err(|e| AiCoreutilsError::InvalidInput(format!("Invalid IP regex: {}", e)))?,
        ));

        // Phone number pattern (US format)
        self.patterns.push((
            PatternType::PhoneNumber,
            Regex::new(
                r"\b(?:\+?1[-.\s]?)?\(?[0-9]{3}\)?[-.\s]?[0-9]{3}[-.\s]?[0-9]{4}\b"
            ).map_err(|e| AiCoreutilsError::InvalidInput(format!("Invalid phone regex: {}", e)))?,
        ));

        // Credit card pattern
        self.patterns.push((
            PatternType::CreditCard,
            Regex::new(
                r"\b(?:\d{4}[-\s]?){3}\d{4}\b"
            ).map_err(|e| AiCoreutilsError::InvalidInput(format!("Invalid credit card regex: {}", e)))?,
        ));

        // SSN pattern
        self.patterns.push((
            PatternType::Ssn,
            Regex::new(
                r"\b\d{3}-\d{2}-\d{4}\b"
            ).map_err(|e| AiCoreutilsError::InvalidInput(format!("Invalid SSN regex: {}", e)))?,
        ));

        // Date pattern (ISO 8601 and common formats)
        self.patterns.push((
            PatternType::Date,
            Regex::new(
                r"\b\d{4}[-/]\d{1,2}[-/]\d{1,2}\b|\b\d{1,2}[-/]\d{1,2}[-/]\d{4}\b"
            ).map_err(|e| AiCoreutilsError::InvalidInput(format!("Invalid date regex: {}", e)))?,
        ));

        // Hex pattern
        self.patterns.push((
            PatternType::Hex,
            Regex::new(
                r"\b0x[0-9A-Fa-f]+\b"
            ).map_err(|e| AiCoreutilsError::InvalidInput(format!("Invalid hex regex: {}", e)))?,
        ));

        // Base64 pattern (detect likely Base64 strings)
        self.patterns.push((
            PatternType::Base64,
            Regex::new(
                r"[A-Za-z0-9+/]{20,}={0,2}"
            ).map_err(|e| AiCoreutilsError::InvalidInput(format!("Invalid Base64 regex: {}", e)))?,
        ));

        // UUID pattern
        self.patterns.push((
            PatternType::Uuid,
            Regex::new(
                r"\b[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}\b"
            ).map_err(|e| AiCoreutilsError::InvalidInput(format!("Invalid UUID regex: {}", e)))?,
        ));

        // File path pattern
        self.patterns.push((
            PatternType::FilePath,
            Regex::new(
                r"[A-Za-z]:\\[^\s]*|/[^\s]*"
            ).map_err(|e| AiCoreutilsError::InvalidInput(format!("Invalid file path regex: {}", e)))?,
        ));

        Ok(())
    }

    /// Detect all patterns in the given text
    pub fn detect_patterns(&self, text: &str) -> Vec<PatternMatch> {
        let mut matches = Vec::new();

        for (pattern_type, regex) in &self.patterns {
            for capture in regex.find_iter(text) {
                let confidence = self.calculate_confidence(&text[capture.start()..capture.end()], pattern_type);

                if confidence >= self.config.min_confidence {
                    matches.push(PatternMatch {
                        pattern: regex.as_str().to_string(),
                        matched_text: capture.as_str().to_string(),
                        start: capture.start(),
                        end: capture.end(),
                        confidence,
                        pattern_type: pattern_type.clone(),
                    });
                }
            }
        }

        matches
    }

    /// Calculate confidence score for a pattern match
    fn calculate_confidence(&self, matched_text: &str, pattern_type: &PatternType) -> f64 {
        let mut confidence = 0.5; // Base confidence

        // Increase confidence based on pattern type and content
        match pattern_type {
            PatternType::Email => {
                if matched_text.contains('@') && matched_text.contains('.') {
                    confidence = 0.95;
                }
            }
            PatternType::Url => {
                if matched_text.starts_with("http://") || matched_text.starts_with("https://") {
                    confidence = 0.98;
                } else if matched_text.starts_with("www.") {
                    confidence = 0.85;
                }
            }
            PatternType::IpAddress => {
                confidence = 0.99; // Regex is very specific
            }
            PatternType::Uuid => {
                confidence = 0.99; // Very specific pattern
            }
            PatternType::Base64 => {
                // Higher confidence for longer strings
                if matched_text.len() >= 40 {
                    confidence = 0.9;
                } else {
                    confidence = 0.6;
                }
            }
            _ => {
                // Default confidence for other patterns
                confidence = 0.8;
            }
        }

        confidence
    }

    /// Analyze content and return detailed results
    pub fn analyze_content(&self, text: &str, path: &Path) -> Result<ContentAnalysis> {
        let statistics = self.calculate_statistics(text);

        let mut patterns_by_type = HashMap::new();
        let mut issues = Vec::new();

        let matches = if self.config.detect_patterns {
            self.detect_patterns(text)
        } else {
            Vec::new()
        };

        // Group patterns by type
        for pattern_match in &matches {
            let type_name = format!("{:?}", pattern_match.pattern_type);
            *patterns_by_type.entry(type_name).or_insert(0) += 1;
        }

        // Detect potential issues
        if statistics.entropy > 7.8 {
            issues.push("High entropy detected - file may be encrypted or compressed".to_string());
        }

        if statistics.whitespace_ratio > 0.9 {
            issues.push("Very high whitespace ratio - file may be sparse or empty".to_string());
        }

        if patterns_by_type.contains_key("Ssn") {
            issues.push("SSN patterns detected - consider data privacy".to_string());
        }

        if patterns_by_type.contains_key("CreditCard") {
            issues.push("Credit card patterns detected - consider security implications".to_string());
        }

        Ok(ContentAnalysis {
            path: path.display().to_string(),
            total_patterns: matches.len(),
            patterns_by_type,
            matches,
            statistics,
            issues,
        })
    }

    /// Calculate text statistics
    fn calculate_statistics(&self, text: &str) -> TextStatistics {
        let lines: Vec<&str> = text.lines().collect();
        let words: Vec<&str> = text.split_whitespace().collect();

        let max_line_length = lines.iter().map(|l| l.len()).max().unwrap_or(0);
        let total_line_length: usize = lines.iter().map(|l| l.len()).sum();
        let avg_line_length = if lines.is_empty() {
            0.0
        } else {
            total_line_length as f64 / lines.len() as f64
        };

        let whitespace_count = text.chars().filter(|c| c.is_whitespace()).count();
        let whitespace_ratio = if text.is_empty() {
            0.0
        } else {
            whitespace_count as f64 / text.len() as f64
        };

        let entropy = self.calculate_entropy(text);

        TextStatistics {
            characters: text.chars().count(),
            bytes: text.len(),
            lines: lines.len(),
            words: words.len(),
            avg_line_length,
            max_line_length,
            whitespace_ratio,
            entropy,
        }
    }

    /// Calculate Shannon entropy of text
    fn calculate_entropy(&self, text: &str) -> f64 {
        if text.is_empty() {
            return 0.0;
        }

        let mut char_counts = HashMap::new();
        for c in text.chars() {
            *char_counts.entry(c).or_insert(0) += 1;
        }

        let length = text.len() as f64;
        let mut entropy = 0.0;

        for &count in char_counts.values() {
            if count > 0 {
                let probability = count as f64 / length;
                entropy -= probability * probability.log2();
            }
        }

        entropy
    }
}

impl Default for PatternDetector {
    fn default() -> Self {
        Self::new().expect("Failed to create default PatternDetector")
    }
}

/// File classifier for determining file types
pub struct FileClassifier;

impl FileClassifier {
    /// Classify a file based on its extension and content
    pub fn classify(path: &Path, content: &[u8]) -> Result<FileClassification> {
        let _file_name = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");

        let extension = path.extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");

        let (file_type, mime_type, is_binary) = Self::determine_type(extension, content);

        let encoding = if is_binary {
            "binary".to_string()
        } else {
            "utf-8".to_string()
        };

        let language = if !is_binary {
            Self::detect_language(extension, content)
        } else {
            None
        };

        let confidence = Self::calculate_confidence(extension, content);

        Ok(FileClassification {
            path: path.display().to_string(),
            file_type,
            confidence,
            encoding,
            mime_type,
            is_binary,
            language,
        })
    }

    /// Determine file type based on extension and content
    fn determine_type(extension: &str, content: &[u8]) -> (String, String, bool) {
        match extension.to_lowercase().as_str() {
            "rs" => ("Rust source".to_string(), "text/x-rust".to_string(), false),
            "py" => ("Python source".to_string(), "text/x-python".to_string(), false),
            "js" => ("JavaScript source".to_string(), "text/javascript".to_string(), false),
            "ts" => ("TypeScript source".to_string(), "text/typescript".to_string(), false),
            "json" => ("JSON data".to_string(), "application/json".to_string(), false),
            "xml" => ("XML data".to_string(), "application/xml".to_string(), false),
            "yaml" | "yml" => ("YAML data".to_string(), "application/x-yaml".to_string(), false),
            "md" => ("Markdown".to_string(), "text/markdown".to_string(), false),
            "txt" => ("Plain text".to_string(), "text/plain".to_string(), false),
            "html" | "htm" => ("HTML".to_string(), "text/html".to_string(), false),
            "css" => ("CSS".to_string(), "text/css".to_string(), false),
            "csv" => ("CSV data".to_string(), "text/csv".to_string(), false),
            "toml" => ("TOML config".to_string(), "application/toml".to_string(), false),
            "bin" => ("Binary data".to_string(), "application/octet-stream".to_string(), true),
            "exe" | "dll" | "so" => ("Executable".to_string(), "application/x-executable".to_string(), true),
            "png" => ("PNG image".to_string(), "image/png".to_string(), true),
            "jpg" | "jpeg" => ("JPEG image".to_string(), "image/jpeg".to_string(), true),
            "gif" => ("GIF image".to_string(), "image/gif".to_string(), true),
            "pdf" => ("PDF document".to_string(), "application/pdf".to_string(), true),
            "zip" | "tar" | "gz" | "rar" | "7z" => ("Archive".to_string(), "application/x-archive".to_string(), true),
            _ => {
                // Try to detect from content
                if content.is_empty() {
                    ("Empty".to_string(), "text/plain".to_string(), false)
                } else if Self::is_binary_content(content) {
                    ("Binary data".to_string(), "application/octet-stream".to_string(), true)
                } else {
                    ("Text".to_string(), "text/plain".to_string(), false)
                }
            }
        }
    }

    /// Detect if content is binary
    fn is_binary_content(content: &[u8]) -> bool {
        if content.is_empty() {
            return false;
        }

        // Check first 1000 bytes for binary indicators
        let sample_size = 1000.min(content.len());
        let null_count = content[..sample_size].iter().filter(|&&b| b == 0).count();

        // If more than 1% null bytes, likely binary
        if null_count > sample_size / 100 {
            return true;
        }

        // Check for non-printable characters
        let non_printable = content[..sample_size]
            .iter()
            .filter(|&&b| b < 0x20 && b != b'\t' as u8 && b != b'\n' as u8 && b != b'\r' as u8)
            .count();

        non_printable > sample_size / 20
    }

    /// Detect programming language
    fn detect_language(extension: &str, content: &[u8]) -> Option<String> {
        if extension.is_empty() && content.is_empty() {
            return None;
        }

        Some(match extension.to_lowercase().as_str() {
            "rs" => "rust",
            "py" => "python",
            "js" => "javascript",
            "ts" => "typescript",
            "go" => "go",
            "java" => "java",
            "c" => "c",
            "cpp" | "cc" | "cxx" => "c++",
            "h" | "hpp" => "c/c++",
            "cs" => "c#",
            "php" => "php",
            "rb" => "ruby",
            "sh" => "shell",
            "sql" => "sql",
            "r" => "r",
            "scala" => "scala",
            "kt" => "kotlin",
            "swift" => "swift",
            "lua" => "lua",
            "pl" => "perl",
            _ => {
                // Try to detect from shebang
                if content.starts_with(b"#!/") {
                    let first_line = content.iter()
                        .take_while(|&&b| b != b'\n')
                        .map(|&b| b as char)
                        .collect::<String>();

                    if first_line.contains("bash") || first_line.contains("sh") {
                        return Some("shell".to_string());
                    } else if first_line.contains("python") {
                        return Some("python".to_string());
                    } else if first_line.contains("perl") {
                        return Some("perl".to_string());
                    }
                }
                "unknown"
            }
        }.to_string())
    }

    /// Calculate classification confidence
    fn calculate_confidence(extension: &str, content: &[u8]) -> f64 {
        let mut confidence: f64 = 0.5;

        // Higher confidence for known extensions
        if !extension.is_empty() {
            confidence = 0.9;
        }

        // Adjust based on content
        if !content.is_empty() {
            confidence += 0.05;
        }

        confidence.min(1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_detection_email() {
        let detector = PatternDetector::new().unwrap();
        let text = "Contact us at support@example.com or admin@test.org for help.";
        let matches = detector.detect_patterns(text);

        assert!(!matches.is_empty());
        assert_eq!(matches[0].pattern_type, PatternType::Email);
    }

    #[test]
    fn test_pattern_detection_url() {
        let detector = PatternDetector::new().unwrap();
        let text = "Visit https://example.com or www.test.org";
        let matches = detector.detect_patterns(text);

        assert!(!matches.is_empty());
        assert_eq!(matches[0].pattern_type, PatternType::Url);
    }

    #[test]
    fn test_pattern_detection_ip() {
        let detector = PatternDetector::new().unwrap();
        let text = "Server at 192.168.1.1 is online";
        let matches = detector.detect_patterns(text);

        assert!(!matches.is_empty());
        assert_eq!(matches[0].pattern_type, PatternType::IpAddress);
    }

    #[test]
    fn test_pattern_detection_uuid() {
        let detector = PatternDetector::new().unwrap();
        let text = "ID: 550e8400-e29b-41d4-a716-446655440000";
        let matches = detector.detect_patterns(text);

        assert!(!matches.is_empty());
        assert_eq!(matches[0].pattern_type, PatternType::Uuid);
    }

    #[test]
    fn test_content_analysis() {
        let detector = PatternDetector::new().unwrap();
        let text = "Hello world\nThis is a test\nContact: test@example.com\n";
        let path = Path::new("test.txt");

        let analysis = detector.analyze_content(text, path).unwrap();

        assert_eq!(analysis.statistics.lines, 3);
        // Email is counted as 1 word, not 2
        assert_eq!(analysis.statistics.words, 8);
        assert!(analysis.total_patterns > 0);
    }

    #[test]
    fn test_text_statistics() {
        let detector = PatternDetector::new().unwrap();
        let text = "Hello world\nTest line";

        let stats = detector.calculate_statistics(text);

        assert_eq!(stats.lines, 2);
        assert_eq!(stats.words, 4);
        assert!(stats.avg_line_length > 0.0);
    }

    #[test]
    fn test_entropy_calculation() {
        let detector = PatternDetector::new().unwrap();

        // Low entropy (repeated characters)
        let low_entropy = detector.calculate_entropy("aaaaa");
        assert!(low_entropy < 1.0);

        // High entropy (random characters)
        let high_entropy = detector.calculate_entropy("aBcDeFgH");
        assert!(high_entropy > low_entropy);
    }

    #[test]
    fn test_file_classification_text() {
        let content = b"Hello, world!";
        let path = Path::new("test.txt");

        let classification = FileClassifier::classify(path, content).unwrap();

        assert_eq!(classification.file_type, "Plain text");
        assert!(!classification.is_binary);
        assert!(classification.confidence > 0.8);
    }

    #[test]
    fn test_file_classification_code() {
        let content = b"fn main() { println!(\"Hello\"); }";
        let path = Path::new("test.rs");

        let classification = FileClassifier::classify(path, content).unwrap();

        assert_eq!(classification.file_type, "Rust source");
        assert_eq!(classification.language, Some("rust".to_string()));
        assert!(!classification.is_binary);
    }

    #[test]
    fn test_is_binary_content() {
        // Text content
        assert!(!FileClassifier::is_binary_content(b"Hello, world!"));

        // Binary content (null bytes)
        assert!(FileClassifier::is_binary_content(b"Hello\x00world"));

        // Binary content (many non-printable characters)
        let binary_data: Vec<u8> = (0..100).map(|i: u32| i.wrapping_mul(3) as u8).collect();
        assert!(FileClassifier::is_binary_content(&binary_data));
    }
}
