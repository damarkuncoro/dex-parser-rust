pub mod header;
pub mod class;
pub mod method;

use crate::dex::models::Dex;
use super::DexPrinter;
use std::io::Write;

pub struct DexDumpPrinter;

impl DexPrinter for DexDumpPrinter {
    fn print(&self, dex: &Dex, path: &str, writer: &mut dyn Write) -> std::io::Result<()> {
        writeln!(writer, "Processing '{}'...", path)?;
        writeln!(writer, "Opened '{}', DEX version '035'", path)?;
        header::print_header(dex, writer)?;
        header::print_map_list(dex, writer)?;
        for (i, class) in dex.classes.iter().enumerate() {
            class::print_class(dex, i, class, writer)?;
        }
        Ok(())
    }
}
