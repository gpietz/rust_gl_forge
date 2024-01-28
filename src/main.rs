#![allow(dead_code)]

mod color;
mod conversion_utils;
mod gl_buffer;
mod gl_shader;
mod gl_types;
mod gl_utils;
mod gl_vertex;
mod renderable;
mod sdl_window;
mod string_utils;

use crate::gl_buffer::BufferObject;
use crate::gl_types::{BufferType, BufferUsage, VertexAttributeType};
use crate::gl_vertex::{VertexArrayObject, VertexAttribute};
use crate::renderable::first_triangle::FirstTriangle;
use crate::renderable::Renderable;
use anyhow::Result;
use cgmath::Vector3;
use color::Color;
use gl::types::{GLfloat, GLsizei};
use sdl2::event::Event;
use sdl_window::SdlWindow;
use std::mem::size_of;

fn main() -> Result<()> {
    let mut window = SdlWindow::new(800, 600, "RUST SDL 2024", true)?;
    window.clear_color = Color::new(0.10, 0.10, 0.25, 1.0);

    let mut triangle = FirstTriangle::new()?;
    triangle.setup()?;

    'main_loop: loop {
        for event in window.event_pump.poll_iter() {
            if let Event::Quit { .. } = event {
                break 'main_loop;
            }
        }

        window.clear();

        triangle.draw();

        window.swap();
    }

    Ok(())
}
