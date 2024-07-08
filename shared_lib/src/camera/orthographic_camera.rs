use anyhow::{Context, Result};
use cgmath::{ortho, Matrix4, Point3, SquareMatrix, Transform, Vector3};

use crate::camera::Camera;

/// The `OrthographicCamera` struct represents a camera with an orthographic projection,
/// defined by six values that form the viewing frustum.
#[derive(Debug, Copy, Clone)]
pub struct OrthographicCamera {
    ///  The left plane of the camera frustum.
    pub left: f32,
    /// The right plane of the camera frustum.
    pub right: f32,
    /// The top plane of the camera frustum.
    pub top: f32,
    /// The bottom plane of the camera frustum.
    pub bottom: f32,
    /// The near plane of the camera frustum. Default is 0.1.
    pub near: f32,
    /// The far plane of the camera frustum. Default is 2000.
    pub far: f32,
    /// Gets or sets the zoom factor of the camera. Default is 1.
    pub zoom: i32,
    /// The position of the camera in world space.
    pub position: Point3<f32>,

    projection_matrix: Matrix4<f32>,
    projection_matrix_inverse: Matrix4<f32>,
    view_matrix: Matrix4<f32>,
    matrix_world_inverse: Matrix4<f32>,
    view_projection_matrix: Matrix4<f32>,
}

impl OrthographicCamera {
    /// Updates the camera projection matrix.
    /// This method should be called whenever any parameters are changed.
    pub fn update_projection_matrix(&mut self) -> Result<()> {
        let dx = (self.right - self.left) / (2.0 * self.zoom as f32);
        let dy = (self.top - self.bottom) / (2.0 * self.zoom as f32);
        let cx = (self.right + self.left) / 2.0;
        let cy = (self.top + self.bottom) / 2.0;

        let left = cx - dx;
        let right = cx + dx;
        let top = cy + dy;
        let bottom = cy - dy;

        self.projection_matrix = ortho(left, right, top, bottom, self.near, self.far);
        self.projection_matrix_inverse =
            self.projection_matrix.invert().context("Matrix is not invertible")?;

        self.view_matrix = Matrix4::look_at_rh(
            self.position,
            Point3::new(0.0, 0.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
        );
        self.matrix_world_inverse =
            self.view_matrix.invert().context("Matrix is not invertible")?;

        Ok(())
    }
}

impl Default for OrthographicCamera {
    fn default() -> Self {
        Self {
            left: -1.0,
            right: -1.0,
            top: -1.0,
            bottom: -1.0,
            near: 0.1,
            far: 2000.0,
            zoom: 1,
            position: Point3::<f32>::new(0.0, 0.0, 0.0),
            projection_matrix: Matrix4::<f32>::identity(),
            projection_matrix_inverse: Matrix4::<f32>::identity(),
            view_matrix: Matrix4::<f32>::identity(),
            matrix_world_inverse: Matrix4::<f32>::identity(),
            view_projection_matrix: Matrix4::<f32>::identity(),
        }
    }
}

impl Camera for OrthographicCamera {
    fn get_matrix_world_inverse(&self) -> &Matrix4<f32> {
        &self.matrix_world_inverse
    }

    fn get_projection_matrix(&self) -> &Matrix4<f32> {
        &self.projection_matrix
    }

    fn get_projection_matrix_inverse(&self) -> &Matrix4<f32> {
        &self.projection_matrix_inverse
    }

    fn get_view_projection_matrix(&self) -> &Matrix4<f32> {
        &self.view_projection_matrix
    }

    fn copy(&mut self, source: &Self) -> &mut Self {
        self.left = source.left;
        self.right = source.right;
        self.top = source.top;
        self.bottom = source.bottom;
        self.near = source.near;
        self.far = source.far;
        self.zoom = source.zoom;
        self
    }
}

/// Implementing the Into trait for OrthographicCamera to convert it into a reference to Matrix4
impl<'a> Into<&'a Matrix4<f32>> for &'a OrthographicCamera {
    /// Converts a reference to OrthographicCamera into a reference to its view-projection matrix
    fn into(self) -> &'a Matrix4<f32> {
        self.get_view_projection_matrix()
    }
}
