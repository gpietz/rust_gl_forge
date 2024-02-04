use crate::gl_buffer::BufferObject;
use crate::gl_draw;
use crate::gl_shader::{Shader, ShaderProgram};
use crate::gl_types::{BufferType, BufferUsage, PrimitiveType, ShaderType};
use crate::gl_vertex::{RgbVertex, Vertex, VertexArrayObject};
use crate::renderable::Renderable;
use anyhow::Result;
use gl::types::GLfloat;
use std::time::Instant;

//////////////////////////////////////////////////////////////////////////////
// - IndexedQuad -
//////////////////////////////////////////////////////////////////////////////

pub struct ShaderTriangle {
    vao: VertexArrayObject,
    vbo: BufferObject<RgbVertex>,
    shader: ShaderProgram,
    use_uniform: bool,
    start_time: Instant,
}

impl ShaderTriangle {
    pub fn new(use_uniform: bool) -> Result<ShaderTriangle> {
        let start_time = Instant::now();

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
        let mut vertex_shader = Shader::from_file("assets/shaders/simpleVertexShader.glsl", ShaderType::Vertex)?;
        #[rustfmt::skip]
        let mut fragment_shader = Shader::from_file("assets/shaders/simpleFragmentShader.glsl", ShaderType::Fragment)?;

        // Create the shader program
        let shader = ShaderProgram::new(&mut vertex_shader, &mut fragment_shader)?;

        Ok(ShaderTriangle {
            vao,
            vbo,
            shader,
            use_uniform,
            start_time,
        })
    }

    fn get_current_time_in_seconds(&self) -> f64 {
        self.start_time.elapsed().as_secs_f64()
    }
}

impl Renderable for ShaderTriangle {
    fn draw(&mut self) {
        self.vao.bind();
        self.vbo.bind();
        self.shader.bind();

        let mut current_time = -1f64;
        if self.use_uniform {
            current_time = self.get_current_time_in_seconds();
        };

        let time_location = self.shader.get_uniform_location("time").unwrap();
        self.shader
            .set_uniform_value(time_location, current_time as GLfloat)
            .unwrap();

        gl_draw::draw_primitive(PrimitiveType::Triangles, 3);
    }
}
