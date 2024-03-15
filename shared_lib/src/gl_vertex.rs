use crate::gl_vertex_attribute::VertexAttribute;

//////////////////////////////////////////////////////////////////////////////
// - Vertex -
//////////////////////////////////////////////////////////////////////////////

/// Represents a vertex type in a graphics rendering context.
///
/// The `Vertex` trait is designed to be implemented by types that represent
/// a single vertex in a mesh or graphical object. Implementors of this trait
/// are required to define the structure of a vertex in terms of its attributes,
/// such as position, color, texture coordinates, normals, etc., which are essential
/// for rendering operations.
///
/// Implementing this trait allows for generic handling of different vertex structures
/// across rendering systems, shaders, and buffer management mechanisms.
///
/// # Required Methods
///
/// - `attributes`: Returns a list of vertex attributes that describe the structure
///   and layout of the vertex data in memory. These attributes are used to configure
///   vertex attribute pointers for shader program inputs and to interpret the vertex
///   buffer contents correctly during rendering.
///
/// # Example
///
/// ```
/// use shared_lib::gl_vertex::*;
/// use shared_lib::gl_vertex_attribute::*;
/// use shared_lib::gl_types::*;
///
/// struct MyVertex {
///     position: [f32; 3],
///     color: [f32; 4],
/// }
///
/// impl Vertex for MyVertex {
///     fn attributes(&self) -> Vec<VertexAttribute> {
///         vec![
///             VertexAttribute::new("position", 3, VertexDataType::Float, false, 28, 0),
///             VertexAttribute::new("color", 4, VertexDataType::Float, false, 28, 12),
///         ]
///     }
/// }
/// ```
///
/// In this example, `MyVertex` represents a custom vertex type with position and color
/// attributes. The `attributes` method returns a vector of `VertexAttribute` instances,
/// each describing one of the vertex's attributes, including its name, component count,
/// data type, normalization flag, stride, and offset within the vertex structure.
///
/// Note: The specific parameters and structure of `VertexAttribute` may vary based on
/// your implementation and the needs of your rendering engine.
pub trait Vertex {
    fn attributes() -> Vec<VertexAttribute>;
}
