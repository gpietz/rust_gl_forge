use std::time::{Duration, Instant};

use shared_lib::gl_shader_manager::ShaderManager;
use shared_lib::gl_texture::TextureManager;
use shared_lib::gl_vertex_attribute::VertexLayoutManager;
use shared_lib::prelude::SdlWindow;
use shared_lib::sdl_window::SdlKeyboardState;

pub(crate) struct RenderContext {
    delta_time: f32,
    frame_rate: u32,
    shader_manager: ShaderManager,
    vertex_layout_manager: VertexLayoutManager,
    texture_manager: TextureManager,
    keyboard_state: SdlKeyboardState,

    last_update_time: Instant,
    last_fps_time: Instant,
    frame_count: u32,
}

impl RenderContext {
    pub fn new() -> Self {
        let time_now = Instant::now();
        Self {
            delta_time: 0.0,
            frame_rate: 0,
            shader_manager: ShaderManager::default(),
            vertex_layout_manager: VertexLayoutManager::default(),
            texture_manager: TextureManager::default(),
            last_update_time: time_now,
            last_fps_time: time_now,
            frame_count: 0,
            keyboard_state: SdlKeyboardState::default(),
        }
    }

    pub(crate) fn update(&mut self, window: &SdlWindow) {
        self.update_delta_time();
        self.update_frame_rate();
        self.keyboard_state.update(window);
    }

    /// Calculates and updates the delta time in seconds since the last update,
    fn update_delta_time(&mut self) {
        let now = Instant::now();
        let delta = now.duration_since(self.last_update_time);
        self.last_update_time = Instant::now();
        self.delta_time = delta.as_secs_f32();
    }

    /// Calculates and updates the frame rate every second.
    fn update_frame_rate(&mut self) {
        self.frame_count += 1;
        let now = Instant::now();
        if now.duration_since(self.last_fps_time) > Duration::from_secs(1) {
            self.frame_rate = self.frame_count;
            self.frame_count = 0;
            self.last_fps_time = now;
        }
    }

    pub(crate) fn delta_time(&self) -> f32 {
        self.delta_time
    }

    pub(crate) fn frame_rate(&self) -> u32 {
        self.frame_rate
    }

    pub(crate) fn shader_manager(&mut self) -> &mut ShaderManager {
        &mut self.shader_manager
    }

    pub(crate) fn vertex_layout_manager(&mut self) -> &mut VertexLayoutManager {
        &mut self.vertex_layout_manager
    }

    pub(crate) fn texture_manager(&mut self) -> &mut TextureManager {
        &mut self.texture_manager
    }

    pub(crate) fn keyboard_state(&self) -> &SdlKeyboardState {
        &self.keyboard_state
    }
}
