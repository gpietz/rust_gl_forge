#![allow(dead_code)]

use std::collections::HashMap;
use std::fs::{read, File};
use std::io::Write;
use std::path::Path;
use std::str;

use anyhow::{Context, Result};
use cgmath::Vector2;
use gl::types::{GLsizei, GLuint};
use image::{DynamicImage, Rgba, RgbaImage};
use rusttype::{point, Scale};

use crate::gl_buffer::BufferObject;
use crate::gl_draw::draw_primitive;
use crate::gl_shader::{Shader, ShaderProgram};
use crate::gl_traits::{Bindable, Deletable};
use crate::gl_types::{BufferType, BufferUsage, PrimitiveType};
use crate::gl_vertex::Vertex;
use crate::gl_vertex_array::VertexArrayObject;
use crate::prelude::Color;
use crate::vertices::textured_vertex::TexturedVertex2D;

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
    image: RgbaImage,
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
        let atlas_height = (metrics.ascent - metrics.descent).ceil() as u32;
        let atlas_width = {
            let min_x = glyphs.first().map(|g| g.pixel_bounding_box().unwrap().min.x).unwrap();
            let max_x = glyphs.last().map(|g| g.pixel_bounding_box().unwrap().max.x).unwrap();
            (max_x - min_x) as u32
        };

        let mut texture_image =
            DynamicImage::new_rgba8(atlas_width + 2, atlas_height + 2).to_rgba8();

        let color_rgba = color.to_rgba();
        let mut glyph_data_map: HashMap<char, GlyphData> = HashMap::new();
        for glyph in glyphs {
            if let Some(bounding_box) = glyph.pixel_bounding_box() {
                // Draw the glyph into the image per-pixel by using the draw closure
                glyph.draw(|x, y, v| {
                    texture_image.put_pixel(
                        // Offset the position by the glyph bounding box
                        x + bounding_box.min.x as u32,
                        y + bounding_box.min.y as u32,
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
            }
        }

        Ok(Self {
            font_size,
            dimension: Vector2::new(atlas_width, atlas_height),
            image: texture_image,
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
        TexturedVertex2D::new($x, $y, $u, $v)
    };
}

pub struct FastFontRenderer {
    vao: VertexArrayObject,
    vbo: BufferObject<TexturedVertex2D>,
    texture_atlas: FontTextureAtlas,
    texture_id: u32,
    shader: ShaderProgram,
}

impl FastFontRenderer {
    pub fn new(texture_atlas: FontTextureAtlas) -> Result<Self> {
        let texture_dimension = texture_atlas.dimension;

        // create vertex array object
        let vao = VertexArrayObject::new(true)?;

        // create vertex buffer object
        let vbo = BufferObject::<TexturedVertex2D>::empty(
            BufferType::ArrayBuffer,
            BufferUsage::StaticDraw,
        );

        for _attribute in TexturedVertex2D::attributes() {
            // TODO! Fixme
            //attribute.setup().expect("Failed to set vertex attributes");
        }

        // Create opengl texture
        let mut texture_id: GLuint = 0;
        unsafe {
            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);

            // Create texture image
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                texture_dimension.x as GLsizei,
                texture_dimension.y as GLsizei,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                texture_atlas.image.as_ptr() as *const _,
            );

            // Set texture parameters
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);

            // Release the texture
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }

        // Load shader source
        const VERT_SHADER: &[u8] = include_bytes!("../resources/shaders/font_renderer.vert");
        const FRAG_SHADER: &[u8] = include_bytes!("../resources/shaders/font_renderer.frag");
        let vert_source =
            str::from_utf8(VERT_SHADER).with_context(|| "Error loading vertex shader source")?;
        let frag_source =
            str::from_utf8(FRAG_SHADER).with_context(|| "Error loading fragment shader source")?;

        // Create shader program
        let mut vert_shader = Shader::load_vertex_shader(vert_source)
            .with_context(|| "Error loading vertex shader")?;
        let mut frag_shader = Shader::load_fragment_shader(frag_source)
            .with_context(|| "Error loading fragment shader")?;
        let shader_prog = ShaderProgram::new(&mut vert_shader, &mut frag_shader)
            .with_context(|| "Error loading shader program for font rendering")?;

        // Drop shader objects
        vert_shader.delete()?;
        frag_shader.delete()?;

        println!("Created shader program for font rendering ({})", texture_id);

        Ok(Self {
            vao,
            vbo,
            texture_atlas,
            texture_id,
            shader: shader_prog,
        })
    }

    pub fn texture_id(&self) -> u32 {
        self.texture_id
    }

    pub fn draw_text(&mut self, text: &str, position: &Vector2<f32>, scale: f32) -> Result<()> {
        let (x_start, y_start) = (position.x, position.y);

        self.shader.activate();

        unsafe {
            // Bind texture atlas
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.texture_id);
        }

        self.vao.bind()?;

        let mut x_offset = x_start;
        for c in text.chars() {
            if let Some(glyph) = self.texture_atlas.glyphs.get(&c) {
                // Calculate the vertex positions and texture coordinates based on the glyph data
                // This involves calculating the position and size of each character quad
                // and mapping the correct part of the texture atlas to the quad
                let xpos = x_offset + glyph.x as f32 * scale;
                let ypos = y_start - glyph.y as f32 * scale;

                let w = glyph.width as f32 * scale;
                let h = glyph.height as f32 * scale;

                let atlas_width = self.texture_atlas.dimension.x;
                let atlas_height = self.texture_atlas.dimension.y;

                // Let's calculate the texture coordinates
                let tex_coords = (
                    glyph.x / atlas_width,                   // Left texture coordinate
                    glyph.y / atlas_height,                  // Top texture coordinate
                    (glyph.x + glyph.width) / atlas_width,   // Right texture coordinate
                    (glyph.y + glyph.height) / atlas_height, // Bottom texture coordinate
                );

                // Vertex data for the character-quad
                let vertices = vec![
                    vertex![xpos, ypos + h, tex_coords.0 as f32, tex_coords.1 as f32],
                    vertex![xpos, ypos, tex_coords.0 as f32, tex_coords.3 as f32],
                    vertex![xpos + w, ypos, tex_coords.2 as f32, tex_coords.3 as f32],
                    vertex![xpos + w, ypos, tex_coords.2 as f32, tex_coords.3 as f32],
                    vertex![xpos + w, ypos + h, tex_coords.2 as f32, tex_coords.1 as f32],
                    vertex![xpos, ypos + h, tex_coords.0 as f32, tex_coords.1 as f32],
                ];

                // Update content of VBO memory
                self.vbo.update_data(vertices, false);

                // Draw quad
                draw_primitive(PrimitiveType::Triangles, 6);

                x_offset += (glyph.width >> 6) as f32 * scale;
            }
        }

        Ok(())
    }
}
