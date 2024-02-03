use crate::gl_types::VertexAttributeType;
use crate::gl_vertex_attribute::VertexAttribute;
use anyhow::{anyhow, Result};
use cgmath::{Vector2, Vector3};
use std::mem;
use std::mem::size_of;

//////////////////////////////////////////////////////////////////////////////
// - Vertex -
//////////////////////////////////////////////////////////////////////////////

pub trait Vertex {
    fn size() -> usize;
    fn attributes() -> Vec<VertexAttribute>;
}

impl Vertex for Vector2<f32> {
    fn size() -> usize {
        mem::size_of::<Vector2<f32>>()
    }

    fn attributes() -> Vec<VertexAttribute> {
        vec![VertexAttribute::new(
            0,
            2,
            VertexAttributeType::TexCoord,
            false,
            Self::size(),
            0,
        )]
    }
}

impl Vertex for Vector3<f32> {
    fn size() -> usize {
        mem::size_of::<Vector3<f32>>()
    }

    fn attributes() -> Vec<VertexAttribute> {
        vec![
            VertexAttribute::new(0, 3, VertexAttributeType::Position, false, Self::size(), 0),
            VertexAttribute {
                index: 1,
                size: 3, // r, g, b
                attribute_type: VertexAttributeType::Color,
                normalized: false,
                stride: Self::size(),
                offset: 3 * std::mem::size_of::<f32>(), // Offset after the position
            },
        ]
    }
}

impl Vertex for cgmath::Vector4<f32> {
    fn size() -> usize {
        mem::size_of::<Self>()
    }

    fn attributes() -> Vec<VertexAttribute> {
        vec![VertexAttribute::new(
            0,
            4,
            VertexAttributeType::Color,
            false,
            Self::size(),
            0,
        )]
    }
}

impl Vertex for u32 {
    fn size() -> usize {
        size_of::<u32>()
    }

    fn attributes() -> Vec<VertexAttribute> {
        vec![]
    }
}

//////////////////////////////////////////////////////////////////////////////
// - RgbVertex -
//////////////////////////////////////////////////////////////////////////////

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct RgbVertex {
    pub position: [f32; 3], // x, y, z
    pub color: [f32; 3],    // r, g, b
}

impl Vertex for RgbVertex {
    fn size() -> usize {
        mem::size_of::<Self>()
    }

    fn attributes() -> Vec<VertexAttribute> {
        let position_attr = VertexAttribute {
            index: 0,
            size: 3, // x, y, z
            attribute_type: VertexAttributeType::Position,
            normalized: false,
            stride: Self::size(),
            offset: 0,
        };

        let color_attr = VertexAttribute {
            index: 1,
            size: 3, // r, g, b
            attribute_type: VertexAttributeType::Color,
            normalized: false,
            stride: Self::size(),
            offset: 3 * size_of::<f32>(), // Offset after the position
        };

        vec![position_attr.clone(), color_attr.clone()]
    }
}

//////////////////////////////////////////////////////////////////////////////
// - TexturedVertex -
//////////////////////////////////////////////////////////////////////////////
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct TexturedVertex {
    pub position: [f32; 3],   // x, y, z
    pub tex_coords: [f32; 2], // uv coordinates
}

impl Vertex for TexturedVertex {
    fn size() -> usize {
        size_of::<[f32; 3]>() + size_of::<[f32; 2]>()
    }

    fn attributes() -> Vec<VertexAttribute> {
        let stride = Self::size();
        vec![
            VertexAttribute::new(0, 3, VertexAttributeType::Position, false, stride, 0),
            VertexAttribute::new(
                1,
                2,
                VertexAttributeType::TexCoord,
                false,
                stride,
                size_of::<Vector3<f32>>(),
            ), //
        ]
    }
}

//////////////////////////////////////////////////////////////////////////////
// - Vertex Array Object (VAO) -
//////////////////////////////////////////////////////////////////////////////

pub struct VertexArrayObject {
    id: u32,
}

impl VertexArrayObject {
    /// Create a new Vertex Array Object.
    pub fn new() -> Result<VertexArrayObject> {
        let mut id = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
            if id == 0 {
                return Err(anyhow!("Failed to generate a vertex array object"));
            }
        }
        Ok(VertexArrayObject { id })
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }

    pub fn get_vertex_array_id(&self) -> u32 {
        self.id
    }
}

impl Drop for VertexArrayObject {
    fn drop(&mut self) {
        if self.id != 0 {
            unsafe {
                gl::DeleteVertexArrays(1, &self.id);
            }
            self.id = 0;
        }
    }
}
