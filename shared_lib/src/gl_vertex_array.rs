use crate::gl_prelude::check_gl_error;
use anyhow::{anyhow, Result};
use gl::types::GLint;

use crate::gl_traits::{Bindable, Deletable};

//////////////////////////////////////////////////////////////////////////////
// - Vertex Array Object (VAO) -
//////////////////////////////////////////////////////////////////////////////

pub struct VertexArrayObject {
    id: u32,
}

impl VertexArrayObject {
    /// Create a new Vertex Array Object and bind it.
    pub fn new() -> Result<VertexArrayObject> {
        let vao = VertexArrayObject::create_vao()?;
        unsafe {
            gl::BindVertexArray(vao.id);
            check_gl_error();
        }
        Ok(vao)
    }

    /// Create a new Vertex Array Object without binding it.
    pub fn new_without_bind() -> Result<VertexArrayObject> {
        VertexArrayObject::create_vao()
    }

    fn create_vao() -> Result<VertexArrayObject> {
        let mut id = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
            if id == 0 {
                return Err(anyhow!("Failed to generate a vertex array object"));
            }
            check_gl_error()?;
        }
        Ok(VertexArrayObject {
            id,
        })
    }

    pub fn bind(&self) -> Result<()> {
        unsafe {
            gl::BindVertexArray(self.id);
        }
        check_gl_error()
    }

    pub fn unbind(&self) -> Result<()> {
        unsafe {
            gl::BindVertexArray(0);
        }
        check_gl_error()
    }

    /// Returns the identifier of the vertex array.
    pub fn array_id(&self) -> u32 {
        self.id
    }
}

impl Bindable for VertexArrayObject {
    fn bind(&self) -> Result<()> {
        unsafe {
            gl::BindVertexArray(self.id);
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
        let mut current_vao = 0;
        unsafe {
            gl::GetIntegerv(gl::VERTEX_ARRAY_BINDING, &mut current_vao);
            check_gl_error()?;
        }
        Ok(current_vao == self.id as GLint)
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
