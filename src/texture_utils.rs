use anyhow::Result;
use shared_lib::opengl::texture::Texture;
use shared_lib::opengl::texture_builder::TextureBuilder;

pub(crate) fn create_texture(path: &str, has_alpha: bool, flip_vertical: bool) -> Result<Texture> {
    TextureBuilder::default()
        .path(path)
        .has_alpha(has_alpha)
        .flip_vertical(flip_vertical)
        .build()
}
