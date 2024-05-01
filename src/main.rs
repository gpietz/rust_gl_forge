#![allow(dead_code)]
extern crate gl;

use anyhow::Result;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use scenes::first_triangle::FirstTriangle;
use scenes::indexed_quad::IndexedQuad;
use scenes::shader_triangle::ShaderTriangle;
use shared_lib::color::Color;
use shared_lib::sdl_window::SdlWindow;

use crate::render_context::RenderContext;
use crate::resources::{shaders, textures};
use crate::scene::{RenderScene};
use crate::scenes::projection::Projection;
use crate::scenes::texture_triangle::TextureTriangle;
use crate::scenes::transformation::Transformation;

mod render_context;
mod resources;
mod scene;
mod scene_utils;
mod scenes;
mod texture_utils;
mod traits;
mod vertex_data;
mod vertex_data_2d;
mod vertex_data_3d;

const WINDOW_TITLE: &str = "RUST OPENGL 2024";
pub(crate) const SCREEN_WIDTH: usize = 1024;
pub(crate) const SCREEN_HEIGHT: usize = 768;

pub(crate) const SHADER_SIMPLE_RED: &str = "shader_simple_red";

fn main() -> Result<()> {
    let mut window = SdlWindow::new(SCREEN_WIDTH, SCREEN_HEIGHT, WINDOW_TITLE, true)?;
    window.clear_color = Color::new(0.10, 0.10, 0.25, 1.0);

    //// add_drawable(&mut renderables, FirstText::new);

    // Create scenes
    let mut scenes: Vec<Box<RenderScene>> = vec![
        Box::<FirstTriangle>::default(),
        Box::<IndexedQuad>::default(),
        Box::<ShaderTriangle>::new(ShaderTriangle::new(false)),
        Box::<ShaderTriangle>::new(ShaderTriangle::new(true)),
        Box::<TextureTriangle>::default(),
        Box::<Transformation>::default(),
        Box::<Projection>::default(),
    ];

    // Set the initial drawable to the last one
    let mut current_index = scenes.len().saturating_sub(1);

    // Update window title with the scene index
    let window_title = format!("{} [{}/{}]", WINDOW_TITLE, current_index + 1, scenes.len());
    window.set_window_title(&window_title)?;

    // Create the render context object
    let mut render_context = RenderContext::new();
    textures::add_textures(render_context.texture_manager());
    shaders::add_shaders(render_context.shader_manager());

    // Required variables for frame rate tracking
    let mut show_fps = false;
    let mut last_active_scene = usize::MAX;
    'main_loop: loop {
        for event in window.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main_loop,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => match keycode {
                    Keycode::Escape => break 'main_loop,
                    Keycode::F1 => current_index = current_index.saturating_sub(1),
                    Keycode::F2 => {
                        current_index = current_index.saturating_add(1).min(scenes.len() - 1)
                    }
                    Keycode::F12 => {
                        show_fps = !show_fps;
                        println!(
                            "FPS tracking {}",
                            if show_fps { "activated" } else { "disabled" }
                        );
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        // Update render context
        render_context.update(&window);

        window.clear();

        // Activating or deactivating scenes
        if last_active_scene != current_index {
            // Deactivate last scene
            if last_active_scene != usize::MAX {
                if let Some(scene) = scenes.get_mut(last_active_scene) {
                    scene.deactivate(&mut render_context, false)?;
                }
            }
            // Activate new scene
            if let Some(scene) = scenes.get_mut(current_index) {
                scene.activate(&mut render_context)?;
            }
            last_active_scene = current_index;
        }

        // Iterates over all scenes to update each one with the current render context
        // and delta time. `update_tick` is called for all scenes, and `update` is
        // called only for the active scene identified by `current_index`.
        let delta_time = render_context.delta_time();
        for scene_index in 0..scenes.len() {
            if let Some(scene) = scenes.get_mut(scene_index) {
                // Calls update_tick on each scene, passing the context, time since last
                // frame, and whether this scene is currently active.
                scene.update_tick(
                    &mut render_context,
                    delta_time,
                    scene_index == current_index,
                )?;

                // Calls the main update method only on the active scene.
                if scene_index == current_index {
                    scene.update(&mut render_context)?;
                }
            }
        }

        // Render active scene
        if let Some(scene) = scenes.get_mut(current_index) {
            // Render scene
            scene.draw(&mut render_context)?;
        }

        // Swap display buffers
        window.swap();

        // Update window title with scene number and fps tracking
        let window_title = if show_fps {
            format!(
                "{} [{}/{}] (FPS: {})",
                WINDOW_TITLE,
                current_index + 1,
                scenes.len(),
                render_context.frame_rate()
            )
        } else {
            format!("{} [{}/{}]", WINDOW_TITLE, current_index + 1, scenes.len())
        };
        window.set_window_title(&window_title)?;
    } // loop end
    Ok(())
}
