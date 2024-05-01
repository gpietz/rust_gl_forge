#![allow(dead_code)]

use cgmath::{InnerSpace, Matrix4, Point3, Vector3};

#[derive(Debug)]
pub struct Camera {
    /// Represents the location of the camera in 3D space.
    pub position: Point3<f32>,
    /// The direction that the camera is looking towards. This is typically a normalized vector.
    pub front: Vector3<f32>,
    /// This vector represents the upward direction relative to the camera. This helps define
    /// the orientation of the camera along with the front vector.
    pub up: Vector3<f32>,
    // Speed of the camera's movement.
    pub speed: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: Point3::new(0.0, 0.0, 0.0),
            front: Vector3::new(0.0, 0.0, -1.0),
            up: Vector3::new(0.0, 1.0, 0.0),
            speed: 1.0,
        }
    }
}

impl Camera {
    pub fn update_view_mat4(&mut self, view: &mut Matrix4<f32>) {
        *view = Matrix4::look_at_rh(self.position, self.position + self.front, self.up);
    }

    pub fn move_forward(&mut self, delta_time: f32) {
        let offset = self.front * self.speed * delta_time;
        self.position += offset;
    }

    pub fn move_backward(&mut self, delta_time: f32) {
        let offset = self.front * self.speed * delta_time;
        self.position -= offset;
    }

    pub fn move_along_front(&mut self, delta_time: f32, direction: f32) {
        let offset = self.front * self.speed * direction;
        self.position += offset;
    }

    // Optionally, you could merge move_forward and move_backward into one
    // method by accepting a direction multiplier:
    pub fn strafe(&mut self, delta_time: f32, direction: f32) {
        let right = self.front.cross(self.up).normalize();
        let offset = right * self.speed * delta_time * direction;
        self.position += offset;
    }

    pub fn set_speed(&mut self, new_speed: f32) {
        self.speed = new_speed;
    }

    pub fn reset(&mut self, reset_speed: bool) {
        self.position = Point3::new(0.0, 0.0, 0.0);
        self.front = Vector3::new(0.0, 0.0, -1.0);
        self.up = Vector3::new(0.0, 1.0, 0.0);
        if reset_speed {
            self.speed = 1.0;
        }
    }

    pub fn reset_position(&mut self) {
        self.position = Point3::new(0.0, 0.0, 0.0);
    }
}

impl Clone for Camera {
    fn clone(&self) -> Self {
        Self {
            position: self.position,
            front: self.front,
            up: self.up,
            speed: self.speed,
        }
    }
}
