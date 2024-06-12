use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;
use std::time::{Duration, Instant};

use shared_lib::gl_vertex_attribute::VertexLayoutManager;
use shared_lib::opengl::shader_manager::ShaderManager;
use shared_lib::opengl::texture_manager::TextureManager;
use shared_lib::prelude::SdlWindow;
use shared_lib::sdl_window::SdlKeyboardState;

pub(crate) struct RenderContext {
    window: Rc<RefCell<SdlWindow>>,
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
    pub fn new(window: Rc<RefCell<SdlWindow>>) -> Self {
        let time_now = Instant::now();
        Self {
            window,
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

    /// Returns an immutable reference to the `SdlWindow` managed by `RefCell`.
    ///
    /// This function provides safe, read-only access to the `SdlWindow`. It uses
    /// `RefCell::borrow`, which checks at runtime to ensure that no mutable
    /// references exist before granting access.
    ///
    /// # Panics
    /// If a mutable reference is already active, as this violates Rust's
    /// borrowing rules enforced by `RefCell`.
    ///
    /// # Returns
    /// A `Ref<SdlWindow>`, a RAII guard ensuring safe access to the `SdlWindow`.
    ///
    /// # Example
    /// ```
    /// let window_ref = render_context.window();
    /// println!("Window ID: {}", window_ref.id());
    /// ```
    pub(crate) fn window(&self) -> Ref<SdlWindow> {
        self.window.borrow()
    }

    /// Returns a mutable reference to the `SdlWindow` managed by `RefCell`.
    ///
    /// This function provides safe, write access to the `SdlWindow`. It uses
    /// `RefCell::borrow_mut`, which checks at runtime that no other references
    /// (mutable or immutable) are active, thereby preventing data races.
    ///
    /// # Panics
    /// If any references (mutable or immutable) are currently active, as
    /// this would break the borrowing rules enforced by `RefCell`.
    ///
    /// # Returns
    /// A `RefMut<SdlWindow>`, a RAII guard that allows safe mutable access to
    /// the `SdlWindow`.
    ///
    /// # Example
    /// ```
    /// let mut window_mut = render_context.window_mut();
    /// window_mut.set_title("New Title");
    /// ```
    pub(crate) fn window_mut(&self) -> RefMut<SdlWindow> {
        self.window.borrow_mut()
    }

    /// Retrieves the size of the drawable area of the window.
    ///
    /// This method returns the size of the framebuffer that is used for rendering.
    /// The drawable size can be different from the window size due to high-DPI displays
    /// or other factors that affect the actual rendering area.
    ///
    /// # Returns
    ///
    /// A tuple `(u32, u32)` where:
    /// - The first element is the width of the drawable area in pixels.
    /// - The second element is the height of the drawable area in pixels.
    ///
    /// # Example
    ///
    /// ```rust
    /// let render_context = RenderContext { window: Rc::new(RefCell::new(sdl_window)) };
    /// let (width, height) = render_context.get_drawable_size();
    /// println!("Drawable size: width = {}, height = {}", width, height);
    /// ```
    ///
    /// # Note
    ///
    /// This method borrows the window reference and internally calls `SDL_GL_GetDrawableSize`
    /// to get the size of the drawable area from the SDL2 window.
    pub fn get_drawable_size(&self) -> (u32, u32) {
        self.window.borrow().get_drawable_size()
    }
}
