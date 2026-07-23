use std::ffi::CStr;
use std::os::raw::c_char;
use crate::dex::parsers::DexParser;

/// FFI function to get the number of classes in a DEX file.
/// Can be called from C, Python (ctypes), Java (JNI), etc.
#[no_mangle]
pub extern "C" fn get_class_count(path: *const c_char) -> i32 {
    if path.is_null() { return -1; }

    let c_str = unsafe { CStr::from_ptr(path) };
    let path_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return -2,
    };

    match DexParser::parse_file(path_str) {
        Ok(dex) => dex.class_defs.len() as i32,
        Err(_) => -3,
    }
}
