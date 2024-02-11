use shared_lib::gl_texture::{Texture, TextureBuilder};
use anyhow::Result;

pub(crate) fn create_texture(path: &str, has_alpha: bool, flip_vertical: bool) -> Result<Texture> {
    TextureBuilder::default()
        .path(path)
        .has_alpha(has_alpha)
        .flip_vertical(flip_vertical)
        .build()
}
