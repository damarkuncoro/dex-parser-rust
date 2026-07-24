use crate::dex::core::models::Dex;
use super::DexPrinter;
use std::io::Write;

pub struct JsonPrinter;

impl DexPrinter for JsonPrinter {
    fn print(&self, dex: &Dex, _path: &str, writer: &mut dyn Write) -> std::io::Result<()> {
        match serde_json::to_string_pretty(dex) {
            Ok(json) => writeln!(writer, "{}", json),
            Err(e) => Err(std::io::Error::new(std::io::ErrorKind::Other, e)),
        }
    }
}
