use cgmath::Vector3;

use shared_lib::gl_prelude::{BufferType, BufferUsage, VertexAttributeType};
use shared_lib::opengl::buffer_object::BufferObject;
use shared_lib::opengl::vertex_array_object::VertexArrayObject;

use crate::render_context::RenderContext;
use crate::resources::shaders;
use crate::scene::{Scene, SceneResult};

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

            let vao =
                VertexArrayObject::new_with_attribute_types(vec![VertexAttributeType::Position]);
            let vbo = BufferObject::new_with_vao(
                &vao,
                BufferType::ArrayBuffer,
                BufferUsage::StaticDraw,
                vertices,
            );
            let ibo = BufferObject::new_with_vao(
                &vao,
                BufferType::ElementArrayBuffer,
                BufferUsage::StaticDraw,
                indices,
            );

            self.vao = Some(vao);
            self.vbo = Some(vbo);
            self.ibo = Some(ibo);
        }
        Ok(())
    }

    fn draw(&mut self, context: &mut RenderContext) -> SceneResult {
        if let Some(vao) = self.vao.as_mut() {
            context
                .shader_manager()
                .activate_shader(shaders::SIMPLE_RED);
            vao.render(true, 6);
        }
        Ok(())
    }
}
