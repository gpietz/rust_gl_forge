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
pub mod string_utils;
pub mod vertices;
pub mod traits;

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
