use clap::{Parser, ValueEnum};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
#[command(author, version = VERSION, about = "A high-performance, modular DEX parser written in Rust", long_about = None)]
pub struct Cli {
    /// Path to the DEX/APK file
    pub path: String,

    /// Output format
    #[arg(short, long, value_enum, default_value_t = OutputFormat::Text)]
    pub format: OutputFormat,

    /// Exclude instructions from output
    #[arg(long)]
    pub no_instructions: bool,

    /// Exclude analysis results from output
    #[arg(long)]
    pub no_analysis: bool,

    /// Include full metadata (strings, types, methods) in output
    #[arg(long)]
    pub include_metadata: bool,

    /// Path to a custom analysis configuration file (JSON)
    #[arg(short, long)]
    pub config: Option<String>,

    /// Generate a Graphviz DOT file for the call graph
    #[arg(long)]
    pub call_graph: Option<String>,

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
    /// Interactive HTML Dashboard
    Html,
}
