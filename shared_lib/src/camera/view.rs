use crate::math::angle::Angle;
use crate::rectangle::Rectangle;
use crate::sdl_window::WindowTrait;
use cgmath::Vector2;
use std::borrow::Cow;

/// 2D camera that defines what region is show on screen.
pub struct View {
    center: Vector2<f32>,
    size: Vector2<f32>,
    rotation: Angle,
}

impl View {
    pub fn from_rect(rect: Rectangle<f32>) -> Self {
        Self {
            center: rect.get_center(),
            size: rect.get_size(),
            rotation: Angle::ZERO,
        }
    }

    pub fn from_window(window: &dyn WindowTrait) -> Self {
        let (width, height) = (window.get_size().width as f32, window.get_size().height as f32);
        Self {
            center: Vector2::new(width / 2.0, height / 2.0),
            size: Vector2::new(width, height),
            rotation: Angle::ZERO,
        }
    }

    pub fn set_center<'a>(&mut self, new_center: impl Into<Cow<'a, Vector2<f32>>>) {
        let new_center = new_center.into();
        self.center.x = new_center.x;
        self.center.y = new_center.y;
    }

    pub fn get_center(&self) -> &Vector2<f32> {
        &self.center
    }

    pub fn set_size<'a>(&mut self, new_size: impl Into<Cow<'a, Vector2<f32>>>) {
        let new_size = new_size.into();
        self.size.x = new_size.x;
        self.size.y = new_size.y;
    }

    pub fn get_size(&self) -> &Vector2<f32> {
        &self.size
    }
}
