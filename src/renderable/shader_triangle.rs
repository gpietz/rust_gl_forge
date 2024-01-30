use crate::gl_buffer::BufferObject;
use crate::gl_shader::{Shader, ShaderProgram};
use crate::gl_types::{BufferType, BufferUsage, ShaderType};
use crate::gl_vertex::{RgbVertex, Vertex, VertexArrayObject};
use crate::renderable::Renderable;
use anyhow::Result;

//////////////////////////////////////////////////////////////////////////////
// - IndexedQuad -
//////////////////////////////////////////////////////////////////////////////

pub struct ShaderTriangle {
    vao: VertexArrayObject,
    vbo: BufferObject<RgbVertex>,
    shader: ShaderProgram,
}

impl ShaderTriangle {
    pub fn new() -> Result<ShaderTriangle> {
        let vertices = vec![
            RgbVertex {
                position: [0.5, -0.5, 0.0],
                color: [1.0, 0.0, 0.0],
            },
            RgbVertex {
                position: [-0.5, -0.5, 0.0],
                color: [0.0, 1.0, 0.0],
            },
            RgbVertex {
                position: [0.0, 0.5, 0.0],
                color: [0.0, 0.0, 1.0],
            },
        ];

        let vao = VertexArrayObject::new()?;
        vao.bind();

        let vbo = BufferObject::new(BufferType::ArrayBuffer, BufferUsage::StaticDraw, vertices);
        vbo.bind();

        for attribute in RgbVertex::attributes() {
            attribute.setup()?;
            attribute.enable()?;
        }

        // Load shaders
        #[rustfmt::skip]
        let vertex_shader = Shader::from_file("assets/shaders/simpleVertexShader.glsl", ShaderType::Vertex)?;
        #[rustfmt::skip]
        let fragment_shader = Shader::from_file("assets/shaders/simpleFragmentShader.glsl", ShaderType::Fragment)?;

        // Create the shader program
        let shader = ShaderProgram::new(vertex_shader, fragment_shader)?;

        Ok(ShaderTriangle { vao, vbo, shader })
    }
}

impl Renderable for ShaderTriangle {
    fn draw(&mut self) {
        self.vao.bind();
        self.vbo.bind();
        self.shader.bind();
        unsafe {
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }
}
