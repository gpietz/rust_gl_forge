#![allow(dead_code)]

use anyhow::Result;
use cgmath::Matrix4;
use image::imageops::ColorMap;
use std::any::Any;
use thiserror::Error;

pub mod moveable_camera;
pub mod orthographic_camera;
pub mod perspective_camera;

/// The `Camera` trait defines the essential properties and behaviors that any camera
/// implementation should have.
/// It includes methods for accessing the inverse world transformation matrix,
/// the projection matrix, and its inverse, as well as a method for copying
/// properties from another camera.
pub trait Camera: CameraClone {
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
    fn copy_from(&mut self, source: &dyn Camera) -> Result<(), CameraError>;

    /// Provides a reference to the trait object `Any`, allowing for downcasting.
    fn as_any(&self) -> &dyn Any;
}

/// Helper trait for cloning Box<dyn Camera>
pub trait CameraClone {
    /// Creates a boxed clone of the `Camera` trait object.
    ///
    /// This function returns a `Box` containing a clone of the object implementing
    /// the `Camera` trait. This is useful for duplicating trait objects stored on the heap.
    fn clone_box(&self) -> Box<dyn Camera>;
}

impl<T> CameraClone for T
where
    T: 'static + Camera + Clone,
{
    fn clone_box(&self) -> Box<dyn Camera> {
        Box::new(self.clone())
    }
}

/// Represents errors that can occur when dealing with camera types in a graphics application.
///
/// This enum defines various error types related to camera operations and ensures detailed
/// error messages for debugging and handling errors.
///
/// # Variants
///
/// * `NotPerspectiveCamera` - Indicates that the source is not a `PerspectiveCamera`.
/// * `NotOrthographicCamera` - Indicates that the source is not an `OrthographicCamera`.
/// * `UnknownCameraType` - Indicates that the camera type is unknown.
///
/// # Example
///
/// ```ignore
/// use thiserror::Error;
///
/// #[derive(Error, Debug)]
/// pub enum CameraError {
///     #[error("Source is not a PerspectiveCamera")]
///     NotPerspectiveCamera,
///
///     #[error("Source is not a OrthographicCamera")]
///     NotOrthographicCamera,
///
///     #[error("Unknown camera type")]
///     UnknownCameraType,
/// }
///
/// fn check_camera(camera: &Camera) -> Result<(), CameraError> {
///     match camera {
///         Camera::Perspective(_) => Ok(()),
///         Camera::Orthographic(_) => Ok(()),
///         _ => Err(CameraError::UnknownCameraType),
///     }
/// }
/// ```
#[derive(Error, Debug)]
pub enum CameraError {
    /// Indicates that the source is not a `PerspectiveCamera`.
    #[error("Source is not a PerspectiveCamera")]
    NotPerspectiveCamera,

    /// Indicates that the source is not an `OrthographicCamera`.
    #[error("Source is not a OrthographicCamera")]
    NotOrthographicCamera,

    /// Indicates that the camera type is unknown.
    #[error("Unknown camera type")]
    UnknownCameraType,
}

pub trait CameraMovement {
    fn move_forward(&mut self, distance: Option<f32>);
    fn move_backward(&mut self, distance: Option<f32>);
    fn move_left(&mut self, distance: Option<f32>);
    fn move_right(&mut self, distance: Option<f32>);
    fn move_up(&mut self, distance: Option<f32>);
    fn move_down(&mut self, distance: Option<f32>);

    /// Rotates the camera by the specified angles in degrees.
    ///
    /// This method adjusts the camera's orientation by adding the specified pitch, yaw,
    /// and roll angles to the current orientation. The angles are provided as a tuple of
    /// three `f32` values, representing the rotation around the X, Y, and Z axes respectively.
    ///
    /// # Parameters
    /// - `angles`: A tuple `(pitch, yaw, roll)` where:
    ///   - `pitch` (f32): The angle to rotate around the X-axis (in degrees).
    ///   - `yaw` (f32): The angle to rotate around the Y-axis (in degrees).
    ///   - `roll` (f32): The angle to rotate around the Z-axis (in degrees).
    ///
    /// # Example
    /// ```ignore
    /// let mut camera_movement = CameraMovement::new();
    /// camera_movement.rotate((10.0, 15.0, 5.0));
    /// ```
    ///
    /// This example rotates the camera by 10 degrees around the X-axis, 15 degrees around the Y-axis,
    /// and 5 degrees around the Z-axis.
    fn rotate(&mut self, angles: (f32, f32, f32)); // (pitch, yaw, roll)
}
