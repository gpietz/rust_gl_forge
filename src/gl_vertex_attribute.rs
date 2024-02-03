use anyhow::Context;

//////////////////////////////////////////////////////////////////////////////
// - VertexAttribute -
//////////////////////////////////////////////////////////////////////////////

use crate::gl_types::VertexAttributeType;
use crate::gl_utils::{as_c_void, check_gl_error};
use gl::types::{GLboolean, GLsizei};

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

    pub fn setup(&self) -> anyhow::Result<()> {
        let (_, data_type, _) = self.attribute_type.to_gl_data();
        unsafe {
            gl::EnableVertexAttribArray(self.index);
            gl::VertexAttribPointer(
                self.index,
                self.size,
                data_type,
                self.normalized as GLboolean,
                self.stride as GLsizei,
                as_c_void(self.offset),
            );
            check_gl_error().context("Failed to set up VertexAttribute")?;
        }
        Ok(())
    }

    pub fn enable(&self) -> anyhow::Result<()> {
        unsafe {
            gl::EnableVertexAttribArray(self.index);
            check_gl_error().context("Failed to enable VertexAttribute")?;
        }
        Ok(())
    }

    pub fn disable(&self) -> anyhow::Result<()> {
        unsafe {
            gl::DisableVertexAttribArray(self.index);
            check_gl_error().context("Failed to disable VertexAttribute")?;
        }
        Ok(())
    }
}
