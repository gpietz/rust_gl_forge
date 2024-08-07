#![allow(unused)]
extern crate gl;

use crate::color::Color;
use anyhow::Result;
use cgmath::num_traits::{Num, Signed, Unsigned};
use cgmath::Matrix4;
use image::codecs::hdr::HdrEncoder;
use rusttype::Scale;
use std::borrow::Cow;

pub mod apps;
pub mod camera;
pub mod color;
mod component;
pub mod conversion_utils;
pub mod core;
mod geometry;
pub mod geometry_manager;
pub mod gl_draw;
pub mod gl_traits;
pub mod gl_types;
pub mod gl_utils;
pub mod input;
pub mod math;
pub mod mesh;
pub mod opengl;
pub mod projection;
pub mod rectangle;
pub mod sdl_window;
pub mod serialization;
pub mod shapes;
pub mod string_utils;
pub mod sys_event;
pub mod text;
pub mod vertex;
pub mod vertices;

pub mod prelude {
    pub use crate::color::*;
    pub use crate::sdl_window::SdlWindow;
}

pub mod gl_prelude {
    pub use crate::gl_draw::*;
    pub use crate::gl_traits::*;
    pub use crate::gl_types::*;
    pub use crate::gl_utils::*;
}

//////////////////////////////////////////////////////////////////////////////
// - Position2D -
//////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Position2D {
    pub x: f32,
    pub y: f32,
}

impl Position2D {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
// - Size2D -
//////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Size2D<T: Num + Copy> {
    pub width: T,
    pub height: T,
}

impl<T: Num + Copy> Size2D<T> {
    pub fn new(width: T, height: T) -> Self {
        Self {
            width,
            height,
        }
    }
}

impl<T> Into<Scale> for Size2D<T>
where
    T: Into<f32> + Num + Copy,
{
    fn into(self) -> Scale {
        Scale {
            x: self.width.into(),
            y: self.height.into(),
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
// - Drawable -
//////////////////////////////////////////////////////////////////////////////

pub trait Drawable {
    fn draw(&mut self) -> Result<()>;

    fn set_position(&mut self, position2d: Position2D) -> Result<()>;
    fn get_position(&self) -> &Position2D;

    fn set_size(&mut self, width: f32, height: f32) -> Result<()>;
    fn get_size(&self) -> &Size2D<f32>;

    fn set_color(&mut self, color: Color) -> Result<()>;
    fn get_color(&self) -> &Color;

    fn set_projection_matrix(&mut self, projection_matrix: &Matrix4<f32>) -> Result<()>;
    fn get_projection_matrix(&self) -> &Matrix4<f32>;
}

//////////////////////////////////////////////////////////////////////////////
// - RenderTarget -
//////////////////////////////////////////////////////////////////////////////

pub trait RenderTarget<'a> {
    fn clear(&self);

    /// Clears the entire target with a single color.
    fn clear_with_color(&mut self, color: impl Into<Cow<'a, Color>>);
}

//////////////////////////////////////////////////////////////////////////////
// - DataState -
//////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum RenderDataState {
    None,
    Provided,
    Uploaded,
    NeedsUpdate,
}

pub(crate) trait RenderPrepare {
    fn prepare_render(&self);
}
