use crate::gl_prelude::check_gl_error;
use crate::gl_traits::{Bindable, Deletable};
use crate::gl_types::{BufferType, BufferUsage};
use crate::opengl::vertex_array_object::VertexArrayObject;
use anyhow::Result;
use gl::types::{GLint, GLsizeiptr};
use std::ffi::c_void;
use std::mem::size_of;
use std::ptr;

//////////////////////////////////////////////////////////////////////////////
// - BufferObject -
//////////////////////////////////////////////////////////////////////////////

pub struct BufferObject<T> {
    id: u32,
    buffer_type: BufferType,
    buffer_usage: BufferUsage,
    data: Vec<T>,
}

impl<T> BufferObject<T> {
    /// Creates an empty `BufferObject` with specified buffer type and usage,
    /// optimized for either vertex or index data.
    ///
    /// This constructor is useful for initializing buffer objects for different
    /// content types without initial data. It's ideal for buffers that will be
    /// dynamically updated later.
    ///
    /// Parameters:
    /// - `r#type`: Specifies the buffer type (e.g., `ArrayBuffer` for vertex
    ///   data, `ElementArrayBuffer` for index data), affecting its use in the
    ///   graphics pipeline.
    /// - `usage`: Defines the data store's usage pattern (e.g., `StaticDraw`,
    ///   `DynamicDraw`), aiding driver optimization.
    ///   `IndexData`, both empty.
    /// - `bind`: Whether to bind the buffer immediately after creation,
    ///   necessary before data upload but can be deferred.
    ///
    /// Returns:
    /// A new `BufferObject<T>` instance (with `T` implementing `Vertex`),
    /// initialized per parameters but without data, ready for later use.
    ///
    /// Example:
    /// ```no-run
    /// let empty_vertex_buffer = BufferObject::empty(
    ///     BufferType::ArrayBuffer,
    ///     BufferUsage::StaticDraw,
    ///     true // Bind immediately
    /// );
    /// ```
    pub fn empty(r#type: BufferType, usage: BufferUsage) -> BufferObject<T> {
        Self::new(r#type, usage, Vec::<T>::new())
    }

    /// Creates a new buffer object with specified type, usage, data, and optional binding.
    ///
    /// This function generates a new OpenGL buffer object and fills it with the provided data.
    /// The type of buffer (e.g., vertex buffer, index buffer) and usage pattern (e.g., static draw,
    /// dynamic draw) are specified to optimize OpenGLs handling of the data. If `bind` is true,
    /// the buffer is also bound to its corresponding buffer target, making it the current buffer
    /// for subsequent OpenGL operations.
    ///
    /// # Parameters
    /// - `r#type`: The type of the buffer (`BufferType`), indicating its purpose in OpenGL
    ///   (e.g., `GL_ARRAY_BUFFER` for vertex attributes).
    /// - `usage`: The usage pattern of the buffer (`BufferUsage`), which hints at how often the
    ///   buffer's data will be updated and used.
    /// - `data`: A vector of data to be uploaded to the GPU. The type `T` must conform to the
    ///   `Vertex` trait, ensuring it can be correctly interpreted as vertex data.
    ///
    /// # Returns
    /// A `BufferObject<T>` instance representing the newly created OpenGL buffer.
    ///
    /// # Safety
    /// This function interacts directly with the OpenGL API, which involves unsafe operations.
    /// It is the caller's responsibility to ensure that the function is called in a context where
    /// an OpenGL context is available and made current. Misuse may lead to undefined behavior,
    /// including crashes.
    ///
    /// # Example
    /// ```no-run
    /// let vertices: Vec<Vertex> = vec![...]; // Vertex data
    /// let vertex_buffer = BufferObject::new(BufferType::ArrayBuffer, BufferUsage::StaticDraw, vertices, true);
    /// // Now, `vertex_buffer` is ready to be used in rendering operations.
    /// ```
    ///
    /// Note: This function expects the caller to manage the OpenGL context and ensure it is
    /// available and current. Failing to do so could result in OpenGL errors or undefined behavior.

    pub fn new(type_: BufferType, usage: BufferUsage, data: Vec<T>) -> BufferObject<T> {
        let mut id = 0;
        let buffer_type = type_.to_gl_enum();
        unsafe {
            gl::GenBuffers(1, &mut id);
            gl::BindBuffer(buffer_type, id);

            if !data.is_empty() {
                gl::BufferData(
                    buffer_type,
                    (data.len() * size_of::<T>()) as GLsizeiptr,
                    data.as_ptr() as *const c_void,
                    usage.to_gl_enum(),
                );
            }
        }

        BufferObject {
            id,
            buffer_type: type_,
            buffer_usage: usage,
            data,
        }
    }

    pub fn new_with_vao(
        vao: &VertexArrayObject,
        type_: BufferType,
        usage: BufferUsage,
        data: Vec<T>,
    ) -> Self {
        vao.bind();
        let vbo = Self::new(type_, usage, data);
        VertexArrayObject::unbind();
        vbo
    }

    pub fn buffer_id(&self) -> u32 {
        self.id
    }

    pub fn buffer_usage(&self) -> BufferUsage {
        self.buffer_usage
    }

    pub fn data(&self) -> &Vec<T> {
        &self.data
    }

    pub fn data_len(&self) -> usize {
        self.data.len()
    }

    pub fn data_size(&self) -> usize {
        self.data.len() + size_of::<T>()
    }

    /// Updates the data of the buffer object.
    ///
    /// This function updates the internal data of the buffer object with the provided vertices.
    /// Optionally, it can also update the buffer's usage pattern if a new one is provided.
    /// It then binds the buffer and updates its data in the OpenGL context.
    ///
    /// # Parameters
    /// - `vertices`: A vector of type `T` containing the new vertex data to be stored
    ///   in the buffer.
    /// - `usage`: An optional parameter of type `BufferUsage` that specifies the new usage
    ///   pattern of the buffer. If `None`, the current usage pattern remains unchanged.
    ///
    /// # Returns
    /// - `Result<()>`: Returns `Ok(())` if the operation is successful, or an error if
    ///   an OpenGL error is encountered.
    ///
    /// # Safety
    /// This function involves unsafe operations to interact with the OpenGL API. It assumes
    /// that the OpenGL context is properly initialized and that the buffer ID is valid.
    ///
    /// # Example
    /// ```no-run
    /// buffer.update_data(new_vertices, Some(BufferUsage::DynamicDraw))?;
    /// ```
    pub fn update_data(&mut self, vertices: Vec<T>, usage: Option<BufferUsage>) {
        self.data = vertices;

        if let Some(new_usage) = usage {
            self.buffer_usage = new_usage;
        }

        let buffer_type = self.buffer_type.to_gl_enum();

        let data_len = self.data.len();
        let data_size = data_len * size_of::<T>();

        unsafe {
            gl::BindBuffer(buffer_type, self.id);
            gl::BufferData(
                buffer_type,
                data_size as GLsizeiptr,
                self.data.as_ptr() as *const c_void,
                self.buffer_usage.to_gl_enum(),
            );
        }
    }

    /// Clears the data from the buffer object.
    ///
    /// This function removes all data from the buffer, effectively resetting its content.
    /// It's useful for reusing the buffer for different data or clearing memory that is no
    /// longer needed. The buffer itself remains valid and can be refilled with new data.
    ///
    /// # Parameters
    /// - `unbind`: A boolean indicating whether to unbind the buffer after clearing its data.
    ///   If `true`, the buffer is unbound from its target to prevent accidental modifications.
    ///
    /// # Safety
    /// Interacts directly with the OpenGL API, requiring a valid OpenGL context to be current
    /// on the calling thread. Incorrect use can lead to undefined behavior, including program
    /// crashes. The caller must ensure that this operation is safe in the current context.
    pub fn clear_data(&mut self, unbind: bool) {
        // Reset the BufferContent to an empty state
        self.data = Vec::new();

        let buffer_type = self.buffer_type.to_gl_enum();
        let buffer_usage = self.buffer_usage.to_gl_enum();

        unsafe {
            gl::BindBuffer(buffer_type, self.id);
            // Update the buffer with zero size to clear its data on the GPU
            gl::BufferData(buffer_type, 0, ptr::null(), buffer_usage);

            if unbind {
                gl::BindBuffer(buffer_type, 0);
            }
        }
    }
}

impl<T> Bindable for BufferObject<T> {
    fn bind(&self) -> Result<()> {
        unsafe { gl::BindBuffer(self.buffer_type.to_gl_enum(), self.id) }
        check_gl_error()
    }

    fn unbind(&self) -> Result<()> {
        unsafe {
            gl::BindBuffer(self.buffer_type.to_gl_enum(), 0);
        }
        check_gl_error()
    }

    fn is_bound(&self) -> Result<bool> {
        let mut current_buffer_id = 0;
        unsafe {
            gl::GetIntegerv(self.buffer_type.to_gl_enum(), &mut current_buffer_id);
            check_gl_error()?;
        }
        Ok(current_buffer_id == self.id as GLint)
    }
}

impl<T> Deletable for BufferObject<T> {
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

impl<T> Drop for BufferObject<T> {
    fn drop(&mut self) {
        if let Err(err) = self.delete() {
            eprintln!("Error while dropping BufferObject: {}", err);
            // You might choose to log the error or take other appropriate actions here.
        }
    }
}

/// A macro to unbind multiple buffer objects and handle any potential errors.
///
/// This macro takes a variadic list of buffer objects and attempts to unbind each one by calling
/// its `unbind()` method. If any `unbind()` operation results in an error, the macro prints
/// an error message to the standard error output (`stderr`).
///
/// # Usage
///
/// ```ignore
/// unbind_buffers!(vbo, ibo, another_buffer);
/// ```
///
/// In the example above, the macro will call `unbind()` on each of the provided buffers in the order
/// they are listed. If any of these calls return an `Err`, the error is captured and printed
/// to `stderr`.
///
/// # Parameters
///
/// - `$buffer`: An expression that resolves to a buffer object which implements the `unbind()` method
///   returning a `Result`. Multiple buffer objects can be passed, separated by commas.
///
/// # Error Handling
///
/// The macro uses pattern matching to check if the `unbind()` call results in an error (`Err`).
/// If an error is encountered, the macro outputs an error message including the error details.
/// This can help with debugging and ensures that errors are not silently ignored.
///
/// # Example
///
/// ```ignore
/// let vbo = BufferObject::<u32> { /* initialization */ };
/// let ibo = BufferObject::<u32> { /* initialization */ };
///
/// unbind_buffers!(&vbo, &ibo);
/// ```
///
/// This will attempt to unbind both `vbo` and `ibo`. If any of the unbind operations fail, an
/// error message will be printed to `stderr`.
///
/// # Notes
///
/// - The macro is designed to provide basic error handling by logging errors. If more sophisticated
///   error handling is needed, consider modifying the macro or handling errors in a different manner.
#[macro_export]
macro_rules! unbind_buffers {
    ($($buffer:expr),*) => {
        $(
            if let Err(e) = $buffer.unbind() {
                eprintln!("Error unbinding buffer: {:?}", e);
            }
        )*
    };
}
