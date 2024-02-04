#![allow(dead_code)]
extern crate gl;
extern crate sdl2;

mod color;
mod conversion_utils;
mod gl_buffer;
mod gl_shader;
mod gl_texture;
mod gl_types;
mod gl_utils;
mod gl_vertex;
mod gl_vertex_attribute;
mod renderable;
mod sdl_window;
mod string_utils;

use crate::renderable::first_triangle::FirstTriangle;
use crate::renderable::indexed_quad::IndexedQuad;
use crate::renderable::shader_triangle::ShaderTriangle;
use crate::renderable::texture_triangle::TextureTriangle;
use crate::renderable::Renderable;
use anyhow::Result;
use color::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl_window::SdlWindow;

fn main() -> Result<()> {
    let mut window = SdlWindow::new(800, 600, "RUST SDL 2024", true)?;
    window.clear_color = Color::new(0.10, 0.10, 0.25, 1.0);

    let mut drawables: Vec<Box<dyn Renderable>> = Vec::new();
    add_drawable(&mut drawables, FirstTriangle::new);
    add_drawable(&mut drawables, IndexedQuad::new);
    add_drawable(&mut drawables, || ShaderTriangle::new(false));
    add_drawable(&mut drawables, || ShaderTriangle::new(true));
    add_drawable(&mut drawables, TextureTriangle::new);

    // Set the initial drawable to the last one
    let mut current_index = drawables.len().saturating_sub(1);

    'main_loop: loop {
        for event in window.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main_loop,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => match keycode {
                    Keycode::Escape => break 'main_loop,
                    Keycode::F1 => {
                        // Logic for F1 key
                        if current_index > 0 {
                            current_index -= 1;
                        } else {
                            current_index = 0;
                        }
                    }
                    Keycode::F2 => {
                        // Logic for F2 key
                        if current_index < drawables.len() - 1 {
                            current_index += 1;
                        } else {
                            current_index = drawables.len() - 1;
                        }
                    }
                    Keycode::F3 => {
                        // Logic for F3 key
                        if let Some(drawable) = drawables.get_mut(current_index) {
                            drawable.toggle_mode();
                        }
                    }
                    _ => {}
                },
                _ => {}
            }

            if let Event::Quit { .. } = event {
                break 'main_loop;
            }
        }

        window.clear();

        // Draw the current active drawable
        if let Some(drawable) = drawables.get_mut(current_index) {
            drawable.draw();
        }

        window.swap();
    }

    Ok(())
}

fn add_drawable<F, R, E>(drawables: &mut Vec<Box<dyn Renderable>>, creator: F)
where
    F: FnOnce() -> Result<R, E>,
    R: Renderable + 'static, // Ensure R implements Renderable and has a static lifetime
    E: std::fmt::Debug,      // E can be any type that implements Debug (for error handling)
{
    match creator() {
        Ok(drawable) => drawables.push(Box::new(drawable)),
        Err(e) => eprintln!("Failed to create drawable: {:?}", e),
    }
}
