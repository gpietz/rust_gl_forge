use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

use anyhow::{Context, Result};
use image::{DynamicImage, Rgba, RgbaImage};
use rusttype::{Font, Scale, VMetrics};
use crate::opengl::texture_utils::get_texture_from_gpu;

pub struct GlyphData {
    pub(crate) index: u8,
    pub(crate) char: char,
    pub(crate) x: u32,
    pub(crate) y: u32,
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) bearing_x: i32,
    pub(crate) bearing_y: i32,
    pub(crate) advance_with: f32,
}

pub struct FontAtlas {
    pub(crate) texture_id: u32,
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) glyphs: HashMap<char, GlyphData>,
    pub(crate) metrics: VMetrics,
    pub(crate) scale: Scale,
}

impl FontAtlas {
    pub fn new(font: &Font, scale: Scale, color: Rgba<u8>) -> FontAtlas {
        let characters =
            "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+-.,;:_#*@?!=()[]<>";
        let metrics = font.v_metrics(scale);
        let offset = rusttype::point(0.0, metrics.ascent);
        let glyphs: Vec<_> = font.layout(characters, scale, offset).collect();

        // Spacing between glyphs
        // let glyphs: Vec<_> = font
        //     .layout(characters, scale, point(20.0, 20.0 + metrics.ascent))
        //     .collect();

        // Calculate atlas dimension, source code from rusttype examples
        let glyphs_height = (metrics.ascent - metrics.descent).ceil() as u32;
        let glyphs_width = {
            let min_x = glyphs.first().map(|g| g.pixel_bounding_box().unwrap().min.x).unwrap();
            let max_x = glyphs.last().map(|g| g.pixel_bounding_box().unwrap().max.x).unwrap();
            (max_x - min_x) as u32
        };

        // If spacing between glyphs is used there must be added some padding (+40, +40))
        let mut texture_image = DynamicImage::new_rgba8(glyphs_width, glyphs_height).to_rgba8();
        let mut glyph_data_map = HashMap::new();

        if glyphs.len() != characters.len() {
            panic!("Glyphs length is not equal to characters length!");
        }

        let mut char_index = 0;
        for glyph in glyphs {
            if let Some(bounding_box) = glyph.pixel_bounding_box() {
                glyph.draw(|x, y, v| {
                    texture_image.put_pixel(
                        x + bounding_box.min.x as u32,
                        y + bounding_box.min.y as u32,
                        Rgba([color[0], color[1], color[2], (v * 255.0) as u8]),
                    )
                });

                let glyph_char = characters.chars().nth(char_index).unwrap();
                let glyph_data = GlyphData {
                    index: glyph_data_map.len() as u8,
                    char: glyph_char,
                    x: bounding_box.min.x as u32,
                    y: bounding_box.min.y as u32,
                    width: bounding_box.width() as u32,
                    height: bounding_box.height() as u32,
                    bearing_x: bounding_box.min.x,
                    bearing_y: bounding_box.min.y,
                    advance_with: glyph.unpositioned().h_metrics().advance_width,
                };

                glyph_data_map.insert(glyph_char, glyph_data);
                char_index += 1;
            }
        }

        //texture_image.save("font_atlas_original.png").unwrap();

        //vertical_flip(&mut texture_image);
        let texture_data = texture_image.into_raw();

        let texture_id = unsafe {
            let mut texture: u32 = 0;
            gl::GenTextures(1, &mut texture);
            gl::BindTexture(gl::TEXTURE_2D, texture);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                glyphs_width as i32,
                glyphs_height as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                texture_data.as_ptr() as *const _,
            );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::GenerateMipmap(gl::TEXTURE_2D);
            texture
        };

        FontAtlas {
            texture_id,
            width: glyphs_width,
            height: glyphs_height,
            glyphs: glyph_data_map,
            metrics,
            scale,
        }
    }

    pub fn save_font_texture(&self, file_path: &str) -> Result<()> {
        let texture = get_texture_from_gpu(self.texture_id, self.width as i32, self.height as i32);
        texture.save(file_path).with_context(|| "Error saving texture atlas image")
    }

    pub fn save_font_mapping(&self, file_path: &str) -> Result<()> {
        save_mapping_to_xml(&self.glyphs, file_path).with_context(|| "Error in saving font mapping")
    }

    pub fn average_glyph_width(&self) -> f32 {
        let total_width: u32 = self.glyphs.values().map(|glyph| glyph.width).sum();
        let num_glyphs = self.glyphs.len() as f32;
        total_width as f32 / num_glyphs
    }

    pub fn line_height(&self) -> f32 {
        let max_glyph_height = self.glyphs.values().map(|glyph| glyph.height).max().unwrap();
        max_glyph_height as f32 + self.metrics.line_gap
    }

    pub fn text_dimensions(&self, text: &str) -> (f32, f32) {
        let mut width = 0.0f32;
        let mut height = 0.0f32;

        for ch in text.chars() {
            if let Some(glyph) = self.glyphs.get(&ch) {
                width += glyph.advance_with;
                let glyph_height = glyph.bearing_y as f32 + glyph.height as f32;
                if glyph_height > height {
                    height = glyph_height;
                }
            } else if ch == ' ' {
                width += self.space_width();
            } else {
                width += self.average_glyph_width() / 2.0;
            }
        }

        (width, height)
    }

    pub fn space_width(&self) -> f32 {
        // Space
        if let Some(glyph) = self.glyphs.get(&' ') {
            return glyph.advance_with;
        } else if let Some(glyph) = self.glyphs.get(&'x') {
            return glyph.advance_with / 2.0;
        }
        self.average_glyph_width() / 2.0
    }
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
    let mut glyph_data_vec: Vec<_> = glyph_data_map.iter().collect();
    glyph_data_vec.sort_by(|a, b| a.1.index.cmp(&b.1.index));

    // Create xml data from the glyph mapping
    let mut xml = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<GlyphMapping>\n");
    for (key, glyph) in glyph_data_vec {
        let glyph_xml = format!(
            "\t<GlyphData character=\"{}\" x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" />\n",
            key, glyph.x, glyph.y, glyph.width, glyph.height
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

fn vertical_flip(image: &mut RgbaImage) {
    let width = image.width();
    let height = image.height();

    for y in 0..(height / 2) {
        for x in 0..width {
            let top_pixel = *image.get_pixel(x, y);
            let bottom_pixel = *image.get_pixel(x, height - y - 1);
            image.put_pixel(x, y, bottom_pixel);
            image.put_pixel(x, height - y - 1, top_pixel);
        }
    }
}
