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
    /// Create a new Vertex Array Object.
    pub fn new() -> Result<VertexArrayObject> {
        let vao = VertexArrayObject::create_vao()?;
        unsafe {
            gl::BindVertexArray(vao.id);
        }
        Ok(vao)
    }

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
        }
        Ok(VertexArrayObject { id })
    }

    /// Returns the identifier of the vertex array.
    pub fn array_id(&self) -> u32 {
        self.id
    }
}

impl Bindable for VertexArrayObject {
    type Target = VertexArrayObject;

    fn bind(&mut self) -> Result<&mut Self::Target> {
        unsafe {
            gl::BindVertexArray(self.id);
        }
        Ok(self)
    }

    fn unbind(&mut self) -> Result<&mut Self::Target> {
        unsafe {
            gl::BindVertexArray(0);
        }
        Ok(self)
    }

    fn is_bound(&self) -> bool {
        let mut current_vao = 0;
        unsafe {
            gl::GetIntegerv(gl::VERTEX_ARRAY_BINDING, &mut current_vao);
        }
        current_vao == self.id as GLint
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
