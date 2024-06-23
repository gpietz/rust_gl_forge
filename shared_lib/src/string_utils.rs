use gl::types::GLubyte;
use std::ffi::{CStr, CString};
use std::iter;

pub fn convert_glubyte_to_string(glubyte_ptr: *const GLubyte) -> String {
    let c_str = unsafe { CStr::from_ptr(glubyte_ptr as *const _) };
    c_str.to_string_lossy().to_string()
}

pub fn create_whitespace_cstring_with_len(len: usize) -> CString {
    // Create a CString filled with len number of spaces
    let buffer: Vec<u8> = iter::repeat(b' ').take(len).collect();
    unsafe { CString::from_vec_unchecked(buffer) }
}

pub fn readable_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    const TB: u64 = GB * 1024;

    if bytes < KB {
        format!("{} B", bytes)
    } else if bytes < MB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else if bytes < GB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes < TB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else {
        format!("{:.2} TB", bytes as f64 / TB as f64)
    }
}
