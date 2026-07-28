use crate::analysis::core::models::XrefMap;
use std::io::Write;

pub struct DotExporter;

impl DotExporter {
    pub fn export_call_graph(xref: &XrefMap, writer: &mut dyn Write) -> std::io::Result<()> {
        writeln!(writer, "digraph CallGraph {{")?;
        writeln!(writer, "  rankdir=LR;")?;
        writeln!(writer, "  node [shape=box, style=filled, color=lightblue];")?;

        for (caller, targets) in &xref.method_to_methods {
            let caller_esc = caller.replace("\"", "\\\"");
            for target in targets {
                let target_esc = target.replace("\"", "\\\"");
                writeln!(writer, "  \"{}\" -> \"{}\";", caller_esc, target_esc)?;
            }
        }

        writeln!(writer, "}}")?;
        Ok(())
    }
}
