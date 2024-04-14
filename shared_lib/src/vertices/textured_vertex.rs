use crate::color::Color;
use crate::gl_prelude::{Vertex, VertexAttribute, VertexAttributeType};
use crate::vertices::{VertexColor, VertexTexCoords};

//////////////////////////////////////////////////////////////////////////////
// - TexturedVertex -
//////////////////////////////////////////////////////////////////////////////

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub enum TexturedVertex {
    Vertex2D(TexturedVertex2D),
    Vertex3D(TexturedVertex3D),
}

impl Default for TexturedVertex {
    fn default() -> Self {
        TexturedVertex::Vertex3D(TexturedVertex3D::default())
    }
}

impl VertexTexCoords for TexturedVertex {
    fn with_tex_coords(self, u: f32, v: f32) -> Self {
        match self {
            TexturedVertex::Vertex2D(mut vertex) => {
                vertex.tex_coords = [u, v];
                TexturedVertex::Vertex2D(vertex)
            }
            TexturedVertex::Vertex3D(mut vertex) => {
                vertex.tex_coords = [u, v];
                TexturedVertex::Vertex3D(vertex)
            }
        }
    }

    fn set_tex_coords(&mut self, u: f32, v: f32) -> &mut Self {
        match self {
            TexturedVertex::Vertex2D(vertex) => {
                vertex.tex_coords = [u, v];
            }
            TexturedVertex::Vertex3D(vertex) => {
                vertex.tex_coords = [u, v];
            }
        }
        self
    }
}

impl VertexColor for TexturedVertex {
    fn with_color(self, r: f32, g: f32, b: f32, a: f32) -> Self {
        match self {
            TexturedVertex::Vertex2D(mut vertex) => {
                vertex.color = [r, g, b, a];
                TexturedVertex::Vertex2D(vertex)
            }
            TexturedVertex::Vertex3D(mut vertex) => {
                vertex.color = [r, g, b, a];
                TexturedVertex::Vertex3D(vertex)
            }
        }
    }

    fn set_color(&mut self, r: f32, g: f32, b: f32, a: f32) -> &mut Self {
        match self {
            TexturedVertex::Vertex2D(vertex) => {
                vertex.color = [r, g, b, a];
            }
            TexturedVertex::Vertex3D(vertex) => {
                vertex.color = [r, g, b, a];
            }
        }
        self
    }

    fn set_color_ref(&mut self, color: &Color) -> &mut Self {
        match self {
            TexturedVertex::Vertex2D(vertex) => {
                vertex.color = [color.r, color.g, color.b, color.a];
            }
            TexturedVertex::Vertex3D(vertex) => {
                vertex.color = [color.r, color.g, color.b, color.a];
            }
        }
        self
    }

    fn set_color_array(&mut self, color: [f32; 4]) -> &mut Self {
        match self {
            TexturedVertex::Vertex2D(vertex) => {
                vertex.color = [color[0], color[1], color[2], color[3]];
            }
            TexturedVertex::Vertex3D(vertex) => {
                vertex.color = [color[0], color[1], color[2], color[3]];
            }
        }
        self
    }

    fn set_rgb(&mut self, r: f32, g: f32, b: f32) -> &mut Self {
        match self {
            TexturedVertex::Vertex2D(vertex) => {
                vertex.color[0] = r;
                vertex.color[1] = g;
                vertex.color[2] = b;
            }
            TexturedVertex::Vertex3D(vertex) => {
                vertex.color[0] = r;
                vertex.color[1] = g;
                vertex.color[2] = b;
            }
        }
        self
    }

    fn set_opacity(&mut self, opacity: f32) -> &mut Self {
        match self {
            TexturedVertex::Vertex2D(vertex) => {
                vertex.color[3] = opacity;
            }
            TexturedVertex::Vertex3D(vertex) => {
                vertex.color[3] = opacity;
            }
        }
        self
    }
}

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

impl Vertex for TexturedVertex2D {
    fn attributes() -> Vec<VertexAttribute> {
        vec![
            VertexAttribute::new(VertexAttributeType::Position2D),
            VertexAttribute::new(VertexAttributeType::TexCoord),
            VertexAttribute::new(VertexAttributeType::Color),
        ]
    }
}

//////////////////////////////////////////////////////////////////////////////
// - TexturedVertex3D -
//////////////////////////////////////////////////////////////////////////////

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct TexturedVertex3D {
    pub position: [f32; 3],   // XYZ coordinates
    pub tex_coords: [f32; 2], // UV texture coordinates
    pub color: [f32; 4],      // color of the vertex
}

impl Default for TexturedVertex3D {
    fn default() -> TexturedVertex3D {
        Self {
            position: [0.0, 0.0, 0.0],
            tex_coords: [0.0, 0.0],
            color: [0.0, 0.0, 0.0, 1.0],
        }
    }
}

impl TexturedVertex3D {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            position: [x, y, z],
            ..Default::default()
        }
    }
}

impl Vertex for TexturedVertex3D {
    fn attributes() -> Vec<VertexAttribute> {
        vec![
            VertexAttribute::new(VertexAttributeType::Position),
            VertexAttribute::new(VertexAttributeType::TexCoord),
            VertexAttribute::new(VertexAttributeType::Color),
        ]
    }
}
