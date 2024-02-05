use crate::gl_buffer::BufferObject;
use crate::gl_draw;
use crate::gl_shader::{ShaderFactory, ShaderProgram};
use crate::gl_types::{BufferType, BufferUsage, PrimitiveType, VertexAttributeType};
use crate::gl_vertex::VertexArrayObject;
use crate::gl_vertex_attribute::VertexAttribute;
use crate::renderable::Renderable;
use anyhow::Result;
use cgmath::Vector3;
use gl::types::GLfloat;

use std::mem::size_of;

//////////////////////////////////////////////////////////////////////////////
// - FirstTriangle -
//////////////////////////////////////////////////////////////////////////////

pub struct FirstTriangle {
    vao: VertexArrayObject,
    vbo: BufferObject<Vector3<f32>>,
    position_attribute: VertexAttribute,
    shader: ShaderProgram,
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
            3 * size_of::<GLfloat>(),
            0,
        );
        position.setup()?;
        position.enable()?;

        // Create shader program
        let shader = ShaderFactory::from_files(
            "assets/shaders/simple_color/vertex_shader.glsl",
            "assets/shaders/simple_color/fragment_shader.glsl",
        )?;

        Ok(FirstTriangle {
            vao,
            vbo,
            position_attribute: position,
            shader,
        })
    }
}

impl Renderable for FirstTriangle {
    fn draw(&mut self) {
        self.vao.bind();
        self.vbo.bind();
        self.shader.bind();
        gl_draw::draw_primitive(PrimitiveType::Triangles, 3);
    }
}
