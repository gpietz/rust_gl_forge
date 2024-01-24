use sdl2::{Sdl, video::{GLContext, SwapInterval, Window}, EventPump};
use anyhow::{Result, Error};
use gl::load_with;

use crate::color::Color;

//////////////////////////////////////////////////////////////////////////////
// - SdlWindow -
//////////////////////////////////////////////////////////////////////////////

pub struct SdlWindow {
    pub sdl: Sdl,
    pub window: Window,
    pub gl_context: GLContext,
    pub gl: (),
    pub event_pump: EventPump,
    pub clear_color: Color,
}

impl SdlWindow {
    /// Creates a new `SdlWindow` with OpenGL context.
    ///
    /// This function initializes the SDL2 library and creates a new window with the specified width, height, and title.
    /// It sets up the video subsystem, initializes an OpenGL context, and optionally enables VSync based on the `enable_vsync` parameter.
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
    /// A `Result<SdlWindow>`, which is `Ok` if the window, OpenGL context, and associated subsystems were successfully created,
    /// or an `Err` containing an `anyhow::Error` if an error occurred during initialization.
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
    /// let window = SdlWindow::new(800, 600, "My OpenGL Window", true);
    /// match window {
    ///     Ok(win) => { /* Proceed with window and OpenGL operations */ },
    ///     Err(e) => { /* Handle error (e.g., logging) */ }
    /// }
    /// ```
    pub fn new(width: usize, height: usize, title: &str, enable_vsync: bool) -> Result<SdlWindow> {
        let sdl = sdl2::init().map_err(Error::msg)?;
        let video_subsystem = sdl.video().map_err(Error::msg)?;
        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(3, 3);
        let window = video_subsystem.window(title, width as u32, height as u32)
            .opengl()
            .build()
            .map_err(Error::msg)?;
        let gl_context = window.gl_create_context().map_err(Error::msg)?;
        let gl = load_with(|s| {
            video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void
        });
        if enable_vsync {
            window.subsystem().gl_set_swap_interval(SwapInterval::VSync).map_err(Error::msg)?;
        }
        let event_pump = sdl.event_pump().map_err(Error::msg)?;
        Ok(SdlWindow { sdl, window, gl_context, gl, event_pump, clear_color: Color::BLACK })
    }

    /// Swaps the window's display buffer and clears the screen with the current clear color.
    ///
    /// This function first sets the OpenGL clear color to the RGBA values specified in `self.clear_color`.
    /// It then clears the color buffer to apply the clear color to the entire screen.
    /// Finally, it swaps the window's display buffer to update the display with the new frame.
    ///
    /// This function should be called at the end of each frame to render the updated frame to the screen.
    ///
    /// # Safety
    ///
    /// The function contains an `unsafe` block because it makes raw OpenGL calls, which can lead to undefined behavior if used incorrectly. 
    /// Ensure that a valid OpenGL context is current in the thread before calling this function.
    ///
    /// # Example
    ///
    /// ```
    /// // Assuming 'window' is an instance of a struct that has this swap method
    /// window.swap();
    /// ```
    pub fn swap(&self) {
        let r = self.clear_color.r;
        let g = self.clear_color.g;
        let b = self.clear_color.b;
        let a = self.clear_color.a;

        unsafe {
            gl::ClearColor(r,g,b,a);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        self.window.gl_swap_window();
    }
}
