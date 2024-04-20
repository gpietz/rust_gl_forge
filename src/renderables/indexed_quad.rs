use anyhow::Result;
use cgmath::Vector3;

use shared_lib::{
    gl_draw,
    gl_prelude::{
        Bindable, BufferObject, BufferType, BufferUsage, PrimitiveType, ShaderFactory,
        ShaderProgram, VertexArrayObject, VertexAttributeType, VertexLayoutManager,
    },
    gl_types::IndicesValueType,
};

use crate::renderables::Renderable;

//////////////////////////////////////////////////////////////////////////////
// - IndexedQuad -
//////////////////////////////////////////////////////////////////////////////

pub struct IndexedQuad {
    vao: VertexArrayObject,
    vbo: BufferObject<Vector3<f32>>,
    ibo: BufferObject<u32>,
    shader: ShaderProgram,
    vlm: VertexLayoutManager,
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

        let vao = VertexArrayObject::new(true)?;
        let vbo = BufferObject::new(BufferType::ArrayBuffer, BufferUsage::StaticDraw, vertices);
        let ibo = BufferObject::new(
            BufferType::ElementArrayBuffer,
            BufferUsage::StaticDraw,
            indices,
        );

        // Create shader program
        let shader = ShaderFactory::from_files(
            "assets/shaders/simple/simple_red_shader.vert",
            "assets/shaders/simple/simple_red_shader.frag",
        )?;

        let mut vlm = VertexLayoutManager::empty();
        vlm.add_attribute(VertexAttributeType::Position.into())
            .setup_attributes_for_shader(shader.program_id())?;

        Ok(IndexedQuad {
            vao,
            vbo,
            ibo,
            shader,
            vlm,
        })
    }
}

impl Renderable for IndexedQuad {
    fn draw(&mut self, _: f32) -> Result<()> {
        self.vao.bind()?;
        self.vbo.bind()?;
        self.ibo.bind()?;
        self.shader.activate();
        gl_draw::draw_elements(PrimitiveType::Triangles, 6, IndicesValueType::Int);
        Ok(())
    }
}
