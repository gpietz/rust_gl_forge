use crate::color::Color;
use crate::gl_prelude::{Vertex, VertexAttribute, VertexAttributeType};
use crate::vertices::{VertexColor, VertexTexCoords};

//////////////////////////////////////////////////////////////////////////////
// - TexturedVertex -
//////////////////////////////////////////////////////////////////////////////

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct TexturedVertex {
    pub position: [f32; 3],   // XYZ coordinates
    pub tex_coords: [f32; 2], // UV texture coordinates
    pub color: [f32; 4],      // color of the vertex
}

impl TexturedVertex {
    pub fn new_xyz_uv(x: f32, y: f32, z: f32, u: f32, v: f32) -> TexturedVertex {
        Self {
            position: [x, y, z],
            tex_coords: [u, v],
            ..Default::default()
        }
    }
}

impl Default for TexturedVertex {
    fn default() -> Self {
        Self {
            position: [0.0, 0.0, 0.0],
            tex_coords: [0.0, 0.0],
            color: [0.0, 0.0, 0.0, 1.0],
        }
    }
}

impl Vertex for TexturedVertex {
    fn attributes() -> Vec<VertexAttribute> {
        vec![
            VertexAttribute::new(VertexAttributeType::Position),
            VertexAttribute::new(VertexAttributeType::TexCoord),
            VertexAttribute::new(VertexAttributeType::Color),
        ]
    }
}

impl VertexTexCoords for TexturedVertex {
    fn with_tex_coords(self, u: f32, v: f32) -> Self {
        Self {
            position: self.position,
            tex_coords: [u, v],
            color: self.color,
        }
    }

    fn set_tex_coords(&mut self, u: f32, v: f32) -> &mut Self {
        self.tex_coords = [u, v];
        self
    }
}

impl VertexColor for TexturedVertex {
    fn with_color(self, r: f32, g: f32, b: f32, a: f32) -> Self {
        Self {
            position: self.position,
            tex_coords: self.tex_coords,
            color: [r, g, b, a],
        }
    }

    fn set_color(&mut self, r: f32, g: f32, b: f32, a: f32) -> &mut Self {
        self.color = [r, g, b, a];
        self
    }

    fn set_color_ref(&mut self, color: &Color) -> &mut Self {
        self.color = [color.r, color.g, color.b, color.a];
        self
    }

    fn set_color_array(&mut self, color: [f32; 4]) -> &mut Self {
        self.color = [color[0], color[1], color[2], color[3]];
        self
    }

    fn set_rgb(&mut self, r: f32, g: f32, b: f32) -> &mut Self {
        self.color[0] = r;
        self.color[1] = g;
        self.color[2] = b;
        self
    }

    fn set_opacity(&mut self, opacity: f32) -> &mut Self {
        self.color[3] = opacity;
        self
    }
}
