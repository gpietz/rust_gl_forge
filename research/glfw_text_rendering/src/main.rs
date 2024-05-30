use std::ffi::CString;

use anyhow::Result;
use cgmath::{ortho, Matrix, Matrix4};
use gl::types::{GLfloat, GLsizei, GLsizeiptr, GLuint};
use glfw::{
    Action, Context, GlfwReceiver, Key, OpenGlProfileHint, Window, WindowEvent, WindowHint,
};
use image::Rgba;
use rusttype::{Font, Scale};

use crate::font_atlas::FontAtlas;
use crate::shader_program::{ShaderProgram, ShaderType};

mod font_atlas;
mod shader_program;
mod texture_utils;

const SCREEN_WIDTH: u32 = 1024;
const SCREEN_HEIGHT: u32 = 768;
const TAB_WIDTH_IN_SPACES: usize = 4;

fn main() -> Result<()> {
    // Initialize GLFW
    let mut glfw = glfw::init(glfw::fail_on_errors)?;

    // Set window hints for OpenGL version
    glfw.window_hint(WindowHint::ContextVersionMajor(3));
    glfw.window_hint(WindowHint::ContextVersionMinor(3));
    glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));

    // Create an OpenGL window
    let (mut window, events) = glfw
        .create_window(
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            "OpenGL Text Rendering",
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create OpenGL window");

    // Make the window's context current
    window.make_current();
    window.set_key_polling(true);

    // Load OpenGL function pointers
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    // Print OpenGL version
    let version = unsafe {
        let version_str = gl::GetString(gl::VERSION);
        std::ffi::CStr::from_ptr(version_str as *const i8)
            .to_str()
            .unwrap()
    };
    println!("OpenGL Version: {}", version);

    // Initialize OpenGL settings
    initialize_opengl();

    // Initialize shaders and texture atlas
    let mut binding = ShaderProgram::new();
    let shader_program = binding
        .with_file(
            ShaderType::Vertex,
            "assets/shaders/research/text_rendering.vert",
        )?
        .with_file(
            ShaderType::Fragment,
            "assets/shaders/research/text_rendering.frag",
        )?
        .compile()?;

    let font_data = include_bytes!("../../../assets/fonts/Roboto-Regular.ttf") as &[u8];
    let font = Font::try_from_bytes(font_data).unwrap();
    let scale = Scale::uniform(42.0);
    let color = Rgba([255, 255, 255, 255]);

    let shader_program_id = shader_program.id as GLuint;
    let screen_dimensions = [SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32];

    // Create and bind VAO
    let mut vao: GLuint = 0;
    let mut vbo: GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
    }

    let font_atlas = FontAtlas::new(&font, scale, color);

    font_atlas.save_font_texture("font_atlas_gpu.png")?;
    font_atlas.save_font_mapping("font_atlas.xml")?;

    let text = "Hello World! <-+-> Cool!\r\nTest123\n\tX-Test... 99,88 EUR";
    let text_dimensions = font_atlas.text_dimensions(text);
    let text_xpos = (SCREEN_WIDTH >> 1) as f32 - (text_dimensions.0 / 2.0);
    let text_ypos = (SCREEN_HEIGHT >> 1) as f32 - (text_dimensions.1 / 2.0);
    let text_color = [1.0, 1.0, 1.0];

    // Main-Loop
    while !window.should_close() {
        process_events(&mut window, &events, &font_atlas)?;
        unsafe {
            gl::BindVertexArray(vao);
        }
        render_text(
            shader_program_id,
            vbo,
            &font_atlas,
            text,
            (text_xpos, text_ypos),
            &text_color,
        )?;
        window.swap_buffers();
        glfw.poll_events();
    }

    Ok(())
}

fn initialize_opengl() {
    unsafe {
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }
}

fn process_events(
    window: &mut Window,
    events: &GlfwReceiver<(f64, WindowEvent)>,
    font_atlas: &FontAtlas,
) -> Result<()> {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                window.set_should_close(true);
            }
            glfw::WindowEvent::Key(Key::F1, _, Action::Press, _) => {
                font_atlas.save_font_texture("font_atlas_gpu2.png")?;
                println!("Texture atlas saved as font_atlas_gpu2.png");
            }
            _ => {}
        }
    }

    Ok(())
}

fn create_projection_matrix(width: f32, height: f32) -> Matrix4<f32> {
    ortho(0.0, width, height, 0.0, -1.0, 0.0)
}

fn render_text(
    shader_program: GLuint,
    vbo: GLuint,
    font_atlas: &FontAtlas,
    text: &str,
    text_pos: (f32, f32),
    color: &[f32; 3],
) -> Result<()> {
    unsafe {
        // Clear screen with black background
        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);

        // Use the shader program
        gl::UseProgram(shader_program);

        // Bind the texture
        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_2D, font_atlas.texture_id);

        // Set uniforms
        let color_location =
            gl::GetUniformLocation(shader_program, CString::new("textColor").unwrap().as_ptr());
        gl::Uniform3f(color_location, color[0], color[1], color[2]);

        let projection = create_projection_matrix(SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32);
        let projection_cstr = CString::new("projection").unwrap();
        let projection_location = gl::GetUniformLocation(shader_program, projection_cstr.as_ptr());
        gl::UniformMatrix4fv(projection_location, 1, gl::FALSE, projection.as_ptr());

        // Create vertex buffer for the text
        let vertices = create_vertices_for_text(font_atlas, text, text_pos.0, text_pos.1);

        // Bind VBO + set vertex data
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
            vertices.as_ptr() as *const _,
            gl::DYNAMIC_DRAW,
        );

        gl::VertexAttribPointer(
            0,
            4,
            gl::FLOAT,
            gl::FALSE,
            4 * std::mem::size_of::<GLfloat>() as GLsizei,
            std::ptr::null(),
        );
        gl::EnableVertexAttribArray(0);

        gl::DrawArrays(gl::TRIANGLES, 0, (vertices.len() / 4) as i32);

        //gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        //gl::DeleteBuffers(1, &vbo);
    }

    Ok(())
}

fn create_vertices_for_text(
    font_atlas: &FontAtlas,
    text: &str,
    start_x: f32,
    start_y: f32,
) -> Vec<f32> {
    let line_height = font_atlas.line_height();
    let space_width = font_atlas.space_width();
    let tab_with = space_width * TAB_WIDTH_IN_SPACES as f32;
    let mut vertices = Vec::new();
    let mut x = start_x;
    let mut y = start_y;

    for ch in text.chars() {
        if ch == ' ' {
            x += space_width;
            continue;
        } else if ch == '\r' {
            x = start_x;
            continue;
        } else if ch == '\n' {
            x = start_x;
            y += line_height;
            continue;
        } else if ch == '\t' {
            x += tab_with;
            continue;
        }

        if let Some(glyph) = font_atlas.glyphs.get(&ch) {
            let xpos = x;
            let ypos = y + glyph.bearing_y as f32;

            let w = glyph.width as f32;
            let h = glyph.height as f32;

            let u0 = glyph.x as f32 / font_atlas.width as f32;
            let v0 = (glyph.y as f32 + glyph.height as f32) / font_atlas.height as f32;
            let u1 = (glyph.x as f32 + glyph.width as f32) / font_atlas.width as f32;
            let v1 = glyph.y as f32 / font_atlas.height as f32;

            // First triangle
            vertices.push(xpos);
            vertices.push(ypos + h);
            vertices.push(u0);
            vertices.push(v0);

            vertices.push(xpos);
            vertices.push(ypos);
            vertices.push(u0);
            vertices.push(v1);

            vertices.push(xpos + w);
            vertices.push(ypos);
            vertices.push(u1);
            vertices.push(v1);

            // Second triangle
            vertices.push(xpos);
            vertices.push(ypos + h);
            vertices.push(u0);
            vertices.push(v0);

            vertices.push(xpos + w);
            vertices.push(ypos);
            vertices.push(u1);
            vertices.push(v1);

            vertices.push(xpos + w);
            vertices.push(ypos + h);
            vertices.push(u1);
            vertices.push(v0);

            x += glyph.advance_with;
        }
    }

    vertices
}
