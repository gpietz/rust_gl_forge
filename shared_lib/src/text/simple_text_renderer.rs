use std::ptr;

use anyhow::{Context, Result};
use cgmath::ortho;
use gl::types::{GLfloat, GLsizei};
use rusttype::Scale;
use sha2::digest::typenum::op;

use crate::color::Color;
use crate::gl_prelude::{check_gl_error2, BufferType, BufferUsage, PrimitiveType, ShaderType};
use crate::gl_types::ProjectionMatrix;
use crate::gl_utils::check_gl_error;
use crate::opengl::blend_guard::BlendGuard;
use crate::opengl::buffer_object::BufferObject;
use crate::opengl::font::Font;
use crate::opengl::shader_program::ShaderProgram;
use crate::opengl::vertex_array_object::VertexArrayObject;
use crate::text::font_atlas::FontAtlas;
use crate::{check_gl_panic, gl_draw, Position2D};

const TAB_WIDTH_IN_SPACES: usize = 4;

//////////////////////////////////////////////////////////////////////////////
// - SimpleTextRenderer -
//////////////////////////////////////////////////////////////////////////////

pub struct SimpleTextRenderer<'a> {
    font_atlas: FontAtlas,
    shader_program: ShaderProgram,
    vao: VertexArrayObject,
    vbo: BufferObject<f32>,
    options: Option<&'a TextRenderOptions>,
}

impl<'a> SimpleTextRenderer<'a> {
    pub fn new(font: &Font, scale: f32) -> Result<Self> {
        let uniform_scale = Scale::uniform(scale);
        let rt_font = &*font.font;
        let font_atlas = FontAtlas::new(rt_font, uniform_scale, Color::WHITE.into());
        Self::from_font_atlas(font_atlas)
    }

    pub fn from_font_atlas(font_atlas: FontAtlas) -> Result<Self> {
        let vao = VertexArrayObject::new();
        check_gl_error2();

        let vbo = BufferObject::empty(BufferType::ArrayBuffer, BufferUsage::StaticDraw);
        check_gl_error2();

        setup_vertex_layout()?;
        check_gl_panic!("Failed to setup vertex layout");

        let shader_program = create_shader_program()?;
        check_gl_panic!("Failed to create shader program");

        let text_renderer = Self {
            font_atlas,
            shader_program,
            vao,
            vbo,
            options: None,
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

    pub fn render_text<S: AsRef<str>>(
        &mut self,
        text: S,
        position: Position2D,
        options: Option<&TextRenderOptions>,
    ) -> Result<()> {
        let mut text_color = Color::WHITE;
        let mut opacity = 1.0;

        if let Some(opt) = options {
            text_color = opt.color;
            opacity = opt.opacity.clamp(0.0, 1.0);
        }

        // activate vao
        self.vao.bind();

        unsafe {
            // Bind and activate the texture
            gl::BindTexture(gl::TEXTURE_2D, self.font_atlas.texture_id);
            check_gl_error().with_context(|| "Failed to bind texture");
            gl::ActiveTexture(gl::TEXTURE0);
            check_gl_error().with_context(|| "Failed to activate texture");
        }

        self.shader_program.activate();

        // Calculate projection matrix
        let screen_dimensions = (1440.0f32, 1080.0f32);
        let projection = ortho(0.0, screen_dimensions.0, screen_dimensions.1, 0.0, -1.0, 0.0);
        // TODO ^^ Add screen dimensions for projection calculations ^^

        // Set uniforms (color + projection)
        let rgb = text_color.to_rgb();
        self.shader_program.set_uniform_3f(
            "textColor",
            rgb[0] as f32,
            rgb[1] as f32,
            rgb[2] as f32,
        )?;
        self.shader_program.set_uniform_matrix("projection", false, &projection);

        // Create vertex data for text
        let text = text.as_ref();
        let vertices = create_vertices_for_text(&self.font_atlas, text, position.x, position.y);
        let triangle_count = (vertices.len() / 4) as u32;

        // Update vertex data
        self.vbo.update_data(vertices, None);

        // Enable blend mode
        let mut blend_guard = BlendGuard::default();
        blend_guard.enable();

        gl_draw::draw_arrays(PrimitiveType::Triangles, 0, triangle_count as usize);
        Ok(())
    }

    pub fn set_options(&mut self, options: Option<&'a TextRenderOptions>) -> Result<()> {
        self.options = options;
        Ok(())
    }

    pub fn get_options(&self) -> Option<&'a TextRenderOptions> {
        self.options
    }

    pub fn get_options_mut(&mut self) -> Option<&mut &'a TextRenderOptions> {
        self.options.as_mut()
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
    check_gl_panic!("Loading shaders failed?");

    shader_program.compile()?;
    check_gl_panic!("Shader compile failed");

    Ok(shader_program)
}

fn setup_vertex_layout() -> Result<()> {
    unsafe {
        gl::VertexAttribPointer(
            0,
            4,
            gl::FLOAT,
            gl::FALSE,
            4 * std::mem::size_of::<GLfloat>() as GLsizei,
            ptr::null(),
        );
        check_gl_error()?;

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
