use crate::dex::core::models::Dex;
use std::io::Write;

pub fn export_analysis(dex: &Dex, writer: &mut dyn Write) -> std::io::Result<()> {
    let report = &dex.analysis;

    let (null_padded, actual_gaps): (Vec<_>, Vec<_>) = report.forensic_gaps.iter().partition(|g| g.is_null_padded);
    let alignment_bytes: usize = null_padded.iter().map(|g| g.length).sum();
    let unexplained_bytes: usize = actual_gaps.iter().map(|g| g.length).sum();

    writeln!(writer, "\n📊 Analysis Summary:")?;
    writeln!(writer, "  - Methods Analyzed: {}", report.stats.total_methods_analyzed)?;
    writeln!(writer, "  - Instructions Scanned: {}", report.stats.total_instructions_scanned)?;

    if report.stats.unknown_opcodes_count > 0 || report.stats.spec_violation_count > 0 {
        writeln!(writer, "  - Instruction Anomalies:")?;
        if report.stats.spec_violation_count > 0 {
            writeln!(writer, "    [!] Spec Violations: {} instructions use officially UNUSED opcodes (⚠ Critical: Custom VM/Anti-Analysis)", report.stats.spec_violation_count)?;
        }
        if report.stats.unknown_opcodes_count > 0 {
            writeln!(writer, "    [!] Undefined Opcodes: {} instructions are completely undefined", report.stats.unknown_opcodes_count)?;
        }

        let mut sorted_unknowns: Vec<_> = report.stats.unknown_opcodes_distribution.iter().collect();
        sorted_unknowns.sort_by(|a, b| b.1.cmp(a.1));

        if !sorted_unknowns.is_empty() {
            write!(writer, "    [!] Top anomalies: ")?;
            for (i, (op, count)) in sorted_unknowns.iter().take(5).enumerate() {
                if i > 0 { write!(writer, ", ")?; }
                write!(writer, "0x{:02x} ({})", op, count)?;
            }
            writeln!(writer)?;
        }
    } else {
        writeln!(writer, "  - Unknown Instructions: 0")?;
    }

    if report.stats.max_consecutive_nops > 5 {
        writeln!(writer, "  - Junk Code Pattern: Found sequence of {} consecutive NOPs (⚠ Suspicious padding)", report.stats.max_consecutive_nops)?;
    }

    if report.stats.dead_code_count > 0 {
        writeln!(writer, "  - Dead Code: {} unreachable instructions (⚠ Potentially hidden/junk code)", report.stats.dead_code_count)?;
    }

    writeln!(writer, "  - Alignment Padding: {} bytes", alignment_bytes)?;

    if unexplained_bytes > 0 {
        writeln!(writer, "  - Unexplained/Skipped Bytes: {} bytes (⚠ Potentially hidden data)", unexplained_bytes)?;
        for gap in actual_gaps {
            writeln!(writer, "    [!] Offset 0x{:06x}: Content [ {} ] ({} bytes, entropy: {:.2})",
                gap.offset, gap.data_preview, gap.length, gap.entropy)?;
        }
    }

    let suspicious_gaps: Vec<_> = report.forensic_gaps.iter().filter(|g| g.is_suspicious).collect();

    if !suspicious_gaps.is_empty() {
        writeln!(writer, "\n⚠ Obfuscation/Packer Analysis:")?;
        writeln!(writer, "Found {} suspicious skipped data blocks (potential hidden payloads):", suspicious_gaps.len())?;
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
            if let Some(details) = &result.details {
                writeln!(writer, "      Algorithm : {}", details.algorithm)?;
                writeln!(writer, "      Mode      : {}", details.mode)?;
                writeln!(writer, "      Padding   : {}", details.padding)?;
                writeln!(writer, "      Risk      : {}", details.risk)?;
                writeln!(writer, "      Reason    : {}", details.reason)?;
            }
        }
    }
    Ok(())
}
