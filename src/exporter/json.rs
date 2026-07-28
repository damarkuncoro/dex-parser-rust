use crate::dex::core::models::{Dex, Apk};
use crate::exporter::core::{Exporter, ExportOptions};
use std::io::Write;

pub struct JsonExporter;

impl Exporter for JsonExporter {
    fn export_dex(&self, dex: &Dex, writer: &mut dyn Write, _options: &ExportOptions) -> std::io::Result<()> {
        match serde_json::to_string_pretty(dex) {
            Ok(json) => writeln!(writer, "{}", json),
            Err(e) => Err(std::io::Error::new(std::io::ErrorKind::Other, e)),
        }
    }

    fn export_apk(&self, apk: &Apk, writer: &mut dyn Write, _options: &ExportOptions) -> std::io::Result<()> {
        match serde_json::to_string_pretty(apk) {
            Ok(json) => writeln!(writer, "{}", json),
            Err(e) => Err(std::io::Error::new(std::io::ErrorKind::Other, e)),
        }
    }
}
