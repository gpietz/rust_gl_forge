use std::fs::read;
use std::path::Path;
use anyhow::Context;

pub struct Font<'a> {
    font_path: Option<String>,
    pub(crate) font: Box<rusttype::Font<'a>>,
}

impl<'a> Font<'a> {
    pub fn from_file<P: AsRef<Path>>(font_path: P) -> anyhow::Result<Font<'a>> {
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
