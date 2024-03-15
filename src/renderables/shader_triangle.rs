use super::RenderContext;
use crate::renderables::Renderable;
use anyhow::Result;
use gl::types::GLfloat;
use shared_lib::{
    gl_draw,
    gl_prelude::{
        Bindable, BufferObject, BufferType, BufferUsage, PrimitiveType, ShaderFactory,
        ShaderProgram, VertexArrayObject, VertexAttribute, VertexAttributeSpecs,
        VertexAttributeType, VertexLayoutManager,
    },
    gl_types::VertexDataType,
    gl_vertex::Vertex,
};
use std::time::Instant;

//////////////////////////////////////////////////////////////////////////////
// - ShaderTriangle -
//////////////////////////////////////////////////////////////////////////////

pub struct ShaderTriangle {
    vao: VertexArrayObject,
    vbo: BufferObject<ColorPointVertex>,
    shader: ShaderProgram,
    vlm: VertexLayoutManager,
    use_uniform: bool,
    start_time: Instant,
}

impl ShaderTriangle {
    pub fn new(use_uniform: bool) -> Result<ShaderTriangle> {
        let start_time = Instant::now();

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

        let vao = VertexArrayObject::new(true)?;
        let vbo = BufferObject::new(BufferType::ArrayBuffer, BufferUsage::StaticDraw, vertices);

        // Create the shader program
        let shader = ShaderFactory::from_files(
            "assets/shaders/simple/shader_triangle.vert",
            "assets/shaders/simple/shader_triangle.frag",
        )?;

        // Setup the vertex layout
        let vlm = VertexLayoutManager::new_and_setup::<ColorPointVertex>(&shader)?;

        Ok(ShaderTriangle {
            vao,
            vbo,
            shader,
            vlm,
            use_uniform,
            start_time,
        })
    }

    fn get_current_time_in_seconds(&self) -> f64 {
        self.start_time.elapsed().as_secs_f64()
    }
}

impl Renderable for ShaderTriangle {
    fn draw(&mut self, _: f32) -> Result<()> {
        self.vao.bind()?;
        self.vbo.bind()?;
        self.shader.activate();

        let mut current_time = -1f64;
        if self.use_uniform {
            current_time = self.get_current_time_in_seconds();
        };

        let time_location = self.shader.get_uniform_location("time").unwrap();
        self.shader
            .set_uniform_value(time_location, current_time as GLfloat)
            .unwrap();

        gl_draw::draw_primitive(PrimitiveType::Triangles, 3);
        Ok(())
    }
}

//////////////////////////////////////////////////////////////////////////////
// - ColorPointVertex -
//////////////////////////////////////////////////////////////////////////////

struct ColorPointVertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

impl Vertex for ColorPointVertex {
    // The color must be specified with 3 components here because normally the color includes
    // a fourth value, which indicates the alpha channel (transparency).
    fn attributes() -> Vec<VertexAttribute> {
        vec![
            VertexAttribute::new(VertexAttributeType::Position),
            VertexAttribute::new(VertexAttributeType::Color)
                .with_attribute_specs(VertexAttributeSpecs::new(3, VertexDataType::Float, false)),
        ]
    }
}
