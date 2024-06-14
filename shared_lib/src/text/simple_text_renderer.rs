use crate::color::Color;
use crate::gl_prelude::{check_gl_error2, BufferType, BufferUsage, PrimitiveType, ShaderType};
use crate::gl_types::ProjectionMatrix;
use crate::gl_utils::check_gl_error;
use crate::opengl::buffer_object::BufferObject;
use crate::opengl::shader_program::ShaderProgram;
use crate::opengl::vertex_array_object::VertexArrayObject;
use crate::projection::{HasOptionalProjection, Projection};
use crate::text::font_atlas::FontAtlas;
use crate::{gl_draw, Position2D, Size2D};
use anyhow::Result;
use cgmath::{Matrix, Matrix4, SquareMatrix, Vector2};
use gl::types::{GLfloat, GLint, GLsizei, GLuint};
use image::imageops::unsharpen;
use rusttype::Scale;
use std::borrow::Cow;
use std::{ptr, vec};
use crate::opengl::font::Font;

const TAB_WIDTH_IN_SPACES: usize = 4;

//////////////////////////////////////////////////////////////////////////////
// - SimpleTextRenderer -
//////////////////////////////////////////////////////////////////////////////

pub struct SimpleTextRenderer {
    font_atlas: FontAtlas,
    shader_program: ShaderProgram,
    vao: VertexArrayObject,
    vbo: BufferObject<f32>,
}

impl SimpleTextRenderer {
    pub fn new(font: &Font, scale: f32) -> Result<Self> {
        let uniform_scale = Scale::uniform(scale);
        let rt_font = &*font.font;
        let font_atlas = FontAtlas::new(rt_font, uniform_scale, Color::WHITE.into());
        Self::from_font_atlas(font_atlas)
    }

    pub fn from_font_atlas(font_atlas: FontAtlas) -> Result<Self> {
        println!("--> Create VAO");
        let vao = VertexArrayObject::new()?;
        check_gl_error2();

        println!("--> Create VBO");
        let vbo = BufferObject::empty(BufferType::ArrayBuffer, BufferUsage::StaticDraw);
        check_gl_error2();

        println!("--> Create vertex layout");
        setup_vertex_layout()?;

        //println!("--> Create shader");
        let shader_program = create_shader_program()?;

        let text_renderer = Self {
            font_atlas,
            shader_program,
            vao,
            vbo,
        };

        Ok(text_renderer)
    }

    pub fn scale_width(&self) -> f32 {
        self.font_atlas.scale.x
    }

    pub fn scale_height(&self) -> f32 {
        self.font_atlas.scale.y
    }

    pub fn texture_id(self) -> u32 {
        self.font_atlas.texture_id
    }

    pub fn render_text(
        &mut self,
        text: &str,
        position: Position2D,
        options: Option<&TextRenderOptions>,
    ) -> Result<()> {
        let mut text_color = Color::WHITE;
        let mut opacity = 1.0;
        let mut projection = Matrix4::<f32>::identity();

        println!("Render Text****");

        if let Some(opt) = options {
            text_color = opt.color;
            opacity = opt.opacity.clamp(0.0, 1.0);
        }

        // Use the shader program
        self.shader_program.activate();

        println!("Shader activated****");

        unsafe {
            // Bind the texture
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.font_atlas.texture_id);
        }

        // Set uniforms (color + projection)
        let rgb = text_color.to_rgb();
        self.shader_program.set_uniform_3f(
            "textColor",
            rgb[0] as f32,
            rgb[1] as f32,
            rgb[2] as f32,
        )?;
        self.shader_program.set_uniform_matrix("projection", false, &projection);

        println!("Create vertx stuff****");

        // Create vertex data for text
        let vertices = create_vertices_for_text(&self.font_atlas, text, position.x, position.y);
        let triangle_count = (vertices.len() / 4) as u32;

        println!("Draw call****");

        // Update vertex data
        self.vbo.update_data(vertices, None);

        gl_draw::draw_arrays(PrimitiveType::Triangles, 0, triangle_count as usize);

        Ok(())
    }
}

fn create_shader_program() -> Result<ShaderProgram> {
    let mut shader_program = ShaderProgram::new();
    shader_program.add_source(
        ShaderType::Vertex,
        include_bytes!("../../resources/shaders/text_rendering.vert"),
    )?;
    shader_program.add_source(
        ShaderType::Fragment,
        include_bytes!("../../resources/shaders/text_rendering.frag"),
    )?;
    shader_program.compile();

    Ok(shader_program)
}

fn setup_vertex_layout() -> Result<()> {
    //VertexLayoutManager::new().add_attribute()
    unsafe {
        check_gl_error2()?;
        println!("**** Vertex layout ****");

        gl::VertexAttribPointer(
            0,
            4,
            gl::FLOAT,
            gl::FALSE,
            4 * std::mem::size_of::<GLfloat>() as GLsizei,
            ptr::null(),
        );
        check_gl_error2()?;
        gl::EnableVertexAttribArray(0);
        check_gl_error()?;
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
            let x_pos = x;
            let y_pos = y + glyph.bearing_y as f32;

            let w = glyph.width as f32;
            let h = glyph.height as f32;

            let u0 = glyph.x as f32 / font_atlas.width as f32;
            let v0 = (glyph.y as f32 + glyph.height as f32) / font_atlas.height as f32;
            let u1 = (glyph.x as f32 + glyph.width as f32) / font_atlas.width as f32;
            let v1 = glyph.y as f32 / font_atlas.height as f32;

            // First triangle
            vertices.push(x_pos);
            vertices.push(y_pos + h);
            vertices.push(u0);
            vertices.push(v0);

            vertices.push(x_pos);
            vertices.push(y_pos);
            vertices.push(u0);
            vertices.push(v1);

            vertices.push(x_pos + w);
            vertices.push(y_pos);
            vertices.push(u1);
            vertices.push(v1);

            // Second triangle
            vertices.push(x_pos);
            vertices.push(y_pos + h);
            vertices.push(u0);
            vertices.push(v0);

            vertices.push(x_pos + w);
            vertices.push(y_pos);
            vertices.push(u1);
            vertices.push(v1);

            vertices.push(x_pos + w);
            vertices.push(y_pos + h);
            vertices.push(u1);
            vertices.push(v0);

            x += glyph.advance_with;
        }
    }

    vertices
}

//////////////////////////////////////////////////////////////////////////////
// - TextRenderOptions -
//////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Copy, Clone)]
pub struct TextRenderOptions {
    pub color: Color,
    pub opacity: f32,
    pub projection: Option<ProjectionMatrix>,
}

impl Default for TextRenderOptions {
    fn default() -> Self {
        Self {
            color: Color::WHITE,
            projection: None,
            opacity: 1.0,
        }
    }
}
