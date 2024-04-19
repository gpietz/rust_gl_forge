#![allow(dead_code)]
use crate::meshes::DynamicVertex;
use anyhow::Result;
use gl::types::{GLint, GLsizeiptr, GLvoid};
use gl::{
    BindBuffer, BindVertexArray, BufferData, DeleteBuffers, DeleteVertexArrays, DrawArrays,
    DrawElements, GenBuffers, GenVertexArrays, ARRAY_BUFFER, ELEMENT_ARRAY_BUFFER, STATIC_DRAW,
    TRIANGLES, UNSIGNED_INT,
};
use std::mem::size_of;
use std::ptr::null;

//////////////////////////////////////////////////////////////////////////////
// - BasicMesh  -
//////////////////////////////////////////////////////////////////////////////

pub struct BasicMesh {
    vao: u32,
    vbo: u32,
    ebo: u32,

    // Vertex and index data
    vertices: usize,
    indices: Vec<u32>,
}

impl BasicMesh {
    pub fn new(vertices: Vec<Box<dyn DynamicVertex>>) -> Result<Self> {
        let vertices_len = vertices.len();

        // Collect all vertex data into a single buffer
        let mut vertex_data: Vec<u8> = Vec::new();
        for vertex in vertices.into_iter() {
            let data = vertex.as_bytes();
            vertex_data.extend_from_slice(data);
        }

        let mut vao: u32 = 0;
        let mut vbo: u32 = 0;

        unsafe {
            // Create the vertex array object
            GenVertexArrays(1, &mut vao);
            BindVertexArray(vao);

            // Create the vertex buffer object
            GenBuffers(1, &mut vbo);
            BindBuffer(ARRAY_BUFFER, vbo);
            let data = vertex_data.as_ptr() as *const GLvoid;
            let size = vertex_data.len() as GLsizeiptr;

            BufferData(ARRAY_BUFFER, size, data, STATIC_DRAW);
        }

        Ok(BasicMesh {
            vao,
            vbo,
            ebo: 0,
            vertices: vertices_len,
            indices: vec![],
        })
    }

    pub fn add_indices(&mut self, indices: impl IntoIterator<Item = u32>) -> Result<()> {
        unsafe {
            BindVertexArray(self.vao);

            if self.ebo == 0 {
                GenBuffers(1, &mut self.ebo);
            }
            BindBuffer(ELEMENT_ARRAY_BUFFER, self.ebo);

            self.indices = indices.into_iter().collect();
            let size = (self.indices.len() * size_of::<u32>()) as GLsizeiptr;
            let data = self.indices.as_ptr() as *const GLvoid;

            BufferData(ELEMENT_ARRAY_BUFFER, size, data, STATIC_DRAW);
        }

        Ok(())
    }

    pub fn clear_indices(&mut self) {
        unsafe {
            BindVertexArray(self.vao);
            BindBuffer(ELEMENT_ARRAY_BUFFER, self.ebo);
            BufferData(ELEMENT_ARRAY_BUFFER, 0, null(), STATIC_DRAW);
        }
        self.indices.clear();
    }

    pub fn has_indices(&self) -> bool {
        self.indices.len() > 0
    }

    pub fn draw(&self) {
        unsafe {
            BindVertexArray(self.vao);

            if self.has_indices() {
                DrawElements(TRIANGLES, self.indices.len() as i32, UNSIGNED_INT, null());
            } else {
                DrawArrays(TRIANGLES, 0, self.vertices as GLint);
            }
        }
    }
}

impl Drop for BasicMesh {
    fn drop(&mut self) {
        unsafe {
            if self.vbo != 0 {
                DeleteBuffers(1, &self.vbo);
            }
            if self.ebo != 0 {
                DeleteBuffers(1, &self.ebo);
            }
            if self.vao != 0 {
                DeleteVertexArrays(1, &self.vao);
            }
        }
    }
}
