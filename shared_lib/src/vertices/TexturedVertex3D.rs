use crate::{
    gl_prelude::{Vertex, VertexAttribute},
    gl_types::VertexAttributeType,
    prelude::Color,
};

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
    fn default() -> Self {
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

    pub fn with_tex_coords(self, u: f32, v: f32) -> Self {
        Self {
            position: self.position,
            tex_coords: [u, v],
            ..Default::default()
        }
    }

    pub fn with_color(self, r: f32, g: f32, b: f32, a: f32) -> Self {
        Self {
            position: self.position,
            tex_coords: self.tex_coords,
            color: [r, g, b, a],
        }
    }

    pub fn set_tex_coords(&mut self, u: f32, v: f32) -> &mut Self {
        self.tex_coords = [u, v];
        self
    }

    pub fn set_color(&mut self, r: f32, g: f32, b: f32, a: f32) -> &mut Self {
        self.color = [r, g, b, a];
        self
    }

    pub fn set_color_ref(&mut self, color: &Color) -> &mut Self {
        self.color = [color.r, color.g, color.b, color.a];
        self
    }

    pub fn set_color_array(&mut self, color: [f32; 4]) -> &mut Self {
        self.color = [color[0], color[1], color[2], color[3]];
        self
    }

    pub fn set_rgb(&mut self, r: f32, g: f32, b: f32) -> &mut Self {
        self.color[0] = r;
        self.color[1] = g;
        self.color[2] = b;
        self
    }

    pub fn set_opacity(&mut self, opacity: f32) -> &mut Self {
        self.color[3] = opacity;
        self
    }
}

impl From<([f32; 3], [f32; 2], [f32; 4])> for TexturedVertex3D {
    fn from(value: ([f32; 3], [f32; 2], [f32; 4])) -> Self {
        let pos_array = value.0;
        let tex_array = value.1;
        let col_array = value.2;
        Self {
            position: [pos_array[0], pos_array[1], pos_array[2]],
            tex_coords: [tex_array[0], tex_array[1]],
            color: [col_array[0], col_array[1], col_array[2], col_array[3]],
        }
    }
}

impl Vertex for TexturedVertex3D {
    fn attributes() -> Vec<crate::gl_prelude::VertexAttribute> {
        vec![
            VertexAttribute::new(VertexAttributeType::Position),
            VertexAttribute::new(VertexAttributeType::TexCoord),
            VertexAttribute::new(VertexAttributeType::Color),
        ]
    }
}
