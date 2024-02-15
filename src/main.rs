#![allow(dead_code)]
extern crate gl;
extern crate rusttype;
extern crate sdl2;

mod renderables;
mod texture_utils;
mod vertex_data;

use crate::renderables::first_triangle::FirstTriangle;
use crate::renderables::indexed_quad::IndexedQuad;
use crate::renderables::shader_triangle::ShaderTriangle;
use crate::renderables::texture_triangle::TextureTriangle;
use crate::renderables::transformation::Transformation;
use crate::renderables::Renderable;
use anyhow::Result;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use shared_lib::color::Color;
use shared_lib::sdl_window::SdlWindow;
use std::time::{Duration, Instant};

const WINDOW_TITLE: &str = "RUST SDL 2024";

fn main() -> Result<()> {
    let mut window = SdlWindow::new(800, 600, WINDOW_TITLE, true)?;
    window.clear_color = Color::new(0.10, 0.10, 0.25, 1.0);

    let mut renderables: Vec<Box<dyn Renderable>> = Vec::new();
    add_drawable(&mut renderables, FirstTriangle::new);
    add_drawable(&mut renderables, IndexedQuad::new);
    add_drawable(&mut renderables, || ShaderTriangle::new(false));
    add_drawable(&mut renderables, || ShaderTriangle::new(true));
    add_drawable(&mut renderables, TextureTriangle::new);
    add_drawable(&mut renderables, Transformation::new);

    // Set the initial drawable to the last one
    let mut current_index = renderables.len().saturating_sub(1);

    // Initializes tracking of the update interval;
    // essential for calculating delta time for smooth transformations.
    let mut last_update_time = Instant::now();

    // Required variables for frame rate tracking
    let mut last_fps_time = Instant::now();
    let mut frame_count: u32 = 0;
    let mut last_frame_rate: u32 = 0;
    let mut show_fps = false;

    'main_loop: loop {
        // Calculate the delta time
        let delta_time = get_delta_time(&mut last_update_time);

        // Calculate the frame rate value
        let frame_rate = get_frame_rate(&mut last_fps_time, &mut frame_count, &mut last_frame_rate);

        // Process key events
        let mut window_title_reset_required = false;
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
                        if current_index < renderables.len() - 1 {
                            current_index += 1;
                        } else {
                            current_index = renderables.len() - 1;
                        }
                    }
                    Keycode::F3 => {
                        // Logic for F3 key
                        if let Some(drawable) = renderables.get_mut(current_index) {
                            drawable.toggle_mode();
                        }
                    }
                    Keycode::F4 => {
                        // Logic for F3 key
                        if let Some(drawable) = renderables.get_mut(current_index) {
                            drawable.toggle_shape();
                        }
                    }
                    Keycode::F12 => {
                        show_fps = !show_fps;
                        println!(
                            "FPS tracking {}",
                            if show_fps { "activated" } else { "deactivated" }
                        );
                        if !show_fps {
                            window_title_reset_required = true;
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

        // Rest the window title, if required
        if window_title_reset_required {
            window.set_window_title(WINDOW_TITLE)?;
        }

        window.clear();

        // Draw the current active drawable
        if let Some(drawable) = renderables.get_mut(current_index) {
            drawable.draw(delta_time)?;
        }

        window.swap();

        if frame_rate > 0 && show_fps {
            let window_title = format!("{} (FPS: {})", WINDOW_TITLE, frame_rate);
            window.set_window_title(&window_title)?;
        }
    }

    Ok(())
}

fn add_drawable<F, R, E>(renderables: &mut Vec<Box<dyn Renderable>>, creator: F)
where
    F: FnOnce() -> Result<R, E>,
    R: Renderable + 'static, // Ensure R implements Renderable and has a static lifetime
    E: std::fmt::Debug,      // E can be any type that implements Debug (for error handling)
{
    match creator() {
        Ok(drawable) => renderables.push(Box::new(drawable)),
        Err(e) => eprintln!("Failed to create drawable: {:?}", e),
    }
}

/// Calculates and returns the delta time in seconds since the last update,
/// and updates the last update time.
fn get_delta_time(last_update_time: &mut Instant) -> f32 {
    let now = Instant::now();
    let delta = now.duration_since(*last_update_time);
    *last_update_time = Instant::now();
    delta.as_secs_f32()
}

/// Calculates and updates the frame rate every second.
fn get_frame_rate(
    last_fps_time: &mut Instant,
    frame_count: &mut u32,
    last_frame_rate: &mut u32,
) -> u32 {
    *frame_count += 1;

    let now = Instant::now();
    if now.duration_since(*last_fps_time) > Duration::from_secs(1) {
        *last_frame_rate = *frame_count;
        *frame_count = 0;
        *last_fps_time = Instant::now();
    }

    *last_frame_rate
}
