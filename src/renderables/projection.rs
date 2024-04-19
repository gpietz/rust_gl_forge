use std::fmt::{Display, Formatter};

use anyhow::Result;
use cgmath::{perspective, vec3, Deg, Matrix4};
use sdl2::keyboard::Keycode;

use shared_lib::gl_prelude::IndicesValueType;
use shared_lib::vertices::textured_vertex::TexturedVertex;
use shared_lib::{
    gl_draw,
    gl_prelude::{
        Bindable, BufferObject, PrimitiveType, ShaderProgram, VertexArrayObject,
        VertexLayoutManager,
    },
    gl_texture::Texture,
};

use crate::vertex_data_3d::create_vbo;
use crate::{renderables::Renderable, texture_utils::create_texture};

const MAX_MODEL_DISTANCE: f32 = -16.0;
const MIN_MODEL_DISTANCE: f32 = -1.0;
const MODEL_DISTANCE_SPEED: f32 = 0.05;

//////////////////////////////////////////////////////////////////////////////
// - Transformation  -
//////////////////////////////////////////////////////////////////////////////

pub struct Projection {
    vbo_plane: Plane,
    vbo_cube: Cube,
    textures: [Texture; 2],
    shader: ShaderProgram,
    rotation_angle: f32,
    rotation_speed: i32,
    scale_time: f32,
    model_distance: f32,
    render_mode: RenderMode,
    vlm: VertexLayoutManager,
}

impl Projection {
    pub fn new() -> Result<Projection> {
        // Create vertex buffer objects (2x)

        let vbo_cube = Cube::new()?;
        let vbo_plane = Plane::new()?;

        // Load textures
        let textures = [
            create_texture("assets/textures/crate8.jpg", false, false)?,
            create_texture("assets/textures/awesomeface2.png", true, true)?,
        ];

        // Create shader program
        let shader = ShaderProgram::from_files(&[
            "assets/shaders/simple/projection.vert",
            "assets/shaders/simple/projection.frag",
        ])?;

        // AAARGH
        //let vlm =  VertexLayoutManager::new::<TexturedVertex>();
        let vlm = VertexLayoutManager::new_and_setup::<TexturedVertex>(&shader)?;

        // Setup vertex layout
        Ok(Projection {
            vbo_plane,
            vbo_cube,
            textures,
            shader,
            rotation_angle: 0.0,
            rotation_speed: 16,
            scale_time: 0.0,
            model_distance: -3.0,
            render_mode: RenderMode::TiltedPlane,
            vlm,
        })
    }
}

impl Renderable for Projection {
    fn draw(&mut self, delta_time: f32) -> Result<()> {
        // Activate shader
        self.shader.activate();

        // Bind textures
        self.textures[0].bind_as_unit(0);
        self.textures[1].bind_as_unit(1);

        // Set texture units once after shader is activated
        self.shader.set_uniform("texture1", 0)?;
        self.shader.set_uniform("texture2", 1)?;

        // Update rotation and calculate transformations
        self.rotation_angle += self.rotation_speed as f32 * delta_time;
        self.rotation_angle %= 360.0;

        let screen_width = crate::SCREEN_WIDTH;
        let screen_height = crate::SCREEN_HEIGHT;
        let screen_aspect = screen_width as f32 / screen_height as f32;

        let model = Matrix4::from_angle_x(Deg(-55.0));
        let view = Matrix4::from_translation(vec3(0.0, 0.0, self.model_distance));
        let projection = perspective(Deg(45.0), screen_aspect, 0.1, 100.0);

        self.shader.set_uniform_matrix("model", false, &model)?;
        self.shader.set_uniform_matrix("view", false, &view)?;
        self.shader
            .set_uniform_matrix("projection", false, &projection)?;

        // Activate and render bases on the current mode
        match self.render_mode {
            RenderMode::TiltedPlane => {
                self.vbo_plane.vao.bind()?;
                gl_draw::draw_elements(
                    PrimitiveType::Triangles,
                    self.vbo_plane.ibo.data_len() as u32,
                    IndicesValueType::Int,
                );
            }
            _ => {
                self.vbo_cube.vbo.bind()?;
                gl_draw::draw_primitive(
                    PrimitiveType::Triangles,
                    self.vbo_cube.vbo.data_len() as u32,
                );
            }
        }

        Ok(())
    }

    fn toggle_mode(&mut self) {
        //TODO Fix this function!!!!
        self.render_mode = self.render_mode.next();
        // Important: Since data structure changes we need to update vertex attributes
        match self.render_mode {
            RenderMode::TiltedPlane => {
                VertexLayoutManager::new_and_setup::<TexturedVertex>(&self.shader)
                    .expect("Failed to update vertex layout for 2D mode!");
            }
            _ => {
                VertexLayoutManager::new_and_setup::<TexturedVertex>(&self.shader)
                    .expect("Failed to update vertex layout for 3D mode!");
            }
        }
        println!("Render mode: {}", self.render_mode);
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
    //Cube,
}

impl RenderMode {
    fn next(self) -> Self {
        match self {
            RenderMode::TiltedPlane => RenderMode::CubeNoDepth,
            RenderMode::CubeNoDepth => RenderMode::TiltedPlane,
        }
    }
}

impl Display for RenderMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RenderMode::TiltedPlane => write!(f, "Tilted Plane"),
            RenderMode::CubeNoDepth => write!(f, "Cube No Depth"),
            //RenderMode::Cube => write!(f, "Cube"),
        }
    }
}

trait Render {
    fn render(&self);
}

//////////////////////////////////////////////////////////////////////////////
// - Plane -
//////////////////////////////////////////////////////////////////////////////

struct Plane {
    pub vao: VertexArrayObject,
    pub vbo: BufferObject<TexturedVertex>,
    pub ibo: BufferObject<u32>,
}

impl Plane {
    pub fn new() -> Result<Plane> {
        let vertex_data = crate::vertex_data_2d::create_quad();
        let mut vao = VertexArrayObject::new(true)?;
        let vbo = vertex_data.create_vbo();
        let ibo = vertex_data.create_ibo();
        //vao.unbind()?;
        Ok(Plane { vao, vbo, ibo })
    }
}

//////////////////////////////////////////////////////////////////////////////
// - Cube -
//////////////////////////////////////////////////////////////////////////////

struct Cube {
    pub vao: VertexArrayObject,
    pub vbo: BufferObject<TexturedVertex>,
}

impl Cube {
    pub fn new() -> Result<Cube> {
        let vertex_data = crate::vertex_data_3d::create_cube();
        let mut vao = VertexArrayObject::new(true)?;
        let vbo = create_vbo(vertex_data);
        Ok(Cube { vao, vbo })
    }
}
