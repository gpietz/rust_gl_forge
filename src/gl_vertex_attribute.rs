use anyhow::Context;

//////////////////////////////////////////////////////////////////////////////
// - VertexAttribute -
//////////////////////////////////////////////////////////////////////////////

use crate::gl_types::VertexAttributeType;
use crate::gl_utils::{as_c_void, check_gl_error};
use gl::types::{GLboolean, GLenum, GLsizei};

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

    /// Checks if the vertex attribute array at the specified index is enabled.
    ///
    /// This function queries the OpenGL state to determine whether the vertex attribute array
    /// at the given index is enabled or disabled. Vertex attribute arrays are used to store
    /// per-vertex data that is used during rendering. Enabling an attribute array means that
    /// it is actively used in the rendering pipeline.
    ///
    /// # Parameters
    ///
    /// - `self`: A reference to the struct that implements this method.
    ///
    /// # Returns
    ///
    /// - `bool`: `true` if the vertex attribute array is enabled; `false` if it is disabled.
    ///
    /// # Example
    ///
    /// ```rust
    /// use gl_vertex_attribute::VertexAttribute;
    ///
    /// let vao = VertexAttribute::new(0, 0, ); // Create a new vertex attribute array at index 0
    /// vao.enable(); // Enable the vertex attribute array
    ///
    /// if vao.is_enabled() {
    ///     println!("Vertex attribute array at index 0 is enabled.");
    /// } else {
    ///     println!("Vertex attribute array at index 0 is disabled.");
    /// }
    /// ```
    ///
    /// # Safety
    ///
    /// This function uses unsafe OpenGL calls to query the state of the vertex attribute array.
    /// Ensure that the OpenGL context is properly initialized before calling this function.
    ///
    pub fn is_enabled(&self) -> bool {
        unsafe {
            let enabled = gl::IsEnabled(gl::VERTEX_ATTRIB_ARRAY_ENABLED | (self.index as GLenum));
            enabled == gl::TRUE
        }
    }
}
