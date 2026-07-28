pub mod header;
pub mod class;
pub mod method;
pub mod analysis;

use crate::dex::core::models::{Dex, Apk};
use crate::exporter::core::{Exporter, ExportOptions};
use std::io::Write;

pub struct TextExporter;

impl Exporter for TextExporter {
    fn export_dex(&self, dex: &Dex, writer: &mut dyn Write, options: &ExportOptions) -> std::io::Result<()> {
        header::export_header(dex, writer)?;

        if options.include_analysis {
            analysis::export_analysis(dex, writer)?;
        }

        for (i, class_def) in dex.class_defs.iter().enumerate() {
            class::export_class(class_def, i, writer, options)?;
        }

        Ok(())
    }

    fn export_apk(&self, apk: &Apk, writer: &mut dyn Write, options: &ExportOptions) -> std::io::Result<()> {
        writeln!(writer, "APK Intelligence Report")?;
        writeln!(writer, "=======================")?;

        if let Some(manifest) = &apk.manifest {
            writeln!(writer, "\n[Manifest Information]")?;
            writeln!(writer, "  Package Name: {}", manifest.package_name)?;
            if !manifest.permissions.is_empty() {
                writeln!(writer, "  Permissions ({}):", manifest.permissions.len())?;
                for p in &manifest.permissions {
                    writeln!(writer, "    - {}", p)?;
                }
            }
        }

        if !apk.intelligence.resolved_resources.is_empty() {
            writeln!(writer, "\n[Resolved Resources]")?;
            let mut ids: Vec<_> = apk.intelligence.resolved_resources.keys().collect();
            ids.sort();
            for &id in ids {
                writeln!(writer, "  0x{:08x} -> {}", id, apk.intelligence.resolved_resources[&id])?;
            }
        }

        let intel = &apk.intelligence;

        if options.include_analysis {
            writeln!(writer, "\n[Security Summary]")?;
            writeln!(writer, "  Suspicious Gaps      : {}", intel.global_security_summary.total_suspicious_gaps)?;
            writeln!(writer, "  Sensitive Indicators : {}", intel.global_security_summary.total_sensitive_indicators)?;
            writeln!(writer, "  Potentially Packed   : {}", intel.global_security_summary.potentially_packed)?;

            if !intel.behavioral_indicators.is_empty() {
                writeln!(writer, "\n[Behavioral Indicators]")?;
                for indicator in &intel.behavioral_indicators {
                    writeln!(writer, "  [{}] {}", indicator.category, indicator.content)?;
                }
            }
        }

        writeln!(writer, "\n[Files Analyzed]")?;
        for (i, dex) in apk.dex_files.iter().enumerate() {
            let name = &apk.dex_names[i];
            writeln!(writer, "\n--- {} ---", name)?;
            self.export_dex(dex, writer, options)?;
        }

        Ok(())
    }
}
