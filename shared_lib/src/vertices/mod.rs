use crate::color::Color;

pub mod textured_vertex;

pub trait VertexTexCoords {
    fn with_tex_coords(self, u: f32, v: f32) -> Self;
    fn set_tex_coords(&mut self, u: f32, v: f32) -> &mut Self;
}

pub trait VertexColor {
    fn with_color(self, r: f32, g: f32, b: f32, a: f32) -> Self;
    fn set_color(&mut self, r: f32, g: f32, b: f32, a: f32) -> &mut Self;
    fn set_color_ref(&mut self, color: &Color) -> &mut Self;
    fn set_color_array(&mut self, color: [f32; 4]) -> &mut Self;
    fn set_rgb(&mut self, r: f32, g: f32, b: f32) -> &mut Self;
    fn set_opacity(&mut self, opacity: f32) -> &mut Self;
}
