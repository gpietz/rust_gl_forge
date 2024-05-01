use std::fmt::{Display, Formatter};
use std::time::{Duration, Instant};

use anyhow::Result;
use cgmath::{perspective, vec3, Deg, InnerSpace, Matrix4, Point3, Rad, Vector3};
use chrono::{Local, Timelike};
use sdl2::keyboard::Keycode;

use shared_lib::camera::Camera;
use shared_lib::gl_buffer::BufferObject;
use shared_lib::gl_draw;
use shared_lib::gl_prelude::PrimitiveType;
use shared_lib::gl_shader::ShaderProgram;
use shared_lib::gl_texture::Texture;
use shared_lib::gl_traits::Bindable;
use shared_lib::gl_types::{Capability, IndicesValueType};
use shared_lib::gl_vertex_array::VertexArrayObject;
use shared_lib::gl_vertex_attribute::VertexLayoutManager;
use shared_lib::sdl_window::SdlKeyboardState;
use shared_lib::vertices::textured_vertex::TexturedVertex;

use crate::render_context::RenderContext;
use crate::resources::{shaders, textures};
use crate::scene::{Scene, SceneResult};
use crate::scene_utils::query_texture;
use crate::vertex_data_3d::create_vbo;

const MAX_MODEL_DISTANCE: f32 = -16.0;
const MIN_MODEL_DISTANCE: f32 = -1.0;
const MODEL_DISTANCE_SPEED: f32 = 0.05;
const RADIUS: f32 = 10.0;

//////////////////////////////////////////////////////////////////////////////
// - Projection  -
//////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub(crate) struct Projection {
    render_models: Vec<RenderModel>,
    textures: Vec<Texture>,
    rotation_angle: f32,
    rotation_speed: i32,
    scale_time: f32,
    model_distance: f32,
    render_mode: RenderMode,
    cube_positions: Vec<[f32; 3]>,
    cube_rotations: Vec<CubeRotation>,
    last_update: Option<Instant>,
    paused: bool,
    first_only: bool,
    camera_mode: CameraMode,
    start_time: Option<Instant>,
    camera: Camera,
    vlm: Option<VertexLayoutManager>,
    rotation_paused: bool,
}

impl Projection {
    fn check_rotation_update_required(&mut self) -> bool {
        let now = Instant::now();
        match self.last_update {
            Some(last_update) if now.duration_since(last_update) < Duration::from_secs(3) => false,
            _ => {
                self.last_update = Some(now);
                true
            }
        }
    }

    fn update_rotations(&mut self, delta_time: f32) {
        let now = Instant::now();
        if self.check_rotation_update_required() {
            for rotation in &mut self.cube_rotations {
                rotation.update();
            }
            self.last_update = Some(now);

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

    fn is_multiple_cubes(&self) -> bool {
        matches!(
            self.render_mode,
            RenderMode::MultipleCubes | RenderMode::MultipleCubesRotating
        )
    }

    fn get_shader_mut(context: &mut RenderContext) -> Result<&mut ShaderProgram> {
        context
            .shader_manager()
            .get_shader_mut(shaders::SIMPLE_PROJECTION)
    }

    fn process_keyboard_input(&mut self, keyboard_state: &SdlKeyboardState) -> SceneResult {
        if keyboard_state.is_key_pressed(Keycode::F3) {
            self.toggle_mode();
        }
        if keyboard_state.is_key_pressed(Keycode::F4) {
            self.toggle_depth_test();
        }
        if keyboard_state.is_key_pressed(Keycode::R) {
            self.model_distance = -3.0;
            self.print_distance();
        }
        if (keyboard_state.is_key_down(Keycode::W) || keyboard_state.is_key_down(Keycode::Up))
            && self.model_distance > MAX_MODEL_DISTANCE
        {
            self.model_distance -= MODEL_DISTANCE_SPEED * get_speed_factor(keyboard_state);
            self.print_distance();
        }
        if (keyboard_state.is_key_down(Keycode::S) || keyboard_state.is_key_down(Keycode::Down))
            && self.model_distance < MIN_MODEL_DISTANCE
        {
            self.model_distance += MODEL_DISTANCE_SPEED * get_speed_factor(keyboard_state);
            self.print_distance();
        }
        if keyboard_state.is_key_pressed(Keycode::Space) {
            self.rotation_paused = !self.rotation_paused;
        }

        return Ok(());

        fn get_speed_factor(keyboard_state: &SdlKeyboardState) -> f32 {
            if keyboard_state.is_shift_pressed() {
                4.0
            } else {
                1.0
            }
        }
    }

    fn print_distance(&self) {
        println!("Model distance: {:.2}", self.model_distance);
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

        let vlm = self
            .vlm
            .as_mut()
            .expect("No VLM present in projection scene");
        model
            .update_vertex_layout(vlm)
            .unwrap_or_else(|e| panic!("Couldn't update vertex layout: {}", e));
    }

    fn toggle_depth_test(&mut self) {
        let depth_test_enabled = Capability::DepthTest.is_enabled();
        if !depth_test_enabled {
            Capability::DepthTest.enable();
            println!("Depth-Test enabled");
        } else {
            Capability::DepthTest.disable();
            println!("Depth-Test disabled");
        }
    }
}

impl Scene<RenderContext> for Projection {
    fn activate(&mut self, context: &mut RenderContext) -> SceneResult {
        if self.render_models.is_empty() {
            // Set some default values
            self.model_distance = -3.0;
            self.rotation_speed = 16;

            // Set starting time for this scene
            self.start_time = Some(Instant::now());

            // Create models for rendering
            self.render_models.push(RenderModel::create_plane()?);
            self.render_models.push(RenderModel::create_cube()?);

            // Load textures
            self.textures
                .push(query_texture(context, textures::CRATE8)?);
            self.textures
                .push(query_texture(context, textures::AWESOMEFACE2)?);

            // Create shader program
            Self::get_shader_mut(context)?;

            // Setup vertex layout for first model and store layout manager in struct
            let mut vlm = VertexLayoutManager::new::<TexturedVertex>();
            self.render_models[0].update_vertex_layout(&mut vlm)?;
            self.vlm = Some(vlm);

            // Created vector with positions for cubes
            self.cube_positions = vec![
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
            for _ in 0..10 {
                self.cube_rotations.push(CubeRotation::new());
            }
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
        // Activate shader
        let shader = Self::get_shader_mut(context)?;
        shader.activate();

        // Bind textures
        self.textures[0].bind_as_unit(0);
        self.textures[1].bind_as_unit(1);

        // Set texture units once after shader is activated
        shader.set_uniform("texture1", 0)?;
        shader.set_uniform("texture2", 1)?;

        // Calculate transformation
        let screen_width = crate::SCREEN_WIDTH;
        let screen_height = crate::SCREEN_HEIGHT;
        let screen_aspect = screen_width as f32 / screen_height as f32;

        let model = Matrix4::from_angle_x(Deg(-55.0));
        let mut view = Matrix4::from_translation(vec3(0.0, 0.0, self.model_distance));
        let projection = perspective(Deg(45.0), screen_aspect, 0.1, 100.0);

        // Calculations for camera
        if self.is_multiple_cubes() {
            match self.camera_mode {
                CameraMode::Circle | CameraMode::Rotate => {
                    let time_elapsed = self
                        .start_time
                        .expect("Start time hasn't been set in projection scene!")
                        .elapsed()
                        .as_secs_f32();

                    let cam_x = time_elapsed.sin() * RADIUS;
                    let cam_z = time_elapsed.cos() * RADIUS;

                    match self.camera_mode {
                        CameraMode::Circle => {
                            view = Matrix4::look_at_rh(
                                Point3::new(cam_x, 0.0, cam_z),
                                Point3::new(0.0, 0.0, 0.0),
                                vec3(0.0, 1.0, 0.0),
                            );
                        }
                        _ => {
                            let eye = Point3::new(cam_x, 0.0, cam_z);
                            let target = Point3::new(0.0, 0.0, 0.0);
                            let up = Vector3::new(0.0, 1.0, 0.0);
                            view = Matrix4::look_at_rh(eye, target, up);
                        }
                    }
                }
                CameraMode::Keyboard => {
                    self.camera.update_view_mat4(&mut view);
                }
                _ => {}
            }
        }

        // Send transformation matrices to GPU
        if self.render_mode != RenderMode::MultipleCubes {
            shader.set_uniform_matrix("model", false, &model)?;
        }
        shader.set_uniform_matrix("view", false, &view)?;
        shader.set_uniform_matrix("projection", false, &projection)?;

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
                    shader.set_uniform_matrix("model", false, &model)?;
                    if i == 0 || !self.first_only {
                        self.render_models[1].render()?;
                    }
                }

                if self.render_mode == RenderMode::MultipleCubesRotating && !self.paused {
                    self.update_rotations(context.delta_time());
                }
            }
            _ => {
                self.render_models[1].render()?;
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

#[derive(Default)]
struct RenderModel {
    vao: Option<VertexArrayObject>,
    vbo: Option<BufferObject<TexturedVertex>>,
    ibo: Option<BufferObject<u32>>,
}

impl RenderModel {
    pub fn create_plane() -> Result<RenderModel> {
        let vertex_data = crate::vertex_data_2d::create_quad();
        let vao = VertexArrayObject::new()?;
        let vbo = vertex_data.create_vbo();
        let ibo = vertex_data.create_ibo();
        Ok(RenderModel {
            vao: Some(vao),
            vbo: Some(vbo),
            ibo: Some(ibo),
        })
    }

    pub fn create_cube() -> Result<RenderModel> {
        let vertex_data = crate::vertex_data_3d::create_cube();
        let vao = VertexArrayObject::new()?;
        let vbo = create_vbo(vertex_data);
        Ok(RenderModel {
            vao: Some(vao),
            vbo: Some(vbo),
            ibo: None,
        })
    }

    pub fn bind(&mut self) -> Result<()> {
        if let (Some(vao), Some(vbo)) = (self.vao.as_mut(), self.vbo.as_mut()) {
            vao.bind()?;
            vbo.bind()?; // Not required?
        }
        Ok(())
    }

    pub fn update_vertex_layout(&mut self, vlm: &mut VertexLayoutManager) -> Result<()> {
        self.bind()?;
        vlm.setup_attributes()?;
        Ok(())
    }

    pub fn render(&mut self) -> Result<()> {
        // Attempt to bind the VAO
        self.bind()?;
        match &self.ibo {
            Some(ibo) => {
                gl_draw::draw_elements(
                    PrimitiveType::Triangles,
                    ibo.data_len() as u32,
                    IndicesValueType::Int,
                );
            }
            _ => {
                let vertex_count = self
                    .vbo
                    .as_ref()
                    .expect("VBO object not defined in projection scene")
                    .data_len();
                gl_draw::draw_primitive(PrimitiveType::Triangles, vertex_count as u32);
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

//////////////////////////////////////////////////////////////////////////////
// - CameraMode -
//////////////////////////////////////////////////////////////////////////////

#[derive(Default, Copy, Clone, PartialEq)]
enum CameraMode {
    #[default]
    None,
    Circle,
    Rotate,
    Keyboard,
}

impl CameraMode {
    fn next(self) -> Self {
        match self {
            CameraMode::None => CameraMode::Circle,
            CameraMode::Circle => CameraMode::Rotate,
            CameraMode::Rotate => CameraMode::Keyboard,
            CameraMode::Keyboard => CameraMode::None,
        }
    }
}

impl Display for CameraMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CameraMode::None => write!(f, "None"),
            CameraMode::Circle => write!(f, "Circle"),
            CameraMode::Rotate => write!(f, "Rotate"),
            CameraMode::Keyboard => write!(f, "Keyboard"),
        }
    }
}
