#![allow(dead_code)]

pub mod view;

use cgmath::{Deg, InnerSpace, Matrix4, Point3, Rad, Vector3};

use crate::input::mouse_adapter::MouseAdapter;

#[derive(Debug)]
pub struct Camera {
    /// Represents the location of the camera in 3D space.
    pub position: Point3<f32>,
    /// The forward direction the camera is looking towards. This vector should be normalized
    /// to ensure consistent operation in calculations involving direction.
    pub front: Vector3<f32>,
    // A vector pointing upwards from the camera. This vector, in conjunction with `front`,
    /// defines the camera's orientation. It should be orthogonal to `front` for proper camera
    /// alignment.
    pub up: Vector3<f32>,
    /// The direction vector of the camera, indicating where the camera is currently pointing.
    /// This is typically used to calculate the camera's viewing direction and should be normalized.
    pub direction: Vector3<f32>,
    /// The speed at which the camera moves through the scene.
    /// Units are typically meters per second but depend on the scene scale.
    pub speed: f32,

    pub sensitivity: f32,

    yaw: f32,
    pitch: f32,
    first_mouse: bool,
    last_mouse_position: [f32; 2],
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: Point3::new(0.0, 0.0, 0.0),
            front: Vector3::new(0.0, 0.0, -1.0),
            up: Vector3::new(0.0, 1.0, 0.0),
            direction: Vector3::new(0.0, 0.0, -1.0),
            speed: 1.0,
            sensitivity: 0.1,
            yaw: -90.0,
            pitch: 0.0,
            first_mouse: true,
            last_mouse_position: [0.0, 0.0],
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

    pub fn reset(&mut self, reset_direction: bool, reset_speed: bool) {
        self.position = Point3::new(0.0, 0.0, 0.0);
        self.front = Vector3::new(0.0, 0.0, -1.0);
        self.up = Vector3::new(0.0, 1.0, 0.0);
        if reset_direction {
            self.direction = self.front;
        }
        if reset_speed {
            self.speed = 1.0;
        }
    }

    pub fn reset_position(&mut self) {
        self.position = Point3::new(0.0, 0.0, 0.0);
    }

    pub fn reset_direction(&mut self) {
        self.direction = self.front;
    }

    pub fn update_direction(&mut self, mouse_adapter: &impl MouseAdapter) {
        let x_pos = mouse_adapter.mouse_x() as f32;
        let y_pos = mouse_adapter.mouse_y() as f32;

        if self.first_mouse {
            self.last_mouse_position[0] = x_pos;
            self.last_mouse_position[1] = y_pos;
            self.first_mouse = false;
        }

        let mut x_offset = x_pos - self.last_mouse_position[0];
        let mut y_offset = self.last_mouse_position[1] - y_pos;

        self.last_mouse_position[0] = x_pos;
        self.last_mouse_position[1] = y_pos;

        x_offset *= self.sensitivity;
        y_offset *= self.sensitivity;

        self.yaw += x_offset;
        self.pitch += y_offset;

        self.pitch = self.pitch.clamp(-89.0, 89.0);

        // Convert the angles from degrees to radians once and then use
        // them for the trigonometric functions.
        let pitch_rad = Rad::from(Deg(self.pitch));
        let yaw_rad = Rad::from(Deg(self.yaw));

        let pitch_cos = pitch_rad.0.cos();
        let pitch_sin = pitch_rad.0.sin();
        let yaw_cos = yaw_rad.0.cos();
        let yaw_sin = yaw_rad.0.sin();

        // Compute the trigonometric functions once.
        let front = Vector3 {
            x: yaw_cos * pitch_cos,
            y: pitch_sin,
            z: yaw_sin * pitch_cos,
        };

        self.front = front.normalize();
    }
}

impl Clone for Camera {
    fn clone(&self) -> Self {
        Self {
            position: self.position,
            front: self.front,
            up: self.up,
            direction: self.direction,
            speed: self.speed,
            sensitivity: self.sensitivity,
            yaw: self.yaw,
            pitch: self.pitch,
            first_mouse: self.first_mouse,
            last_mouse_position: self.last_mouse_position,
        }
    }
}
