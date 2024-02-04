use crate::gl_buffer::BufferObject;
use crate::gl_shader::{Shader, ShaderProgram};
use crate::gl_types::{BufferType, BufferUsage, ShaderType, VertexAttributeType};
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

        // Load shaders
        let mut vertex_shader = Shader::from_file(
            "assets/shaders/simple_color/vertex_shader.glsl",
            ShaderType::Vertex,
        )?;
        let mut fragment_shader = Shader::from_file(
            "assets/shaders/simple_color/fragment_shader.glsl",
            ShaderType::Fragment,
        )?;

        // Create the shader program
        let shader = ShaderProgram::new(&mut vertex_shader, &mut fragment_shader)?;

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
        unsafe {
            self.vao.bind();
            self.vbo.bind();
            self.shader.bind();
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }
}
