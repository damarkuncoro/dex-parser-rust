use dex_parser_rust::analysis::forensics::engine::data_flow::TaintEngine;
use dex_parser_rust::analysis::core::models::{XrefMap};
use std::collections::{HashSet};

#[test]
fn test_inter_procedural_taint_propagation() {
    let mut findings = Vec::new();
    let mut source_returners = HashSet::new();
    let mut xref_map = XrefMap::default();

    // Skenario:
    // MethodB (Source Returner) dipanggil oleh MethodA.
    let method_a = "LMethodA;->run()V".to_string();
    let method_b = "LMethodB;->getData()LString;".to_string();

    source_returners.insert(method_b.clone());

    let mut targets = Vec::new();
    targets.push(method_b.clone());
    xref_map.method_to_methods.insert(method_a.clone(), targets);

    TaintEngine::propagate_inter_procedural(&mut findings, &source_returners, &xref_map);

    // Assert: MethodA harus ditandai memiliki Indirect Taint
    assert_eq!(findings.len(), 1);
    assert!(findings[0].category.contains("Indirect Taint"));
    assert!(findings[0].content.contains("MethodA"));
    assert!(findings[0].content.contains("MethodB"));
}
