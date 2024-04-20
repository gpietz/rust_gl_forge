use anyhow::Result;
use cgmath::Vector3;

use shared_lib::{
    gl_draw::draw_primitive,
    gl_prelude::{
        Bindable, BufferObject, BufferType, BufferUsage, PrimitiveType, ShaderFactory,
        ShaderProgram, VertexArrayObject, VertexAttribute, VertexAttributeType,
        VertexLayoutManager,
    },
};

use crate::renderables::Renderable;

//////////////////////////////////////////////////////////////////////////////
// - FirstTriangle -
//////////////////////////////////////////////////////////////////////////////

pub struct FirstTriangle {
    vao: VertexArrayObject,
    vbo: BufferObject<Vector3<f32>>,
    shader: ShaderProgram,
    vlm: VertexLayoutManager,
}

impl FirstTriangle {
    pub fn new() -> Result<FirstTriangle> {
        let vertices = vec![
            Vector3::new(-0.5, -0.5, 0.0), // left
            Vector3::new(0.5, -0.5, 0.0),  // right
            Vector3::new(0.0, 0.5, 0.0),   // top
        ];

        let vao = VertexArrayObject::new(true)?;
        let vbo = BufferObject::new(BufferType::ArrayBuffer, BufferUsage::StaticDraw, vertices);

        // Create shader program
        let shader = ShaderFactory::from_files(
            "assets/shaders/simple/simple_red_shader.vert",
            "assets/shaders/simple/simple_red_shader.frag",
        )?;

        let mut vlm = VertexLayoutManager::empty();
        vlm.add_attribute(VertexAttributeType::Position.into())
            .setup_attributes_for_shader(shader.program_id())?;

        Ok(FirstTriangle {
            vao,
            vbo,
            shader,
            vlm,
        })
    }
}

impl Renderable for FirstTriangle {
    fn draw(&mut self, _: f32) -> Result<()> {
        self.vao.bind()?;
        self.vbo.bind()?;
        self.shader.activate();
        draw_primitive(PrimitiveType::Triangles, 3);
        Ok(())
    }
}
