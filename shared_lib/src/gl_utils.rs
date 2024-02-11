use anyhow::Result;
use std::os::raw::c_void;
use std::ptr;

/// Converts an `usize` offset to a raw pointer of type `*const c_void`.
///
/// This function is designed to convert numerical offsets to equivalent raw pointers.
/// It specifically handles the case where an offset of 0 is intended to represent a null pointer.
///
/// # Arguments
///
/// * `offset` - An `usize` value representing an offset or address. The offset 0 is treated
///   specially and results in a null pointer.
///
/// # Returns
///
/// A raw pointer of type `*const c_void`. If the provided `offset` is 0, this function returns
/// a null pointer. Otherwise, it casts the `offset` to a `*const c_void` pointer.
///
/// # Examples
///
/// ```
/// use std::ptr;
/// use std::os::raw::c_void;
/// use shared_lib::gl_utils::as_c_void; // Adjust this line to use the correct module path
///
/// let ptr: *const c_void = as_c_void(0);
/// assert_eq!(ptr, ptr::null());
///
/// let non_null_ptr: *const c_void = as_c_void(10);
/// // This assertion would depend on the context and is not generally applicable
/// // assert_eq!(non_null_ptr, 10 as *const c_void);
/// ```
///
/// # Safety
///
/// This function returns a raw pointer, which can potentially lead to undefined behavior if not used
/// carefully. The caller must ensure that the offset passed to this function, if non-zero, represents a
/// valid memory address that respects the aliasing rules of Rust. Additionally, because this function can
/// return null pointers, callers must handle the resulting pointers appropriately to avoid dereferencing null.
#[inline]
pub fn as_c_void(offset: usize) -> *const c_void {
    if offset == 0 {
        ptr::null()
    } else {
        offset as *const c_void
    }
}

#[inline]
pub fn check_gl_error() -> Result<()> {
    let mut errors = Vec::new();

    loop {
        let error_code = unsafe { gl::GetError() };
        if error_code == gl::NO_ERROR {
            break;
        }

        let error_msg = match error_code {
            gl::INVALID_ENUM => "INVALID_ENUM",
            gl::INVALID_VALUE => "INVALID_VALUE",
            gl::INVALID_OPERATION => "INVALID_OPERATION",
            gl::INVALID_FRAMEBUFFER_OPERATION => "INVALID_FRAMEBUFFER_OPERATION",
            gl::STACK_OVERFLOW => "STACK_OVERFLOW",
            gl::STACK_UNDERFLOW => "STACK_UNDERFLOW",
            gl::OUT_OF_MEMORY => "OUT_OF_MEMORY",
            _ => "UNKNOWN_ERROR",
        };

        errors.push(format!("OpenGL error [{}]: {}", error_code, error_msg));
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(anyhow::anyhow!(errors.join(", ")))
    }
}
