use std::ffi::c_void;

use anyhow::Result;
use gl::types::{GLint, GLsizei, GLuint};

use crate::gl_traits::{Bindable, Deletable};
use crate::opengl::vertex_attribute::VertexAttribute;

//////////////////////////////////////////////////////////////////////////////
// - Vertex Array Object (VAO) -
//////////////////////////////////////////////////////////////////////////////

pub struct VertexArrayObject {
    id: u32,
    layout: Option<Vec<VertexAttribute>>,
}

impl VertexArrayObject {
    /// Create a new Vertex Array Object and bind it.
    pub fn new() -> Self {
        let mut id = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }
        Self {
            id,
            layout: None,
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }

    pub fn unbind() {
        unsafe {
            gl::BindVertexArray(0);
        }
    }

    fn is_bound(&self) -> Result<bool> {
        let mut current_vao = 0;
        unsafe {
            gl::GetIntegerv(gl::VERTEX_ARRAY_BINDING, &mut current_vao);
        }
        Ok(current_vao == self.id as GLint)
    }

    /// Returns the identifier of the vertex array.
    pub fn array_id(&self) -> u32 {
        self.id
    }

    pub fn set_layout<T: AsRef<[VertexAttribute]>>(&mut self, attributes: T) {
        let attributes = attributes.as_ref().to_vec();
        self.layout = Some(attributes.clone());

        self.bind();
        for (i, attr) in attributes.iter().enumerate() {
            let normalized = if let Some(n) = attr.normalized {
                if n {
                    gl::TRUE
                } else {
                    gl::FALSE
                }
            } else {
                gl::FALSE
            };
            let stride = attr.stride.unwrap_or(0) as GLsizei;
            let offset = attr.offset.unwrap_or(0) as *const c_void;
            let type_ = attr.data_type.to_gl_enum();
            let size = attr.data_type.size();

            unsafe {
                gl::VertexAttribPointer(
                    i as GLuint,
                    attr.components as GLint,
                    type_,
                    normalized,
                    stride,
                    offset,
                );
                gl::EnableVertexAttribArray(i as GLuint);
            }
        }
        Self::unbind();
    }

    pub fn clear_layout(&mut self) {
        if let Some(attributes) = &self.layout {
            self.bind();
            for (i, attr) in attributes.iter().enumerate() {
                unsafe {
                    gl::DisableVertexAttribArray(i as GLuint);
                }
            }
            self.layout = None;
            Self::unbind();
        }
    }
    
    pub fn has_layout(&self) -> bool {
         self.layout.is_some()
    }
}

impl Deletable for VertexArrayObject {
    fn delete(&mut self) -> Result<()> {
        if self.id != 0 {
            unsafe {
                gl::DeleteVertexArrays(1, &self.id);
            }
            self.id = 0;
        }
        Ok(())
    }
}

impl Drop for VertexArrayObject {
    fn drop(&mut self) {
        if let Err(err) = self.delete() {
            eprintln!("Error while dropping VertexArrayObject: {}", err);
            // You might choose to log the error or take other appropriate actions here.
        }
    }
}
