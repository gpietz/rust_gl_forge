use crate::gl_utils;
use anyhow::{Context, Result};
use bitflags::*;
use gl::types::GLbitfield;
use gl::types::{GLboolean, GLenum, GLsizei, GLuint};
use gl_utils::*;
use std::os::raw::c_void;

//////////////////////////////////////////////////////////////////////////////
// - BufferType -
//////////////////////////////////////////////////////////////////////////////

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

#[derive(Copy, Clone, Debug)]
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

#[derive(Clone, Copy)]
pub enum VertexAttributeType {
    Position,
    Color,
    TexCoord,
    Normal,
}

impl VertexAttributeType {
    /// Sets up a vertex attribute pointer in OpenGL.
    ///
    /// This method enables a vertex attribute array at the specified `index`, and defines
    /// its data layout according to the `VertexAttributeType` instance. It specifies the
    /// number of components per attribute, the type of each component, whether the data
    /// should be normalized, the stride between consecutive attributes, and the offset
    /// within the buffer where the attribute's data begins.
    ///
    /// # Parameters
    /// - `index`: The index of the vertex attribute to set up.
    /// - `stride`: The byte offset between consecutive attributes.
    /// - `offset`: A pointer to the first component of the first attribute in the buffer.
    ///
    /// # Returns
    /// A `Result` indicating success or containing an error.
    ///
    /// # Errors
    /// Returns an error if an OpenGL error occurs during attribute setup.
    ///
    /// # Safety
    /// This method contains unsafe code that calls into the OpenGL API. It is the caller's
    /// responsibility to ensure that the provided parameters are valid and that the OpenGL
    /// context is correctly set up.
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
    pub fn to_gl_enum(&self) -> GLenum {
        match self {
            ShaderType::Vertex => gl::VERTEX_SHADER,
            ShaderType::Fragment => gl::FRAGMENT_SHADER,
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
// - PrimitiveType -
//////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimitiveType {
    Points,
    LineStrip,
    LineLoop,
    Lines,
    LineStripAdjacency,
    LinesAdjacency,
    TriangleStrip,
    TriangleFan,
    Triangles,
    TriangleStripAdjacency,
    TrianglesAdjacency,
    Patches,
}

impl PrimitiveType {
    pub fn to_gl_enum(&self) -> u32 {
        match self {
            PrimitiveType::Points => gl::POINTS,
            PrimitiveType::LineStrip => gl::LINE_STRIP,
            PrimitiveType::LineLoop => gl::LINE_LOOP,
            PrimitiveType::Lines => gl::LINES,
            PrimitiveType::LineStripAdjacency => gl::LINE_STRIP_ADJACENCY,
            PrimitiveType::LinesAdjacency => gl::LINES_ADJACENCY,
            PrimitiveType::TriangleStrip => gl::TRIANGLE_STRIP,
            PrimitiveType::TriangleFan => gl::TRIANGLE_FAN,
            PrimitiveType::Triangles => gl::TRIANGLES,
            PrimitiveType::TriangleStripAdjacency => gl::TRIANGLE_STRIP_ADJACENCY,
            PrimitiveType::TrianglesAdjacency => gl::TRIANGLES_ADJACENCY,
            PrimitiveType::Patches => gl::PATCHES,
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
// - IndicesValueType -
//////////////////////////////////////////////////////////////////////////////

/// Enum `IndicesValueType` specifies the type of values in indices for OpenGL.
/// It corresponds to the accepted values for the type of indices used in functions like `glDrawElements`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndicesValueType {
    Byte,
    Short,
    Int,
}

impl IndicesValueType {
    /// Converts `IndicesValueType` to its corresponding OpenGL constant.
    pub fn to_gl_enum(&self) -> u32 {
        match self {
            IndicesValueType::Byte => gl::UNSIGNED_BYTE,
            IndicesValueType::Short => gl::UNSIGNED_SHORT,
            IndicesValueType::Int => gl::UNSIGNED_INT,
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
// - BufferBit -
//////////////////////////////////////////////////////////////////////////////

bitflags! {
    /// A set of bitflags used to specify the buffers to be cleared in OpenGL rendering operations.
    ///
    /// This struct wraps OpenGL's buffer bit constants to be used with functions like `glClear`
    /// to clear specific buffers. Each flag can be combined using bitwise OR (`|`) to clear multiple buffers
    /// with a single call.
    ///
    /// # Examples
    ///
    /// Clearing both the color and depth buffers:
    ///
    /// ```
    /// # extern crate gl;
    /// # use gl::types::*;
    /// # bitflags::bitflags! {
    /// #     pub struct BufferBit : GLbitfield {
    /// #         pub const GL_COLOR_BUFFER_BIT = gl::COLOR_BUFFER_BIT;
    /// #         pub const GL_DEPTH_BUFFER_BIT = gl::DEPTH_BUFFER_BIT;
    /// #         pub const GL_STENCIL_BUFFER_BIT = gl::STENCIL_BUFFER_BIT;
    /// #     }
    /// # }
    /// # fn main() {
    /// let flags = BufferBit::GL_COLOR_BUFFER_BIT | BufferBit::GL_DEPTH_BUFFER_BIT;
    /// unsafe { gl::Clear(flags.bits()); }
    /// # }
    /// ```
    ///
    /// This struct provides a safer, Rust-friendly way of specifying buffer bits for operations
    /// like clearing the framebuffer, while ensuring type safety and better integration with Rust's
    /// features.
    pub struct BufferBit : GLbitfield {
        /// Indicates the buffers currently enabled for color writing.
        /// Use this flag to clear the color buffer and reset the color values
        /// of the framebuffer to the predefined clear values.
        const COLOR_BUFFER_BIT = gl::COLOR_BUFFER_BIT;
        /// Indicates the depth buffer.
        /// Use this flag to clear the depth buffer and reset the depth information
        /// of the framebuffer, typically used to prepare for a new round of depth testing
        /// for rendering a new frame.
        const DEPTH_BUFFER_BIT = gl::DEPTH_BUFFER_BIT;
        /// Indicates the stencil buffer.
        /// Use this flag to clear the stencil buffer and reset the stencil information
        /// of the framebuffer, which is often used in complex rendering techniques
        /// such as stencil testing for masking parts of the scene.
        const STENCIL_BUFFER_BIT = gl::STENCIL_BUFFER_BIT;
    }
}

impl BufferBit {
    pub fn to_gl(&self) -> GLbitfield {
        self.bits()
    }
}

//////////////////////////////////////////////////////////////////////////////
// - TextureTarget -
//////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureTarget {
    Texture1D,
    Texture2D,
    Texture3D,
    Texture1DArray,
    Texture2DArray,
    TextureRectangle,
    TextureCubeMap,
    TextureCubeMapArray,
    TextureBuffer,
    Texture2DMultisample,
    Texture2DMultisampleArray,
}

impl TextureTarget {
    pub fn to_gl_enum(&self) -> gl::types::GLenum {
        match self {
            Self::Texture1D => gl::TEXTURE_1D,
            Self::Texture2D => gl::TEXTURE_2D,
            Self::Texture3D => gl::TEXTURE_3D,
            Self::Texture1DArray => gl::TEXTURE_1D_ARRAY,
            Self::Texture2DArray => gl::TEXTURE_2D_ARRAY,
            Self::TextureRectangle => gl::TEXTURE_RECTANGLE,
            Self::TextureCubeMap => gl::TEXTURE_CUBE_MAP,
            Self::TextureCubeMapArray => gl::TEXTURE_CUBE_MAP_ARRAY,
            Self::TextureBuffer => gl::TEXTURE_BUFFER,
            Self::Texture2DMultisample => gl::TEXTURE_2D_MULTISAMPLE,
            Self::Texture2DMultisampleArray => gl::TEXTURE_2D_MULTISAMPLE_ARRAY,
        }
    }
}
