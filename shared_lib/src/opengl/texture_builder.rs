use anyhow::Context;

use crate::gl_prelude::TextureTarget;
use crate::opengl::texture::Texture;

//////////////////////////////////////////////////////////////////////////////
// - TextureBuilder -
//////////////////////////////////////////////////////////////////////////////

#[derive(Default, Debug)]
pub struct TextureBuilder {
    path: Option<String>,
    has_alpha: bool,
    flip_horizontal: bool,
    flip_vertical: bool,
    uniform_name: Option<String>,
    texture_target: Option<TextureTarget>,
}

impl TextureBuilder {
    pub fn path<P: Into<String>>(mut self, path: P) -> Self {
        self.path = Some(path.into());
        self
    }

    pub fn has_alpha(mut self, value: bool) -> Self {
        self.has_alpha = value;
        self
    }

    pub fn flip_horizontal(mut self, value: bool) -> Self {
        self.flip_horizontal = value;
        self
    }

    pub fn flip_vertical(mut self, value: bool) -> Self {
        self.flip_vertical = value;
        self
    }

    pub fn with_uniform_name(mut self, uniform_name: &str) -> Self {
        self.uniform_name = Some(uniform_name.to_string());
        self
    }

    pub fn with_texture_target(mut self, texture_target: TextureTarget) -> Self {
        self.texture_target = Some(texture_target);
        self
    }

    pub fn build(&self) -> anyhow::Result<Texture> {
        let uniform_name = self.uniform_name.clone().unwrap_or_default();
        let texture_target = self.texture_target.unwrap_or(TextureTarget::Texture2D);
        Texture::new(
            self.path.clone().with_context(|| "No path specified")?,
            self.has_alpha,
            self.flip_horizontal,
            self.flip_vertical,
            &uniform_name,
            texture_target,
        )
    }
}