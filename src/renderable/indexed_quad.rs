use crate::gl_buffer::BufferObject;
use crate::gl_types::{BufferType, BufferUsage, VertexAttributeType};
use crate::gl_vertex::{VertexArrayObject, VertexAttribute};
use crate::renderable::Renderable;
use anyhow::Result;
use cgmath::Vector3;
use gl::types::GLfloat;
use std::mem::size_of;
use std::ptr;

//////////////////////////////////////////////////////////////////////////////
// - IndexedQuad -
//////////////////////////////////////////////////////////////////////////////

pub struct IndexedQuad {
    vao: VertexArrayObject,
    vbo: BufferObject<Vector3<f32>>,
    ibo: BufferObject<u32>,
    position_attribute: VertexAttribute,
}

impl IndexedQuad {
    pub fn new() -> Result<IndexedQuad> {
        let vertices = vec![
            Vector3::new(0.5, 0.5, 0.0),
            Vector3::new(0.5, -0.5, 0.0),
            Vector3::new(-0.5, -0.5, 0.0),
            Vector3::new(-0.5, 0.5, 0.0),
        ];
        let indices = vec![0, 1, 3, 1, 2, 3];

        let vao = VertexArrayObject::new()?;
        vao.bind();

        let vbo = BufferObject::new(BufferType::ArrayBuffer, BufferUsage::StaticDraw, vertices);
        vbo.bind();

        let ibo = BufferObject::new(
            BufferType::ElementArrayBuffer,
            BufferUsage::StaticDraw,
            indices,
        );
        ibo.bind();

        let position_attribute = VertexAttribute::new(
            0,
            3,
            VertexAttributeType::Position,
            false,
            3 * size_of::<GLfloat>(),
            0,
        );
        position_attribute.setup()?;
        position_attribute.enable()?;

        Ok(IndexedQuad {
            vao,
            vbo,
            ibo,
            position_attribute,
        })
    }
}

impl Renderable for IndexedQuad {
    fn draw(&mut self) {
        unsafe {
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
        }
    }
}
