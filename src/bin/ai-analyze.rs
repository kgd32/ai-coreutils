//! AI-Analyze: Intelligent file analysis utility
//!
//! Provides AI-powered pattern detection, file classification, and content analysis.

use ai_coreutils::error::Result;
use ai_coreutils::jsonl;
use ai_coreutils::ml_ops::{FileClassifier, MlConfig, PatternDetector};
use clap::Parser;
use std::fs;
use std::path::PathBuf;

/// AI-powered file analysis utility with pattern detection and classification
#[derive(Parser, Debug)]
#[command(name = "ai-analyze")]
#[command(about = "AI-powered file analysis with pattern detection and classification", long_about = None)]
struct Cli {
    /// Files or directories to analyze
    files: Vec<PathBuf>,

    /// Enable pattern detection
    #[arg(short = 'p', long, default_value_t = true)]
    patterns: bool,

    /// Enable file classification
    #[arg(short = 'c', long, default_value_t = true)]
    classify: bool,

    /// Show detailed statistics
    #[arg(short = 's', long)]
    statistics: bool,

    /// Detect specific pattern types (comma-separated: email,url,ip,phone,ssn,creditcard,uuid,date,hex,base64)
    #[arg(short = 't', long)]
    pattern_types: Option<String>,

    /// Minimum confidence threshold (0.0 to 1.0)
    #[arg(short = 'm', long, default_value_t = 0.5)]
    min_confidence: f64,

    /// Recursive directory analysis
    #[arg(short = 'r', long)]
    recursive: bool,

    /// Output results in JSONL format
    #[arg(short = 'j', long, default_value_t = true)]
    jsonl: bool,

    /// Verbose output
    #[arg(short = 'v', long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Validate confidence threshold
    if cli.min_confidence < 0.0 || cli.min_confidence > 1.0 {
        jsonl::output_error(
            "Confidence threshold must be between 0.0 and 1.0",
            "INVALID_ARGUMENT",
            None,
        )?;
        std::process::exit(1);
    }

    let config = MlConfig {
        analyze_entropy: cli.statistics,
        detect_patterns: cli.patterns,
        min_confidence: cli.min_confidence,
        max_samples: 10000,
    };

    let detector = PatternDetector::with_config(config)?;

    // Process each input file/directory
    for file_path in &cli.files {
        if file_path.is_dir() {
            if cli.recursive {
                analyze_directory_recursive(&detector, &cli, file_path)?;
            } else {
                jsonl::output_error(
                    &format!("{} is a directory (use -r for recursive)", file_path.display()),
                    "IS_DIRECTORY",
                    Some(file_path.display().to_string().as_str()),
                )?;
            }
        } else if file_path.exists() {
            analyze_file(&detector, &cli, file_path)?;
        } else {
            jsonl::output_error(
                &format!("File not found: {}", file_path.display()),
                "FILE_NOT_FOUND",
                Some(file_path.display().to_string().as_str()),
            )?;
        }
    }

    Ok(())
}

fn analyze_file(detector: &PatternDetector, cli: &Cli, file_path: &PathBuf) -> Result<()> {
    if cli.verbose {
        jsonl::output_info(serde_json::json!({
            "file": file_path.display().to_string(),
            "operation": "analyze",
            "status": "starting",
        }))?;
    }

    // Read file content
    let content = fs::read(file_path)
        .map_err(ai_coreutils::error::AiCoreutilsError::Io)?;

    // Classify file
    if cli.classify {
        let classification = FileClassifier::classify(file_path, &content)?;

        if cli.jsonl {
            jsonl::output_result(serde_json::json!({
                "type": "classification",
                "file": file_path.display().to_string(),
                "file_type": classification.file_type,
                "mime_type": classification.mime_type,
                "encoding": classification.encoding,
                "is_binary": classification.is_binary,
                "language": classification.language,
                "confidence": classification.confidence,
            }))?;
        }
    }

    // Analyze content for patterns
    if cli.patterns {
        let text = String::from_utf8_lossy(&content);
        let analysis = detector.analyze_content(&text, file_path)?;

        if cli.jsonl {
            jsonl::output_result(serde_json::json!({
                "type": "analysis",
                "file": file_path.display().to_string(),
                "total_patterns": analysis.total_patterns,
                "patterns_by_type": analysis.patterns_by_type,
                "statistics": {
                    "lines": analysis.statistics.lines,
                    "words": analysis.statistics.words,
                    "characters": analysis.statistics.characters,
                    "bytes": analysis.statistics.bytes,
                    "avg_line_length": analysis.statistics.avg_line_length,
                    "max_line_length": analysis.statistics.max_line_length,
                    "whitespace_ratio": analysis.statistics.whitespace_ratio,
                    "entropy": analysis.statistics.entropy,
                },
                "issues": analysis.issues,
            }))?;

            // Output individual pattern matches if verbose
            if cli.verbose && !analysis.matches.is_empty() {
                for pattern_match in analysis.matches.iter().take(100) {
                    jsonl::output_result(serde_json::json!({
                        "type": "pattern_match",
                        "file": file_path.display().to_string(),
                        "pattern_type": format!("{:?}", pattern_match.pattern_type),
                        "matched_text": pattern_match.matched_text,
                        "position": {
                            "start": pattern_match.start,
                            "end": pattern_match.end,
                        },
                        "confidence": pattern_match.confidence,
                    }))?;
                }
            }
        }

        // Human-readable output if not JSONL
        if !cli.jsonl {
            println!("File: {}", file_path.display());
            println!("Total patterns found: {}", analysis.total_patterns);

            if !analysis.patterns_by_type.is_empty() {
                println!("\nPatterns by type:");
                for (pattern_type, count) in &analysis.patterns_by_type {
                    println!("  {}: {}", pattern_type, count);
                }
            }

            if cli.statistics {
                println!("\nStatistics:");
                println!("  Lines: {}", analysis.statistics.lines);
                println!("  Words: {}", analysis.statistics.words);
                println!("  Characters: {}", analysis.statistics.characters);
                println!("  Bytes: {}", analysis.statistics.bytes);
                println!(
                    "  Avg line length: {:.2}",
                    analysis.statistics.avg_line_length
                );
                println!(
                    "  Max line length: {}",
                    analysis.statistics.max_line_length
                );
                println!(
                    "  Whitespace ratio: {:.2}%",
                    analysis.statistics.whitespace_ratio * 100.0
                );
                println!("  Entropy: {:.4}", analysis.statistics.entropy);
            }

            if !analysis.issues.is_empty() {
                println!("\nIssues detected:");
                for issue in &analysis.issues {
                    println!("  ⚠️  {}", issue);
                }
            }

            println!();
        }
    }

    if cli.verbose {
        jsonl::output_info(serde_json::json!({
            "file": file_path.display().to_string(),
            "operation": "analyze",
            "status": "complete",
        }))?;
    }

    Ok(())
}

fn analyze_directory_recursive(
    detector: &PatternDetector,
    cli: &Cli,
    dir_path: &PathBuf,
) -> Result<()> {
    use walkdir::WalkDir;

    let walker = WalkDir::new(dir_path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok());

    for entry in walker {
        let path = entry.path();

        if path.is_file() {
            if let Err(e) = analyze_file(detector, cli, &path.to_path_buf()) {
                jsonl::output_error(
                    &format!("Failed to analyze {}: {}", path.display(), e),
                    "ANALYSIS_FAILED",
                    Some(path.display().to_string().as_str()),
                )?;
            }
        }
    }

    Ok(())
}
