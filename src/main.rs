#![allow(dead_code)]

mod color;
mod conversion_utils;
mod gl_buffer;
mod gl_shader;
mod gl_types;
mod gl_utils;
mod gl_vertex;
mod sdl_window;
mod string_utils;

use crate::gl_buffer::BufferObject;
use crate::gl_types::{BufferType, BufferUsage, VertexAttributeType};
use crate::gl_vertex::{VertexArrayObject, VertexAttribute};
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

    let vertices = vec![
        Vector3::new(-0.5, -0.5, 0.0), // left
        Vector3::new(0.5, -0.5, 0.0),  // right
        Vector3::new(0.0, 0.5, 0.0),   // top
    ];

    let vao = VertexArrayObject::new()?;
    vao.bind();

    let vbo = BufferObject::new(BufferType::ArrayBuffer, BufferUsage::StaticDraw, vertices);
    vbo.bind();

    let position = VertexAttribute::new(
        0,
        3,
        VertexAttributeType::Position,
        false,
        3 * size_of::<GLfloat>() as GLsizei,
        0,
    );
    position.setup()?;
    position.enable()?;

    'main_loop: loop {
        for event in window.event_pump.poll_iter() {
            if let Event::Quit { .. } = event {
                break 'main_loop;
            }
        }

        unsafe {
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        window.swap();
    }

    Ok(())
}
