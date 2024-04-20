use std::fmt::{Display, Formatter};
use std::time::{Duration, Instant};

use anyhow::Result;
use cgmath::{perspective, vec3, Deg, InnerSpace, Matrix4, Rad, Vector3};
use chrono::{Local, Timelike};
use sdl2::keyboard::Keycode;

use shared_lib::gl_prelude::IndicesValueType;
use shared_lib::gl_types::Capability;
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
    render_models: [RenderModel; 2],
    textures: [Texture; 2],
    shader: ShaderProgram,
    rotation_angle: f32,
    rotation_speed: i32,
    scale_time: f32,
    model_distance: f32,
    render_mode: RenderMode,
    vlm: VertexLayoutManager,
    cube_positions: Vec<[f32; 3]>,
    cube_rotations: Vec<CubeRotation>,
    last_update: Instant,
}

impl Projection {
    pub fn new() -> Result<Projection> {
        // Create 3D models to render (2x)
        let mut render_models = [RenderModel::create_plane()?, RenderModel::create_cube()?];

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

        // Setup the vertex layout
        let mut vlm = VertexLayoutManager::new_and_setup::<TexturedVertex>()?;
        render_models[0].update_vertex_layout(&mut vlm)?;

        // Created vector with positions for cubes
        let cube_positions = vec![
            [0.0, 0.0, 0.0],
            [2.0, 5.0, -15.0],
            [-1.5, -2.2, -2.5],
            [-3.8, -2.0, -12.3],
            [2.4, -0.4, -3.5],
            [-1.7, 3.0, -7.5],
            [1.3, -2.0, -2.5],
            [1.5, 2.0, -2.5],
            [1.5, 0.2, -1.5],
            [-1.3, 1.0, -1.5],
        ];

        // Create vector for cube rotations
        let mut cube_rotations = Vec::new();
        for _ in 0..10 {
            cube_rotations.push(CubeRotation::new());
        }

        // Setup vertex layout
        Ok(Projection {
            render_models,
            textures,
            shader,
            rotation_angle: 0.0,
            rotation_speed: 16,
            scale_time: 0.0,
            model_distance: -3.0,
            render_mode: RenderMode::TiltedPlane,
            vlm,
            cube_positions,
            cube_rotations,
            last_update: Instant::now(),
        })
    }

    fn update_rotations(&mut self, delta_time: f32) {
        let now = Instant::now();
        if now.duration_since(self.last_update) > Duration::from_secs(3) {
            for rotation in &mut self.cube_rotations {
                rotation.update();
            }
            self.last_update = now;

            let time_now = Local::now();
            println!(
                "Cube rotation updated: {:02}:{:02}:{:02}",
                time_now.hour(),
                time_now.minute(),
                time_now.second()
            );
        }

        for rotation in &mut self.cube_rotations {
            rotation.angle += rotation.speed * delta_time;
        }
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

        if self.render_mode != RenderMode::MultipleCubes {
            self.shader.set_uniform_matrix("model", false, &model)?;
        }
        self.shader.set_uniform_matrix("view", false, &view)?;
        self.shader
            .set_uniform_matrix("projection", false, &projection)?;

        // Render models based on the active rendering mode
        match self.render_mode {
            RenderMode::TiltedPlane => {
                self.render_models[0].render()?;
            }
            RenderMode::MultipleCubes | RenderMode::MultipleCubesRotating => {
                for (i, pos) in self.cube_positions.iter().enumerate() {
                    let pos_vector3 = Vector3::new(pos[0], pos[1], pos[2]);
                    let translation = Matrix4::from_translation(pos_vector3);
                    let rotation: Matrix4<f32>;

                    if self.render_mode != RenderMode::MultipleCubesRotating {
                        let angle = Rad::from(Deg(20.0 * i as f32));
                        let axis = Vector3::new(1.0, 0.3, 0.5).normalize();
                        rotation = Matrix4::from_axis_angle(axis, angle);
                    } else {
                        let cube_rotation = &self.cube_rotations[i];
                        let rotation_x = Matrix4::from_angle_x(Deg(cube_rotation.angle.x));
                        let rotation_y = Matrix4::from_angle_y(Deg(cube_rotation.angle.y));
                        let rotation_z = Matrix4::from_angle_z(Deg(cube_rotation.angle.z));

                        // Combine rotations: Note the order of multiplication matters
                        rotation = rotation_x * rotation_y * rotation_z;
                    }

                    let model = translation * rotation;
                    self.shader.set_uniform_matrix("model", false, &model)?;
                    self.render_models[1].render()?;
                }

                if self.render_mode == RenderMode::MultipleCubesRotating {
                    self.update_rotations(delta_time);
                }
            }
            _ => {
                self.render_models[1].render()?;
            }
        }

        Ok(())
    }

    fn toggle_mode(&mut self) {
        self.render_mode = self.render_mode.next();

        // Enable/Disable the depth testing capability
        match self.render_mode {
            RenderMode::CubeNoDepth => Capability::DepthTest.disable(),
            _ => Capability::DepthTest.enable(),
        }

        // Update vertex layout attributes (very important!)
        let model = match self.render_mode {
            RenderMode::TiltedPlane => &mut self.render_models[0],
            _ => &mut self.render_models[1],
        };
        model
            .update_vertex_layout(&mut self.vlm)
            .unwrap_or_else(|e| panic!("Couldn't update vertex layout: {}", e));
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
            Keycode::F4 => {
                Capability::DepthTest.disable();
                println!("Depth-Test disabled");
                true
            }
            Keycode::F5 => {
                Capability::DepthTest.enable();
                println!("Depth-Test enabled");
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
    CubeDepth,
    MultipleCubes,
    MultipleCubesRotating,
}

impl RenderMode {
    fn next(self) -> Self {
        match self {
            RenderMode::TiltedPlane => RenderMode::CubeNoDepth,
            RenderMode::CubeNoDepth => RenderMode::CubeDepth,
            RenderMode::CubeDepth => RenderMode::MultipleCubes,
            RenderMode::MultipleCubes => RenderMode::MultipleCubesRotating,
            RenderMode::MultipleCubesRotating => RenderMode::TiltedPlane,
        }
    }
}

impl Display for RenderMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RenderMode::TiltedPlane => write!(f, "Tilted Plane"),
            RenderMode::CubeNoDepth => write!(f, "Cube No Depth"),
            RenderMode::CubeDepth => write!(f, "Cube"),
            RenderMode::MultipleCubes => write!(f, "Multiple Cubes"),
            RenderMode::MultipleCubesRotating => write!(f, "Multiple Cubes Rotating"),
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
// - RenderModel -
//////////////////////////////////////////////////////////////////////////////

struct RenderModel {
    vao: VertexArrayObject,
    vbo: BufferObject<TexturedVertex>,
    ibo: Option<BufferObject<u32>>,
}

impl RenderModel {
    pub fn new(
        vao: VertexArrayObject,
        vbo: BufferObject<TexturedVertex>,
        ibo: Option<BufferObject<u32>>,
    ) -> Self {
        Self { vao, vbo, ibo }
    }

    pub fn create_plane() -> Result<RenderModel> {
        let vertex_data = crate::vertex_data_2d::create_quad();
        let vao = VertexArrayObject::new(true)?;
        let vbo = vertex_data.create_vbo();
        let ibo = vertex_data.create_ibo();
        Ok(RenderModel::new(vao, vbo, Some(ibo)))
    }

    pub fn create_cube() -> Result<RenderModel> {
        let vertex_data = crate::vertex_data_3d::create_cube();
        let vao = VertexArrayObject::new(true)?;
        let vbo = create_vbo(vertex_data);
        Ok(RenderModel::new(vao, vbo, None))
    }

    pub fn bind(&mut self) -> Result<()> {
        self.vao.bind()?;
        self.vbo.bind()?;
        Ok(())
    }

    pub fn update_vertex_layout(&mut self, vlm: &mut VertexLayoutManager) -> Result<()> {
        self.bind()?;
        vlm.setup_attributes()?;
        Ok(())
    }

    pub fn render(&mut self) -> Result<()> {
        // Attempt to bind the VAO
        self.vao.bind()?;
        match &self.ibo {
            Some(ibo) => {
                gl_draw::draw_elements(
                    PrimitiveType::Triangles,
                    ibo.data_len() as u32,
                    IndicesValueType::Int,
                );
            }
            _ => {
                gl_draw::draw_primitive(PrimitiveType::Triangles, self.vbo.data_len() as u32);
            }
        }

        Ok(())
    }
}

//////////////////////////////////////////////////////////////////////////////
// - CubeRotation -
//////////////////////////////////////////////////////////////////////////////

struct CubeRotation {
    angle: Vector3<f32>,
    speed: Vector3<f32>,
}

impl CubeRotation {
    fn new() -> Self {
        Self {
            angle: Vector3::new(0.0, 0.0, 0.0),
            speed: Self::random_speed(),
        }
    }

    fn random_speed() -> Vector3<f32> {
        let mut rng = rand::thread_rng();
        Vector3::new(
            rand::Rng::gen_range(&mut rng, -90.0..90.0),
            rand::Rng::gen_range(&mut rng, -90.0..90.0),
            rand::Rng::gen_range(&mut rng, -90.0..90.0),
        )
    }

    fn update(&mut self) {
        self.speed = Self::random_speed();
    }
}
