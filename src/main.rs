#![allow(dead_code)]

mod color;
mod gl_buffer;
mod gl_shader;
mod gl_types;
mod gl_utils;
mod gl_vertex;
mod sdl_window;
mod string_utils;

use anyhow::Result;
use color::Color;
use sdl2::event::Event;
use sdl_window::SdlWindow;

fn main() -> Result<()> {
    let mut window = SdlWindow::new(800, 600, "RUST SDL 2024", true)?;
    window.clear_color = Color::new(0.10, 0.10, 0.25, 1.0);

    'main_loop: loop {
        for event in window.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main_loop,
                _ => {}
            }
        }

        window.swap();
    }

    Ok(())
}
