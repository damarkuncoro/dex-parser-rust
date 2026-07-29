use dex_parser_rust::dex::core::models::{Class, EncodedMethod, Code, Instruction};
use dex_parser_rust::analysis::core::{AnalysisEngine, InstructionVisitor, AnalysisConfig};
use dex_parser_rust::analysis::forensics::engine::data_flow::DataFlowVisitor;
use std::sync::Arc;

#[test]
fn test_taint_propagation_and_leak() {
    let config = Arc::new(AnalysisConfig::default());

    // Skenario:
    // 1. Panggil getDeviceId (Source) -> Simpan di v0
    // 2. Move v0 ke v1 (Propagasi)
    // 3. Panggil sendTextMessage menggunakan v1 (Sink / LEAK!)
    let instructions = vec![
        Instruction {
            offset: 0,
            opcode: 0x71, // invoke-static
            name: "invoke-static".to_string(),
            description: "invoke-static {v0}, Landroid/telephony/TelephonyManager;->getDeviceId".to_string(),
            index: Some(1),
            resolved_value: Some("Landroid/telephony/TelephonyManager;->getDeviceId".to_string()),
            registers: vec![0],
            target_offset: None,
            immediates: vec![],
        },
        Instruction {
            offset: 2,
            opcode: 0x01, // move v2, v0
            name: "move".to_string(),
            description: "move v2, v0".to_string(),
            index: None,
            resolved_value: None,
            registers: vec![2, 0],
            target_offset: None,
            immediates: vec![],
        },
        Instruction {
            offset: 4,
            opcode: 0x71, // invoke-static
            name: "invoke-static".to_string(),
            description: "invoke-static {v2}, Landroid/telephony/SmsManager;->sendTextMessage".to_string(),
            index: Some(2),
            resolved_value: Some("Landroid/telephony/SmsManager;->sendTextMessage".to_string()),
            registers: vec![2],
            target_offset: None,
            immediates: vec![],
        },
    ];

    let method = EncodedMethod {
        signature: "LTest;->leak()V".to_string(),
        code: Some(Code {
            instructions,
            ..Default::default()
        }),
        ..Default::default()
    };

    let class = Class {
        name: "LTest;".to_string(),
        direct_methods: vec![method],
        ..Default::default()
    };

    let visitor = DataFlowVisitor::new(config);
    {
        let mut visitors: Vec<Box<dyn InstructionVisitor>> = vec![Box::new(visitor)];
        AnalysisEngine::walk_classes(&vec![class], &mut visitors);

        let v = visitors[0].as_any().downcast_ref::<DataFlowVisitor>().unwrap();
        assert_eq!(v.findings.len(), 1);
        assert!(v.findings[0].category.contains("Sensitive Taint"));
        assert!(v.findings[0].content.contains("SmsManager;->sendTextMessage"));
    }
}
