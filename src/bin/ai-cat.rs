//! AI-optimized cat utility
//!
//! Concatenates and displays file contents with memory mapping and JSONL output.

use ai_coreutils::{jsonl::JsonlRecord, memory::SafeMemoryAccess, Result};
use clap::Parser;
use std::path::PathBuf;

/// AI-optimized cat: Concatenate files with JSONL output
#[derive(Parser, Debug)]
#[command(name = "ai-cat")]
#[command(about = "AI-optimized cat with memory mapping and JSONL output", long_about = None)]
struct Cli {
    /// Files to concatenate
    #[arg(required = true)]
    files: Vec<PathBuf>,

    /// Show line numbers
    #[arg(short, long)]
    number: bool,

    /// Show memory pointer (for AI agent memory access)
    #[arg(short = 'p', long)]
    mem_ptr: bool,

    /// Output JSONL (always enabled for AI agents)
    #[arg(short, long, default_value_t = true)]
    json: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    for file in &cli.files {
        if let Err(e) = cat_file(file, &cli) {
            let error_record = JsonlRecord::error(
                format!("Failed to read {}: {}", file.display(), e),
                "CAT_ERROR"
            );
            println!("{}", error_record.to_jsonl()?);
        }
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

    let ptr = mem_access.as_ptr();

    let record = JsonlRecord::result(serde_json::json!({
        "file": path.display().to_string(),
        "content": content,
        "size": mem_access.size(),
        "memory_pointer": if cli.mem_ptr { Some(format!("{:?}", ptr)) } else { None },
    }));

    println!("{}", record.to_jsonl()?);

    Ok(())
}
