#![allow(dead_code)]

use cgmath::Matrix4;

pub mod orthographic_camera;
pub mod perspective_camera;

/// The `Camera` trait defines the essential properties and behaviors that any camera
/// implementation should have.
/// It includes methods for accessing the inverse world transformation matrix,
/// the projection matrix, and its inverse, as well as a method for copying
/// properties from another camera.
pub trait Camera: Copy + Clone {
    /// Returns the inverse of the world transformation matrix of the camera.
    fn get_matrix_world_inverse(&self) -> &Matrix4<f32>;

    /// Returns the projection matrix of the camera.
    fn get_projection_matrix(&self) -> &Matrix4<f32>;

    /// Returns the inverse of the projection matrix.
    fn get_projection_matrix_inverse(&self) -> &Matrix4<f32>;

    /// Returns the combined view-projection matrix for the camera.
    /// This matrix transforms world coordinates to clip space.
    fn get_view_projection_matrix(&self) -> &Matrix4<f32>;

    /// Copies the properties from the source camera into this one.
    fn copy(&mut self, source: &Self) -> &mut Self;
}
