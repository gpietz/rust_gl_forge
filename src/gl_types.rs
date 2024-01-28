use anyhow::{Context, Result};
use gl_utils::*;
use std::os::raw::c_void;

//////////////////////////////////////////////////////////////////////////////
// - BufferTypes -
//////////////////////////////////////////////////////////////////////////////
use crate::gl_utils;
use gl::types::{GLboolean, GLenum, GLsizei, GLuint};

pub enum BufferType {
    /// Stores vertex attributes like vertex coordinates, normals, texture coordinates, etc.
    ArrayBuffer,
    /// Used for indexing vertices, allowing the reuse of vertex data for multiple primitives.
    ElementArrayBuffer,
    /// Stores uniform data for shaders. This allows for efficient sharing of data between
    /// multiple shaders.
    UniformBuffer,
    /// Stores data for a buffer texture, a special texture type that is accessed with a texel
    /// fetch operation in GLSL.
    TextureBuffer,
    /// Not a buffer in the traditional sense, but a target that contains attachments like color,
    /// depth, and stencil buffers for rendering.
    Framebuffer,
    /// Used as a rendering destination, typically for offscreen rendering, and can store data
    /// like color, depth, and stencil.
    Renderbuffer,
    /// Used for copying data between buffers.
    CopyReadBuffer,
    /// Used for copying data between buffers.
    CopyWriteBuffer,
    /// Used in pixel transfer operations, like reading from or writing to textures.
    PixelPackBuffer,
    /// Used in pixel transfer operations, like reading from or writing to textures.
    PixelUnpackBuffer,
    /// Captures output from the vertex shader or geometry shader.
    TransformFeedbackBuffer,
    /// Stores atomic counters, used for achieving synchronization and consistency across
    /// shader invocations.
    AtomicCounterBuffer,
    /// Used for storing indirect drawing commands.
    DrawIndirectBuffer,
    /// Similar to the draw indirect buffer, but used for compute shader dispatch commands.
    DispatchIndirectBuffer,
    /// Provides read-write storage for data that is accessed by shaders. It's more flexible
    /// compared to uniform buffers.
    ShaderStorageBuffer,
}

impl BufferType {
    pub fn to_gl_enum(&self) -> u32 {
        match self {
            BufferType::ArrayBuffer => gl::ARRAY_BUFFER,
            BufferType::ElementArrayBuffer => gl::ELEMENT_ARRAY_BUFFER,
            BufferType::UniformBuffer => gl::UNIFORM_BUFFER,
            BufferType::TextureBuffer => gl::TEXTURE_BUFFER,
            BufferType::Framebuffer => gl::FRAMEBUFFER,
            BufferType::Renderbuffer => gl::RENDERBUFFER,
            BufferType::CopyReadBuffer => gl::COPY_READ_BUFFER,
            BufferType::CopyWriteBuffer => gl::COPY_WRITE_BUFFER,
            BufferType::PixelPackBuffer => gl::PIXEL_PACK_BUFFER,
            BufferType::PixelUnpackBuffer => gl::PIXEL_UNPACK_BUFFER,
            BufferType::TransformFeedbackBuffer => gl::TRANSFORM_FEEDBACK_BUFFER,
            BufferType::AtomicCounterBuffer => gl::ATOMIC_COUNTER_BUFFER,
            BufferType::DrawIndirectBuffer => gl::DRAW_INDIRECT_BUFFER,
            BufferType::DispatchIndirectBuffer => gl::DISPATCH_INDIRECT_BUFFER,
            BufferType::ShaderStorageBuffer => gl::SHADER_STORAGE_BUFFER,
        }
    }

    pub fn all_types() -> Vec<BufferType> {
        vec![
            BufferType::ArrayBuffer,
            BufferType::ElementArrayBuffer,
            BufferType::UniformBuffer,
            BufferType::TextureBuffer,
            BufferType::Framebuffer,
            BufferType::Renderbuffer,
            BufferType::CopyReadBuffer,
            BufferType::CopyWriteBuffer,
            BufferType::PixelPackBuffer,
            BufferType::PixelUnpackBuffer,
            BufferType::TransformFeedbackBuffer,
            BufferType::AtomicCounterBuffer,
            BufferType::DrawIndirectBuffer,
            BufferType::DispatchIndirectBuffer,
            BufferType::ShaderStorageBuffer,
        ]
    }
}

//////////////////////////////////////////////////////////////////////////////
// - BufferUsage -
//////////////////////////////////////////////////////////////////////////////

pub enum BufferUsage {
    /// Used when the data in the buffer will not change or will change only infrequently and is
    /// used primarily for drawing.
    StaticDraw,
    /// Used when the data in the buffer will change frequently and is used primarily for drawing.
    DynamicDraw,
    /// Used when the data in the buffer will change on every draw and is used primarily
    /// for drawing.
    StreamDraw,
    /// Used for buffers that are not changed by the application and are read by the GPU.
    StaticRead,
    /// Used for buffers that are changed frequently by the application and are read by the GPU.
    DynamicRead,
    /// Used for buffers that are changed and read by the GPU once.
    StreamRead,
    /// Used for buffers that are not changed by the application but are used for copying data
    /// from one buffer to another.
    StaticCopy,
    /// Used for buffers that are changed frequently and used for copying data from one buffer
    /// to another.
    DynamicCopy,
    /// sed for buffers that are changed and used for copying data from one buffer to another once.
    StreamCopy,
}

impl BufferUsage {
    pub fn to_gl_enum(&self) -> u32 {
        match self {
            BufferUsage::StaticDraw => gl::STATIC_DRAW,
            BufferUsage::DynamicDraw => gl::DYNAMIC_DRAW,
            BufferUsage::StreamDraw => gl::STREAM_DRAW,
            BufferUsage::StaticRead => gl::STATIC_READ,
            BufferUsage::DynamicRead => gl::DYNAMIC_READ,
            BufferUsage::StreamRead => gl::STREAM_READ,
            BufferUsage::StaticCopy => gl::STATIC_COPY,
            BufferUsage::DynamicCopy => gl::DYNAMIC_COPY,
            BufferUsage::StreamCopy => gl::STREAM_COPY,
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
// - VertexAttributeType -
//////////////////////////////////////////////////////////////////////////////

pub enum VertexAttributeType {
    Position,
    Color,
    TexCoord,
    Normal,
}

impl VertexAttributeType {
    pub fn setup(&self, index: GLuint, stride: GLsizei, offset: *const c_void) -> Result<()> {
        let (size, r#type, normalized) = self.to_gl_data();
        unsafe {
            gl::EnableVertexAttribArray(index);
            gl::VertexAttribPointer(
                index,
                size,
                r#type,
                normalized,
                stride,
                offset as *const c_void,
            );
            check_gl_error().context(format!("Failed to set up attribute {}", index))?;
        }

        Ok(())
    }

    pub fn to_gl_data(&self) -> (i32, GLenum, GLboolean) {
        match self {
            // 3 components per position, float, not normalized
            VertexAttributeType::Position => (3, gl::FLOAT, gl::FALSE),
            // 4 components per color, float, not normalized
            VertexAttributeType::Color => (4, gl::FLOAT, gl::FALSE),
            // 2 components per texture coordinate, float, not normalized
            VertexAttributeType::TexCoord => (2, gl::FLOAT, gl::FALSE),
            // 3 components per normal, float, not normalized
            VertexAttributeType::Normal => (3, gl::FLOAT, gl::FALSE),
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
// - ShaderType -
//////////////////////////////////////////////////////////////////////////////

pub enum ShaderType {
    Vertex,
    Fragment,
}

impl ShaderType {
    pub fn to_gl_enum(&self) -> u32 {
        match self {
            ShaderType::Vertex => gl::VERTEX_SHADER,
            ShaderType::Fragment => gl::FRAGMENT_SHADER,
        }
    }
}
