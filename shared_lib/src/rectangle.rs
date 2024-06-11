use crate::Position2D;
use cgmath::num_traits::{Float, FromPrimitive};
use cgmath::Vector2;
use std::ops::Add;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Rectangle<T> {
    pub left: T,
    pub top: T,
    pub width: T,
    pub height: T,
}

impl<T> Rectangle<T>
where
    T: Add<Output = T> + PartialOrd + Copy + Float + FromPrimitive,
{
    /// Constructor for the rectangle
    pub const fn new(left: T, top: T, width: T, height: T) -> Self {
        Self {
            left,
            top,
            width,
            height,
        }
    }

    /// Method to get the right side of the rectangle
    pub fn right(&self) -> T
    where
        T: Add<Output = T> + Copy,
    {
        self.left + self.width
    }

    /// Method to get the bottom side of the rectangle
    pub fn bottom(&self) -> T
    where
        T: Add<Output = T> + Copy,
    {
        self.top + self.height
    }

    /// Method to check if a point is inside the rectangle
    pub fn contains(&self, x: T, y: T) -> bool
    where
        T: PartialOrd,
    {
        x >= self.left
            && x <= Rectangle::right(self)
            && y >= self.top
            && y <= Rectangle::bottom(self)
    }

    /// Method to check if two rectangles intersect
    pub fn intersects(&self, other: &Self) -> bool {
        self.left < other.right()
            && self.right() > other.left
            && self.top < other.bottom()
            && self.bottom() > other.top
    }

    /// Get the upper left position of the rectangle.
    pub fn get_position(&self) -> Vector2<T> {
        Vector2::new(self.left, self.top)
    }

    /// Get size of the rectangle.
    pub fn get_size(&self) -> Vector2<T> {
        Vector2::new(self.width, self.height)
    }

    pub fn get_center(&self) -> Vector2<T> {
        let divider = T::from_f32(2.0).unwrap();
        Vector2::new(self.width / divider, self.height / divider)
    }
}
