mod cli;

use std::fs::File;
use std::io::Read;
use clap::Parser;
use dex_parser_rust::dex::{DexParser, display::{DexPrinter, text::DexDumpPrinter, json::JsonPrinter}};
use crate::cli::{Cli, OutputFormat};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    if args.verbose {
        eprintln!("Verbose mode enabled");
    }

    let mut file = File::open(&args.path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let parser = DexParser::new(&buffer);
    let dex = parser.parse()?;

    let printer: Box<dyn DexPrinter> = match args.format {
        OutputFormat::Json => Box::new(JsonPrinter),
        OutputFormat::Text => Box::new(DexDumpPrinter),
    };

    printer.print(&dex, &args.path);

    Ok(())
}
