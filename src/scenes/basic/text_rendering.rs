use anyhow::Result;
use shared_lib::opengl::font::Font;

use shared_lib::text::simple_text_renderer::SimpleTextRenderer;
use shared_lib::Position2D;

use crate::render_context::RenderContext;
use crate::scene::{Scene, SceneResult};

//////////////////////////////////////////////////////////////////////////////
// - FirstText -
//////////////////////////////////////////////////////////////////////////////

pub struct TextRendering {
    text_renderer: SimpleTextRenderer,
}

impl TextRendering {
    pub fn new() -> Result<TextRendering> {
        let font = Font::from_file("assets/fonts/Roboto-Regular.ttf")?;
        let text_renderer = SimpleTextRenderer::new(&font, 36.0)?;
        Ok(Self { text_renderer })
    }
}

impl Scene<RenderContext> for TextRendering {
    fn draw(&mut self, _context: &mut RenderContext) -> SceneResult {
        let position = Position2D::new(10.0, 10.0);
        self.text_renderer
            .render_text("Hello world!", position, None)?;
        Ok(())
    }
}
