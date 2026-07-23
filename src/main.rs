mod cli;

use crate::cli::{Cli, OutputFormat};
use clap::Parser;
use dex_parser_rust::dex::{
    display::{json::JsonPrinter, text::DexDumpPrinter, DexPrinter},
    DexParser,
};
use std::fs::File;
use std::io::Read;

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
