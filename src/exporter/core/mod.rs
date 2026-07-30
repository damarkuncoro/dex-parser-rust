use crate::dex::core::models::{Dex, Apk};
use std::io::Write;

#[derive(Default, Clone)]
pub struct ExportOptions {
    pub include_instructions: bool,
    pub include_analysis: bool,
    pub include_metadata: bool,
}

/// Trait for exporting analysis and parsing results.
pub trait Exporter {
    /// Export a single DEX file.
    fn export_dex(&self, dex: &Dex, writer: &mut dyn Write, options: &ExportOptions) -> std::io::Result<()>;

    /// Export an entire APK (multiple DEX files + global intelligence).
    fn export_apk(&self, apk: &Apk, writer: &mut dyn Write, options: &ExportOptions) -> std::io::Result<()>;
}
