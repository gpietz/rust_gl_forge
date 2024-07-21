// use crate::render_context::RenderContext;
// use crate::resources::shaders;
// use crate::scene::{Scene, SceneResult};
// use crate::vertex_data_3d::create_vbo;
// use cgmath::{perspective, vec3, Deg, Matrix4, Point3, Vector3};
// use shared_lib::camera::Camera;
// use shared_lib::gl_draw;
// use shared_lib::gl_prelude::PrimitiveType;
// use shared_lib::opengl::buffer_object::BufferObject;
// use shared_lib::opengl::vertex_array_object::VertexArrayObject;
// //use shared_lib::vertices::textured_vertex::TexturedVertex;
//
// #[derive(Default)]
// pub struct LightCube {
//     camera: Camera,
//     light_pos: Option<Vector3<f32>>,
//     light_vao: Option<VertexArrayObject>,
//     vao: Option<VertexArrayObject>,
//     vbo: Option<BufferObject<TexturedVertex>>,
// }
//
// impl LightCube {
//     fn draw_cube(&mut self) -> SceneResult {
//         if let Some(vao) = self.vao.as_mut() {
//             vao.bind()?;
//             gl_draw::draw_arrays(PrimitiveType::Triangles, 0, 36);
//         }
//         Ok(())
//     }
// }
//
// impl Scene<RenderContext> for LightCube {
//     fn activate(&mut self, _context: &mut RenderContext) -> SceneResult {
//         if self.vao.is_none() {
//             self.camera.position = Point3::new(0.0, 0.0, 3.0);
//             self.light_pos = Some(Vector3::new(1.2, 1.0, 2.0));
//
//             let vertex_data = crate::vertex_data_3d::create_cube();
//             self.vao = Some(VertexArrayObject::new()?);
//             self.vbo = Some(create_vbo(vertex_data));
//         }
//         Ok(())
//     }
//
//     fn draw(&mut self, context: &mut RenderContext) -> SceneResult {
//         let screen_width = crate::SCREEN_WIDTH;
//         let screen_height = crate::SCREEN_HEIGHT;
//         let screen_aspect = screen_width as f32 / screen_height as f32;
//         let view_matrix = self.camera.get_view_matrix();
//
//         let shader = context
//             .shader_manager()
//             .get_shader_mut(shaders::LIGHT_CUBE)?;
//         shader.activate();
//         shader.set_uniform_matrix("projection", false, &view_matrix)?;
//
//         self.draw_cube()?;
//         Ok(())
//     }
// }
