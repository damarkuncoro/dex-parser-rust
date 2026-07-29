use crate::analysis::{
    ForensicAnalyzer, AnalysisReport, XrefVisitor,
    BehaviorVisitor, AnalysisEngine,
    StatsVisitor, InstructionStats, TokenizerVisitor, ObfuscationVisitor, ResourceVisitor,
    DataFlowVisitor, ScoringEngine, CryptoVisitor
};
use crate::analysis::control_flow::CfgBuilder;
use crate::dex::core::models::Class;
use crate::dex::core::utils::byte_tracker::ByteTracker;
use crate::analysis::core::config::CompiledConfig;
use crate::analysis::core::visitor::InstructionVisitor;
use std::sync::Arc;
use parking_lot::Mutex;
use rayon::prelude::*;

pub struct AnalysisResults {
    pub report: AnalysisReport,
    pub byte_gaps: Vec<(usize, usize)>,
}

pub fn run(
    buffer: &[u8],
    strings: &[&[u8]],
    classes: &[Class],
    tracker: Arc<Mutex<ByteTracker>>,
    compiled_config: Arc<CompiledConfig>,
) -> AnalysisResults {
    let byte_gaps = tracker.lock().get_gaps();

    // 1. Parallel heavy-lifting (Forensics)
    eprintln!("      - Scanning for forensic indicators & entropy...");
    let forensic_data = ForensicAnalyzer::run(buffer, strings, &byte_gaps, &compiled_config);

    // 2. Multi-Pass Analysis (Modular Pipeline)
    eprintln!("      - Running modular analysis pipeline (Stats, XREF, Behavior)...");
    let shared_stats = Arc::new(Mutex::new(InstructionStats::default()));
    let config_arc = Arc::new(compiled_config.config.clone());

    let visitors: Vec<Box<dyn InstructionVisitor>> = vec![
        Box::new(StatsVisitor::new(shared_stats.clone())),
        Box::new(XrefVisitor::new()),
        Box::new(BehaviorVisitor::new(config_arc.clone())),
        Box::new(TokenizerVisitor::new(config_arc.clone())),
        Box::new(ObfuscationVisitor::new()),
        Box::new(ResourceVisitor::new()),
        Box::new(DataFlowVisitor::new(config_arc.clone())),
        Box::new(CryptoVisitor::new()),
    ];

    // Run all visitors in a single parallel pass!
    let results = AnalysisEngine::walk_classes_parallel(classes, &visitors);

    // 3. Extract results from visitors
    eprintln!("      - Consolidating results...");
    let stats = shared_stats.lock().clone();

    let xref_map = results.iter()
        .find(|v| v.as_any().is::<XrefVisitor>())
        .and_then(|v| v.as_any().downcast_ref::<XrefVisitor>())
        .map(|v| v.map.clone())
        .unwrap_or_default();

    let behavioral_indicators = results.iter()
        .find(|v| v.as_any().is::<BehaviorVisitor>())
        .and_then(|v| v.as_any().downcast_ref::<BehaviorVisitor>())
        .map(|v| {
            let mut results = v.results.clone();
            results.sort_by(|a, b| a.content.cmp(&b.content));
            results.dedup_by(|a, b| a.content == b.content);
            results
        })
        .unwrap_or_default();

    let method_tokens = results.iter()
        .find(|v| v.as_any().is::<TokenizerVisitor>())
        .and_then(|v| v.as_any().downcast_ref::<TokenizerVisitor>())
        .map(|v| v.results.clone())
        .unwrap_or_default();

    let obfuscation_indicators = results.iter()
        .find(|v| v.as_any().is::<ObfuscationVisitor>())
        .and_then(|v| v.as_any().downcast_ref::<ObfuscationVisitor>())
        .map(|v| v.results.clone())
        .unwrap_or_default();

    let potential_resource_ids = results.iter()
        .find(|v| v.as_any().is::<ResourceVisitor>())
        .and_then(|v| v.as_any().downcast_ref::<ResourceVisitor>())
        .map(|v| v.found_ids.keys().cloned().collect::<Vec<_>>())
        .unwrap_or_default();

    let taint_data = results.iter()
        .find(|v| v.as_any().is::<DataFlowVisitor>())
        .and_then(|v| v.as_any().downcast_ref::<DataFlowVisitor>())
        .map(|v| (v.findings.clone(), v.source_returners.clone()))
        .unwrap_or_default();

    let mut taint_findings = taint_data.0;
    let source_returners = taint_data.1;

    // --- INTER-PROCEDURAL PROPAGATION ---
    crate::analysis::forensics::engine::data_flow::TaintEngine::propagate_inter_procedural(
        &mut taint_findings,
        &source_returners,
        &xref_map
    );

    let crypto_findings = results.iter()
        .find(|v| v.as_any().is::<CryptoVisitor>())
        .and_then(|v| v.as_any().downcast_ref::<CryptoVisitor>())
        .map(|v| v.findings.clone())
        .unwrap_or_default();

    // Add forensic and obfuscation indicators
    let mut final_indicators = behavioral_indicators;
    final_indicators.extend(forensic_data.1);
    final_indicators.extend(obfuscation_indicators);
    final_indicators.extend(taint_findings);
    final_indicators.extend(crypto_findings);

    let total_instructions = classes.par_iter()
        .map(|c| {
            c.direct_methods.iter().chain(c.virtual_methods.iter())
                .map(|m| m.code.as_ref().map(|code| code.instructions.len()).unwrap_or(0))
                .sum::<usize>()
        })
        .sum::<usize>();

    // 5. Create the final report
    let mut report = AnalysisReport::new(
        forensic_data.0,
        final_indicators,
        xref_map,
        method_tokens,
        total_instructions
    );
    report.potential_resource_ids = potential_resource_ids;

    let mut total_dead_code = 0;
    for class in classes {
        for method in class.direct_methods.iter().chain(class.virtual_methods.iter()) {
            if let Some(code) = &method.code {
                let cfg = CfgBuilder::build(&code.instructions);
                let reachable_offsets: std::collections::HashSet<usize> = cfg.iter()
                    .flat_map(|b| b.instructions.clone())
                    .collect();

                for (idx, _) in code.instructions.iter().enumerate() {
                    if !reachable_offsets.contains(&idx) {
                        total_dead_code += 1;
                    }
                }
            }
        }
    }

    report.stats.call_count = stats.call_count;
    report.stats.jump_count = stats.jump_count;
    report.stats.string_count = stats.string_count;
    report.stats.unknown_opcodes_count = stats.unknown_opcodes_count;
    report.stats.spec_violation_count = stats.spec_violation_count;
    report.stats.unknown_opcodes_distribution = stats.unknown_opcodes_distribution;
    report.stats.max_consecutive_nops = stats.max_consecutive_nops;
    report.stats.dead_code_count = total_dead_code;

    // 6. Final Risk Assessment
    report.risk_assessment = ScoringEngine::assess(&report, &compiled_config.config);

    AnalysisResults {
        report,
        byte_gaps,
    }
}
