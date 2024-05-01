use std::fmt;
use std::fmt::Display;
use std::os::raw::c_void;

use anyhow::{Context, Result};
use gl::types::{GLboolean, GLenum, GLsizei, GLuint};
use sdl2::keyboard::Keycode::V;

use gl_utils::*;

use crate::gl_traits::ToOpenGL;
use crate::gl_utils;
use crate::gl_vertex_attribute::VertexAttribute;

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
// - VertexDataType -
//////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Copy)]
pub enum VertexDataType {
    Byte,
    UnsignedByte,
    Short,
    UnsignedShort,
    Int,
    UnsignedInt,
    HalfFloat,
    Float,
    Double,
    Fixed,
    #[allow(non_camel_case_types)]
    Int_2_10_10_10_Rev,
    #[allow(non_camel_case_types)]
    UnsignedInt_2_10_10_10_Rev,
    #[allow(non_camel_case_types)]
    UnsignedInt_10F_11F_11F_Rev,
}

impl VertexDataType {
    pub fn from_gl_enum(gl_enum: GLenum) -> Option<VertexDataType> {
        match gl_enum {
            gl::BYTE => Some(VertexDataType::Byte),
            gl::UNSIGNED_BYTE => Some(VertexDataType::UnsignedByte),
            gl::SHORT => Some(VertexDataType::Short),
            gl::UNSIGNED_SHORT => Some(VertexDataType::UnsignedShort),
            gl::INT => Some(VertexDataType::Int),
            gl::UNSIGNED_INT => Some(VertexDataType::UnsignedInt),
            gl::HALF_FLOAT => Some(VertexDataType::HalfFloat),
            gl::FLOAT => Some(VertexDataType::Float),
            gl::DOUBLE => Some(VertexDataType::Double),
            gl::FIXED => Some(VertexDataType::Fixed),
            gl::INT_2_10_10_10_REV => Some(VertexDataType::Int_2_10_10_10_Rev),
            gl::UNSIGNED_INT_2_10_10_10_REV => Some(VertexDataType::UnsignedInt_2_10_10_10_Rev),
            gl::UNSIGNED_INT_10F_11F_11F_REV => Some(VertexDataType::UnsignedInt_10F_11F_11F_Rev),
            _ => None,
        }
    }

    pub fn to_gl_enum(&self) -> GLenum {
        match self {
            VertexDataType::Byte => gl::BYTE,
            VertexDataType::UnsignedByte => gl::UNSIGNED_BYTE,
            VertexDataType::Short => gl::SHORT,
            VertexDataType::UnsignedShort => gl::UNSIGNED_SHORT,
            VertexDataType::Int => gl::INT,
            VertexDataType::UnsignedInt => gl::UNSIGNED_INT,
            VertexDataType::HalfFloat => gl::HALF_FLOAT,
            VertexDataType::Float => gl::FLOAT,
            VertexDataType::Double => gl::DOUBLE,
            VertexDataType::Fixed => gl::FIXED,
            VertexDataType::Int_2_10_10_10_Rev => gl::INT_2_10_10_10_REV,
            VertexDataType::UnsignedInt_2_10_10_10_Rev => gl::UNSIGNED_INT_2_10_10_10_REV,
            VertexDataType::UnsignedInt_10F_11F_11F_Rev => gl::UNSIGNED_INT_10F_11F_11F_REV,
        }
    }

    pub fn size(&self) -> usize {
        match self {
            VertexDataType::Byte => std::mem::size_of::<i8>(),
            VertexDataType::UnsignedByte => std::mem::size_of::<u8>(),
            VertexDataType::Short => std::mem::size_of::<i16>(),
            VertexDataType::UnsignedShort => std::mem::size_of::<u16>(),
            VertexDataType::Int => std::mem::size_of::<i32>(),
            VertexDataType::UnsignedInt => std::mem::size_of::<u32>(),
            // No direct Rust equivalent, but known to be 2 bytes
            VertexDataType::HalfFloat => 2,
            VertexDataType::Float => std::mem::size_of::<f32>(),
            VertexDataType::Double => std::mem::size_of::<f64>(),
            // Typically represented as a 32-bit quantity, no direct Rust equivalent
            VertexDataType::Fixed => 4,
            // Packed into a 32-bit integer
            VertexDataType::Int_2_10_10_10_Rev => 4,
            // Packed into a 32-bit integer
            VertexDataType::UnsignedInt_2_10_10_10_Rev => 4,
            VertexDataType::UnsignedInt_10F_11F_11F_Rev => 4,
        }
    }
}

impl Display for VertexDataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VertexDataType::Byte => write!(f, "Byte"),
            VertexDataType::UnsignedByte => write!(f, "UnsignedByte"),
            VertexDataType::Short => write!(f, "Short"),
            VertexDataType::UnsignedShort => write!(f, "UnsignedShort"),
            VertexDataType::Int => write!(f, "Int"),
            VertexDataType::UnsignedInt => write!(f, "UnsignedInt"),
            VertexDataType::HalfFloat => write!(f, "HalfFloat"),
            VertexDataType::Float => write!(f, "Float"),
            VertexDataType::Double => write!(f, "Double"),
            VertexDataType::Fixed => write!(f, "Fixed"),
            VertexDataType::Int_2_10_10_10_Rev => write!(f, "Int_2_10_10_10_Rev"),
            VertexDataType::UnsignedInt_2_10_10_10_Rev => write!(f, "UnsignedInt_2_10_10_10_Rev"),
            VertexDataType::UnsignedInt_10F_11F_11F_Rev => write!(f, "UnsignedInt_10F_11F_11F_Rev"),
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
// - VertexAttributeType -
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy)]
pub enum VertexAttributeType {
    /// **3 components per position, float, not normalized**
    Position,
    /// **2 components per position, float, not normalized**
    Position2D,
    /// **4 components per color, float, not normalized**
    Color,
    /// **2 components per texture coordinate, float, not normalized**
    TexCoord,
    /// **3 components per normal, float, not normalized**
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

    /// Converts the `VertexAttributeType` to OpenGL data format.
    ///
    /// This function maps each variant of `VertexAttributeType` to a tuple
    /// representing the format of the corresponding vertex attribute in OpenGL.
    /// Specifically, it returns a tuple containing the number of components per
    /// vertex attribute, the data type of each component, and a flag indicating
    /// whether the data should be normalized.
    ///
    /// # Returns
    /// A tuple consisting of:
    /// - An `i32` representing the number of components per vertex attribute.
    /// - A `GLenum` specifying the data type of each component in the attribute.
    /// - A `GLboolean` indicating whether the attribute data should be normalized (`gl::TRUE`)
    ///   or not (`gl::FALSE`).
    ///
    /// # Variants
    /// - `VertexAttributeType::Position`: Represents a vertex position with 3 components per position,
    ///   using floating-point values, and not normalized.
    /// - `VertexAttributeType::Color`: Represents a vertex color with 4 components per color,
    ///   using floating-point values, and not normalized.
    /// - `VertexAttributeType::TexCoord`: Represents a texture coordinate with 2 components per coordinate,
    ///   using floating-point values, and not normalized.
    /// - `VertexAttributeType::Normal`: Represents a vertex normal with 3 components per normal,
    ///   using floating-point values, and not normalized.
    ///
    /// # Note
    /// This function is specifically designed to be used in conjunction with the `glVertexAttribPointer`
    /// function, enabling easy setup of vertex attribute pointers in OpenGL.
    pub fn to_gl_data(&self) -> (i32, GLenum, GLboolean) {
        match self {
            // 3 components per position, float, not normalized
            VertexAttributeType::Position => (3, gl::FLOAT, gl::FALSE),
            // 2 components per position, float, not normalized
            VertexAttributeType::Position2D => (2, gl::FLOAT, gl::FALSE),
            // 4 components per color, float, not normalized
            VertexAttributeType::Color => (4, gl::FLOAT, gl::FALSE),
            // 2 components per texture coordinate, float, not normalized
            VertexAttributeType::TexCoord => (2, gl::FLOAT, gl::FALSE),
            // 3 components per normal, float, not normalized
            VertexAttributeType::Normal => (3, gl::FLOAT, gl::FALSE),
        }
    }

    pub fn to_vertex_attribute(&self) -> VertexAttribute {
        match self {
            VertexAttributeType::Position => {
                VertexAttribute::new(3, VertexDataType::Float).name("position".to_string())
            }
            VertexAttributeType::Position2D => {
                VertexAttribute::new(2, VertexDataType::Float).name("position".to_string())
            }
            VertexAttributeType::Color => {
                VertexAttribute::new(4, VertexDataType::Float).name("color".to_string())
            }
            VertexAttributeType::TexCoord => {
                VertexAttribute::new(2, VertexDataType::Float).name("tex_coord".to_string())
            }
            VertexAttributeType::Normal => {
                VertexAttribute::new(3, VertexDataType::Float).name("normal".to_string())
            }
        }
    }
}

impl Into<VertexAttribute> for VertexAttributeType {
    fn into(self) -> VertexAttribute {
        match self {
            VertexAttributeType::Position => VertexAttribute {
                components: 3,
                data_type: VertexDataType::Float,
                normalized: Some(false),
                ..Default::default()
            },
            VertexAttributeType::Position2D => VertexAttribute {
                components: 2,
                data_type: VertexDataType::Float,
                normalized: Some(false),
                ..Default::default()
            },
            VertexAttributeType::Color => VertexAttribute {
                components: 4,
                data_type: VertexDataType::Float,
                normalized: Some(false),
                ..Default::default()
            },
            VertexAttributeType::TexCoord => VertexAttribute {
                components: 2,
                data_type: VertexDataType::Float,
                normalized: Some(false),
                ..Default::default()
            },
            VertexAttributeType::Normal => VertexAttribute {
                components: 3,
                data_type: VertexDataType::Float,
                normalized: Some(false),
                ..Default::default()
            },
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
// - ShaderType -
//////////////////////////////////////////////////////////////////////////////

pub enum ShaderType {
    Vertex,
    Fragment,
    Geometry,
    Compute,
}

impl ShaderType {
    pub fn to_gl_enum(&self) -> GLenum {
        match self {
            ShaderType::Vertex => gl::VERTEX_SHADER,
            ShaderType::Fragment => gl::FRAGMENT_SHADER,
            ShaderType::Geometry => gl::GEOMETRY_SHADER,
            ShaderType::Compute => gl::COMPUTE_SHADER,
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
// - RenderMask -
//////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RenderMask {
    pub color: bool,
    pub depth: bool,
    pub stencil: bool,
}

impl RenderMask {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn color(mut self) -> Self {
        self.color = true;
        self
    }

    pub fn depth(mut self) -> Self {
        self.depth = true;
        self
    }

    pub fn stencil(mut self) -> Self {
        self.stencil = true;
        self
    }

    pub fn color_and_depth(mut self) -> Self {
        self.color = true;
        self.depth = true;
        self
    }

    pub fn color_and_stencil(mut self) -> Self {
        self.color = true;
        self.depth = true;
        self
    }

    pub fn depth_and_stencil(mut self) -> Self {
        self.depth = true;
        self.stencil = true;
        self
    }

    pub fn all(mut self) -> Self {
        self.color = true;
        self.depth = true;
        self.stencil = true;
        self
    }
}

impl ToOpenGL for RenderMask {
    fn to_opengl(&self) -> u32 {
        let mut mask = 0;
        if self.color {
            mask |= gl::COLOR_BUFFER_BIT;
        }
        if self.depth {
            mask |= gl::DEPTH_BUFFER_BIT;
        }
        if self.stencil {
            mask |= gl::STENCIL_BUFFER_BIT;
        }
        mask
    }
}

impl From<(bool, bool, bool)> for RenderMask {
    fn from(value: (bool, bool, bool)) -> Self {
        RenderMask {
            color: value.0,
            depth: value.1,
            stencil: value.2,
        }
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
    pub fn to_gl_enum(&self) -> GLenum {
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

//////////////////////////////////////////////////////////////////////////////
// - Capability -
//////////////////////////////////////////////////////////////////////////////

/// Represents OpenGL capabilities that can be enabled or disabled.
pub enum Capability {
    /// Capability to perform alpha testing.
    //AlphaTest = gl::ALPHA_TEST as isize,
    /// Capability to blend pixels.
    Blend = gl::BLEND as isize,
    /// Capability to perform depth comparisons and update the depth buffer.
    DepthTest = gl::DEPTH_TEST as isize,
    /// Capability to cull polygons based on their winding in window coordinates.
    CullFace = gl::CULL_FACE as isize,
    /// Capability to perform scissor test, that is to discard fragments that are outside of the scissor rectangle.
    ScissorTest = gl::SCISSOR_TEST as isize,
    /// Capability to simulate fog.
    //Fog = gl::FOG as isize,
    /// Capability to use dithering when merging fragment colors and depth values.
    Dither = gl::DITHER as isize,
    /// Capability for line smoothing.
    LineSmooth = gl::LINE_SMOOTH as isize,
    /// Capability for polygon smoothing.
    PolygonSmooth = gl::POLYGON_SMOOTH as isize,
    /// Capability to update stencil buffer.
    StencilTest = gl::STENCIL_TEST as isize,
}

impl Capability {
    /// Converts the capability to its corresponding OpenGL enum value.
    pub fn to_gl_enum(self) -> GLenum {
        self as GLenum
    }

    /// Enables this OpenGL capability.
    pub fn enable(self) {
        unsafe { gl::Enable(self.to_gl_enum()) }
    }

    /// Disables this OpenGL capability.
    pub fn disable(self) {
        unsafe { gl::Disable(self.to_gl_enum()) }
    }

    /// Returns true if the OpenGL capability is currently enabled.
    pub fn is_enabled(self) -> bool {
        unsafe { gl::IsEnabled(self.to_gl_enum()) > 0 }
    }
}
