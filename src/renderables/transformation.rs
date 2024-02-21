use std::fmt::{Display, Formatter};
use std::time::Instant;

use anyhow::Result;
use cgmath::{vec3, Deg, Matrix4, Rad, SquareMatrix};
use sdl2::keyboard::Keycode;
use shared_lib::{
    gl_buffer::BufferObject,
    gl_draw,
    gl_shader::{ShaderFactory, ShaderProgram},
    gl_texture::Texture,
    gl_traits::Bindable,
    gl_types::{IndicesValueType, PrimitiveType},
    gl_vertex::{TexturedVertex, VertexArrayObject},
};

use crate::texture_utils::create_texture;

use super::Renderable;

const MAX_ROTATION_SPEED: i32 = 512;
const ROTATION_SPEED_CHANGE: i32 = 16;

//////////////////////////////////////////////////////////////////////////////
// - Transformation  -
//////////////////////////////////////////////////////////////////////////////

pub struct Transformation {
    vao: VertexArrayObject,
    vbo: BufferObject<TexturedVertex>,
    ibo: BufferObject<u32>,
    textures: [Texture; 2],
    shader: ShaderProgram,
    vertex_count: u32,
    start_time: Instant,
    rotation_angle: f32,
    render_mode: RenderMode,
    rotation_speed: i32,
}

impl Transformation {
    pub fn new() -> Result<Transformation> {
        // ** create vertex data ***
        let vertex_data = crate::vertex_data::create_quad();
        let vao = VertexArrayObject::new_and_bind()?;
        let vbo = vertex_data.create_vbo();
        let ibo = vertex_data.create_ibo();
        vertex_data.set_vertex_attributes();

        // *** load textures ***
        let textures = [
            create_texture("assets/textures/container.jpg", false, false)?,
            create_texture("assets/textures/awesomeface2.png", true, true)?,
        ];

        // *** create shader program ***
        let shader = ShaderFactory::from_files(
            "assets/shaders/transformation/transform.vs.glsl",
            "assets/shaders/transformation/transform.fs.glsl",
        )?;

        Ok(Transformation {
            vao,
            vbo,
            ibo,
            textures,
            shader,
            vertex_count: vertex_data.indices.len() as u32,
            start_time: Instant::now(),
            rotation_angle: 0.0,
            render_mode: RenderMode::Normal,
            rotation_speed: 16,
        })
    }

    fn get_seconds(&self) -> u32 {
        let duration = self.start_time.elapsed();
        let seconds = duration.as_secs() as f64 + duration.subsec_nanos() as f64 * 1e-9;
        seconds as u32
    }
}

impl Renderable for Transformation {
    fn draw(&mut self, delta_time: f32) -> Result<()> {
        // Activate buffers
        self.vao.bind()?;
        self.vbo.bind()?;
        self.ibo.bind()?;

        // Activate textures
        self.textures[0].bind_as_unit(0);
        self.textures[1].bind_as_unit(1);

        // Activate shaders and bind to texture units
        self.shader.bind();
        self.shader.set_uniform("texture1", 0)?;
        self.shader.set_uniform("texture2", 1)?;

        // create transformation
        self.rotation_angle += self.rotation_speed as f32 * delta_time;
        self.rotation_angle %= 360.0;

        let mut transform: Matrix4<f32> = Matrix4::identity();
        let rotation_angle_radians: Rad<f32> = Deg(self.rotation_angle).into();
        let required_render_cycles = match self.render_mode {
            RenderMode::SecondQuad => 2,
            _ => 1,
        };
        match self.render_mode {
            RenderMode::Normal => {
                transform = transform * Matrix4::from_angle_z(-rotation_angle_radians);
            }
            RenderMode::TransformRotate | RenderMode::SecondQuad => {
                transform = transform * Matrix4::<f32>::from_translation(vec3(0.5, -0.5, 0.0));
                transform = transform * Matrix4::from_angle_z(-rotation_angle_radians);
            }
            RenderMode::RotateTransform => {
                transform = transform * Matrix4::from_angle_z(-rotation_angle_radians);
                transform = transform * Matrix4::<f32>::from_translation(vec3(0.5, -0.5, 0.0));
            }
        }

        for render_cycle in 0..required_render_cycles {
            // Get matrix uniform location an set matrix
            self.shader.bind();
            self.shader
                .set_uniform_matrix("transform", false, &transform)?;

            gl_draw::draw_elements(
                PrimitiveType::Triangles,
                self.vertex_count,
                IndicesValueType::Int,
            );

            if self.render_mode == RenderMode::SecondQuad && render_cycle == 0 {
                transform = Matrix4::identity();
                transform = transform * Matrix4::<f32>::from_translation(vec3(-0.5, 0.5, 0.0));
                transform = transform * Matrix4::from_angle_z(-rotation_angle_radians);
            }
        }

        Ok(())
    }

    fn toggle_mode(&mut self) {
        self.render_mode = match self.render_mode {
            RenderMode::Normal => RenderMode::TransformRotate,
            RenderMode::TransformRotate => RenderMode::RotateTransform,
            RenderMode::RotateTransform => RenderMode::SecondQuad,
            RenderMode::SecondQuad => RenderMode::Normal,
        };
        println!("Render mode: {}", self.render_mode);
    }

    fn key_pressed(&mut self, key: &Keycode) -> bool {
        match key {
            Keycode::Plus => {
                self.rotation_speed =
                    (self.rotation_speed + ROTATION_SPEED_CHANGE).min(MAX_ROTATION_SPEED);
            }
            Keycode::Minus => {
                self.rotation_speed =
                    (self.rotation_speed - ROTATION_SPEED_CHANGE).max(-MAX_ROTATION_SPEED);
            }
            Keycode::R => {
                self.rotation_speed = 16; // Assuming 16 is a default or reset value
            }
            _ => return false, // Directly return false if none of the keys match
        }

        println!("Rotation speed: {}", self.rotation_speed);
        true // Return true if any of the keys match
    }
}

//////////////////////////////////////////////////////////////////////////////
// - RenderMode -
//////////////////////////////////////////////////////////////////////////////

#[derive(PartialEq)]
enum RenderMode {
    Normal,
    TransformRotate,
    RotateTransform,
    SecondQuad,
    // SecondQuadScale,
    // SecondQuadScaleRotate,
}

impl Display for RenderMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RenderMode::Normal => write!(f, "Normal"),
            RenderMode::TransformRotate => write!(f, "TransformRotate"),
            RenderMode::RotateTransform => write!(f, "RotateTransform"),
            RenderMode::SecondQuad => write!(f, "SecondQuad"),
        }
    }
}
