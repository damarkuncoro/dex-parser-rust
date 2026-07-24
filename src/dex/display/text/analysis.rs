use crate::dex::core::models::Dex;
use std::io::Write;

pub fn print_analysis(dex: &Dex, writer: &mut dyn Write) -> std::io::Result<()> {
    let report = &dex.analysis;
    let suspicious_gaps: Vec<_> = report.forensic_gaps.iter().filter(|g| g.is_suspicious).collect();

    if !suspicious_gaps.is_empty() {
        writeln!(writer, "\n⚠ Obfuscation/Packer Analysis:")?;
        writeln!(writer, "Found {} suspicious unreferenced data gaps:", suspicious_gaps.len())?;
        for gap in suspicious_gaps {
            writeln!(
                writer,
                "  - Offset: 0x{:06x}, Size: {} bytes, Entropy: {:.2} (High - Likely Encrypted/Packed)",
                gap.offset, gap.length, gap.entropy
            )?;
        }
    }

    if !report.sensitive_indicators.is_empty() {
        writeln!(writer, "\n🔍 Sensitive Data Indicators:")?;
        for result in &report.sensitive_indicators {
            writeln!(writer, "  [{}] {}", result.category, result.content)?;
        }
    }
    Ok(())
}
