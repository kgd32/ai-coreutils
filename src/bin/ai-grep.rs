//! AI-optimized grep utility
//!
//! Searches for patterns in files with structured JSONL output.
//! Supports async concurrent file processing.

use ai_coreutils::{
    async_ops::{async_grep_file, async_walk_dir, AsyncConfig},
    jsonl::JsonlRecord,
    memory::SafeMemoryAccess,
    Result,
};
use clap::Parser;
use futures::stream::{self, StreamExt};
use std::path::PathBuf;
use walkdir::WalkDir;

/// AI-optimized grep: Search files with JSONL output
#[derive(Parser, Debug, Clone)]
#[command(name = "ai-grep")]
#[command(about = "AI-optimized grep with structured output", long_about = None)]
struct Cli {
    /// Pattern to search for
    pattern: String,

    /// Files/directories to search
    #[arg(required = true)]
    paths: Vec<PathBuf>,

    /// Recursive directory search
    #[arg(short, long)]
    recursive: bool,

    /// Enable async concurrent file processing
    #[arg(short = 'a', long)]
    async_mode: bool,

    /// Maximum concurrent operations in async mode
    #[arg(short = 'j', long, default_value_t = 10)]
    max_concurrent: usize,

    /// Show line numbers
    #[arg(short = 'n', long)]
    line_number: bool,

    /// Show count of matches
    #[arg(short = 'c', long)]
    count: bool,

    /// Case insensitive search
    #[arg(short, long)]
    ignore_case: bool,

    /// Invert match (show non-matching lines)
    #[arg(short = 'v', long)]
    invert_match: bool,

    /// List matching files only
    #[arg(short = 'l', long)]
    files_with_matches: bool,

    /// List non-matching files only
    #[arg(short = 'L', long)]
    files_without_match: bool,

    /// Show only matching part
    #[arg(short = 'o', long)]
    only_matching: bool,

    /// Fixed strings (not regex)
    #[arg(short = 'F', long)]
    fixed_strings: bool,

    /// Extended regex
    #[arg(short = 'E', long)]
    extended_regex: bool,

    /// Context: show NUM lines after match
    #[arg(short = 'A', long, value_name = "NUM")]
    after_context: Option<usize>,

    /// Context: show NUM lines before match
    #[arg(short = 'B', long, value_name = "NUM")]
    before_context: Option<usize>,

    /// Context: show NUM lines around match
    #[arg(short = 'C', long, value_name = "NUM")]
    context: Option<usize>,

    /// Output JSONL (always enabled for AI agents)
    #[arg(short, long, default_value_t = true)]
    json: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Determine if we should use async mode
    let use_async = cli.async_mode && (cli.recursive || cli.paths.len() > 1);

    if use_async {
        let rt = tokio::runtime::Runtime::new()?;
        rt.block_on(async_main(cli))
    } else {
        sync_main(cli)
    }
}

fn sync_main(cli: Cli) -> Result<()> {
    for path in &cli.paths {
        if path.is_dir() {
            if cli.recursive {
                if let Err(e) = grep_directory(path, &cli) {
                    let error_record = JsonlRecord::error(
                        format!("Failed to search directory {}: {}", path.display(), e),
                        "GREP_ERROR",
                    );
                    println!("{}", error_record.to_jsonl()?);
                }
            } else {
                let error_record = JsonlRecord::error(
                    format!("{} is a directory (use -r for recursive search)", path.display()),
                    "GREP_ERROR",
                );
                println!("{}", error_record.to_jsonl()?);
            }
        } else if let Err(e) = grep_file(path, &cli) {
            let error_record = JsonlRecord::error(
                format!("Failed to search {}: {}", path.display(), e),
                "GREP_ERROR",
            );
            println!("{}", error_record.to_jsonl()?);
        }
    }

    Ok(())
}

async fn async_main(cli: Cli) -> Result<()> {
    let config = AsyncConfig {
        max_concurrent: cli.max_concurrent,
        buffer_size: 8192,
        progress: false,
    };

    // Collect all files to search
    let mut all_files = Vec::new();

    for path in &cli.paths {
        if path.is_dir() && cli.recursive {
            // Use async directory walking
            let dir_files = async_walk_dir(path).await?;
            all_files.extend(dir_files);
        } else if path.is_file() {
            all_files.push(path.clone());
        }
    }

    // Process files concurrently
    let pattern = cli.pattern.clone();
    let case_insensitive = cli.ignore_case;
    let invert_match = cli.invert_match;

    let results = stream::iter(all_files)
        .map(|file| {
            let pattern = pattern.clone();
            async move {
                let matches = async_grep_file(&file, &pattern, case_insensitive, invert_match)
                    .await
                    .unwrap_or_default();
                (file, matches)
            }
        })
        .buffer_unordered(config.max_concurrent)
        .collect::<Vec<_>>()
        .await;

    // Output results
    for (path, matches) in results {
        for m in matches {
            let record = JsonlRecord::MatchRecord {
                timestamp: chrono::Utc::now(),
                file: path.display().to_string(),
                line_number: m.line_number,
                line_content: m.line,
                match_start: 0,
                match_end: 0,
            };
            println!("{}", record.to_jsonl()?);
        }
    }

    Ok(())
}

fn grep_file(path: &PathBuf, cli: &Cli) -> Result<bool> {
    // Use memory mapping for efficient searching
    let mem_access = SafeMemoryAccess::new(path)?;

    let content = if let Some(data) = mem_access.get(0, mem_access.size()) {
        String::from_utf8_lossy(data).to_string()
    } else {
        return Ok(false);
    };

    let search_pattern = if cli.ignore_case {
        cli.pattern.to_lowercase()
    } else {
        cli.pattern.clone()
    };

    let mut match_count = 0;
    let mut has_match = false;
    let lines: Vec<&str> = content.lines().collect();

    for (line_num, line) in lines.iter().enumerate() {
        let search_line = if cli.ignore_case {
            line.to_lowercase()
        } else {
            line.to_string()
        };

        let line_matches = search_line.contains(&search_pattern);
        let should_show = if cli.invert_match {
            !line_matches
        } else {
            line_matches
        };

        if should_show && line_matches {
            match_count += 1;
            has_match = true;
        }

        if should_show {
            if cli.files_with_matches {
                // Just mark that we found a match, will output at end
                if line_matches {
                    has_match = true;
                }
                continue;
            }

            if cli.files_without_match {
                continue;
            }

            if cli.count {
                continue;
            }

            // Output the match
            if line_matches {
                let match_start = search_line.find(&search_pattern).unwrap_or(0);
                let match_end = match_start + search_pattern.len();

                if cli.only_matching {
                    // Output only the matching part
                    let record = JsonlRecord::MatchRecord {
                        timestamp: chrono::Utc::now(),
                        file: path.display().to_string(),
                        line_number: line_num + 1,
                        line_content: line[match_start..match_end].to_string(),
                        match_start: 0,
                        match_end: match_end - match_start,
                    };
                    println!("{}", record.to_jsonl()?);
                } else {
                    let output_line = if cli.line_number {
                        format!("{}:{}", line_num + 1, line)
                    } else {
                        line.to_string()
                    };

                    let record = JsonlRecord::MatchRecord {
                        timestamp: chrono::Utc::now(),
                        file: path.display().to_string(),
                        line_number: if cli.line_number {
                            line_num + 1
                        } else {
                            0
                        },
                        line_content: output_line,
                        match_start,
                        match_end,
                    };

                    println!("{}", record.to_jsonl()?);

                    // Handle context
                    let after = cli.after_context.or(cli.context).unwrap_or(0);
                    let before = cli.before_context.or(cli.context).unwrap_or(0);

                    // Output context before
                    if before > 0 && line_num > 0 {
                        let start = line_num.saturating_sub(before);
                        for ctx_line in lines[start..line_num].iter() {
                            let record = JsonlRecord::MatchRecord {
                                timestamp: chrono::Utc::now(),
                                file: path.display().to_string(),
                                line_number: 0,
                                line_content: ctx_line.to_string(),
                                match_start: 0,
                                match_end: 0,
                            };
                            println!("{}", record.to_jsonl()?);
                        }
                    }

                    // Output context after
                    if after > 0 && line_num + after < lines.len() {
                        let end = if line_num + after + 1 < lines.len() {
                            line_num + after + 1
                        } else {
                            lines.len()
                        };
                        for ctx_line in lines[line_num + 1..end].iter() {
                            let record = JsonlRecord::MatchRecord {
                                timestamp: chrono::Utc::now(),
                                file: path.display().to_string(),
                                line_number: 0,
                                line_content: ctx_line.to_string(),
                                match_start: 0,
                                match_end: 0,
                            };
                            println!("{}", record.to_jsonl()?);
                        }
                    }
                }
            } else if cli.invert_match {
                // Show non-matching lines
                let record = JsonlRecord::MatchRecord {
                    timestamp: chrono::Utc::now(),
                    file: path.display().to_string(),
                    line_number: line_num + 1,
                    line_content: line.to_string(),
                    match_start: 0,
                    match_end: 0,
                };
                println!("{}", record.to_jsonl()?);
            }
        }
    }

    // Handle file-listing modes
    if cli.files_with_matches && has_match {
        let record = JsonlRecord::result(serde_json::json!({
            "file": path.display().to_string(),
        }));
        println!("{}", record.to_jsonl()?);
    }

    if cli.files_without_match && !has_match {
        let record = JsonlRecord::result(serde_json::json!({
            "file": path.display().to_string(),
            "matches": false,
        }));
        println!("{}", record.to_jsonl()?);
    }

    if cli.count {
        let record = JsonlRecord::result(serde_json::json!({
            "file": path.display().to_string(),
            "match_count": match_count,
        }));
        println!("{}", record.to_jsonl()?);
    }

    Ok(has_match)
}

fn grep_directory(dir: &PathBuf, cli: &Cli) -> Result<()> {
    let walker = WalkDir::new(dir).follow_links(true).into_iter();

    for entry in walker.filter_map(|e| e.ok()) {
        let path = entry.path();

        if path.is_file() {
            if let Err(e) = grep_file(&path.to_path_buf(), cli) {
                let error_record = JsonlRecord::error(
                    format!("Failed to search {}: {}", path.display(), e),
                    "GREP_ERROR",
                );
                println!("{}", error_record.to_jsonl()?);
            }
        }
    }

    Ok(())
}
