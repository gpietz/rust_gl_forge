use crate::gl_prelude::Bindable;
use crate::gl_types::VertexAttributeType;
use crate::gl_utils::check_gl_error;
use anyhow::Result;
use crate::opengl::vertex_attribute::VertexAttribute;
use crate::opengl::vertex_layout_manager::VertexLayoutError;

//////////////////////////////////////////////////////////////////////////////
// - Vertex -
//////////////////////////////////////////////////////////////////////////////

pub trait Vertex {
    fn attributes() -> Vec<VertexAttribute>;
    fn layout_size() -> usize {
        Self::attributes().iter().map(|attr| attr.calculate_size()).sum()
    }
}

//////////////////////////////////////////////////////////////////////////////
// - VertexLayout -
//////////////////////////////////////////////////////////////////////////////

pub struct VertexLayout {
    vao_id: u32,
    attributes: Vec<VertexAttribute>,
    is_setup: bool,
    shader_id: Option<u32>,
}

impl VertexLayout {
    pub fn new() -> Result<Self, VertexLayoutError> {
        let vao_id = unsafe { Self::create_vao()? };
        Ok(VertexLayout {
            vao_id,
            attributes: Vec::new(),
            is_setup: false,
            shader_id: None,
        })
    }

    unsafe fn create_vao() -> Result<u32, VertexLayoutError> {
        let mut vao_id = 0;
        gl::GenVertexArrays(1, &mut vao_id);
        if vao_id == 0 {
            return Err(VertexLayoutError::OpenGL("Failed to generate VAO".to_string()));
        }
        gl::BindVertexArray(vao_id);
        Ok(vao_id)
    }

    pub fn new_with_vao_id(vao_id: u32) -> Result<Self, VertexLayoutError> {
        if vao_id == 0 {
            return Err(VertexLayoutError::InvalidVAOId);
        }

        Ok(VertexLayout {
            vao_id,
            attributes: Vec::new(),
            is_setup: false,
            shader_id: None,
        })
    }

    pub fn vao_id(&self) -> u32 {
        self.vao_id
    }
}

impl Bindable for VertexLayout {
    fn bind(&self) -> Result<()> {
        unsafe {
            gl::BindVertexArray(self.vao_id);
        }
        check_gl_error()
    }

    fn unbind(&self) -> Result<()> {
        unsafe {
            gl::BindVertexArray(0);
        }
        check_gl_error()
    }

    fn is_bound(&self) -> Result<bool> {
        todo!()
    }
}
