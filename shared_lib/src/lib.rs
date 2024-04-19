extern crate gl;

#[allow(dead_code)]
#[allow(unused_macros)]
pub mod color;
pub mod conversion_utils;
pub mod gl_buffer;
pub mod gl_draw;
pub mod gl_font;
pub mod gl_shader;
pub mod gl_shader_manager;
pub mod gl_texture;
pub mod gl_traits;
pub mod gl_types;
pub mod gl_utils;
pub mod gl_vertex;
pub mod gl_vertex_array;
pub mod gl_vertex_attribute;
pub mod meshes;
pub mod sdl_window;
pub mod serialization;
pub mod string_utils;
pub mod vertices;

pub mod prelude {
    pub use crate::color::*;
    pub use crate::sdl_window::SdlWindow;
}
pub mod gl_prelude {
    pub use crate::gl_buffer::*;
    pub use crate::gl_draw::*;
    pub use crate::gl_font::*;
    pub use crate::gl_shader::*;
    pub use crate::gl_shader_manager::*;
    pub use crate::gl_texture::*;
    pub use crate::gl_traits::*;
    pub use crate::gl_types::*;
    pub use crate::gl_utils::*;
    pub use crate::gl_vertex::*;
    pub use crate::gl_vertex_array::*;
    pub use crate::gl_vertex_attribute::*;
}

/// The `Drawable` trait defines the interface for renderable objects.
///
/// This trait requires implementing the `draw` method, which handles
/// the rendering logic for the object. It returns a `Result<(), E>` to
/// allow for error handling in cases where drawing might fail due to
/// various reasons, such as OpenGL context issues or shader compilation errors.
///
/// # Examples
///
/// Implementing the `Drawable` trait for a custom `Square` struct:
///
/// ```no-run
/// struct Square {
///     // Square-specific fields
/// }
///
/// impl Drawable for Square {
///     fn draw(&self) -> Result<(), std::io::Error> {
///         // Implementation for drawing the square
///         Ok(())
///     }
/// }
/// ```
pub trait Drawable {
    /// Draws the object.
    ///
    /// This method should contain all the logic necessary to render the object
    /// on the screen. It returns a `Result<(), E>` where `E` is the error type.
    ///
    /// # Errors
    ///
    /// This method should return an error if any issues occur during the drawing
    /// process.
    fn draw(&self) -> anyhow::Result<()>;
}
