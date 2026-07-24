use crate::dex::core::models::Dex;
use crate::dex::display::DexPrinter;
use std::io::{Write};

pub mod header;
pub mod class;
pub mod method;

pub struct DexDumpPrinter;

impl DexPrinter for DexDumpPrinter {
    fn print(&self, dex: &Dex, path: &str, writer: &mut dyn Write) -> std::io::Result<()> {
        writeln!(writer, "Processing '{}'...", path)?;
        header::print_header(dex, writer)?;

        for (i, class_def) in dex.class_defs.iter().enumerate() {
            class::print_class(class_def, i, dex, writer)?;
        }

        Ok(())
    }
}
