mod cli;

use crate::cli::{Cli, OutputFormat};
use clap::Parser;
use dex_parser_rust::dex::{
    apk::ApkParser,
    display::{json::JsonPrinter, text::DexDumpPrinter, DexPrinter},
    DexParser,
};
use std::fs::File;
use std::io::{Read, Write, ErrorKind};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let mut file = File::open(&args.path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let printer: Box<dyn DexPrinter> = match args.format {
        OutputFormat::Json => Box::new(JsonPrinter),
        OutputFormat::Text => Box::new(DexDumpPrinter),
    };

    let mut stdout = std::io::stdout().lock();

    // Check if it's an APK (ZIP) or raw DEX
    if buffer.starts_with(b"PK\x03\x04") {
        let dex_files = ApkParser::parse_apk(&buffer)?;
        for (i, dex) in dex_files.iter().enumerate() {
            if args.format == OutputFormat::Text {
                if let Err(e) = writeln!(stdout, "\n--- DEX File #{} ---", i) {
                    if e.kind() == ErrorKind::BrokenPipe { return Ok(()); }
                    return Err(e.into());
                }
            }
            if let Err(e) = printer.print(dex, &format!("{} [DEX #{}]", args.path, i), &mut stdout) {
                if e.kind() == ErrorKind::BrokenPipe { return Ok(()); }
                return Err(e.into());
            }
        }
    } else {
        // Use the new Public API
        let dex = DexParser::parse(&buffer)?;
        if let Err(e) = printer.print(&dex, &args.path, &mut stdout) {
            if e.kind() == ErrorKind::BrokenPipe { return Ok(()); }
            return Err(e.into());
        }
    }

    Ok(())
}
