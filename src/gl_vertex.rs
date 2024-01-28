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
        vec![VertexAttribute::new(
            0,
            3,
            VertexAttributeType::Position,
            false,
            Self::size(),
            0,
        )]
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
// - VertexAttribute -
//////////////////////////////////////////////////////////////////////////////

pub struct VertexAttribute {
    index: u32,
    size: i32,
    attribute_type: VertexAttributeType,
    normalized: bool,
    stride: usize,
    offset: usize,
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
            check_gl_error().context("Failed to set up VertexAttribute")?;
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

    pub fn bind(&self) {
        unsafe { gl::BindVertexArray(self.id) }
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
