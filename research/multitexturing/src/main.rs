extern crate gl;

use std::{ffi::c_void, mem, ptr};

use anyhow::Result;
use gl::types::{GLfloat, GLsizei, GLsizeiptr};
use sdl2::event::Event;
use shared_lib::{gl_shader::ShaderFactory, gl_texture::{Texture, TextureBuilder}, gl_utils, prelude::*};

fn main() -> Result<()> {
    let mut window = SdlWindow::new(800, 600, "RESEARCH: MULTITEXTURING", true)?;
    window.clear_color = Color::BLACK;

    // load shader
    let mut shader = ShaderFactory::from_files(
        "resources/shaders/texture.vs", 
        "resources/shaders/texture.fs"
    )?;

    // *** setup vertex data ***
    let vertices: [f32; 32] = [
        // positions       // colors        // texture coords
         0.5,  0.5, 0.0,   1.0, 0.0, 0.0,   1.0, 1.0, // top right
         0.5, -0.5, 0.0,   0.0, 1.0, 0.0,   1.0, 0.0, // bottom right
        -0.5, -0.5, 0.0,   0.0, 0.0, 1.0,   0.0, 0.0, // bottom left
        -0.5,  0.5, 0.0,   1.0, 1.0, 0.0,   0.0, 1.0  // top left
    ];

    let indices = [
        0, 1, 3,  // first Triangle
        1, 2, 3   // second Triangle
    ];

    // *** create buffer objects ***
    let (mut VBO, mut VAO, mut EBO) = (0, 0, 0);
    unsafe {
        gl::GenVertexArrays(1, &mut VAO);
        gl::GenBuffers(1, &mut VBO);
        gl::GenBuffers(1, &mut EBO);

        gl::BindVertexArray(VAO);

        gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
        gl::BufferData(gl::ARRAY_BUFFER,
                       (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       &vertices[0] as *const f32 as *const c_void,
                       gl::STATIC_DRAW);

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, EBO);
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                       (indices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       &indices[0] as *const i32 as *const c_void,
                       gl::STATIC_DRAW);

        // position attribute
        let stride = 8 * mem::size_of::<GLfloat>() as GLsizei;
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
        gl::EnableVertexAttribArray(0);
        // color attribute
        gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, stride, (3 * mem::size_of::<GLfloat>()) as *const c_void);
        gl::EnableVertexAttribArray(1);
        // texture coord attribute
        gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, stride, (6 * mem::size_of::<GLfloat>()) as *const c_void);
        gl::EnableVertexAttribArray(2);
    }

    // *** load textures ***
    let texture1 = TextureBuilder::default().path("resources/textures/container.jpg").build()?;
    let texture2 = TextureBuilder::default().path("resources/textures/awesomeface2.png").flip_vertical(true).has_alpha(true).build()?;

    // *** assign textures in shader ***
    shader.bind();
    let texture1_location = shader.get_uniform_location("texture1")?;
    let texture2_location = shader.get_uniform_location("texture2")?;
    shader.set_uniform_value(texture1_location, texture1.get_texture_id() as i32)?;
    shader.set_uniform_value(texture2_location, texture2.get_texture_id() as i32)?;
    println!("Texture #1 uniform location: {}", texture1_location);
    println!("Texture #2 uniform location: {}", texture2_location);

    // main loop
    'main_loop: loop {
        for event in window.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main_loop,
                _ => {}
            }
        }
        //window.clear();

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            // gl::ActiveTexture(texture1.get_texture_id());
            // gl::BindTexture(gl::TEXTURE_2D, texture1.get_texture_id());
            // texture2.bind();
            // gl::BindTexture(gl::TEXTURE_2D, texture2.get_texture_id());            

            shader.bind();
            gl::BindVertexArray(VAO);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
        }

        window.swap();
    }

    Ok(())
}
