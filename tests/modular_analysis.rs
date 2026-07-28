use dex_parser_rust::dex::core::models::{Class, EncodedMethod, Code, Instruction};
use dex_parser_rust::analysis::core::{AnalysisEngine, InstructionVisitor, InstructionStats, StatsVisitor};
use dex_parser_rust::analysis::cross_ref::builder::XrefVisitor;
use std::sync::Arc;
use parking_lot::Mutex;

#[test]
fn test_modular_pipeline_stats_and_xref() {
    // 1. Setup mock instructions
    let instructions = vec![
        Instruction {
            offset: 0,
            opcode: 0x1a, // const-string
            name: "const-string".to_string(),
            description: "const-string v0, \"test\"".to_string(),
            index: Some(1),
            resolved_value: Some("test".to_string()),
            registers: vec![0],
            target_offset: None,
            immediates: vec![],
        },
        Instruction {
            offset: 2,
            opcode: 0x71, // invoke-static
            name: "invoke-static".to_string(),
            description: "invoke-static {v0}, Ljava/lang/System;->out".to_string(),
            index: Some(2),
            resolved_value: Some("Ljava/lang/System;->out".to_string()),
            registers: vec![0],
            target_offset: None,
            immediates: vec![],
        },
    ];

    let method = EncodedMethod {
        name: "testMethod".to_string(),
        signature: "LTest;->testMethod()V".to_string(),
        code: Some(Code {
            registers_size: 1,
            ins_size: 0,
            outs_size: 1,
            insns_size: 4,
            instructions,
            catches: vec![],
            debug_info: None,
        }),
        ..Default::default()
    };

    let class = Class {
        name: "LTest;".to_string(),
        direct_methods: vec![method],
        ..Default::default()
    };

    let classes = vec![class];

    // 2. Setup Visitors
    let stats = Arc::new(Mutex::new(InstructionStats::default()));
    let stats_visitor = StatsVisitor::new(stats.clone());
    let xref_visitor = XrefVisitor::new();

    let visitors: Vec<Box<dyn InstructionVisitor>> = vec![
        Box::new(stats_visitor),
        Box::new(xref_visitor),
    ];

    // 3. Run Pipeline (Parallel)
    let results = AnalysisEngine::walk_classes_parallel(&classes, &visitors);

    // 4. Verify Stats
    let final_stats = stats.lock();
    assert_eq!(final_stats.string_count, 1);
    assert_eq!(final_stats.call_count, 1);

    // 5. Verify XREFs
    let xref_v = results.iter()
        .find(|v| v.as_any().is::<XrefVisitor>())
        .and_then(|v| v.as_any().downcast_ref::<XrefVisitor>())
        .expect("XrefVisitor not found");

    let calls = xref_v.map.method_to_methods.get("LTest;->testMethod()V").unwrap();
    assert!(calls.contains(&"Ljava/lang/System;->out".to_string()));

    let strings = xref_v.map.method_to_strings.get("LTest;->testMethod()V").unwrap();
    assert!(strings.contains(&"test".to_string()));
}
