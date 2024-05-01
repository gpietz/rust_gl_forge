use shared_lib::gl_texture::Texture;

use crate::render_context::RenderContext;
use crate::scene::SceneError;

pub(crate) fn query_texture(
    context: &mut RenderContext,
    texture_name: &str,
) -> Result<Texture, SceneError> {
    let texture_manager = context.texture_manager();
    texture_manager
        .get_texture(texture_name)
        .map_err(|_| SceneError::TextLoadError {
            name: texture_name.to_string(),
        })
}
