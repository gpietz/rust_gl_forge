use crate::gl_types::VertexAttributeType;
use crate::gl_vertex::Vertex;
use crate::gl_vertex_attribute::VertexAttribute;

//////////////////////////////////////////////////////////////////////////////
// - TexturedVertex2D -
//////////////////////////////////////////////////////////////////////////////

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct TexturedVertex2D {
    pub position: [f32; 2],   // XY coordinates
    pub tex_coords: [f32; 2], // UV texture coordinates
    pub color: [f32; 4],      // RGBA color values
}

impl TexturedVertex2D {
    pub fn new(x: f32, y: f32, u: f32, v: f32) -> TexturedVertex2D {
        TexturedVertex2D {
            position: [x, y],
            tex_coords: [u, v],
            color: [0.0, 0.0, 0.0, 1.0],
        }
    }
}

impl From<[f32; 4]> for TexturedVertex2D {
    fn from(value: [f32; 4]) -> Self {
        Self {
            position: [value[0], value[1]],
            tex_coords: [value[2], value[3]],
            color: [0.0, 0.0, 0.0, 1.0],
        }
    }
}

impl Vertex for TexturedVertex2D {
    fn attributes() -> Vec<VertexAttribute> {
        vec![
            VertexAttribute::new(VertexAttributeType::Position2D),
            VertexAttribute::new(VertexAttributeType::TexCoord),
            VertexAttribute::new(VertexAttributeType::Color),
        ]
    }
}
