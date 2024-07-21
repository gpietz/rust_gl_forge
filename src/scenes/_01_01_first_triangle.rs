use cgmath::Vector3;

use shared_lib::gl_prelude::BufferType;
use shared_lib::gl_prelude::BufferUsage;
use shared_lib::gl_prelude::VertexAttributeType;
use shared_lib::opengl::buffer_object::BufferObject;
use shared_lib::opengl::vertex_array_object::VertexArrayObject;

use crate::render_context::RenderContext;
use crate::resources::shaders;
use crate::scene::{Scene, SceneResult};

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

            let vao = VertexArrayObject::new_with_attribute_types(vec![VertexAttributeType::Position]);
            let vbo = BufferObject::new_with_vao(
                &vao,
                BufferType::ArrayBuffer,
                BufferUsage::StaticDraw,
                vertices,
            );
            
            self.vao = Some(vao);
            self.vbo = Some(vbo);
        }
        Ok(())
    }

    fn draw(&mut self, context: &mut RenderContext) -> SceneResult {
        if let Some(vao) = self.vao.as_mut() {
            context.shader_manager().activate_shader(shaders::SIMPLE_RED);
            vao.render(false, 3);
        }
        Ok(())
    }
}
