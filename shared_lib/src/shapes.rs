use std::hash::{Hash, Hasher};

use anyhow::Result;
use cgmath::Matrix4;

use crate::{Position2D, Size2D};
use crate::color::Color;
use crate::projection::Projection;
use crate::shapes::rectangle::Rectangle;

pub mod rectangle;

//////////////////////////////////////////////////////////////////////////////
// - ShapesFactory -
//////////////////////////////////////////////////////////////////////////////

pub struct ShapesFactory {
    display_size: Size2D<u32>,
}

impl ShapesFactory {
    pub fn new(display_size: Size2D<u32>) -> ShapesFactory {
        let matrix = ShapesFactory::create_orthographic_projection(&display_size);
        Self { display_size }
    }

    pub fn new_with_dimensions(width: u32, height: u32) -> ShapesFactory {
        let display_size = Size2D::new(width, height);
        Self::new(display_size)
    }

    pub fn display_size(&self) -> &Size2D<u32> {
        &self.display_size
    }

    fn create_orthographic_projection(display_size: &Size2D<u32>) -> Matrix4<f32> {
        let projection = Projection::new_orthographic(
            0.0,
            0.0,
            display_size.width as f32,
            display_size.height as f32,
            -1.0,
            1.0,
        );
        *projection.get_matrix()
    }

    pub fn create_rectangle(
        &self,
        x: f32,
        y: f32,
        width: u32,
        height: u32,
        color: Color,
    ) -> Result<Rectangle> {
        let projection = Self::create_orthographic_projection(&self.display_size);
        let position = Position2D::new(x, y);
        let size = Size2D::<f32>::new(width as f32, height as f32);
        let rectangle = Rectangle::new(position, size, color, projection)?;
        Ok(rectangle)
    }
}
