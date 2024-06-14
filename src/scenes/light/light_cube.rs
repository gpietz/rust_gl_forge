use crate::render_context::RenderContext;
use crate::scene::{Scene, SceneResult};
use crate::vertex_data_3d::create_vbo;
use cgmath::{Point3, Vector3};
use shared_lib::camera::Camera;
use shared_lib::opengl::buffer_object::BufferObject;
use shared_lib::opengl::vertex_array_object::VertexArrayObject;
use shared_lib::vertices::textured_vertex::TexturedVertex;

#[derive(Default)]
pub struct LightCube {
    camera: Camera,
    light_pos: Option<Vector3<f32>>,
    vao: Option<VertexArrayObject>,
    vbo: Option<BufferObject<TexturedVertex>>,
}

impl Scene<RenderContext> for LightCube {
    fn activate(&mut self, _context: &mut RenderContext) -> SceneResult {
        if self.vao.is_none() {
            self.camera.position = Point3::new(0.0, 0.0, 3.0);
            self.light_pos = Some(Vector3::new(1.2, 1.0, 2.0));

            let vertex_data = crate::vertex_data_3d::create_cube();
            self.vao = Some(VertexArrayObject::new()?);
            self.vbo = Some(create_vbo(vertex_data));
        }
        Ok(())
    }

    fn draw(&mut self, _context: &mut RenderContext) -> SceneResult {
        todo!()
    }
}
