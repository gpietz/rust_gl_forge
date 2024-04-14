use std::fmt::{Display, Formatter};

use anyhow::Result;
use cgmath::{Deg, Matrix4, perspective, vec3};
use sdl2::keyboard::Keycode;

use shared_lib::{
    gl_draw,
    gl_prelude::{
        Bindable, BufferObject, PrimitiveType, ShaderProgram, VertexArrayObject,
        VertexLayoutManager,
    },
    gl_texture::Texture,
};
use shared_lib::vertices::textured_vertex::TexturedVertex3D;

use crate::{renderables::Renderable, texture_utils::create_texture};
use crate::vertex_data_3d::create_vbo;

const MAX_MODEL_DISTANCE: f32 = -16.0;
const MIN_MODEL_DISTANCE: f32 = -1.0;
const MODEL_DISTANCE_SPEED: f32 = 0.05;

//////////////////////////////////////////////////////////////////////////////
// - Transformation  -
//////////////////////////////////////////////////////////////////////////////

pub struct Projection {
    vao: VertexArrayObject,
    vbo: BufferObject<TexturedVertex3D>,
    textures: [Texture; 2],
    shader: ShaderProgram,
    vlm: VertexLayoutManager,
    rotation_angle: f32,
    rotation_speed: i32,
    scale_time: f32,
    model_distance: f32,
    render_mode: RenderMode,
}

impl Projection {
    pub fn new() -> Result<Projection> {
        // ** create vertex data ***
        let vao = VertexArrayObject::new(true)?;
        let vbo = create_vbo(crate::vertex_data_3d::create_cube());

        // *** load textures ***
        let textures = [
            create_texture("assets/textures/crate8.jpg", false, false)?,
            create_texture("assets/textures/awesomeface2.png", true, true)?,
        ];

        // *** create shader program ***
        let shader = ShaderProgram::from_files(&[
            "assets/shaders/simple/projection.vert",
            "assets/shaders/simple/projection.frag",
        ])?;

        // Create vertex layout
        let vlm = VertexLayoutManager::new_and_setup::<TexturedVertex3D>(&shader)?;

        Ok(Projection {
            vao,
            vbo,
            textures,
            shader,
            vlm,
            rotation_angle: 0.0,
            rotation_speed: 16,
            scale_time: 0.0,
            model_distance: -3.0,
            render_mode: RenderMode::TiltedPlane,
        })
    }
}

impl Renderable for Projection {
    fn draw(&mut self, delta_time: f32) -> Result<()> {
        // Activate buffers
        self.vao.bind()?;
        self.vbo.bind()?;

        // Activate textures
        self.textures[0].bind_as_unit(0);
        self.textures[1].bind_as_unit(1);

        // Activate shaders and bind to texture units
        self.shader.activate();
        self.shader.set_uniform("texture1", 0)?;
        self.shader.set_uniform("texture2", 1)?;

        // create transformation
        self.rotation_angle += self.rotation_speed as f32 * delta_time;
        self.rotation_angle %= 360.0;

        // calculate screen aspect ratio
        let screen_width = crate::SCREEN_WIDTH;
        let screen_height = crate::SCREEN_HEIGHT;
        let screen_aspect = screen_width as f32 / screen_height as f32;

        // calculate perspective transformations
        let model = Matrix4::from_angle_x(Deg(-55.0));
        let view = Matrix4::from_translation(vec3(0.0, 0.0, self.model_distance));
        let projection = perspective(Deg(45.0), screen_aspect, 0.1, 100.0);

        // pass calculations to the shader
        self.shader.activate();
        self.shader.set_uniform_matrix("model", false, &model)?;
        self.shader.set_uniform_matrix("view", false, &view)?;
        self.shader
            .set_uniform_matrix("projection", false, &projection)?;

        gl_draw::draw_primitive(PrimitiveType::Triangles, self.vbo.data_len() as u32);

        Ok(())
    }

    fn key_pressed(&mut self, key: &Keycode) -> bool {
        match key {
            Keycode::Up | Keycode::W | Keycode::PageUp => {
                if self.model_distance > MAX_MODEL_DISTANCE {
                    self.model_distance -= if *key == Keycode::PageUp {
                        MODEL_DISTANCE_SPEED * 4.0
                    } else {
                        MODEL_DISTANCE_SPEED
                    };
                    println!("Model distance: {}", self.model_distance);
                }
                true
            }
            Keycode::Down | Keycode::S | Keycode::PageDown => {
                if self.model_distance < MIN_MODEL_DISTANCE {
                    self.model_distance += if *key == Keycode::PageDown {
                        MODEL_DISTANCE_SPEED * 4.0
                    } else {
                        MODEL_DISTANCE_SPEED
                    };
                    println!("Model distance: {}", self.model_distance);
                }
                true
            }
            Keycode::R => {
                self.model_distance = -3.0;
                true
            }
            _ => false,
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
// - RenderMode -
//////////////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone, PartialEq)]
enum RenderMode {
    TiltedPlane,
    CubeNoDepth,
    Cube,
}

impl RenderMode {
    fn next(self) -> Self {
        match self {
            RenderMode::TiltedPlane => RenderMode::CubeNoDepth,
            RenderMode::CubeNoDepth => RenderMode::Cube,
            RenderMode::Cube => RenderMode::TiltedPlane,
        }
    }
}

impl Display for RenderMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RenderMode::TiltedPlane => write!(f, "Tilted Plane"),
            RenderMode::CubeNoDepth => write!(f, "Cube No Depth"),
            RenderMode::Cube => write!(f, "Cube"),
        }
    }
}
