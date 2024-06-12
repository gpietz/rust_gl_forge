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
        Self {
            x,
            y,
        }
    }

    pub fn uniform(size: f32) -> Self {
        Self {
            x: size,
            y: size,
        }
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

pub struct Font<'a> {
    font_path: Option<String>,
    pub(crate) font: Box<rusttype::Font<'a>>,
}

impl<'a> Font<'a> {
    pub fn from_file<P: AsRef<Path>>(font_path: P) -> Result<Font<'a>> {
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

    // pub fn create_texture_atlas(&self, font_size: f32, color: &Color) -> Result<FontTextureAtlas> {
    //     self.create_texture_atlas_with_size(FontSize::uniform(font_size), color)
    // }

    // pub fn create_texture_atlas_with_size(
    //     &self,
    //     font_size: FontSize,
    //     color: &Color,
    // ) -> Result<FontTextureAtlas> {
    //     let font: &rusttype::Font = self.font.as_ref();
    //     FontTextureAtlas::new(font, font_size, color)
    // }
}

impl<'a> From<Font<'a>> for rusttype::Font<'a> {
    fn from(font: Font<'a>) -> rusttype::Font<'a> {
        let font_ref = font.font.as_ref();
        font_ref.clone()
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
                        Rgba([color_rgba[0], color_rgba[1], color_rgba[2], (v * 255.0) as u8]),
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
