use std::ffi::CString;
use std::iter;

pub fn create_whitespace_cstring_with_len(len: usize) -> CString {
    // Create a CString filled with len number of spaces
    let buffer: Vec<u8> = iter::repeat(b' ' as u8).take(len).collect();
    unsafe { CString::from_vec_unchecked(buffer) }
}
