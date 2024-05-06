use anyhow::{Context, Error, Result};
use sdl2::keyboard::{Keycode, Mod};
use sdl2::{
    video::{GLContext, SwapInterval, Window},
    EventPump, Sdl,
};
use std::collections::HashSet;

use crate::color::Color;
use crate::gl_traits::ToOpenGL;
use crate::gl_types::RenderMask;
use crate::gl_utils::check_gl_error;
use crate::input::mouse_adapter::{MouseAdapter, MouseButton};

//////////////////////////////////////////////////////////////////////////////
// - SdlWindow -
//////////////////////////////////////////////////////////////////////////////

pub struct SdlWindow {
    pub sdl: Sdl,
    pub window: Window,
    pub gl_context: GLContext,
    pub event_pump: EventPump,
    pub clear_color: Color,
}

impl SdlWindow {
    /// Creates a new `SdlWindow` with OpenGL context.
    ///
    /// This function initializes the SDL2 library and creates a new window with the specified width, height, and title.
    /// It sets up the video subsystem, initializes an OpenGL context, and optionally enables VSync based on the
    /// `enable_vsync` parameter.
    ///
    /// # Arguments
    ///
    /// * `width` - The width of the window in pixels.
    /// * `height` - The height of the window in pixels.
    /// * `title` - The title of the window.
    /// * `enable_vsync` - A boolean value to enable or disable VSync.
    ///   If `true`, VSync is enabled, synchronizing the window's refresh rate with the display's refresh rate.
    ///   If `false`, VSync is disabled.
    ///
    /// # Returns
    ///
    /// A `Result<SdlWindow>`, which is `Ok` if the window, OpenGL context, and associated subsystems were successfully
    /// created, or an `Err` containing an `anyhow::Error` if an error occurred during initialization.
    ///
    /// # Errors
    ///
    /// Returns an `Err` if:
    /// - The SDL2 library fails to initialize.
    /// - The video subsystem cannot be accessed.
    /// - The OpenGL attributes cannot be set.
    /// - The window cannot be created with the specified parameters.
    /// - The OpenGL context cannot be created.
    /// - VSync setting fails (when enabled).
    /// - The event pump cannot be created.
    ///
    /// In each case, the specific error encountered is encapsulated in the returned `anyhow::Error`.
    ///
    /// # Examples
    ///
    /// ```
    /// use sdl_window::SdlWindow;
    /// use shared_lib::sdl_window;
    ///
    /// let window = SdlWindow::new(800, 600, "My OpenGL Window", true);
    /// match window {
    ///     Ok(win) => { /* Proceed with window and OpenGL operations */ }
    ///     Err(e) => { /* Handle error (e.g., logging) */ }
    /// }
    /// ```
    pub fn new(width: usize, height: usize, title: &str, enable_vsync: bool) -> Result<SdlWindow> {
        let sdl = sdl2::init().map_err(Error::msg)?;
        let video_subsystem = sdl.video().map_err(Error::msg)?;
        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(3, 3);
        let window = video_subsystem
            .window(title, width as u32, height as u32)
            .opengl()
            .build()
            .map_err(Error::msg)?;

        let gl_context = window.gl_create_context().map_err(Error::msg)?;

        // load OpenGL function pointers
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const _);
        check_gl_error()?;

        // Set the OpenGL viewport
        unsafe {
            gl::Viewport(0, 0, width as i32, height as i32);
        }

        if enable_vsync {
            window
                .subsystem()
                .gl_set_swap_interval(SwapInterval::VSync)
                .map_err(Error::msg)?;
        }

        let event_pump = sdl.event_pump().map_err(Error::msg)?;
        Ok(SdlWindow {
            sdl,
            window,
            gl_context,
            event_pump,
            clear_color: Color::BLACK,
        })
    }
}

impl SdlWindow {
    /// Clears the framebuffer by setting it to a predefined clear color and clearing the color,
    /// depth, and stencil buffers.
    ///
    /// This method sets the current clear color to the value specified by `self.clear_color` and
    /// clears the color, depth, and stencil buffers to prepare for a new frame. This is typically
    /// called at the start of a render loop to reset the framebuffer state.
    pub fn clear(&self) {
        unsafe {
            gl::ClearColor(
                self.clear_color.r,
                self.clear_color.g,
                self.clear_color.b,
                self.clear_color.a,
            );
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT);
        }
    }

    /// Clears the specified buffers to their preset clear values, or defaults to clearing all if none specified.
    ///
    /// This function clears the OpenGL buffers as indicated by the `buffer_bits` argument. If `buffer_bits`
    /// is `None`, it defaults to clearing the color, depth, and stencil buffers to their preset clear values.
    /// Specifically, if the color buffer is selected (either explicitly or by default), it is cleared to
    /// the color specified by `self.clear_color`.
    ///
    /// # Arguments
    ///
    /// * `buffer_bits` - An optional `BufferBit` flag or combination of flags indicating which buffers to clear.
    ///   If `None`, all buffers (color, depth, and stencil) are cleared.
    pub fn clear_buffer(&self, buffer_bits: Option<RenderMask>) {
        unsafe {
            gl::ClearColor(
                self.clear_color.r,
                self.clear_color.g,
                self.clear_color.b,
                self.clear_color.a,
            );

            match buffer_bits {
                Some(render_mask) => {
                    // If specific bits are provided, clear using those.
                    // This assumes `to_gl()` translates `BufferBit` to the appropriate OpenGL flags.
                    gl::Clear(render_mask.to_opengl());
                }
                None => {
                    // Use a default mask if no bits are specified.
                    // This combines color, depth, and stencil buffer bits as the default clearing
                    // behavior.
                    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT);
                }
            }
        }
    }

    /// Swaps the front and back buffers of the window.
    /// This should be called after rendering to display the updated content.
    pub fn swap(&self) {
        self.window.gl_swap_window();
    }

    /// Sets the title of the window.
    ///
    /// This function attempts to update the window's title to the specified value. If successful, it returns `Ok(())`.
    /// In case of failure, it returns an `Err` with a detailed error message explaining that the window title could not be updated.
    ///
    /// # Arguments
    /// * `title` - A reference to a string slice (`&str`) that holds the new title for the window.
    ///
    /// # Returns
    /// * `Result<()>` - A result type that is `Ok` if the window title was successfully updated, or an `Err` with an error message if the operation failed.
    ///
    /// # Errors
    /// This function can return an error if there is a failure in updating the window's title, encapsulated within the context of the error message "Error occurred while updating the window title."
    pub fn set_window_title(&mut self, title: &str) -> Result<()> {
        self.window
            .set_title(title)
            .with_context(|| "Error occurred while updating the window title.")?;
        Ok(())
    }

    /// Retrieves the current title of the window.
    ///
    /// This method returns a reference to a string slice (`&str`) that represents the current title of the window.
    /// It provides a way to access the window's title at any point after the window has been created or its title has been set.
    ///
    /// # Returns
    /// * `&str` - A string slice representing the window's current title.
    pub fn window_title(&self) -> &str {
        self.window.title()
    }

    /// Retrieves the list of currently pressed keys from the SDL event system.
    ///
    /// This function accesses the SDL event pump to obtain the current keyboard
    /// state and then maps the pressed scancodes to their corresponding `Keycode`.
    ///
    /// # Errors
    /// Returns an error if the SDL event pump cannot be initialized, wrapping the
    /// error with a contextual message for better error handling.
    ///
    /// # Returns
    /// A `Result` containing either:
    /// - A `Vec<Keycode>` representing the list of pressed keys if successful, or
    /// - An `anyhow::Error` detailing the issue if an error occurs during the
    ///   event pump initialization or during the retrieval process.
    ///
    /// # Example
    /// ```no_run
    /// fn example(sdl_context: &sdl2::Sdl) -> anyhow::Result<()> {
    ///     let sdl_window = sdl_context.video().unwrap().window("Example", 800, 600)
    ///         .position_centered().opengl().build().unwrap();
    ///
    ///     let pressed_keys = sdl_window.get_pressed_keys()?;
    ///     for key in pressed_keys {
    ///         println!("Pressed: {:?}", key);
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub fn get_pressed_keys(&self) -> Result<Vec<Keycode>> {
        let pressed_keys: Vec<Keycode> = self
            .event_pump
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();
        Ok(pressed_keys)
    }

    /// Retrieves the current state of the keyboard, including pressed keys and
    /// active modifiers.
    ///
    /// This function consolidates the list of currently pressed keys along with the
    /// state of modifier keys such as Shift, Ctrl, and Alt. It utilizes the SDL
    /// library's capabilities to fetch this data.
    ///
    /// # Errors
    /// Returns an error if there is a failure in fetching the pressed keys. The
    /// error is propagated from the `get_pressed_keys` function.
    ///
    /// # Returns
    /// Returns a `Result` containing:
    /// - `SdlKeyboardState`: A struct containing both the list of currently pressed
    ///   keys and the current modifier state, or
    /// - An error if any occurs during the retrieval of the keyboard state.
    ///
    /// # Example
    /// ```no_run
    /// fn example(sdl_context: &sdl2::Sdl) -> anyhow::Result<()> {
    ///     let keyboard_state = sdl_context.get_keyboard_state()?;
    ///     println!("Pressed keys: {:?}", keyboard_state.pressed_keys);
    ///     println!("Modifiers: {:?}", keyboard_state.modifiers);
    ///     Ok(())
    /// }
    /// ```
    pub fn get_keyboard_state(&self) -> Result<SdlKeyboardState> {
        let pressed_keys = self.get_pressed_keys()?;
        let modifiers = self.sdl.keyboard().mod_state();
        Ok(SdlKeyboardState::new(pressed_keys, modifiers))
    }

    /// Returns the unique identifier of the window.
    ///
    /// This method retrieves the unique ID associated with the window instance,
    /// which is useful for distinguishing between multiple windows.
    ///
    /// # Returns
    /// A `u32` that represents the unique identifier of the window.
    ///
    /// # Examples
    /// ```no-run
    /// let window_id = window.window_id();
    /// println!("Window ID: {}", window_id);
    /// ```
    pub fn window_id(&self) -> u32 {
        self.window.id()
    }
}

impl MouseAdapter for SdlWindow {
    fn focused_window_id(&self) -> Option<u32> {
        self.sdl.mouse().focused_window_id()
    }

    fn is_mouse_in_window(&self) -> bool {
        match self.focused_window_id() {
            Some(focused_window_id) => focused_window_id == self.window_id(),
            None => false,
        }    
    }

    fn show_cursor(&self, show: bool) {
        self.sdl.mouse().show_cursor(show);
    }

    fn is_cursor_showing(&self) -> bool {
        self.sdl.mouse().is_cursor_showing()
    }

    fn capture_mouse(&self, capture_enabled: bool) {
        self.sdl.mouse().capture(capture_enabled);
    }

    fn mouse_x(&self) -> i32 {
        self.event_pump.mouse_state().x()
    }

    fn mouse_y(&self) -> i32 {
        self.event_pump.mouse_state().y()
    }

    fn mouse_xy(&self) -> (i32, i32) {
        self.mouse_position()
    }

    fn mouse_position(&self) -> (i32, i32) {
        let mouse_state = self.event_pump.mouse_state();
        (mouse_state.x(), mouse_state.y())
    }

    fn mouse_position_ref(&self, xpos: &mut i32, ypos: &mut i32) {
        let mouse_state = self.event_pump.mouse_state();
        *xpos = mouse_state.x();
        *ypos = mouse_state.y();
    }

    fn is_mouse_button_pressed(&self, mouse_button: &MouseButton) -> bool {
        let mouse_state = self.event_pump.mouse_state();
        match mouse_button {
            MouseButton::Left => mouse_state.left(),
            MouseButton::Middle => mouse_state.middle(),
            MouseButton::Right => mouse_state.right(),
        }
    }

    fn pressed_mouse_buttons(&self) -> impl Iterator<Item = &MouseButton> {
        MouseButton::variants()
            .iter()
            .filter(move |button| self.is_mouse_button_pressed(button))
    }
}

//////////////////////////////////////////////////////////////////////////////
// - SdlKeyboardState -
//////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct SdlKeyboardState {
    prev_keys: HashSet<Keycode>,
    pressed_keys: HashSet<Keycode>,
    new_keys: HashSet<Keycode>,
    old_keys: HashSet<Keycode>,
    modifiers: Mod,
}

impl SdlKeyboardState {
    pub(crate) fn new(pressed_keys: Vec<Keycode>, modifiers: Mod) -> SdlKeyboardState {
        Self {
            pressed_keys: HashSet::from_iter(pressed_keys),
            modifiers,
            ..Default::default()
        }
    }

    pub fn update(&mut self, window: &SdlWindow) {
        self.pressed_keys =
            HashSet::from_iter(window.get_pressed_keys().expect("Failed to get pressed keys"));
        self.new_keys = &self.pressed_keys - &self.prev_keys;
        self.old_keys = &self.prev_keys - &self.pressed_keys;
        self.prev_keys = self.pressed_keys.clone();

        // Update key modifiers (shift, alt, ctrl)
        self.modifiers = window.sdl.keyboard().mod_state();
    }

    pub fn is_shift_pressed(&self) -> bool {
        self.modifiers.intersects(Mod::LSHIFTMOD) || self.modifiers.intersects(Mod::RSHIFTMOD)
    }

    pub fn is_control_pressed(&self) -> bool {
        self.modifiers.intersects(Mod::LCTRLMOD) || self.modifiers.intersects(Mod::RCTRLMOD)
    }

    pub fn is_alt_pressed(&self) -> bool {
        self.modifiers.intersects(Mod::LALTMOD) || self.modifiers.intersects(Mod::RALTMOD)
    }

    /// Returns `true` if the specified `key` was pressed this frame and not in the
    /// previous frame, indicating a new key press.
    pub fn is_key_pressed(&self, key: Keycode) -> bool {
        self.new_keys.contains(&key)
    }

    /// Returns `true` if the specified `key` is currently pressed, indicating it is
    /// either being held down or was just pressed.
    pub fn is_key_pressed_repeat(&self, key: Keycode) -> bool {
        self.pressed_keys.contains(&key)
    }

    /// Returns `true` if the specified `key` is currently pressed, indicating it is
    /// being held down.
    pub fn is_key_down(&self, key: Keycode) -> bool {
        self.pressed_keys.contains(&key)
    }

    /// Returns `true` if the specified `key` was released this frame after being
    /// pressed in a previous frame.
    pub fn is_key_released(&self, key: Keycode) -> bool {
        self.old_keys.contains(&key)
    }

    /// Returns `true` if the specified `key` is not currently pressed.
    pub fn is_key_up(&self, key: Keycode) -> bool {
        !self.pressed_keys.contains(&key)
    }
}

impl Default for SdlKeyboardState {
    fn default() -> Self {
        Self {
            prev_keys: HashSet::new(),
            pressed_keys: HashSet::new(),
            new_keys: HashSet::new(),
            old_keys: HashSet::new(),
            modifiers: Mod::empty(),
        }
    }
}
