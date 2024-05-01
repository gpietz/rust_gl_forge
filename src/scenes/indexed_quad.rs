use cgmath::Vector3;

use shared_lib::gl_prelude::{BufferType, BufferUsage, VertexAttributeType, VertexLayoutManager};
use shared_lib::{
    gl_draw,
    gl_prelude::{Bindable, BufferObject, PrimitiveType, VertexArrayObject},
    gl_types::IndicesValueType,
};

use crate::render_context::RenderContext;
use crate::scene::{Scene, SceneError, SceneResult};
use crate::resources::shaders;

//////////////////////////////////////////////////////////////////////////////
// - IndexedQuad -
//////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct IndexedQuad {
    vao: Option<VertexArrayObject>,
    vbo: Option<BufferObject<Vector3<f32>>>,
    ibo: Option<BufferObject<u32>>,
}

impl Scene<RenderContext> for IndexedQuad {
    fn activate(&mut self, _context: &mut RenderContext) -> SceneResult {
        if self.vao.is_none() {
            let vertices = vec![
                Vector3::new(0.5, 0.5, 0.0),
                Vector3::new(0.5, -0.5, 0.0),
                Vector3::new(-0.5, -0.5, 0.0),
                Vector3::new(-0.5, 0.5, 0.0),
            ];
            let indices = vec![0, 1, 3, 1, 2, 3];

            let vao = VertexArrayObject::new().map_err(SceneError::VaoCreationError)?;
            let vbo = BufferObject::new(BufferType::ArrayBuffer, BufferUsage::StaticDraw, vertices);
            let ibo = BufferObject::new(
                BufferType::ElementArrayBuffer,
                BufferUsage::StaticDraw,
                indices,
            );

            VertexLayoutManager::from_attribute_types(vec![VertexAttributeType::Position])
                .setup_attributes()?;

            self.vao = Some(vao);
            self.vbo = Some(vbo);
            self.ibo = Some(ibo);
        }
        Ok(())
    }

    fn draw(&mut self, context: &mut RenderContext) -> SceneResult {
        if let Some(vao) = self.vao.as_mut() {
            vao.bind()?;
            
            let shader = context.shader_manager().get_shader(shaders::SIMPLE_RED);
            if let Ok(shader) = shader {
                shader.activate();
                gl_draw::draw_elements(PrimitiveType::Triangles, 6, IndicesValueType::Int);
            }
        }
        Ok(())
    }
}
