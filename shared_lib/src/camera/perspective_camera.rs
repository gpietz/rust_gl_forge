use crate::camera::Camera;
use crate::conversion_utils::convert_to_vector3_vec;
use crate::math::deg_to_rad;
use crate::Position2D;
use anyhow::{anyhow, Context, Result};
use cgmath::{perspective, Deg, Matrix4, Point3, Rad, SquareMatrix, Transform, Vector2, Vector3};
use float_cmp::approx_eq;
use crate::camera::orthographic_camera::OrthographicCamera;

#[derive(Debug, Copy, Clone)]
pub struct PerspectiveCamera {
    /// The position of the camera in 3D space.
    pub position: Point3<f32>,
    /// The point in 3D space that the camera is looking at.
    pub target: Point3<f32>,
    /// The up direction of the camera, typically (0.0, 1.0, 0.0).
    pub up: Vector3<f32>,
    /// The direction vector of the camera, indicating where the camera is pointing.
    pub direction: Vector3<f32>,
    /// The field of view of the camera, in degrees.
    pub fov: f32,
    /// The aspect ratio of the camera's view (width / height).
    pub aspect: f32,
    /// The near clipping plane distance.
    pub near: f32,
    /// The far clipping plane distance.
    pub far: f32,
    /// The zoom factor of the camera.
    pub zoom: i32,
    /// The yaw (rotation around the y-axis) of the camera, in degrees.
    pub yaw: f32,
    /// The pitch (rotation around the x-axis) of the camera, in degrees.
    pub pitch: f32,

    projection_matrix: Matrix4<f32>,
    projection_matrix_inverse: Matrix4<f32>,
    view_matrix: Matrix4<f32>,
    matrix_world_inverse: Matrix4<f32>,
    view_projection_matrix: Matrix4<f32>,
}

impl PerspectiveCamera {
    pub fn new(position: Point3<f32>) -> Self {
        let mut camera = Self {
            position,
            ..PerspectiveCamera::default()
        };
        camera.update_projection_matrix().unwrap();
        camera
    }

    pub fn update_projection_matrix(&mut self) -> Result<()> {
        if self.zoom <= 0 {
            return Err(anyhow!("Zoom must be greater than zero"));
        }

        // Calculate the projection matrix
        let fovy = Deg(self.fov / self.zoom as f32);
        self.projection_matrix = perspective(fovy, self.aspect, self.near, self.far);
        self.projection_matrix =
            self.projection_matrix.invert().context("Projection matrix is not invertible")?;

        // Calculate the view matrix
        let rotation_yaw = Matrix4::from_angle_y(Deg(self.yaw));
        let rotation_pitch = Matrix4::from_angle_x(Deg(self.pitch));
        let rotation_matrix = rotation_yaw * rotation_pitch;
        self.direction = (rotation_matrix * self.direction.extend(0.0)).truncate();

        self.view_matrix = Matrix4::look_at_rh(self.position, self.position + self.direction, self.up);
        self.matrix_world_inverse = self.view_matrix.invert().context("View matrix is not invertible")?;

        // Calculate the combined view-projection matrix
        self.view_projection_matrix = self.projection_matrix * self.view_matrix;

        Ok(())
    }

    /// Sets the zoom level, ensuring it is at least 1.
    /// Returns true if the zoom level was changed, otherwise false.
    pub fn set_zoom(&mut self, zoom: i32) -> bool {
        if self.zoom == zoom {
            return false;
        }
        self.zoom = zoom.max(1);
        true
    }

    /// Sets the zoom level and updates the projection matrix if the zoom level was changed.
    /// Returns Ok(true) if updated, Ok(false) if not, or an error.
    pub fn set_zoom_and_update(&mut self, zoom: i32) -> Result<bool> {
        if self.set_zoom(zoom) {
            self.update_projection_matrix()?;
            return Ok(true);
        }
        Ok(false)
    }

    /// Sets the aspect ratio based on the given width and height,
    /// returns true if the aspect ratio changed.
    pub fn set_aspect_from_width_and_height(&mut self, width: f32, height: f32) -> bool {
        let aspect = width / height;
        if !approx_eq!(f32, self.aspect, aspect, ulps = 2) {
            self.aspect = aspect;
            return true;
        }
        false
    }
}

impl Default for PerspectiveCamera {
    fn default() -> Self {
        let position = Point3::<f32>::new(0.0, 0.0, 0.0);
        Self {
            position,
            target: position + Vector3::new(0.0, 0.0, -1.0),
            up: Vector3::new(0.0, 1.0, 0.0),
            direction: Vector3::new(0.0, 0.0, -1.0),
            fov: 60.0,
            aspect: 1.77,
            near: 0.1,
            far: 50.0,
            zoom: 1,
            yaw: -90.0,
            pitch: 0.0,
            projection_matrix: Matrix4::identity(),
            projection_matrix_inverse: Matrix4::identity(),
            view_matrix: Matrix4::identity(),
            matrix_world_inverse: Matrix4::identity(),
            view_projection_matrix: Matrix4::identity(),
        }
    }
}

impl Camera for PerspectiveCamera {
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
        self.position = source.position;
        self.target = source.target;
        self.up = source.up;
        self.fov = source.fov;
        self.aspect = source.aspect;
        self.near = source.near.max(0.1);
        self.far = source.far;
        self.zoom = source.zoom;
        self.yaw = source.yaw;
        self.pitch = source.pitch;
        self
    }
}

/// Implementing the Into trait for PerspectiveCamera to convert it into a reference to Matrix4
impl<'a> Into<&'a Matrix4<f32>> for &'a PerspectiveCamera {
    /// Converts a reference to PerspectiveCamera into a reference to its view-projection matrix
    fn into(self) -> &'a Matrix4<f32> {
        self.get_view_projection_matrix()
    }
}
