use crate::gl_buffer::BufferObject;
use crate::gl_draw;
use crate::gl_shader::{Shader, ShaderProgram};
use crate::gl_types::{
    BufferType, BufferUsage, IndicesValueType, PrimitiveType, ShaderType, VertexAttributeType,
};
use crate::gl_vertex::VertexArrayObject;
use crate::gl_vertex_attribute::VertexAttribute;
use crate::renderable::Renderable;
use anyhow::Result;
use cgmath::Vector3;
use gl::types::GLfloat;
use std::mem::size_of;

//////////////////////////////////////////////////////////////////////////////
// - IndexedQuad -
//////////////////////////////////////////////////////////////////////////////

pub struct IndexedQuad {
    vao: VertexArrayObject,
    vbo: BufferObject<Vector3<f32>>,
    ibo: BufferObject<u32>,
    position_attribute: VertexAttribute,
    shader: ShaderProgram,
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

        Ok(IndexedQuad {
            vao,
            vbo,
            ibo,
            position_attribute,
            shader,
        })
    }
}

impl Renderable for IndexedQuad {
    fn draw(&mut self) {
        self.vao.bind();
        self.vbo.bind();
        self.ibo.bind();
        self.shader.bind();
        gl_draw::draw_elements(PrimitiveType::Triangles, 6, IndicesValueType::Int);
    }
}
