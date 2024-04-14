use crate::gl_traits::{Bindable, Deletable};
use crate::gl_types::{BufferType, BufferUsage};
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

    pub fn new(r#type: BufferType, usage: BufferUsage, data: Vec<T>) -> BufferObject<T> {
        let mut id = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
            gl::BindBuffer(r#type.to_gl_enum(), id);
            gl::BufferData(
                r#type.to_gl_enum(),
                (data.len() * std::mem::size_of::<T>()) as GLsizeiptr,
                data.as_ptr() as *const c_void,
                usage.to_gl_enum(),
            );
        }

        let buffer_object = BufferObject {
            id,
            buffer_type: r#type,
            buffer_usage: usage,
            data,
        };

        buffer_object
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
    /// ```no-run
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

    /// Updates the buffer's data with new vertex data.
    ///
    /// This function replaces the current buffer data with new data provided by the caller.
    /// It binds the buffer, uploads the new data to the GPU, and optionally unbinds the buffer
    /// based on the `unbind` parameter. This method is useful for dynamic buffer content
    /// updates, such as streaming vertex data for animations or interactive applications.
    ///
    /// # Parameters
    /// - `new_data`: A vector of generic type `T` that represents the new data to be uploaded
    ///   to the buffer. The type `T` must conform to the `Vertex` trait, which ensures that
    ///   the data can be correctly interpreted as vertex attributes.
    /// - `unbind`: A boolean flag that determines whether the buffer should be unbound after
    ///   updating the data. If `true`, the buffer is unbound, which can be useful for preventing
    ///   accidental modifications. If `false`, the buffer remains bound, which might be beneficial
    ///   for subsequent operations that require the buffer to be bound.
    ///
    /// # Safety
    /// This function interacts directly with the OpenGL API, making it unsafe. It requires
    /// the caller to ensure that a valid OpenGL context is current on the calling thread
    /// before invocation. Misuse can lead to undefined behavior, including crashes.
    /// The caller is also responsible for ensuring that the data provided is compatible
    /// with the buffer's intended usage and the vertex attribute layout expected by the
    /// GPU shaders.
    ///
    /// # Examples
    /// Assuming `buffer` is an instance of `BufferObject<Vertex>`, and we have a vector
    /// `vertices` of type `Vertex`:
    /// ```no-run
    /// buffer.update_data(vertices, true);
    /// ```
    /// This will upload `vertices` to the GPU and unbind the buffer afterward.
    ///
    /// # Note
    /// The size and layout of `T` must match the expectations of the shaders that will
    /// utilize this buffer. Incorrect data can lead to rendering errors or GPU crashes.
    /// Additionally, frequent updates to large buffers can impact performance, so consider
    /// usage patterns and buffer strategies (such as double buffering) for dynamic data.

    pub fn update_data(&mut self, new_data: Vec<T>, unbind: bool) {
        self.data = new_data;

        let buffer_type = self.buffer_type.to_gl_enum();
        let buffer_usage = self.buffer_usage.to_gl_enum();

        let data_len = self.data.len();
        let data_size = data_len * size_of::<T>();

        unsafe {
            gl::BindBuffer(buffer_type, self.id);
            gl::BufferData(
                buffer_type,
                data_size as GLsizeiptr,
                self.data.as_ptr() as *const c_void,
                buffer_usage,
            );

            if unbind {
                gl::BindBuffer(buffer_type, 0);
            }
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
