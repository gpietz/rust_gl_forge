use crate::color::Color;
use anyhow::Context;
use cgmath::Vector2;
use image::{DynamicImage, Rgba, RgbaImage};
use rusttype::{point, Scale};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

pub struct FontTextureAtlas {
    font_size: f32,
    dimension: Vector2<u32>,
    image: Box<RgbaImage>,
    glyphs: HashMap<char, GlyphData>,
    color: Color,
}

impl FontTextureAtlas {
    pub fn new(
        font: &rusttype::Font<'static>,
        font_size: f32,
        color: &Color,
    ) -> anyhow::Result<Self> {
        #[rustfmt::skip]
        let characters = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+-.,;:_#*@?!=()[]";
        let scale = Scale::uniform(font_size);
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
                if x_offset + bounding_box.width() as u32 > atlas_width {
                    x_offset = 1; // Neue Zeile beginnen
                    y_offset += atlas_height; // Höhe der aktuellen Zeile hinzufügen und mit Padding
                }

                if y_offset + bounding_box.height() as u32 > atlas_height {
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

    pub fn font_size(&self) -> &f32 {
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

    pub fn save_texture(&self, file_path: &str) -> anyhow::Result<()> {
        self.image
            .save(file_path)
            .with_context(|| "Error in saving texture atlas image")
    }

    pub fn save_font_mapping(&self, file_path: &str) -> anyhow::Result<()> {
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
fn save_mapping_to_xml(
    glyph_data_map: &HashMap<char, GlyphData>,
    file_path: &str,
) -> anyhow::Result<()> {
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
