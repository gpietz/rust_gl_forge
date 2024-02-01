use crate::gl_types::VertexAttributeType;
use crate::gl_utils::*;
use anyhow::{anyhow, Context, Result};
use cgmath::{Vector2, Vector3};
use gl::types::{GLboolean, GLsizei};
use std::os::raw::c_void;

//////////////////////////////////////////////////////////////////////////////
// - Vertex -
//////////////////////////////////////////////////////////////////////////////

pub trait Vertex {
    fn size() -> usize;
    fn attributes() -> Vec<VertexAttribute>;
}

impl Vertex for cgmath::Vector2<f32> {
    fn size() -> usize {
        std::mem::size_of::<Vector2<f32>>()
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

impl Vertex for cgmath::Vector3<f32> {
    fn size() -> usize {
        std::mem::size_of::<Vector3<f32>>()
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
        std::mem::size_of::<Self>()
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
        std::mem::size_of::<u32>()
    }

    fn attributes() -> Vec<VertexAttribute> {
        vec![]
    }
}

//////////////////////////////////////////////////////////////////////////////
// - RgbVertex -
//////////////////////////////////////////////////////////////////////////////

pub struct RgbVertex {
    pub position: [f32; 3], // x, y, z
    pub color: [f32; 3],    // r, g, b
}

impl Vertex for RgbVertex {
    fn size() -> usize {
        std::mem::size_of::<Self>()
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
            offset: 3 * std::mem::size_of::<f32>(), // Offset after the position
        };

        vec![position_attr.clone(), color_attr.clone()]
    }
}

//////////////////////////////////////////////////////////////////////////////
// - VertexAttribute -
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct VertexAttribute {
    pub index: u32,
    pub size: i32,
    pub attribute_type: VertexAttributeType,
    pub normalized: bool,
    pub stride: usize,
    pub offset: usize,
}

impl VertexAttribute {
    pub fn new(
        index: u32,
        size: i32,
        attribute_type: VertexAttributeType,
        normalized: bool,
        stride: usize,
        offset: usize,
    ) -> VertexAttribute {
        VertexAttribute {
            index,
            size,
            attribute_type,
            normalized,
            stride,
            offset,
        }
    }

    pub fn setup(&self) -> Result<()> {
        let (_, data_type, _) = self.attribute_type.to_gl_data();
        unsafe {
            gl::EnableVertexAttribArray(self.index);
            gl::VertexAttribPointer(
                self.index,
                self.size,
                data_type,
                self.normalized as GLboolean,
                self.stride as GLsizei,
                self.offset as *const c_void,
            );
            check_gl_error().context("Failed to set up VertexAttribute")?;
        }
        Ok(())
    }

    pub fn enable(&self) -> Result<()> {
        unsafe {
            gl::EnableVertexAttribArray(self.index);
            check_gl_error().context("Failed to enable VertexAttribute")?;
        }
        Ok(())
    }

    pub fn disable(&self) -> Result<()> {
        unsafe {
            gl::DisableVertexAttribArray(self.index);
            check_gl_error().context("Failed to disable VertexAttribute")?;
        }
        Ok(())
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

    pub fn bind(&self) -> Result<()> {
        unsafe {
            gl::BindVertexArray(self.id);
        }
        check_gl_error()?;
        Ok(())
    }

    pub fn unbind(&self) -> Result<()> {
        unsafe {
            gl::BindVertexArray(0);
        }
        check_gl_error()?;
        Ok(())
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
