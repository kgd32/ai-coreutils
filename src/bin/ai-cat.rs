//! AI-optimized cat utility
//!
//! Concatenates and displays file contents with memory mapping and JSONL output.
//! Supports async processing for multiple files.

use ai_coreutils::{
    async_ops::{async_read_file, AsyncConfig},
    jsonl::JsonlRecord,
    memory::SafeMemoryAccess,
    Result,
};
use clap::Parser;
use std::path::PathBuf;

/// AI-optimized cat: Concatenate files with JSONL output
#[derive(Parser, Debug, Clone)]
#[command(name = "ai-cat")]
#[command(about = "AI-optimized cat with memory mapping and JSONL output", long_about = None)]
struct Cli {
    /// Files to concatenate
    #[arg(required = true)]
    files: Vec<PathBuf>,

    /// Number all output lines
    #[arg(short, long)]
    number: bool,

    /// Number non-blank lines
    #[arg(short = 'b', long)]
    number_nonblank: bool,

    /// Show all characters (including non-printing)
    #[arg(short = 'A', long)]
    show_all: bool,

    /// Show end of lines as $
    #[arg(short, long)]
    show_ends: bool,

    /// Show tabs as ^I
    #[arg(short = 'T', long)]
    show_tabs: bool,

    /// Squeeze multiple blank lines
    #[arg(short, long)]
    squeeze_blank: bool,

    /// Show memory pointer (for AI agent memory access)
    #[arg(short = 'p', long)]
    mem_ptr: bool,

    /// Enable async processing for multiple files
    #[arg(short = 'a', long)]
    async_mode: bool,

    /// Maximum concurrent operations in async mode
    #[arg(short = 'j', long, default_value_t = 10)]
    max_concurrent: usize,

    /// Output JSONL (always enabled for AI-Coreutils agents)
    #[arg(short, long, default_value_t = true)]
    json: bool,
}

#[derive(Debug, Clone)]
struct LineInfo {
    content: String,
    line_number: Option<usize>,
    non_blank_number: Option<usize>,
    is_blank: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.async_mode && cli.files.len() > 1 {
        // Use async runtime for concurrent file processing
        let rt = tokio::runtime::Runtime::new()?;
        rt.block_on(async_main(cli))
    } else {
        // Use synchronous processing
        sync_main(cli)
    }
}

fn sync_main(cli: Cli) -> Result<()> {
    for file in &cli.files {
        if let Err(e) = cat_file(file, &cli) {
            let error_record =
                JsonlRecord::error(format!("Failed to read {}: {}", file.display(), e), "CAT_ERROR");
            println!("{}", error_record.to_jsonl()?);
        }
    }

    Ok(())
}

async fn async_main(cli: Cli) -> Result<()> {
    use futures::stream::{self, StreamExt};

    let config = AsyncConfig {
        max_concurrent: cli.max_concurrent,
        buffer_size: 8192,
        progress: false,
    };

    let files = cli.files.clone();

    // Process files concurrently
    let results = stream::iter(files)
        .map(|file| {
            let cli = cli.clone();
            async move {
                let result = async_cat_file(&file, &cli).await;
                (file, result)
            }
        })
        .buffer_unordered(config.max_concurrent)
        .collect::<Vec<_>>()
        .await;

    // Report results
    for (path, result) in results {
        if let Err(e) = result {
            let error_record = JsonlRecord::error(
                format!("Failed to read {}: {}", path.display(), e),
                "CAT_ERROR",
            );
            println!("{}", error_record.to_jsonl()?);
        }
    }

    Ok(())
}

async fn async_cat_file(path: &PathBuf, cli: &Cli) -> Result<()> {
    // Read file asynchronously
    let data = async_read_file(path).await?;
    let content = String::from_utf8_lossy(&data).to_string();

    let lines: Vec<&str> = content.lines().collect();
    let mut line_infos = Vec::new();

    let squeeze_blank = cli.squeeze_blank;
    let mut last_was_blank = false;
    let mut non_blank_count = 0;

    for (idx, line) in lines.iter().enumerate() {
        let is_blank = line.is_empty();

        // Skip squeezed blanks
        if squeeze_blank && is_blank && last_was_blank {
            continue;
        }

        let line_info = if cli.number_nonblank {
            if is_blank {
                LineInfo {
                    content: String::new(),
                    line_number: None,
                    non_blank_number: None,
                    is_blank: true,
                }
            } else {
                non_blank_count += 1;
                LineInfo {
                    content: line.to_string(),
                    line_number: None,
                    non_blank_number: Some(non_blank_count),
                    is_blank: false,
                }
            }
        } else if cli.number {
            LineInfo {
                content: line.to_string(),
                line_number: Some(idx + 1),
                non_blank_number: None,
                is_blank: false,
            }
        } else if cli.show_all {
            // Convert all characters to visible representation
            let all_chars: String = line
                .chars()
                .map(|c| match c {
                    '\t' => "^I".to_string(),
                    '\n' => "$".to_string(),
                    c if c.is_control() => format!("^{}", c as u32),
                    c => c.to_string(),
                })
                .collect();
            LineInfo {
                content: all_chars,
                line_number: None,
                non_blank_number: None,
                is_blank: false,
            }
        } else if cli.show_ends {
            // Show $ at end of each line
            let with_ends = format!("{}$", line);
            LineInfo {
                content: with_ends,
                line_number: None,
                non_blank_number: None,
                is_blank: false,
            }
        } else if cli.show_tabs {
            // Show tabs as ^I
            let with_tabs = line.replace('\t', "^I");
            LineInfo {
                content: with_tabs,
                line_number: None,
                non_blank_number: None,
                is_blank: false,
            }
        } else {
            LineInfo {
                content: line.to_string(),
                line_number: None,
                non_blank_number: None,
                is_blank: false,
            }
        };

        if !is_blank || !squeeze_blank {
            line_infos.push(line_info);
        }

        last_was_blank = is_blank;
    }

    // Output all lines
    let line_count = line_infos.len();

    for line_info in &line_infos {
        let record = JsonlRecord::result(serde_json::json!({
            "type": "file_content",
            "file": path.display().to_string(),
            "content": line_info.content,
            "line_number": line_info.line_number,
            "line_non_blank_number": line_info.non_blank_number,
            "is_blank": line_info.is_blank,
            "line_count": line_count,
        }));

        println!("{}", record.to_jsonl()?);
    }

    // If only one file and no special formatting, output a summary record
    if cli.files.len() == 1
        && !cli.number
        && !cli.number_nonblank
        && !cli.show_all
        && !cli.show_ends
        && !cli.show_tabs
    {
        let record = JsonlRecord::result(serde_json::json!({
            "type": "file_summary",
            "file": path.display().to_string(),
            "content": content,
            "size": data.len(),
        }));

        println!("{}", record.to_jsonl()?);
    }

    Ok(())
}

fn cat_file(path: &PathBuf, cli: &Cli) -> Result<()> {
    // Use memory mapping for efficient file reading
    let mem_access = SafeMemoryAccess::new(path)?;

    let content = if let Some(data) = mem_access.get(0, mem_access.size()) {
        String::from_utf8_lossy(data).to_string()
    } else {
        return Ok(());
    };

    let lines: Vec<&str> = content.lines().collect();
    let mut line_infos = Vec::new();

    let squeeze_blank = cli.squeeze_blank;
    let mut last_was_blank = false;
    let mut non_blank_count = 0;

    for (idx, line) in lines.iter().enumerate() {
        let is_blank = line.is_empty();

        // Skip squeezed blanks
        if squeeze_blank && is_blank && last_was_blank {
            continue;
        }

        let line_info = if cli.number_nonblank {
            if is_blank {
                LineInfo {
                    content: String::new(),
                    line_number: None,
                    non_blank_number: None,
                    is_blank: true,
                }
            } else {
                non_blank_count += 1;
                LineInfo {
                    content: line.to_string(),
                    line_number: None,
                    non_blank_number: Some(non_blank_count),
                    is_blank: false,
                }
            }
        } else if cli.number {
            LineInfo {
                content: line.to_string(),
                line_number: Some(idx + 1),
                non_blank_number: None,
                is_blank: false,
            }
        } else if cli.show_all {
            // Convert all characters to visible representation
            let all_chars: String = line
                .chars()
                .map(|c| match c {
                    '\t' => "^I".to_string(),
                    '\n' => "$".to_string(),
                    c if c.is_control() => format!("^{}", c as u32),
                    c => c.to_string(),
                })
                .collect();
            LineInfo {
                content: all_chars,
                line_number: None,
                non_blank_number: None,
                is_blank: false,
            }
        } else if cli.show_ends {
            // Show $ at end of each line
            let with_ends = format!("{}$", line);
            LineInfo {
                content: with_ends,
                line_number: None,
                non_blank_number: None,
                is_blank: false,
            }
        } else if cli.show_tabs {
            // Show tabs as ^I
            let with_tabs = line.replace('\t', "^I");
            LineInfo {
                content: with_tabs,
                line_number: None,
                non_blank_number: None,
                is_blank: false,
            }
        } else {
            LineInfo {
                content: line.to_string(),
                line_number: None,
                non_blank_number: None,
                is_blank: false,
            }
        };

        if !is_blank || !squeeze_blank {
            line_infos.push(line_info);
        }

        last_was_blank = is_blank;
    }

    // Output all lines
    let line_count = line_infos.len();

    for line_info in &line_infos {
        let record = JsonlRecord::result(serde_json::json!({
            "type": "file_content",
            "file": path.display().to_string(),
            "content": line_info.content,
            "line_number": line_info.line_number,
            "line_non_blank_number": line_info.non_blank_number,
            "is_blank": line_info.is_blank,
            "line_count": line_count,
        }));

        println!("{}", record.to_jsonl()?);
    }

    // If only one file and no special formatting, output a summary record
    if cli.files.len() == 1
        && !cli.number
        && !cli.number_nonblank
        && !cli.show_all
        && !cli.show_ends
        && !cli.show_tabs
    {
        let ptr = mem_access.as_ptr();
        let size = mem_access.size();

        let record = JsonlRecord::result(serde_json::json!({
            "type": "file_summary",
            "file": path.display().to_string(),
            "content": content,
            "size": size,
            "memory_pointer": if cli.mem_ptr { Some(format!("{:?}", ptr)) } else { None },
        }));

        println!("{}", record.to_jsonl()?);
    }

    Ok(())
}
