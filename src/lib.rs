//! AI-Coreutils: Modern core utilities for AI agents
//!
//! This library provides AI-optimized implementations of GNU core utilities
//! with structured JSONL output and safe memory pointer access.

#![warn(missing_docs)]
#![warn(clippy::all)]

pub mod async_ops;
pub mod error;
pub mod jsonl;
pub mod memory;
pub mod fs_utils;
pub mod simd_ops;
pub mod ml_ops;

// Python bindings (optional)
#[cfg(feature = "python")]
pub mod python;

// Re-export commonly used types
pub use error::{AiCoreutilsError, Result};
pub use jsonl::{JsonlOutput, JsonlRecord};
pub use memory::SafeMemoryAccess;
pub use simd_ops::{SimdConfig, SimdPatternSearcher, SimdByteCounter, SimdTextProcessor, TextMetrics, SimdNewlineCounter, SimdMemoryOps, SimdHasher, SimdEntropyCalculator, SimdWhitespaceDetector, SimdCaseFolder, SimdUtf8Validator, SimdStringComparer, SimdMultiPatternSearcher};
pub use ml_ops::{PatternDetector, FileClassifier, MlConfig, PatternType, ContentAnalysis, FileClassification};
