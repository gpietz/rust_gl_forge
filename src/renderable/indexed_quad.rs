use crate::renderable::Renderable;
use anyhow::Result;
use cgmath::Vector3;
use gl::types::GLfloat;
use shared_lib::gl_buffer::BufferObject;
use shared_lib::gl_draw;
use shared_lib::gl_shader::{ShaderFactory, ShaderProgram};
use shared_lib::gl_traits::Bindable;
use shared_lib::gl_types::{
    BufferType, BufferUsage, IndicesValueType, PrimitiveType, VertexAttributeType,
};
use shared_lib::gl_vertex::VertexArrayObject;
use shared_lib::gl_vertex_attribute::VertexAttribute;
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

        let vao = VertexArrayObject::new_and_bind()?;
        let vbo =
            BufferObject::new_and_bind(BufferType::ArrayBuffer, BufferUsage::StaticDraw, vertices);
        let ibo = BufferObject::new_and_bind(
            BufferType::ElementArrayBuffer,
            BufferUsage::StaticDraw,
            indices,
        );

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

        // Create shader program
        let shader = ShaderFactory::from_files(
            "assets/shaders/simple_color/vertex_shader.glsl",
            "assets/shaders/simple_color/fragment_shader.glsl",
        )?;

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
    fn draw(&mut self, _: f32) -> Result<()> {
        self.vao.bind()?;
        self.vbo.bind()?;
        self.ibo.bind()?;
        self.shader.bind();
        gl_draw::draw_elements(PrimitiveType::Triangles, 6, IndicesValueType::Int);
        Ok(())
    }
}
