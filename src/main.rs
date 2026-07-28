mod cli;

use crate::cli::{Cli, OutputFormat};
use clap::Parser;
use dex_parser_rust::apk::ApkHandler;
use dex_parser_rust::exporter::{Exporter, JsonExporter, TextExporter, ExportOptions};
use dex_parser_rust::analysis::core::config::{AnalysisConfig, CompiledConfig};
use std::fs::File;
use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    // 1. Load Configuration
    let config = if let Some(config_path) = &args.config {
        let mut file = File::open(config_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        serde_json::from_str::<AnalysisConfig>(&content)?
    } else {
        AnalysisConfig::default()
    };
    let compiled_config = CompiledConfig::compile(config)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

    // 2. Load Input Buffer
    let mut file = File::open(&args.path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let exporter: Box<dyn Exporter> = match args.format {
        OutputFormat::Json => Box::new(JsonExporter),
        OutputFormat::Text => Box::new(TextExporter),
    };

    let mut stdout = std::io::stdout().lock();

    // 3. Process with Config and Progress Feedback
    let apk = ApkHandler::process_with_callback_and_config(
        &buffer,
        |msg| { eprintln!("  [>] {}", msg); },
        compiled_config
    )?;

    eprintln!("  [+] Analysis complete. Exporting results...");

    let options = ExportOptions {
        include_instructions: !args.no_instructions,
        include_analysis: !args.no_analysis,
        include_metadata: args.include_metadata,
    };

    exporter.export_apk(&apk, &mut stdout, &options)?;

    Ok(())
}
