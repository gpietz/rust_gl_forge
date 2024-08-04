use std::fmt::{Display, Formatter};
use std::time::{Duration, Instant};

use anyhow::Result;
use cgmath::{perspective, vec3, Deg, InnerSpace, Matrix4, Point3, Rad, Vector3};
use chrono::{Local, Timelike};
use sdl2::keyboard::Keycode;

use crate::render_context::RenderContext;
use crate::resources::{shaders, textures};
use crate::scene::{Scene, SceneResult};
use crate::scene_utils::query_texture;
use crate::{vertex_data_2d, vertex_data_3d};
use shared_lib::camera::moveable_camera::MoveableCamera;
use shared_lib::camera::{Camera, CameraMovement};
use shared_lib::color::Color;
use shared_lib::gl_prelude::Bindable;
use shared_lib::gl_types::Capability;
use shared_lib::opengl::buffer_object::BufferObject;
use shared_lib::opengl::shader_program::ShaderProgram;
use shared_lib::opengl::texture::Texture;
use shared_lib::opengl::vertex_array_object::VertexArrayObject;
use shared_lib::opengl::vertex_layout::VertexLayout;
use shared_lib::sdl_window::SdlKeyboardState;
use shared_lib::shapes::rectangle::Rectangle;
use shared_lib::shapes::ShapesFactory;
use shared_lib::vertices::textured_vertex::TexturedVertex;
use shared_lib::{unbind_buffers, Drawable};

const MAX_MODEL_DISTANCE: f32 = -16.0;
const MIN_MODEL_DISTANCE: f32 = -1.0;
const MODEL_DISTANCE_SPEED: f32 = 0.05;
const RADIUS: f32 = 10.0;

#[derive(Default)]
pub(crate) struct Projection {
    render_models: Vec<RenderModel>,
    textures: Vec<Texture>,
    rotation_angle: f32,
    rotation_speed: i32,
    scale_time: f32,
    model_distance: f32,
    model_strafe: f32,
    render_mode: RenderMode,
    cube_positions: Vec<[f32; 3]>,
    cube_rotations: Vec<CubeRotation>,
    last_update: Option<Instant>,
    first_only: bool,
    camera_mode: CameraMode,
    camera_speed: f32,
    start_time: Option<Instant>,
    camera: MoveableCamera,
    rotation_paused: bool,
    mouse_capture: bool,
    rectangle: Option<Rectangle>,
    synchronized_rotation: bool,
    synchronized_rotation_prev: bool,
}

impl<'a> Projection {
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
            if self.synchronized_rotation {
                let rotation_speed = CubeRotation::random_speed();
                for rotation in &mut self.cube_rotations {
                    if !self.synchronized_rotation_prev {
                        rotation.reset_angle();
                    }
                    rotation.speed = rotation_speed;
                }
            } else {
                for rotation in &mut self.cube_rotations {
                    rotation.update();
                }
            }

            self.last_update = Some(now);
            self.synchronized_rotation_prev = self.synchronized_rotation;

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

    fn get_shader_mut(context: &'a mut RenderContext) -> Result<&'a mut ShaderProgram> {
        context
            .shader_manager()
            .get_shader_mut(shaders::SIMPLE_PROJECTION)
    }

    fn activate_shader(context: &'a mut RenderContext) {
        context
            .shader_manager()
            .activate_shader(shaders::SIMPLE_PROJECTION);
    }

    fn process_keyboard_input(
        &mut self,
        keyboard_state: &SdlKeyboardState,
        delta_time: f32,
    ) -> SceneResult {
        if keyboard_state.is_key_pressed(Keycode::F3) {
            self.toggle_mode();
        }
        if keyboard_state.is_key_pressed(Keycode::F4) {
            self.toggle_depth_test();
        }
        if keyboard_state.is_key_pressed(Keycode::F5) {
            self.toggle_camera_mode();
        }
        if keyboard_state.is_key_pressed(Keycode::R) {
            self.camera.reset_position();
            self.model_distance = -3.0;
            self.print_distance();
            println!("Camera position reset");
            println!("Object distance reset");
        }
        if keyboard_state.is_key_pressed(Keycode::Space) {
            self.rotation_paused = !self.rotation_paused;
            println!(
                "Rotation {}",
                if self.rotation_paused {
                    "paused"
                } else {
                    "active"
                }
            );
        }
        if keyboard_state.is_key_pressed(Keycode::F) {
            self.first_only = !self.first_only;
            println!(
                "Render first cube only: {}",
                if self.first_only {
                    "activated"
                } else {
                    "deactivated"
                }
            );
        }
        if keyboard_state.is_key_pressed(Keycode::X) {
            self.synchronized_rotation = !self.synchronized_rotation;
            println!(
                "Synchronized rotation: {}",
                if self.synchronized_rotation {
                    "activated"
                } else {
                    "deactivated"
                }
            );
        }

        // Movement
        self.process_movement_commands(keyboard_state, delta_time);

        Ok(())
    }

    fn is_keyboard_camera_mode(&self) -> bool {
        matches!(
            self.camera_mode,
            CameraMode::Keyboard | CameraMode::KeyboardMouse
        )
    }

    fn process_movement_commands(&mut self, keyboard_state: &SdlKeyboardState, delta_time: f32) {
        let key_w =
            keyboard_state.is_key_down(Keycode::W) || keyboard_state.is_key_down(Keycode::Up);
        let key_s =
            keyboard_state.is_key_down(Keycode::S) || keyboard_state.is_key_down(Keycode::Down);
        let key_a =
            keyboard_state.is_key_down(Keycode::A) || keyboard_state.is_key_down(Keycode::Left);
        let key_d =
            keyboard_state.is_key_down(Keycode::D) || keyboard_state.is_key_down(Keycode::Right);
        let speed_factor = get_speed_factor(keyboard_state);

        if self.is_keyboard_camera_mode() {
            self.camera.speed = get_speed_factor(keyboard_state) * 3.0;
        }

        if key_w && self.model_distance < MIN_MODEL_DISTANCE {
            self.handle_forward(delta_time, speed_factor);
        }
        if key_s && self.model_distance > MAX_MODEL_DISTANCE {
            self.handle_backward(delta_time, speed_factor);
        }
        if key_a && self.model_distance < MIN_MODEL_DISTANCE {
            self.handle_strafe(delta_time, speed_factor, -1.0);
        }
        if key_d && self.model_distance < MIN_MODEL_DISTANCE {
            self.handle_strafe(delta_time, speed_factor, 1.0);
        }

        fn get_speed_factor(keyboard_state: &SdlKeyboardState) -> f32 {
            match (
                keyboard_state.is_shift_pressed(),
                keyboard_state.is_control_pressed(),
            ) {
                (true, _) => 3.5,
                (_, true) => 7.0,
                _ => 1.5,
            }
        }
    }

    fn handle_forward(&mut self, delta_time: f32, speed_factor: f32) {
        match self.camera_mode {
            CameraMode::Keyboard | CameraMode::KeyboardMouse => {
                self.camera.move_forward(Some(delta_time))
            }
            _ => {
                self.model_distance += MODEL_DISTANCE_SPEED * speed_factor;
                self.print_distance();
            }
        }
    }

    fn handle_backward(&mut self, delta_time: f32, speed_factor: f32) {
        match self.camera_mode {
            CameraMode::Keyboard | CameraMode::KeyboardMouse => {
                self.camera.move_backward(Some(delta_time))
            }
            _ => {
                self.model_distance -= MODEL_DISTANCE_SPEED * speed_factor;
                self.print_distance();
            }
        }
    }

    fn handle_strafe(&mut self, delta_time: f32, speed_factor: f32, direction: f32) {
        match self.camera_mode {
            CameraMode::None => {}
            CameraMode::Keyboard | CameraMode::KeyboardMouse => {
                self.camera.strafe(delta_time, direction)
            }
            _ => {
                let direction = if direction < 0.0 { -1.0 } else { 1.0 };
                self.model_strafe += direction * MODEL_DISTANCE_SPEED * speed_factor * delta_time;
                self.print_distance();
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

        // Update vertex layout attributes (very important!) -- NOT *REALLY* IMPORTANT?
        // let model = match self.render_mode {
        //     RenderMode::TiltedPlane => &mut self.render_models[0],
        //     _ => &mut self.render_models[1],
        // };
    }

    fn toggle_depth_test(&mut self) {
        let depth_test_enabled = Capability::DepthTest.check_enabled();
        if !depth_test_enabled {
            Capability::DepthTest.enable();
            println!("Depth-Test enabled");
        } else {
            Capability::DepthTest.disable();
            println!("Depth-Test disabled");
        }
    }

    fn toggle_camera_mode(&mut self) {
        self.camera_mode = self.camera_mode.next();
        println!("Camera mode: {}", self.camera_mode);
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
            let rm1 = RenderModel::create_plane()?;
            let rm2 = RenderModel::create_cube()?;

            self.render_models.push(rm1);
            self.render_models.push(rm2);

            // Load textures
            self.textures
                .push(query_texture(context, textures::CRATE8)?);
            self.textures
                .push(query_texture(context, textures::AWESOMEFACE2)?);

            // Create shader program
            Self::get_shader_mut(context)?;

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
            self.cube_rotations
                .extend(std::iter::repeat_with(CubeRotation::new).take(10));

            // Create rectangle in upper left corner
            let window_size = context.window().size();
            let mut rectangle = ShapesFactory::new(window_size).create_rectangle(
                10.0,
                10.0,
                300,
                200,
                Color::BLACK,
            )?;
            rectangle.set_fill_color(Some(Color::BLACK));
            //rectangle.set_corner_radius(Some(5.0));
            rectangle.set_opacity(0.6);
            //rectangle.set_strength(3.0);
            //self.rectangle = Some(rectangle);
        }
        Ok(())
    }

    fn update(&mut self, context: &mut RenderContext) -> SceneResult {
        if self.camera_mode == CameraMode::KeyboardMouse {
            let _window = context.window();
            //self.camera.update_direction(&*window);
        }

        self.process_keyboard_input(context.keyboard_state(), context.delta_time())
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
        Self::activate_shader(context);

        // Bind textures
        self.textures[0].bind_as_unit(0);
        self.textures[1].bind_as_unit(1);

        // Set texture units once after shader is activated
        let shader = Self::get_shader_mut(context)?;
        shader.set_uniform("texture1", 0)?;
        shader.set_uniform("texture2", 1)?;

        // Calculate transformation
        let screen_width = crate::SCREEN_WIDTH;
        let screen_height = crate::SCREEN_HEIGHT;
        let screen_aspect = screen_width as f32 / screen_height as f32;

        let model = Matrix4::from_angle_x(Deg(-55.0));
        let mut view = Matrix4::from_translation(vec3(self.model_strafe, 0.0, self.model_distance));
        let projection = perspective(Deg(45.0), screen_aspect, 0.1, 100.0);

        // Calculations for camera
        if self.is_multiple_cubes() {
            match self.camera_mode {
                CameraMode::Circle => {
                    let time_elapsed = self
                        .start_time
                        .expect("Start time hasn't been set in projection scene!")
                        .elapsed()
                        .as_secs_f32();

                    let cam_x = time_elapsed.sin() * RADIUS;
                    let cam_z = time_elapsed.cos() * RADIUS;

                    let eye = Point3::new(cam_x, 0.0, cam_z);
                    let target = Point3::new(0.0, 0.0, 0.0);
                    let up = Vector3::new(0.0, 1.0, 0.0);
                    view = Matrix4::look_at_rh(eye, target, up);
                }
                CameraMode::Keyboard | CameraMode::KeyboardMouse => {
                    // The code for the mouse view is in the update function.
                    view = *self.camera.get_view_projection_matrix();
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
                    let rotation = if self.render_mode != RenderMode::MultipleCubesRotating {
                        let angle = Rad::from(Deg(20.0 * i as f32));
                        let axis = Vector3::new(1.0, 0.3, 0.5).normalize();
                        Matrix4::from_axis_angle(axis, angle)
                    } else {
                        let cube_rotation = &self.cube_rotations[i];
                        let rotation_x = Matrix4::from_angle_x(Deg(cube_rotation.angle.x));
                        let rotation_y = Matrix4::from_angle_y(Deg(cube_rotation.angle.y));
                        let rotation_z = Matrix4::from_angle_z(Deg(cube_rotation.angle.z));

                        // Combine rotations: Note the order of multiplication matters
                        rotation_x * rotation_y * rotation_z
                    };

                    let model = translation * rotation;
                    shader.set_uniform_matrix("model", false, &model)?;
                    if i == 0 || !self.first_only {
                        self.render_models[1].render()?;
                    }
                }

                if self.render_mode == RenderMode::MultipleCubesRotating && !self.rotation_paused {
                    self.update_rotations(context.delta_time());
                }
            }
            _ => {
                self.render_models[1].render()?;
            }
        }

        if let Some(ref mut rect) = self.rectangle {
            rect.draw()?;
        }

        Ok(())
    }
}

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

#[derive(Default)]
struct RenderModel {
    vao: Option<VertexArrayObject>,
    vbo: Option<BufferObject<TexturedVertex>>,
    ibo: Option<BufferObject<u32>>,
}

impl RenderModel {
    pub fn create_plane() -> Result<RenderModel> {
        let vertex_data = vertex_data_2d::create_quad_data(true);
        let vao = VertexArrayObject::new_with_attributes(TexturedVertex::attributes());
        let vbo = vertex_data.create_vbo(&vao);
        let ibo = vertex_data.create_ibo(&vao);
        Ok(RenderModel {
            vao: Some(vao),
            vbo: Some(vbo),
            ibo: Some(ibo),
        })
    }

    pub fn create_cube() -> Result<RenderModel> {
        let vertex_data = vertex_data_3d::create_cube_data();
        let vao = VertexArrayObject::new_with_attributes(TexturedVertex::attributes());
        let vbo = vertex_data.create_vbo(&vao);
        Ok(RenderModel {
            vao: Some(vao),
            vbo: Some(vbo),
            ibo: None,
        })
    }

    pub fn render(&mut self) -> Result<()> {
        if let (Some(vao), Some(vbo), Some(ibo)) = (&self.vao, &self.vbo, &self.ibo) {
            vao.bind();
            vbo.bind()?;
            ibo.bind()?;
            vao.render(true, ibo.data_len());
            unbind_buffers!(vbo, ibo);
        } else if let (Some(vao), Some(vbo)) = (&self.vao, &self.vbo) {
            vao.bind();
            vbo.bind()?;
            vao.render(false, vbo.data_len());
            VertexArrayObject::unbind();
            vbo.unbind()?;
        }

        Ok(())
    }
}

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

    fn reset_angle(&mut self) {
        self.angle = Vector3::new(0.0, 0.0, 0.0);
    }
}

#[derive(Default, Copy, Clone, PartialEq)]
enum CameraMode {
    #[default]
    None,
    Circle,
    Keyboard,
    KeyboardMouse,
}

impl CameraMode {
    fn next(self) -> Self {
        match self {
            CameraMode::None => CameraMode::Circle,
            CameraMode::Circle => CameraMode::Keyboard,
            CameraMode::Keyboard => CameraMode::KeyboardMouse,
            CameraMode::KeyboardMouse => CameraMode::None,
        }
    }
}

impl Display for CameraMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CameraMode::None => write!(f, "None"),
            CameraMode::Circle => write!(f, "Circle"),
            CameraMode::Keyboard => write!(f, "Keyboard"),
            CameraMode::KeyboardMouse => write!(f, "KeyboardMouse"),
        }
    }
}
