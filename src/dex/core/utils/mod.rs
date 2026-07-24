pub mod access_flags;
pub mod byte_tracker;
pub mod mutf8;

#[macro_export]
macro_rules! trace_parse {
    ($($arg:tt)*) => {
        #[cfg(feature = "verbose")]
        {
            #[cfg(target_arch = "wasm32")]
            {
                use wasm_bindgen::JsValue;
                let msg = format!($($arg)*);
                let _ = js_sys::Reflect::get(&js_sys::global(), &JsValue::from_str("console"))
                    .and_then(|console| {
                        js_sys::Reflect::get(&console, &JsValue::from_str("log"))
                            .and_then(|func| {
                                js_sys::Reflect::apply(&func.into(), &console, &js_sys::Array::of1(&JsValue::from_str(&msg)))
                            })
                    });
            }
            #[cfg(not(target_arch = "wasm32"))]
            {
                println!($($arg)*);
            }
        }
    };
}

pub fn calculate_adler32(data: &[u8]) -> u32 {
    let mut a: u32 = 1;
    let mut b: u32 = 0;

    for chunk in data.chunks(5552) {
        for &byte in chunk {
            a += byte as u32;
            b += a;
        }
        a %= 65521;
        b %= 65521;
    }
    (b << 16) | a
}
