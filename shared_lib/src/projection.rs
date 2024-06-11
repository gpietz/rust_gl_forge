use crate::gl_types::ProjectionMatrix;
use crate::Size2D;
use cgmath::{ortho, perspective, Matrix4, Rad, SquareMatrix};
use sdl2::sys::Screen;
use std::mem::Discriminant;

#[derive(Debug, Copy, Clone)]
pub enum ProjectionType {
    Perspective,
    Orthographic,
}

#[derive(Debug, Copy, Clone)]
pub struct Projection {
    fov: Option<f32>,
    aspect_ratio: f32,
    near: f32,
    far: f32,
    projection_matrix: ProjectionMatrix,
}

impl Projection {
    pub fn new_perspective_with_screen(
        fov: f32,
        screen_width: u32,
        screen_height: u32,
        near: f32,
        far: f32,
    ) -> Self {
        let aspect_ratio = screen_width as f32 / screen_height as f32;
        let projection_matrix = perspective(Rad(fov), aspect_ratio, near, far);
        Self {
            fov: Some(fov),
            aspect_ratio,
            near,
            far,
            projection_matrix,
        }
    }

    pub fn new_perspective(fov: f32, aspect_ratio: f32, near: f32, far: f32) -> Self {
        let projection_matrix = perspective(Rad(fov), aspect_ratio, near, far);
        Self {
            fov: Some(fov),
            aspect_ratio,
            near,
            far,
            projection_matrix,
        }
    }

    pub fn new_orthographic(
        left: f32,
        top: f32,
        right: f32,
        bottom: f32,
        near: f32,
        far: f32,
    ) -> Self {
        let projection_matrix = ortho(left, right, bottom, top, near, far);
        Self {
            fov: None,
            aspect_ratio: (right - left) / (top - bottom),
            near,
            far,
            projection_matrix,
        }
    }

    pub fn update_perspective_with_screen(
        &mut self,
        fov: f32,
        screen_width: f32,
        screen_height: f32,
        near: f32,
        far: f32,
    ) {
        self.fov = Some(fov);
        self.aspect_ratio = screen_width / screen_height;
        self.near = near;
        self.far = far;
        self.projection_matrix = perspective(Rad(fov), self.aspect_ratio, near, far);
    }

    pub fn update_perspective(&mut self, fov: f32, aspect_ratio: f32, near: f32, far: f32) {
        self.fov = Some(fov);
        self.aspect_ratio = aspect_ratio;
        self.near = near;
        self.far = far;
        self.projection_matrix = perspective(Rad(fov), aspect_ratio, near, far)
    }

    pub fn update_orthographic(
        &mut self,
        left: f32,
        top: f32,
        right: f32,
        bottom: f32,
        near: f32,
        far: f32,
    ) {
        self.fov = None;
        self.aspect_ratio = (right - left) / (top - bottom);
        self.near = near;
        self.far = far;
        self.projection_matrix = ortho(left, right, bottom, top, near, far);
    }

    pub fn get_matrix(&self) -> &Matrix4<f32> {
        &self.projection_matrix
    }

    pub fn projection_type(&self) -> ProjectionType {
        self.fov.map_or(ProjectionType::Orthographic, |_| ProjectionType::Perspective)
    }

    pub fn is_perspective(&self) -> bool {
        self.fov.is_some()
    }

    pub fn is_orthographic(&self) -> bool {
        self.fov.is_none()
    }
}

impl PartialEq for Projection {
    fn eq(&self, other: &Self) -> bool {
        self.fov == other.fov
            && self.aspect_ratio == other.aspect_ratio
            && self.near == other.near
            && self.far == other.far
    }
}

pub trait HasOptionalProjection {
    fn get_projection(&self) -> Option<&Projection>;
    fn set_projection(&mut self, projection: Option<&Projection>);
    fn has_projection(&self) -> bool;
}

pub fn update_orthogonal_projection(
    projection: Option<&Projection>,
    last_projection: &mut Option<&Projection>,
    projection_matrix: &mut ProjectionMatrix,
) -> bool {
    match projection {
        Some(proj) => {
            if projection_matrix.is_identity()
                || !is_projection_equal_projection(projection, last_projection)
            {
                
                
                //let ortho_projection = ortho(0.0)  
            }
        }
        None => {
            if !projection_matrix.is_identity() {
                *projection_matrix = Matrix4::<f32>::identity();
                return true;
            }
        }
    }

    false
}

fn is_projection_equal_projection(
    projection: Option<&Projection>,
    last_projection: &mut Option<&Projection>,
) -> bool {
    match (projection, last_projection) {
        (Some(proj), Some(last_proj)) => {
            if proj.eq(last_proj) {
                true
            } else {
                false
            }
        }
        (Some(proj), None) => false,
        (None, Some(_)) => false,
        (None, None) => true,
    }
}
