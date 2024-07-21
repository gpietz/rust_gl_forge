use std::time::Instant;

use gl::types::GLfloat;

use shared_lib::gl_prelude::VertexDataType;
use shared_lib::opengl::buffer_object::BufferObject;
use shared_lib::opengl::vertex_array_object::VertexArrayObject;
use shared_lib::opengl::vertex_attribute::VertexAttribute;
use shared_lib::gl_prelude::{BufferType, BufferUsage, VertexAttributeType};
use shared_lib::opengl::vertex_layout::VertexLayout;
use crate::render_context::RenderContext;
use crate::resources::shaders;
use crate::scene::{Scene, SceneResult};

pub struct ShaderTriangle {
    use_uniform: bool,
    vao: Option<VertexArrayObject>,
    vbo: Option<BufferObject<ColorPointVertex>>,
    start_time: Option<Instant>,
}

impl ShaderTriangle {
    pub fn new(use_uniform: bool) -> ShaderTriangle {
        Self {
            use_uniform,
            vao: None,
            vbo: None,
            start_time: None,
        }
    }

    fn get_current_time_in_seconds(&self) -> f64 {
        self.start_time
            .map_or(0.0, |start_time| start_time.elapsed().as_secs_f64())
    }
}

impl Scene<RenderContext> for ShaderTriangle {
    fn activate(&mut self, _context: &mut RenderContext) -> SceneResult {
        if self.vao.is_none() {
            let vertices = vec![
                ColorPointVertex {
                    position: [0.5, -0.5, 0.0],
                    color: [1.0, 0.0, 0.0],
                },
                ColorPointVertex {
                    position: [-0.5, -0.5, 0.0],
                    color: [0.0, 1.0, 0.0],
                },
                ColorPointVertex {
                    position: [0.0, 0.5, 0.0],
                    color: [0.0, 0.0, 1.0],
                },
            ];

            let vao = VertexArrayObject::new_with_attributes(ColorPointVertex::attributes());
            let vbo = BufferObject::new_with_vao(&vao, BufferType::ArrayBuffer, BufferUsage::StaticDraw, vertices);

            self.vao = Some(vao);
            self.vbo = Some(vbo);
            self.start_time = Some(Instant::now());
        }
        Ok(())
    }

    fn draw(&mut self, context: &mut RenderContext) -> SceneResult {
        if let Some(vao) = self.vao.as_ref() {
            vao.bind();

            if let Ok(shader) = context
                .shader_manager()
                .get_shader_mut(shaders::SIMPLE_TRIANGLE)
            {
                let current_time = if self.use_uniform {
                    self.get_current_time_in_seconds()
                } else {
                    -1.0
                };

                if let Ok(time_location) = shader.get_uniform_location("time") {
                    shader.activate();
                    shader.set_uniform_value(time_location, current_time as GLfloat)?;
                    vao.render(false,  3);
                }
            }
        }
        Ok(())
    }
}

struct ColorPointVertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

impl VertexLayout for ColorPointVertex {
    fn attributes() -> Vec<VertexAttribute> {
        vec![
            VertexAttributeType::Position.into(),
            VertexAttribute::new(3, VertexDataType::Float),
        ]
    }
}
