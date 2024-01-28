//////////////////////////////////////////////////////////////////////////////
// - FirstTriangle -
//////////////////////////////////////////////////////////////////////////////

use crate::gl_buffer::BufferObject;
use crate::gl_types::{BufferType, BufferUsage, VertexAttributeType};
use crate::gl_vertex::{VertexArrayObject, VertexAttribute};
use crate::renderable::Renderable;
use anyhow::Result;
use cgmath::Vector3;
use gl::types::{GLfloat, GLsizei};
use std::mem::size_of;

pub struct FirstTriangle {
    vao: VertexArrayObject,
    vbo: BufferObject<Vector3<f32>>,
    position_attribute: VertexAttribute,
}

impl FirstTriangle {
    pub fn new() -> Result<FirstTriangle> {
        let vertices = vec![
            Vector3::new(-0.5, -0.5, 0.0), // left
            Vector3::new(0.5, -0.5, 0.0),  // right
            Vector3::new(0.0, 0.5, 0.0),   // top
        ];

        let vao = VertexArrayObject::new()?;
        vao.bind();

        let vbo = BufferObject::new(BufferType::ArrayBuffer, BufferUsage::StaticDraw, vertices);
        vbo.bind();

        let position = VertexAttribute::new(
            0,
            3,
            VertexAttributeType::Position,
            false,
            3 * size_of::<GLfloat>() as GLsizei,
            0,
        );
        position.setup()?;
        position.enable()?;

        Ok(FirstTriangle {
            vao,
            vbo,
            position_attribute: position,
        })
    }
}

impl Renderable for FirstTriangle {
    fn draw(&mut self) {
        unsafe {
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }
}
