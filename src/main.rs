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

use crate::renderable::first_triangle::FirstTriangle;
use crate::renderable::indexed_quad::IndexedQuad;
use crate::renderable::Renderable;
use anyhow::Result;
use color::Color;
use sdl2::event::Event;
use sdl_window::SdlWindow;

fn main() -> Result<()> {
    let mut window = SdlWindow::new(800, 600, "RUST SDL 2024", true)?;
    window.clear_color = Color::new(0.10, 0.10, 0.25, 1.0);

    // let mut triangle = FirstTriangle::new()?;
    // triangle.setup()?;

    let mut indexed_quad = IndexedQuad::new()?;
    indexed_quad.setup()?;

    'main_loop: loop {
        for event in window.event_pump.poll_iter() {
            if let Event::Quit { .. } = event {
                break 'main_loop;
            }
        }

        window.clear();

        //triangle.draw();
        indexed_quad.draw();

        window.swap();
    }

    Ok(())
}
