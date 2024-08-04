use crate::camera::orthographic_camera::OrthographicCamera;
use crate::camera::perspective_camera::PerspectiveCamera;
use crate::camera::{Camera, CameraError, CameraMovement};
use anyhow::Result;
use cgmath::{InnerSpace, Matrix4};
use std::any::Any;

/// A struct that provides movement capabilities for a `PerspectiveCamera`.
///
/// The `MoveableCamera` struct wraps a `PerspectiveCamera`
/// and allows it to move through the scene with a specified speed.
///
/// # Fields
///
/// * `speed` - The speed at which the camera moves through the scene.
///   Units are typically meters per second but depend on the scene scale.
/// * `camera` - An instance of the `PerspectiveCamera` that this `MoveableCamera` controls.
///
/// This field provides direct access to the underlying `PerspectiveCamera` instance,
/// allowing the `MoveableCamera` to manipulate its position, direction, and other properties
/// to facilitate camera movement within the scene.
///
/// # Example
/// ```ignore
/// let perspective_camera = PerspectiveCamera::default();
/// let mut moveable_camera = MoveableCamera::new(perspective_camera);
///
/// moveable_camera.move_forward(Some(5.0)); // Moves the camera forward by 5 units
/// moveable_camera.move_forward(None); // Moves the camera forward by the default speed
/// ```
#[derive(Debug, Copy, Clone)]
pub struct MoveableCamera {
    /// The speed at which the camera moves through the scene.
    /// Units are typically meters per second but depend on the scene scale.
    pub speed: f32,

    /// The `PerspectiveCamera` that this `MoveableCamera` controls.
    ///
    /// This field provides direct access to the underlying `PerspectiveCamera` instance,
    /// allowing the `MoveableCamera` to manipulate its position, direction, and other properties
    /// to facilitate camera movement within the scene.
    pub camera: PerspectiveCamera,
}

impl Default for MoveableCamera {
    /// Creates a default `MoveableCamera` instance with a default speed and a default `PerspectiveCamera`.
    ///
    /// # Returns
    /// A new instance of `MoveableCamera` with default values.
    fn default() -> Self {
        let perspective_camera = PerspectiveCamera::default();
        Self {
            speed: 1.0,
            camera: perspective_camera,
        }
    }
}

impl MoveableCamera {
    /// Creates a new `MoveableCamera` instance with a specified `PerspectiveCamera`.
    ///
    /// # Parameters
    /// * `camera` - An instance of the `PerspectiveCamera` to be controlled.
    ///
    /// # Returns
    /// A new instance of `MoveableCamera`.
    pub fn new(camera: PerspectiveCamera) -> Self {
        Self {
            speed: 1.0,
            camera,
        }
    }

    /// Resets the camera's movement speed to the default value.
    pub fn reset_speed(&mut self) {
        self.speed = 1.0;
    }

    /// Resets the camera's position to its default state.
    pub fn reset_position(&mut self) {
        self.camera.reset_position();
    }

    /// Moves the camera sideways (left/right) based on the direction and delta_time.
    ///
    /// # Parameters
    /// - `delta_time`: The time elapsed since the last frame.
    /// - `direction`: The direction to move the camera (positive for right, negative for left).
    pub fn strafe(&mut self, delta_time: f32, direction: f32) {
        let right = self.camera.direction.cross(self.camera.up).normalize();
        let offset = right * self.speed * delta_time * direction;
        self.camera.position += offset;
    }
}

impl CameraMovement for MoveableCamera {
    /// Moves the camera forward by a specified distance or by the default speed if no distance is provided.
    fn move_forward(&mut self, distance: Option<f32>) {
        let distance = distance.unwrap_or(self.speed);
        self.camera.position += self.camera.direction * distance;
    }

    fn move_backward(&mut self, distance: Option<f32>) {
        let distance = distance.unwrap_or(self.speed);
        self.camera.position -= self.camera.direction * distance;
    }

    fn move_left(&mut self, distance: Option<f32>) {
        let distance = distance.unwrap_or(self.speed);
        let left = self.camera.direction.cross(self.camera.up).normalize();
        self.camera.position += left * distance;
    }

    fn move_right(&mut self, distance: Option<f32>) {
        let distance = distance.unwrap_or(self.speed);
        let right = self.camera.up.cross(self.camera.direction).normalize();
        self.camera.position += right * distance;
    }

    fn move_up(&mut self, distance: Option<f32>) {
        let distance = distance.unwrap_or(self.speed);
        self.camera.position += self.camera.up * distance;
    }

    fn move_down(&mut self, distance: Option<f32>) {
        let distance = distance.unwrap_or(self.speed);
        self.camera.position -= self.camera.up * distance;
    }

    fn rotate(&mut self, angles: (f32, f32, f32)) {
        let (pitch, yaw, roll) = angles;
        self.camera.pitch += pitch;
        self.camera.yaw += yaw;
        self.camera.roll += roll;
    }
}

impl Camera for MoveableCamera {
    fn get_matrix_world_inverse(&self) -> &Matrix4<f32> {
        self.camera.get_matrix_world_inverse()
    }

    fn get_projection_matrix(&self) -> &Matrix4<f32> {
        self.camera.get_projection_matrix()
    }

    fn get_projection_matrix_inverse(&self) -> &Matrix4<f32> {
        self.camera.get_projection_matrix_inverse()
    }

    fn get_view_projection_matrix(&self) -> &Matrix4<f32> {
        self.camera.get_view_projection_matrix()
    }

    fn copy_from(&mut self, source: &dyn Camera) -> Result<(), CameraError> {
        let source = source.as_any();
        if let Some(moveable_camera) = source.downcast_ref::<MoveableCamera>() {
            self.camera.copy_from(&moveable_camera.camera);
            self.speed = moveable_camera.speed;
        } else if let Some(perspective_camera) = source.downcast_ref::<PerspectiveCamera>() {
            self.camera.copy_from(perspective_camera);
        } else {
            return Err(CameraError::UnknownCameraType);
        }

        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self.camera.as_any()
    }
}
