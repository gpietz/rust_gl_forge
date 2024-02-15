use crate::gl_traits::{Bindable, Deletable};
use crate::gl_types::{BufferType, BufferUsage};
use crate::gl_vertex::Vertex;
use anyhow::Result;
use gl::types::GLint;
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
                (data.len() * std::mem::size_of::<T>()) as gl::types::GLsizeiptr,
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

    pub fn new_and_bind(r#type: BufferType, usage: BufferUsage, data: Vec<T>) -> BufferObject<T> {
        let mut buffer_object = Self::new(r#type, usage, data);
        buffer_object.bind().expect("Failed to bind buffer object");
        buffer_object
    }

    pub fn get_buffer_id(&self) -> u32 {
        self.id
    }

    pub fn buffer_usage(&self) -> BufferUsage {
        self.buffer_usage
    }

    pub fn data(&self) -> &Vec<T> {
        &self.data
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

impl<T: Vertex> Bindable for BufferObject<T> {
    type Target = BufferObject<T>;

    fn bind(&mut self) -> Result<&mut Self::Target> {
        unsafe { gl::BindBuffer(self.buffer_type.to_gl_enum(), self.id) }
        Ok(self)
    }

    fn unbind(&mut self) -> Result<&mut Self::Target> {
        unsafe {
            gl::BindBuffer(self.buffer_type.to_gl_enum(), 0);
        }
        Ok(self)
    }

    fn is_bound(&self) -> bool {
        let mut current_buffer_id = 0;
        unsafe {
            gl::GetIntegerv(self.buffer_type.to_gl_enum(), &mut current_buffer_id);
        }
        current_buffer_id == self.id as GLint
    }
}

impl<T: Vertex> Deletable for BufferObject<T> {
    fn delete(&mut self) -> Result<()> {
        if self.id != 0 {
            unsafe {
                gl::DeleteBuffers(1, &self.id);
            }
            self.id = 0;
        }
        Ok(())
    }
}

impl<T: Vertex> Drop for BufferObject<T> {
    fn drop(&mut self) {
        if let Err(err) = self.delete() {
            eprintln!("Error while dropping BufferObject: {}", err);
            // You might choose to log the error or take other appropriate actions here.
        }
    }
}
