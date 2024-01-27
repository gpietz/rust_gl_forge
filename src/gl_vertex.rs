//////////////////////////////////////////////////////////////////////////////
// - Vertex -
//////////////////////////////////////////////////////////////////////////////

use crate::gl_types::VertexAttributeType;
use crate::gl_utils::*;
use anyhow::{Context, Result};
use cgmath::{Vector2, Vector3};
use gl::types::GLboolean;
use std::os::raw::c_void;

pub trait Vertex {
    fn size() -> i32;
    fn attributes() -> Vec<VertexAttribute>;
}

impl Vertex for cgmath::Vector2<f32> {
    fn size() -> i32 {
        std::mem::size_of::<Vector2<f32>>() as i32
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
    fn size() -> i32 {
        std::mem::size_of::<Vector3<f32>>() as i32
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
    fn size() -> i32 {
        std::mem::size_of::<Self>() as i32
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

//////////////////////////////////////////////////////////////////////////////
// - VertexAttribute -
//////////////////////////////////////////////////////////////////////////////

pub struct VertexAttribute {
    index: u32,
    size: i32,
    attribute_type: VertexAttributeType,
    normalized: bool,
    stride: i32,
    offset: usize,
}

impl VertexAttribute {
    pub fn new(
        index: u32,
        size: i32,
        attribute_type: VertexAttributeType,
        normalized: bool,
        stride: i32,
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
                self.stride,
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
