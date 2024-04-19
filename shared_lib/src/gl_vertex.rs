use crate::gl_vertex_attribute::VertexAttribute;
use crate::vertices::textured_vertex::TexturedVertex;

//////////////////////////////////////////////////////////////////////////////
// - Vertex -
//////////////////////////////////////////////////////////////////////////////

pub trait Vertex {
    fn attributes() -> Vec<VertexAttribute>;
    fn layout_size() -> usize {
        Self::attributes().iter().map(|attr| attr.calculate_size()).sum()
    }
}
