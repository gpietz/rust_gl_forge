use std::fmt::{Display, Formatter};
use std::time::Instant;

use cgmath::{vec3, Deg, Matrix4, Rad, SquareMatrix};
use sdl2::keyboard::Keycode;

use shared_lib::gl_draw;
use shared_lib::gl_types::{IndicesValueType, PrimitiveType};
use shared_lib::opengl::buffer_object::BufferObject;
use shared_lib::opengl::texture::Texture;
use shared_lib::opengl::vertex_array_object::VertexArrayObject;
use shared_lib::opengl::vertex_layout::VertexLayout;
use shared_lib::sdl_window::SdlKeyboardState;
use shared_lib::vertices::textured_vertex::TexturedVertex;

use crate::render_context::RenderContext;
use crate::resources::{shaders, textures};
use crate::scene::{Scene, SceneResult};
use crate::scene_utils::query_texture;
use crate::vertex_data_2d;

const DEFAULT_ROTATION_SPEED: i32 = 16;
const MAX_ROTATION_SPEED: i32 = 512;
const ROTATION_SPEED_CHANGE: i32 = 16;
const MIN_SCALE: f32 = 0.5;
const MAX_SCALE: f32 = 1.5;
const SPEED_CHANGE_DELAY_MS: i32 = 250;

//////////////////////////////////////////////////////////////////////////////
// - Transformation  -
//////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct Transformation {
    vao: Option<VertexArrayObject>,
    vbo: Option<BufferObject<TexturedVertex>>,
    ibo: Option<BufferObject<u32>>,
    render_mode: RenderMode,
    textures: Vec<Texture>,
    rotation_angle: f32,
    rotation_speed: i32,
    rotation_paused: bool,
    scale_time: f32,
    last_speed_change: Option<Instant>,
}

impl Transformation {
    fn process_keyboard_input(&mut self, keyboard_state: &SdlKeyboardState) -> SceneResult {
        if keyboard_state.is_key_pressed(Keycode::F3) {
            self.render_mode = self.render_mode.next();
            println!("Render mode: {}", self.render_mode);
        }
        if keyboard_state.is_key_down(Keycode::Plus) && self.can_change_speed() {
            self.rotation_speed += ROTATION_SPEED_CHANGE;
            self.rotation_speed = self.rotation_speed.min(MAX_ROTATION_SPEED);
            self.print_rotation_speed();
        }
        if keyboard_state.is_key_down(Keycode::Minus) && self.can_change_speed() {
            self.rotation_speed -= ROTATION_SPEED_CHANGE;
            self.rotation_speed = self.rotation_speed.max(-MAX_ROTATION_SPEED);
            self.print_rotation_speed();
        }
        if keyboard_state.is_key_pressed(Keycode::R) {
            self.rotation_speed = DEFAULT_ROTATION_SPEED;
            self.print_rotation_speed();
        }
        if keyboard_state.is_key_pressed(Keycode::Space) {
            self.rotation_paused = !self.rotation_paused;
            #[rustfmt::skip]
            println!("Rotation {}", if self.rotation_paused { "paused"} else { "active" });
        }

        Ok(())
    }

    fn print_rotation_speed(&self) {
        println!("Rotation speed: {}", self.rotation_speed);
    }

    fn can_change_speed(&mut self) -> bool {
        match self.last_speed_change {
            Some(last_change) => {
                let current_time = Instant::now();
                if (current_time - last_change).as_millis() > SPEED_CHANGE_DELAY_MS as u128 {
                    self.last_speed_change = Some(current_time);
                    true
                } else {
                    false
                }
            }
            None => {
                self.last_speed_change = Some(Instant::now());
                true
            }
        }
    }
}

impl Scene<RenderContext> for Transformation {
    fn activate(&mut self, context: &mut RenderContext) -> SceneResult {
        if self.vao.is_none() {
            self.rotation_speed = DEFAULT_ROTATION_SPEED;

            let vertex_data = vertex_data_2d::create_quad();
            self.vao = Some(VertexArrayObject::new_with_attributes(
                TexturedVertex::attributes(),
            ));
            self.vbo = Some(vertex_data.create_vbo());
            self.ibo = Some(vertex_data.create_ibo());

            // Load textures
            self.textures
                .push(query_texture(context, textures::CRATE8)?);
            self.textures
                .push(query_texture(context, textures::AWESOMEFACE2)?);

            // Create shader program
            context
                .shader_manager()
                .get_shader(shaders::SIMPLE_TRANSFORM)?;
        }

        Ok(())
    }

    fn update(&mut self, context: &mut RenderContext) -> SceneResult {
        self.process_keyboard_input(context.keyboard_state())
    }

    fn update_tick(
        &mut self,
        _context: &mut RenderContext,
        delta_time: f32,
        _is_active: bool,
    ) -> SceneResult {
        if !self.rotation_paused {
            // Update rotation calculation
            self.rotation_angle += self.rotation_speed as f32 * delta_time;
            self.rotation_angle %= 360.0;
        }
        Ok(())
    }

    fn draw(&mut self, context: &mut RenderContext) -> SceneResult {
        if let (Some(vao), Some(ibo)) = (self.vao.as_mut(), self.ibo.as_ref()) {
            let delta_time = context.delta_time();

            // Activate textures
            self.textures[0].bind_as_unit(0);
            self.textures[1].bind_as_unit(1);

            // Activate shaders and bind to texture units
            let shader = context
                .shader_manager()
                .get_shader_mut(shaders::SIMPLE_TRANSFORM)?;
            shader.activate();
            shader.set_uniform("texture1", 0)?;
            shader.set_uniform("texture2", 1)?;

            // calculate rotation transformation
            let mut transform: Matrix4<f32> = Matrix4::identity();
            let rotation_angle_radians: Rad<f32> = Deg(self.rotation_angle).into();
            let required_render_cycles = match self.render_mode {
                RenderMode::SecondQuad
                | RenderMode::SecondQuadScale
                | RenderMode::SecondQuadScaleRotate => 2,
                _ => 1,
            };

            // calculate scaling transformation
            self.scale_time += delta_time * 1.0;

            match self.render_mode {
                RenderMode::Normal => {
                    transform = transform * Matrix4::from_angle_z(-rotation_angle_radians);
                }
                RenderMode::RotateTransform => {
                    transform = transform * Matrix4::from_angle_z(-rotation_angle_radians);
                    transform = transform * Matrix4::<f32>::from_translation(vec3(0.5, -0.5, 0.0));
                }
                _ => {
                    transform = transform * Matrix4::<f32>::from_translation(vec3(0.5, -0.5, 0.0));
                    transform = transform * Matrix4::from_angle_z(-rotation_angle_radians);
                }
            }

            for render_cycle in 0..required_render_cycles {
                // Get matrix uniform location an set matrix
                shader.set_uniform_matrix("transform", false, &transform)?;

                // Activate VAO buffer
                vao.bind();

                gl_draw::draw_elements(
                    PrimitiveType::Triangles,
                    ibo.data_len() as u32,
                    IndicesValueType::Int,
                );

                if render_cycle == 0 {
                    transform = Matrix4::identity();
                    transform = transform * Matrix4::<f32>::from_translation(vec3(-0.5, 0.5, 0.0));
                    match self.render_mode {
                        RenderMode::SecondQuad => {
                            transform = transform * Matrix4::from_angle_z(-rotation_angle_radians);
                        }
                        #[rustfmt::skip]
                        RenderMode::SecondQuadScale | RenderMode::SecondQuadScaleRotate => {
                            let scale_factor = (MAX_SCALE - MIN_SCALE) * 0.5 * (1.0 + (self.scale_time.cos())) + MIN_SCALE;
                            let scaling_matrix = Matrix4::from_scale(scale_factor);
                            transform = if self.render_mode == RenderMode::SecondQuadScale {
                                transform * scaling_matrix
                            } else {
                                transform * scaling_matrix * Matrix4::from_angle_z(-rotation_angle_radians)
                            };
                        }
                        _ => {}
                    }
                }
            }
        }

        Ok(())
    }
}

/////////////////////////////////////////////////////////////////////////////
// - RenderMode -
//////////////////////////////////////////////////////////////////////////////

#[derive(Default, Copy, Clone, PartialEq)]
enum RenderMode {
    #[default]
    Normal,
    TransformRotate,
    RotateTransform,
    SecondQuad,
    SecondQuadScale,
    SecondQuadScaleRotate,
}

impl RenderMode {
    fn next(self) -> Self {
        match self {
            RenderMode::Normal => RenderMode::TransformRotate,
            RenderMode::TransformRotate => RenderMode::RotateTransform,
            RenderMode::RotateTransform => RenderMode::SecondQuad,
            RenderMode::SecondQuad => RenderMode::SecondQuadScale,
            RenderMode::SecondQuadScale => RenderMode::SecondQuadScaleRotate,
            RenderMode::SecondQuadScaleRotate => RenderMode::Normal,
        }
    }
}

impl Display for RenderMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RenderMode::Normal => write!(f, "Normal"),
            RenderMode::TransformRotate => write!(f, "TransformRotate"),
            RenderMode::RotateTransform => write!(f, "RotateTransform"),
            RenderMode::SecondQuad => write!(f, "SecondQuad"),
            RenderMode::SecondQuadScale => write!(f, "SecondQuadScale"),
            RenderMode::SecondQuadScaleRotate => write!(f, "SecondQuadScaleRotate"),
        }
    }
}
