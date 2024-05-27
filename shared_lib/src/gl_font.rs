#![allow(dead_code)]

use std::collections::HashMap;
use std::ffi::{c_void, CString};
use std::fs::{read, File};
use std::io::Write;
use std::path::Path;
use std::{mem, ptr, str};

use anyhow::{format_err, Context, Result};
use cgmath::{Matrix, Vector2};
use gl::types::{GLfloat, GLint, GLsizei, GLsizeiptr, GLuint};
use image::codecs::png::CompressionType::Fast;
use image::{DynamicImage, ImageBuffer, Rgba, RgbaImage};
use rusttype::{point, Scale};
use sdl2::libc::printf;

use crate::gl_buffer::BufferObject;
use crate::gl_draw::draw_primitive;
use crate::gl_prelude::{check_gl_error, VertexLayoutManager};
use crate::gl_shader::{Shader, ShaderFactory, ShaderProgram};
use crate::gl_traits::{Bindable, Deletable};
use crate::gl_types::{BufferType, BufferUsage, PrimitiveType, TextureTarget};
use crate::gl_vertex::Vertex;
use crate::gl_vertex_array::VertexArrayObject;
use crate::prelude::Color;
use crate::vertices::textured_vertex::TexturedVertex;

//////////////////////////////////////////////////////////////////////////////
// - FontScale -
//////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Copy, Clone)]
pub struct FontSize {
    pub x: f32,
    pub y: f32,
}

impl FontSize {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn uniform(size: f32) -> Self {
        Self { x: size, y: size }
    }

    fn to_rusttype_scale(&self) -> Scale {
        Scale {
            x: self.x,
            y: self.y,
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
// - Font -
//////////////////////////////////////////////////////////////////////////////

pub struct Font {
    font_path: Option<String>,
    font: Box<rusttype::Font<'static>>,
}

impl Font {
    pub fn from_file<P: AsRef<Path>>(font_path: P) -> Result<Font> {
        let path = font_path.as_ref();
        let path_str = path.to_string_lossy().into_owned();

        // load the font
        let font_data =
            read(path).with_context(|| format!("Error reading font file: {}", path_str))?;
        let font = rusttype::Font::try_from_vec(font_data)
            .with_context(|| format!("Error constructing a font from data {}", path_str))?;

        println!("Loaded font {}", path_str);

        Ok(Self {
            font_path: Some(path_str),
            font: Box::new(font),
        })
    }

    /// Returns a read-only reference to the font's file path, if available.
    ///
    /// This method provides access to the path of the font file used to create
    /// the font instance. It's useful for retrieving the original path of the font
    /// for informational purposes, logging, or debugging. The method returns `None`
    /// if the path was not set during the creation of the `Font` instance.
    pub fn font_path(&self) -> Option<&str> {
        self.font_path.as_deref()
    }

    pub fn create_texture_atlas(&self, font_size: f32, color: &Color) -> Result<FontTextureAtlas> {
        self.create_texture_atlas_with_size(FontSize::uniform(font_size), color)
    }

    pub fn create_texture_atlas_with_size(
        &self,
        font_size: FontSize,
        color: &Color,
    ) -> Result<FontTextureAtlas> {
        let font: &rusttype::Font = self.font.as_ref();
        FontTextureAtlas::new(font, font_size, color)
    }
}

//////////////////////////////////////////////////////////////////////////////
// - FontTextureAtlas -
//////////////////////////////////////////////////////////////////////////////

pub struct FontTextureAtlas {
    font_size: FontSize,
    dimension: Vector2<u32>,
    image: Box<RgbaImage>,
    glyphs: HashMap<char, GlyphData>,
    color: Color,
}

impl FontTextureAtlas {
    pub fn new(font: &rusttype::Font<'static>, font_size: FontSize, color: &Color) -> Result<Self> {
        #[rustfmt::skip]
        let characters = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+-.,;:_#*@?!=()[]";
        let scale = font_size.to_rusttype_scale();
        let metrics = font.v_metrics(scale);
        let offset = point(0.0, metrics.ascent);
        let glyphs: Vec<_> = font.layout(characters, scale, offset).collect();

        // Calculate atlas dimensions
        let mut atlas_height = (metrics.ascent - metrics.descent).ceil() as u32;
        let mut atlas_width = {
            let min_x = glyphs.first().map(|g| g.pixel_bounding_box().unwrap().min.x).unwrap();
            let max_x = glyphs.last().map(|g| g.pixel_bounding_box().unwrap().max.x).unwrap();
            (max_x - min_x) as u32
        };

        println!("Calculated atlas size: {}x{}", atlas_width, atlas_height);

        let mut atlas_width2: u32 = 0;
        let mut atlas_height2: u32 = 0;
        for glyph in &glyphs {
            if let Some(bb) = glyph.pixel_bounding_box() {
                atlas_width2 += bb.width() as u32;
                atlas_height2 = atlas_height2.max(bb.height() as u32);
            }
        }

        atlas_width2 += glyphs.len() as u32;
        atlas_height2 += 10;

        atlas_width = atlas_width2;
        atlas_height = atlas_height2;

        println!("Calculated atlas size: {}x{}", atlas_width2, atlas_height2);

        let mut texture_image = DynamicImage::new_rgba8(atlas_width, atlas_height).to_rgba8();

        let color_rgba = color.to_rgba();
        let mut glyph_data_map: HashMap<char, GlyphData> = HashMap::new();
        let mut x_offset = 1; // Padding berücksichtigen
        let mut y_offset = 1; // Padding berücksichtigen

        for glyph in glyphs {
            if let Some(bounding_box) = glyph.pixel_bounding_box() {
                if x_offset + bounding_box.width() as u32 > atlas_width as u32 {
                    x_offset = 1; // Neue Zeile beginnen
                    y_offset += atlas_height; // Höhe der aktuellen Zeile hinzufügen und mit Padding
                }

                if y_offset + bounding_box.height() as u32 > atlas_height as u32 {
                    return Err(anyhow::anyhow!("Glyph position out of bounds: x_offset={}, y_offset={}, glyph width={}, glyph height={}, atlas width={}, atlas height={}",
                        x_offset, y_offset, bounding_box.width(), bounding_box.height(), atlas_width, atlas_height));
                }

                // Draw the glyph into the image per-pixel by using the draw closure
                glyph.draw(|x, y, v| {
                    texture_image.put_pixel(
                        // Offset the position by the glyph bounding box
                        (x_offset + x),
                        (y_offset + y),
                        // Turn the coverage into an alpha value
                        Rgba([
                            color_rgba[0],
                            color_rgba[1],
                            color_rgba[2],
                            (v * 255.0) as u8,
                        ]),
                    )
                });

                // create texture mapping
                let glyph_data = GlyphData {
                    character: characters
                        .chars()
                        .nth(glyph_data_map.len())
                        .with_context(|| "Failed to get character by index")?,
                    x: bounding_box.min.x as u32,
                    y: bounding_box.min.y as u32,
                    width: bounding_box.width() as u32,
                    height: bounding_box.height() as u32,
                };
                glyph_data_map.insert(glyph_data.character, glyph_data);

                x_offset += bounding_box.width() as u32 + 1;
            }
        }

        Ok(Self {
            font_size,
            dimension: Vector2::new(atlas_width, atlas_height),
            image: Box::new(texture_image),
            glyphs: glyph_data_map,
            color: *color,
        })
    }

    pub fn font_size(&self) -> &FontSize {
        &self.font_size
    }

    pub fn texture_dimension(&self) -> &Vector2<u32> {
        &self.dimension
    }

    pub fn image(&self) -> &RgbaImage {
        &self.image
    }

    fn glyphs(&self) -> &HashMap<char, GlyphData> {
        &self.glyphs
    }

    pub fn color(&self) -> &Color {
        &self.color
    }

    pub fn save_texture(&self, file_path: &str) -> Result<()> {
        self.image
            .save(file_path)
            .with_context(|| "Error in saving texture atlas image")
    }

    pub fn save_font_mapping(&self, file_path: &str) -> Result<()> {
        save_mapping_to_xml(&self.glyphs, file_path).with_context(|| "Error in saving font mapping")
    }

    pub fn get_raw_image(&self) -> Option<Vec<u8>> {
        Some(self.image.as_ref().clone().into_raw())
    }
}

/// Represents the data for a single glyph, including its associated character and texture coordinates.
///
/// The `character` field holds the Unicode character that this glyph represents.
/// The `texture_coords` field contains the texture coordinates in the format [x, y, width, height],
/// which specify the glyph's position and size within a texture atlas.
#[derive(Debug, Clone, Copy)]
struct GlyphData {
    character: char,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

/// A collection of `GlyphData`, intended for serialization/deserialization to/from XML.
///
/// This struct acts as a container for multiple `GlyphData` instances,
/// allowing a collection of glyphs to be easily serialized into XML format
/// or deserialized from XML format.
#[derive(Debug)]
struct GlyphMapping {
    glyphs: Vec<GlyphData>,
}

/// Saves a mapping of character glyphs to an XML file.
///
/// This function takes a reference to a `HashMap` of character-to-`GlyphData` mappings
/// and the path to an output XML file. It converts the `HashMap` into a `GlyphMapping` object,
/// serializes this object to XML, and writes the XML data to the specified file.
///
/// # Arguments
///
/// * `glyph_data_map` - A reference to a `HashMap` mapping `char` to `GlyphData`,
/// representing the glyph data for each character.
/// * `file_path` - A string slice that holds the path to the output XML file.
///
/// # Errors
///
/// This function returns an `Err` if there is an error during serialization,
/// file creation, or writing to the file.
fn save_mapping_to_xml(glyph_data_map: &HashMap<char, GlyphData>, file_path: &str) -> Result<()> {
    let mut glyph_mapping = GlyphMapping {
        glyphs: glyph_data_map.values().cloned().collect(),
    };

    // First, sort by the `x` value and if `x` values are equal, sort by the `y` value
    glyph_mapping.glyphs.sort_by(|a, b| a.x.cmp(&b.x).then(a.y.cmp(&b.y)));

    // Create xml data from the glyph mapping
    let mut xml = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<GlyphMapping>\n");
    for glyph in &glyph_mapping.glyphs {
        let glyph_xml = format!(
            "\t<GlyphData character=\"{}\" x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" />\n",
            glyph.character, glyph.x, glyph.y, glyph.width, glyph.height
        );
        xml.push_str(&glyph_xml);
    }
    xml.push_str("</GlyphMapping>\n");

    let mut xml_file =
        File::create(file_path).with_context(|| "Failed opening file for writing GlyphMapping")?;
    xml_file
        .write_all(xml.as_bytes())
        .with_context(|| "Failed writing GlyphMapping to file")?;

    println!("Saved GlyphMapping to XML file: {}", file_path);
    Ok(())
}

//////////////////////////////////////////////////////////////////////////////
// - FastFontRenderer -
//////////////////////////////////////////////////////////////////////////////

/// Macro to create a TexturedVertex for 2D rendering.
/// This macro simplifies the creation of TexturedVertex instances by allowing
/// you to specify only the necessary 2D coordinates and texture coordinates,
/// automatically filling in a default Z-value.
///
/// # Usage
/// `vertex!(x, y, u, v)` where `x` and `y` are the 2D position coordinates,
/// and `u` and `v` are the texture coordinates.
///
/// # Example
/// ```no-run
/// let vertex = vertex!(1.0, 2.0, 0.5, 0.5);
/// ```
#[macro_export]
macro_rules! vertex {
    ($x:expr, $y:expr, $u:expr, $v:expr) => {
        TexturedVertex::new_xyz_uv($x, $y, 0.0, $u, $v)
    };
}

pub struct FastFontRenderer {
    vao: VertexArrayObject,
    vbo: BufferObject<TexturedVertex>,
    texture_atlas: FontTextureAtlas,
    texture_id: u32,
    shader: ShaderProgram,
}

impl FastFontRenderer {
    pub fn new(texture_atlas: FontTextureAtlas) -> Result<Self> {
        const VERT_SHADER: &[u8] = include_bytes!("../resources/shaders/font_renderer.vert");
        const FRAG_SHADER: &[u8] = include_bytes!("../resources/shaders/font_renderer.frag");

        // Create shader program
        let vert_source =
            str::from_utf8(VERT_SHADER).with_context(|| "Error loading vertex shader source")?;
        let frag_source =
            str::from_utf8(FRAG_SHADER).with_context(|| "Error loading fragment shader source")?;
        let shader_prog = ShaderFactory::from_source(vert_source, frag_source)?;

        // Create vertex array object
        let vao = VertexArrayObject::new()?;

        // Create vertex buffer object
        let vbo = BufferObject::<TexturedVertex>::empty(
            BufferType::ArrayBuffer,
            BufferUsage::DynamicDraw,
        );

        // Setup the vertex layout
        VertexLayoutManager::from_attributes(TexturedVertex::attributes()).setup_attributes()?;

        // Create opengl texture
        let texture_dimension = texture_atlas.dimension;
        println!(
            "texture-dimension: {}x{}",
            texture_dimension.x, texture_dimension.y
        );
        let mut texture_id: GLuint = 0;
        unsafe {
            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);

            println!("texture-id: {}", texture_id);
            println!(
                "texture-dimension: {}x{}",
                texture_dimension.x, texture_dimension.y
            );

            texture_atlas.save_texture("original_texture_image.png")?;

            // Set texture parameters
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            // Create texture image
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as GLint,
                texture_dimension.x as GLint,
                texture_dimension.y as GLint,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                texture_atlas.image.as_ptr() as *const c_void,
            );

            check_gl_error()?;

            gl::GenerateMipmap(gl::TEXTURE_2D);

            // Debugging: Überprüfen Sie die Texturparameter
            let mut width: i32 = 0;
            let mut height: i32 = 0;
            unsafe {
                gl::BindTexture(gl::TEXTURE_2D, texture_id);
                gl::GetTexLevelParameteriv(gl::TEXTURE_2D, 0, gl::TEXTURE_WIDTH, &mut width);
                gl::GetTexLevelParameteriv(gl::TEXTURE_2D, 0, gl::TEXTURE_HEIGHT, &mut height);
            }
            println!("Texture width: {}, height: {}", width, height);
            // Optional: Überprüfen der hochgeladenen Textur
            let uploaded_texture = FastFontRenderer::check_texture(
                texture_id,
                texture_dimension.x as i32,
                texture_dimension.y as i32,
            );
            uploaded_texture.save("uploaded_texture_check.png")?;

            // Release the texture
            //gl::BindTexture(gl::TEXTURE_2D, 0);
        }

        Ok(Self {
            vao,
            vbo,
            texture_atlas,
            texture_id,
            shader: shader_prog,
        })
    }

    fn check_texture(texture_id: GLuint, width: i32, height: i32) -> DynamicImage {
        let mut data = vec![0u8; (width * height * 4) as usize];
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, texture_id);
            gl::GetTexImage(
                gl::TEXTURE_2D,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                data.as_mut_ptr() as *mut _,
            );
        }
        DynamicImage::ImageRgba8(ImageBuffer::from_raw(width as u32, height as u32, data).unwrap())
    }

    pub fn texture_id(&self) -> u32 {
        self.texture_id
    }

    pub fn draw_text(&mut self, text: &str, position: &Vector2<f32>, scale: f32) -> Result<()> {
        let (mut x, mut y) = (position.x, position.y);

        let screen_width = 1024.0;
        let screen_height = 768.0;
        let projection = cgmath::ortho(0.0, screen_width, 0.0, screen_height, -1.0, 1.0);
        self.shader.set_uniform_matrix("projection", false, &projection);
        self.shader.set_uniform_3f("textColor", 1.0, 1.0, 1.0);
        self.shader.activate();
        self.vao.bind()?;
        self.bind_texture();

        let atlas_width = self.texture_atlas.dimension.x as f32;
        let atlas_height = self.texture_atlas.dimension.y as f32;

        for c in text.chars() {
            if let Some(glyph) = self.texture_atlas.glyphs.get(&c) {
                let xpos = x + glyph.x as f32 * scale;
                let ypos = y - (glyph.height as f32 - glyph.y as f32) * scale;

                let w = glyph.width as f32 * scale;
                let h = glyph.height as f32 * scale;

                // Debugging-Information
                #[rustfmt::skip]
                {
                    println!("Glyph: {} at position (x: {}, y: {}) (scale={})", c, glyph.x, glyph.y, scale);
                    println!("Glyph width/height: {},{}", w, h);
                    println!("Calculated position: ({}, {})", xpos, ypos);
                    println!("atlas_width / atlas_height: {}, {}", atlas_width, atlas_height);

                    unsafe {
                        let mut bound_texture: i32 = 0;
                        gl::GetIntegerv(gl::TEXTURE_BINDING_2D, &mut bound_texture);
                        println!("Bound texture ID: {}", bound_texture);
                    }
                };

                let tex_coords = (
                    glyph.x as f32 / atlas_width,
                    glyph.y as f32 / atlas_height,
                    (glyph.x + glyph.width) as f32 / atlas_width,
                    (glyph.y + glyph.height) as f32 / atlas_height,
                );

                let vertices = vec![
                    vertex![xpos, ypos + h, tex_coords.0, tex_coords.1],
                    vertex![xpos, ypos, tex_coords.0, tex_coords.3],
                    vertex![xpos + w, ypos, tex_coords.2, tex_coords.3],
                    vertex![xpos + w, ypos, tex_coords.2, tex_coords.3],
                    vertex![xpos + w, ypos + h, tex_coords.2, tex_coords.1],
                    vertex![xpos, ypos + h, tex_coords.0, tex_coords.1],
                ];

                self.vbo.update_data(vertices, None);
                draw_primitive(PrimitiveType::Triangles, 6);

                x += (glyph.width as f32 + 1.0) * scale;
            }

            // if let Some(glyph) = self.texture_atlas.glyphs.get(&c) {
            //     // Calculate the vertex positions and texture coordinates based on the glyph data
            //     // This involves calculating the position and size of each character quad
            //     // and mapping the correct part of the texture atlas to the quad
            //     let xpos = x_offset + glyph.x as f32 * scale;
            //     let ypos = y_start - glyph.y as f32 * scale;
            //
            //     let w = glyph.width as f32 * scale;
            //     let h = glyph.height as f32 * scale;
            //
            //     let atlas_width = self.texture_atlas.dimension.x as f32;
            //     let atlas_height = self.texture_atlas.dimension.y as f32;
            //
            //     // Let's calculate the texture coordinates
            //     let tex_coords = (
            //         glyph.x as f32 / atlas_width,                 // Left texture coordinate
            //         glyph.y as f32 / atlas_height,                // Top texture coordinate
            //         (glyph.x + glyph.width) as f32 / atlas_width, // Right texture coordinate
            //         (glyph.y + glyph.height) as f32 / atlas_height, // Bottom texture coordinate
            //     );
            //
            //     // Vertex data for the character-quad
            //     let vertices = vec![
            //         vertex![xpos, ypos + h, tex_coords.0, tex_coords.1],
            //         vertex![xpos, ypos, tex_coords.0, tex_coords.3],
            //         vertex![xpos + w, ypos, tex_coords.2, tex_coords.3],
            //         vertex![xpos + w, ypos, tex_coords.2, tex_coords.3],
            //         vertex![xpos + w, ypos + h, tex_coords.2, tex_coords.1],
            //         vertex![xpos, ypos + h, tex_coords.0, tex_coords.1],
            //     ];
            //
            //     // Update content of VBO memory
            //     self.vbo.update_data(vertices, false);
            //
            //     // Draw quad
            //     draw_primitive(PrimitiveType::Triangles, 6);
            //
            //     println!("Glyph: {} at position ({}, {})", c, xpos, ypos);
            //
            //     x_offset += (glyph.width as f32 + 1.0) * scale
            // }
        }

        Ok(())
    }

    fn bind_texture(&self) {
        unsafe {
            // Bind texture atlas
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.texture_id);
        }
    }

    pub fn render_text(&self, text: &str, x: f32, y: f32, scale: f32, color: [f32; 3]) {
        let shader_program = self.shader.program_id();

        unsafe {
            gl::UseProgram(shader_program);

            let color_location =
                gl::GetUniformLocation(shader_program, CString::new("textColor").unwrap().as_ptr());
            gl::Uniform3f(color_location, color[0], color[1], color[2]);

            gl::ActiveTexture(gl::TEXTURE0);

            self.bind_texture();

            let text_location =
                gl::GetUniformLocation(shader_program, CString::new("text").unwrap().as_ptr());
            gl::Uniform1i(text_location, 0);

            let projection_location = gl::GetUniformLocation(
                shader_program,
                CString::new("projection").unwrap().as_ptr(),
            );
            let projection = cgmath::ortho(0.0, 1024.00, 0.0, 768.0, -1.0, 1.0);
            gl::UniformMatrix4fv(projection_location, 1, gl::FALSE, projection.as_ptr());

            let atlas_width = self.texture_atlas.dimension.x as f32;
            let atlas_height = self.texture_atlas.dimension.y as f32;

            let mut x = x;
            for c in text.chars() {
                if let Some(glyph) = self.texture_atlas.glyphs().get(&c) {
                    let xpos = x + glyph.x as f32 * scale;
                    let ypos = y - (glyph.height as f32 - glyph.y as f32) * scale;

                    let w = glyph.width as f32 * scale;
                    let h = glyph.height as f32 * scale;

                    // Texturkoordinaten berechnen
                    let u0 = glyph.x as f32 / atlas_width;
                    let v0 = glyph.y as f32 / atlas_height;
                    let u1 = (glyph.x as f32 + glyph.width as f32) / atlas_width;
                    let v1 = (glyph.y as f32 + glyph.height as f32) / atlas_height;

                    #[rustfmt::skip]
                    println!("Atlas: width: {}, height: {}", atlas_width, atlas_height);
                    println!("Glyph: {}, xpos: {}, ypos: {}, w: {}, h: {}", c, xpos, ypos, w, h);
                    println!("TexCoords: u0: {}, v0: {}, u1: {}, v1: {}", u0, v0, u1, v1);

                    #[rustfmt::skip]
                    let vertices: [f32; 6 * 4] = [
                        xpos,     ypos + h,   u0, v0,
                        xpos,     ypos,       u0, v1,
                        xpos + w, ypos,       u1, v1,

                        xpos,     ypos + h,   u0, v0,
                        xpos + w, ypos,       u1, v1,
                        xpos + w, ypos + h,   u1, v0
                    ];

                    let mut vbo: GLuint = 0;
                    gl::GenBuffers(1, &mut vbo);
                    gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
                    gl::BufferData(
                        gl::ARRAY_BUFFER,
                        (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                        vertices.as_ptr() as *const _,
                        gl::DYNAMIC_DRAW,
                    );

                    gl::VertexAttribPointer(
                        0,
                        3,
                        gl::FLOAT,
                        gl::FALSE,
                        5 * mem::size_of::<GLfloat>() as GLsizei,
                        ptr::null(),
                    );
                    gl::EnableVertexAttribArray(0);
                    gl::VertexAttribPointer(
                        1,
                        2,
                        gl::FLOAT,
                        gl::FALSE,
                        5 * mem::size_of::<GLfloat>() as GLsizei,
                        (3 * mem::size_of::<GLfloat>()) as *const _,
                    );
                    gl::EnableVertexAttribArray(1);

                    gl::DrawArrays(gl::TRIANGLES, 0, 6);

                    gl::BindBuffer(gl::ARRAY_BUFFER, 0);
                    gl::DeleteBuffers(1, &vbo);

                    x += (glyph.width as f32 + 1.0) * scale;
                }
            }
        }
    }
}
