#![allow(dead_code)]

use gl::types::{GLsizeiptr, GLvoid};
use gl::{
    BindBuffer, BindVertexArray, BufferData, DrawElements, GenBuffers, GenVertexArrays,
    ARRAY_BUFFER, ELEMENT_ARRAY_BUFFER, STATIC_DRAW, TRIANGLES, UNSIGNED_INT,
};
use std::mem::size_of;
use std::ptr::null;

//////////////////////////////////////////////////////////////////////////////
// - Vertex -
//////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
#[repr(C)]
pub struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
    tex_color: [f32; 2],
}

//////////////////////////////////////////////////////////////////////////////
// - BasicMesh  -
//////////////////////////////////////////////////////////////////////////////

pub struct BasicMesh {
    vao: u32,
    vbo: u32,
    ebo: u32,

    // Vertex and index data
    vertices: Vec<Vertex>,
    indices: Vec<Vertex>,
}

impl BasicMesh {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<Vertex>) -> Self {
        let mut mesh = BasicMesh {
            vao: 0,
            vbo: 0,
            ebo: 0,
            vertices,
            indices,
        };

        // Initialize OpenGL objects
        mesh.setup_mesh();

        mesh
    }

    fn setup_mesh(&mut self) {
        unsafe {
            // VAO setup
            GenVertexArrays(1, &mut self.vao);
            BindVertexArray(self.vao);

            // VBO setup
            GenBuffers(1, &mut self.vbo);
            BindBuffer(ARRAY_BUFFER, self.vbo);
            let size = (self.vertices.len() * size_of::<Vertex>()) as GLsizeiptr;
            let data = self.vertices.as_ptr() as *const GLvoid;
            BufferData(ARRAY_BUFFER, size, data, STATIC_DRAW);

            // EBO setup
            GenBuffers(1, &mut self.ebo);
            BindBuffer(ELEMENT_ARRAY_BUFFER, self.ebo);
            let size = (self.indices.len() * size_of::<u32>()) as GLsizeiptr;
            let data = self.indices.as_ptr() as *const GLvoid;
            BufferData(ELEMENT_ARRAY_BUFFER, size, data, STATIC_DRAW);

            // Vertex attribute pointer setup
            // This will depend on your vertex layout and shader attributes

            // Unbind the VAO to prevent accidental modificaton
            BindVertexArray(0);
        }
    }

    pub fn draw(&self) {
        //TODO  Bind required shaders and any textures

        unsafe {
            BindVertexArray(self.vao);
            DrawElements(TRIANGLES, self.indices.len() as i32, UNSIGNED_INT, null());
        }
    }
}
