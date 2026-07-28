use wasm_bindgen::prelude::*;
use crate::apk::ApkHandler;
use crate::dex::core::models::{Dex, wasm::{DexSummary, WasmLoadResult}};
use crate::analysis::core::config::{AnalysisConfig, CompiledConfig};
use parking_lot::Mutex;
use once_cell::sync::Lazy;
use serde::Serialize;

#[derive(Serialize)]
pub struct WasmApkResult<'a> {
    pub dex_files: Vec<Dex<'a>>,
    pub class_lookup: std::collections::HashMap<String, usize>,
}

static PARSED_APK: Lazy<Mutex<Option<WasmApkResult<'static>>>> = Lazy::new(|| Mutex::new(None));

fn send_status(_msg: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::JsValue;
        let global = js_sys::global();
        let _ = js_sys::Reflect::get(&global, &JsValue::from_str("postMessage"))
            .and_then(|func| {
                let msg_obj = js_sys::Object::new();
                let _ = js_sys::Reflect::set(&msg_obj, &JsValue::from_str("type"), &JsValue::from_str("STATUS"));
                let _ = js_sys::Reflect::set(&msg_obj, &JsValue::from_str("payload"), &JsValue::from_str(_msg));
                js_sys::Reflect::apply(&func.into(), &global, &js_sys::Array::of1(&msg_obj))
            });
    }
}

#[wasm_bindgen]
pub fn load_apk_wasm(buffer: &[u8]) -> Result<JsValue, JsValue> {
    load_apk_wasm_with_config(buffer, None)
}

#[wasm_bindgen]
pub fn load_apk_wasm_with_config(buffer: &[u8], config_json: Option<String>) -> Result<JsValue, JsValue> {
    send_status("Engine: Initializing...");

    // 1. Setup Configuration
    let config = if let Some(json) = config_json {
        serde_json::from_str::<AnalysisConfig>(&json)
            .map_err(|e| JsValue::from_str(&format!("Invalid config JSON: {}", e)))?
    } else {
        AnalysisConfig::default()
    };

    let compiled_config = CompiledConfig::compile(config)
        .map_err(|e| JsValue::from_str(&format!("Config compilation failed: {}", e)))?;

    let mut storage = PARSED_APK.lock();
    *storage = None;

    let owned_buffer = buffer.to_vec().into_boxed_slice();
    let leaked_buffer: &'static [u8] = Box::leak(owned_buffer);

    // 2. Process APK
    let apk = ApkHandler::process_with_callback_and_config(leaked_buffer, |msg| send_status(msg), compiled_config)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    let mut summaries = Vec::new();
    for (i, dex) in apk.dex_files.iter().enumerate() {
        summaries.push(DexSummary {
            name: apk.dex_names[i].clone(),
            magic: String::from_utf8_lossy(&dex.header.magic).to_string(),
            class_count: dex.class_defs.len(),
            method_count: dex.analysis.stats.total_methods_analyzed,
            instruction_count: dex.analysis.stats.total_instructions_scanned,
            gap_count: dex.byte_gaps.len(),
            total_gap_size: dex.analysis.stats.total_gap_size,
            suspicious_gap_count: dex.analysis.stats.suspicious_gap_count,
            max_entropy: dex.analysis.stats.max_entropy,
            sensitive_string_count: dex.analysis.stats.sensitive_count,
        });
    }

    let class_names: Vec<Vec<String>> = apk.dex_files.iter()
        .map(|dex| dex.class_defs.iter().map(|c| c.name.to_string()).collect())
        .collect();

    let scan_results: Vec<Vec<crate::analysis::ScanResult>> = apk.dex_files.iter()
        .map(|dex| dex.analysis.sensitive_indicators.clone())
        .collect();

    let intel = apk.intelligence.clone();

    let result = WasmApkResult {
        dex_files: unsafe { std::mem::transmute::<Vec<Dex<'_>>, Vec<Dex<'static>>>(apk.dex_files) },
        class_lookup: apk.class_lookup,
    };

    *storage = Some(result);
    send_status("Engine: Analysis complete.");

    Ok(serde_wasm_bindgen::to_value(&WasmLoadResult {
        summaries,
        class_names,
        scan_results,
        global_intelligence: intel,
    }).unwrap())
}

#[wasm_bindgen]
pub fn get_class_details_wasm(dex_idx: usize, class_idx: usize) -> Result<JsValue, JsValue> {
    let storage = PARSED_APK.lock();
    let apk = storage.as_ref().ok_or_else(|| JsValue::from_str("No APK loaded"))?;

    let dex = apk.dex_files.get(dex_idx).ok_or_else(|| JsValue::from_str("Invalid DEX index"))?;
    let class = dex.class_defs.get(class_idx).ok_or_else(|| JsValue::from_str("Invalid Class index"))?;

    Ok(serde_wasm_bindgen::to_value(&class).unwrap())
}

#[wasm_bindgen]
pub fn get_wasm_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
