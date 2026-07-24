use wasm_bindgen::prelude::*;
use crate::dex::parsers::DexParser;
use crate::dex::models::{Dex, Apk};
use std::sync::Mutex;
use once_cell::sync::Lazy;
use serde::Serialize;
use std::io::{Read, Cursor};
use zip::ZipArchive;

// Unified result structure for WASM
#[derive(Serialize)]
pub struct WasmApkResult<'a> {
    pub dex_files: Vec<Dex<'a>>,
    pub class_lookup: std::collections::HashMap<String, usize>,
}

// Global state to hold parsed DEX files in WASM memory
static PARSED_APK: Lazy<Mutex<Option<WasmApkResult<'static>>>> = Lazy::new(|| Mutex::new(None));

// Helper to send status to JS via worker postMessage
fn send_status(msg: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::JsValue;
        let global = js_sys::global();
        let _ = js_sys::Reflect::get(&global, &JsValue::from_str("postMessage"))
            .and_then(|func| {
                let msg_obj = js_sys::Object::new();
                let _ = js_sys::Reflect::set(&msg_obj, &JsValue::from_str("type"), &JsValue::from_str("STATUS"));
                let _ = js_sys::Reflect::set(&msg_obj, &JsValue::from_str("payload"), &JsValue::from_str(msg));
                js_sys::Reflect::apply(&func.into(), &global, &js_sys::Array::of1(&msg_obj))
            });
    }
}

#[wasm_bindgen]
pub fn load_apk_wasm(buffer: &[u8]) -> Result<JsValue, JsValue> {
    send_status("Engine: Locking storage...");
    let mut storage = PARSED_APK.lock().map_err(|_| JsValue::from_str("Lock failed"))?;
    *storage = None;

    send_status("Engine: Preparing buffer...");
    let owned_buffer = buffer.to_vec().into_boxed_slice();
    let leaked_buffer: &'static [u8] = Box::leak(owned_buffer);

    let mut dex_files = Vec::new();

    if leaked_buffer.starts_with(b"PK\x03\x04") {
        send_status("Engine: APK detected, opening archive...");
        let reader = Cursor::new(leaked_buffer);
        let mut archive = ZipArchive::new(reader).map_err(|e| JsValue::from_str(&e.to_string()))?;

        let dex_entry_names: Vec<String> = archive.file_names()
            .filter(|name| name.ends_with(".dex"))
            .map(|s| s.to_string())
            .collect();

        let total_dex = dex_entry_names.len();
        for (i, name) in dex_entry_names.into_iter().enumerate() {
            send_status(&format!("Engine: Parsing {} ({} of {})...", name, i + 1, total_dex));
            let mut file = archive.by_name(&name).map_err(|e| JsValue::from_str(&e.to_string()))?;
            let mut dex_buffer = Vec::new();
            file.read_to_end(&mut dex_buffer).map_err(|e| JsValue::from_str(&e.to_string()))?;

            let leaked_dex: &'static [u8] = Box::leak(dex_buffer.into_boxed_slice());
            let dex = DexParser::parse(leaked_dex).map_err(|e| JsValue::from_str(&e.to_string()))?;
            dex_files.push(dex);
        }
    } else {
        send_status("Engine: Single DEX detected, parsing...");
        let dex = DexParser::parse(leaked_buffer).map_err(|e| JsValue::from_str(&e.to_string()))?;
        dex_files.push(dex);
    };

    if dex_files.is_empty() {
        return Err(JsValue::from_str("No DEX files found"));
    }

    send_status("Engine: Linking global context...");
    let apk = Apk::new(dex_files);

    let result = WasmApkResult {
        dex_files: unsafe { std::mem::transmute::<Vec<Dex<'_>>, Vec<Dex<'static>>>(apk.dex_files) },
        class_lookup: apk.class_lookup,
    };

    send_status("Engine: Generating class metadata...");
    let metadata: Vec<Vec<String>> = result.dex_files.iter()
        .map(|dex| dex.class_defs.iter().map(|c| c.name.to_string()).collect())
        .collect();

    *storage = Some(result);
    send_status("Engine: Analysis complete.");

    Ok(serde_wasm_bindgen::to_value(&metadata).unwrap())
}

#[wasm_bindgen]
pub fn get_class_details_wasm(dex_idx: usize, class_idx: usize) -> Result<JsValue, JsValue> {
    let storage = PARSED_APK.lock().map_err(|_| JsValue::from_str("Lock failed"))?;
    let apk = storage.as_ref().ok_or_else(|| JsValue::from_str("No APK loaded"))?;

    let dex = apk.dex_files.get(dex_idx).ok_or_else(|| JsValue::from_str("Invalid DEX index"))?;
    let class = dex.class_defs.get(class_idx).ok_or_else(|| JsValue::from_str("Invalid Class index"))?;

    Ok(serde_wasm_bindgen::to_value(&class).unwrap())
}

#[wasm_bindgen]
pub fn get_wasm_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
