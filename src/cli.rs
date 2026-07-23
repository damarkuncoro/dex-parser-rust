use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(author, version, about = "A high-performance, modular DEX parser written in Rust", long_about = None)]
pub struct Cli {
    /// Path to the DEX file
    pub path: String,

    /// Output format
    #[arg(short, long, value_enum, default_value_t = OutputFormat::Text)]
    pub format: OutputFormat,

    /// Enable verbose logging
    #[arg(short, long)]
    pub verbose: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum OutputFormat {
    /// Traditional dexdump-like text output
    Text,
    /// Machine-readable JSON output
    Json,
}
