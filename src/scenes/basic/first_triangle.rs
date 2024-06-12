use cgmath::Vector3;

use shared_lib::{
    gl_draw::draw_primitive,
    gl_prelude::{
        BufferObject, BufferType, BufferUsage, PrimitiveType
        , VertexArrayObject, VertexAttributeType, VertexLayoutManager,
    },
};

use crate::render_context::RenderContext;
use crate::resources::shaders;
use crate::scene::{Scene, SceneError, SceneResult};

//////////////////////////////////////////////////////////////////////////////
// - FirstTriangle -
//////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct FirstTriangle {
    vao: Option<VertexArrayObject>,
    vbo: Option<BufferObject<Vector3<f32>>>,
}

impl Scene<RenderContext> for FirstTriangle {
    fn activate(&mut self, _render_context: &mut RenderContext) -> SceneResult {
        if self.vao.is_none() {
            let vertices = vec![
                Vector3::new(-0.5, -0.5, 0.0), // left
                Vector3::new(0.5, -0.5, 0.0),  // right
                Vector3::new(0.0, 0.5, 0.0),   // top
            ];

            let vao = VertexArrayObject::new().map_err(SceneError::VaoCreationError)?;
            let vbo = BufferObject::new(BufferType::ArrayBuffer, BufferUsage::StaticDraw, vertices);
            VertexLayoutManager::from_attribute_types(vec![VertexAttributeType::Position])
                .setup_attributes()?;

            self.vao = Some(vao);
            self.vbo = Some(vbo);
        }
        Ok(())
    }

    fn draw(&mut self, context: &mut RenderContext) -> SceneResult {
        if let Some(vao) = self.vao.as_mut() {
            vao.bind()?;

            let shader = context.shader_manager().get_shader(shaders::SIMPLE_RED);
            if let Ok(shader) = shader {
                shader.activate();
                draw_primitive(PrimitiveType::Triangles, 3);
            }
        }
        Ok(())
    }
}
