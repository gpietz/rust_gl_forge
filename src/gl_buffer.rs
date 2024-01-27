use crate::gl_types::{BufferType, BufferUsage};
use crate::gl_vertex::Vertex;
use cgmath::Vector3;
use std::ffi::c_void;

//////////////////////////////////////////////////////////////////////////////
// - BufferObject -
//////////////////////////////////////////////////////////////////////////////

pub struct BufferObject<T: Vertex> {
    id: u32,
    buffer_type: BufferType,
    buffer_usage: BufferUsage,
    data: Vec<T>,
}

impl<T: Vertex> BufferObject<T> {
    pub fn new(r#type: BufferType, usage: BufferUsage, data: Vec<T>) -> BufferObject<T> {
        let mut id = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
            gl::BindBuffer(r#type.to_gl_enum(), id);
            gl::BufferData(
                r#type.to_gl_enum(),
                (data.len() * std::mem::size_of::<Vector3<f32>>()) as isize,
                data.as_ptr() as *const c_void,
                usage.to_gl_enum(),
            );
        }
        BufferObject {
            id,
            buffer_type: r#type,
            buffer_usage: usage,
            data,
        }
    }

    pub fn get_buffer_id(&self) -> u32 {
        self.id
    }

    pub fn bind(&self) {
        unsafe { gl::BindBuffer(self.buffer_type.to_gl_enum(), self.id) }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(self.buffer_type.to_gl_enum(), 0);
        }
    }

    /// Unbinds all OpenGL buffer types.
    ///
    /// This function iterates over all buffer types defined in the `BufferType` enum
    /// and unbinds each one. It sets the current buffer for each type to '0', which
    /// effectively unbinds any buffer currently bound to that type. This is useful for
    /// ensuring that no buffers remain bound inadvertently, which could lead to unexpected
    /// behavior or performance issues.
    ///
    /// # Safety
    /// This function contains an `unsafe` block, as it directly interacts with the OpenGL
    /// API. The caller must ensure that a valid OpenGL context is current in the thread
    /// where this function is called. Failing to do so could result in undefined behavior,
    /// including program crashes.
    ///
    /// # Examples
    /// ```
    /// // Assuming a valid OpenGL context is available and `BufferType` is defined
    /// unbind_all_buffers();
    /// // At this point, all buffer types are unbound.
    /// ```
    ///
    /// Note: This function is intended for scenarios where a complete reset of buffer
    /// state is required. In typical use cases, it's more efficient to bind and unbind
    /// buffers as needed rather than unbinding all buffer types.
    pub fn unbind_all_buffers() {
        for buffer_type in BufferType::all_types() {
            unsafe {
                gl::BindBuffer(buffer_type.to_gl_enum(), 0);
            }
        }
    }
}

impl<T: Vertex> Drop for BufferObject<T> {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}