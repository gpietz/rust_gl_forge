use anyhow::Result;
use shared_lib::opengl::font::Font;
use std::sync::Arc;

use shared_lib::text::simple_text_renderer::SimpleTextRenderer;
use shared_lib::Position2D;

use crate::render_context::RenderContext;
use crate::scene::{Scene, SceneResult};

//////////////////////////////////////////////////////////////////////////////
// - FirstText -
//////////////////////////////////////////////////////////////////////////////

pub struct TextRendering<'a> {
    font: Arc<Font<'a>>,
    text_renderer: SimpleTextRenderer<'a>,
}

impl<'a> TextRendering<'a> {
    pub fn new() -> Result<TextRendering<'a>> {
        let font = Arc::new(Font::from_file("assets/fonts/Roboto-Regular.ttf")?);
        let text_renderer = SimpleTextRenderer::new(&font, 36.0)?;
        Ok(Self {
            font,
            text_renderer,
        })
    }
}

impl<'a> Scene<RenderContext> for TextRendering<'a> {
    fn draw(&mut self, _context: &mut RenderContext) -> SceneResult {
        let position = Position2D::new(10.0, 10.0);
        self.text_renderer
            .render_text("Hello world!", position, None)?;
        Ok(())
    }
}
