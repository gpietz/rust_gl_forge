#[allow(dead_code)]
#[allow(unused_macros)]

pub mod color;
pub mod conversion_utils;
pub mod gl_buffer;
pub mod gl_draw;
pub mod gl_shader;
pub mod gl_types;
pub mod gl_texture;
pub mod gl_traits;
pub mod gl_utils;
pub mod gl_vertex_attribute;
pub mod gl_vertex;
pub mod sdl_window;
pub mod string_utils;

pub mod prelude {
    pub use crate::color::*;
    pub use crate::sdl_window::SdlWindow;
    pub use crate::gl_utils::*;
    pub use crate::gl_types::*;
}
