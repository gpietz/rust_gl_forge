use anyhow::Result;
use cgmath::Vector2;

use crate::render_context::RenderContext;
use crate::scene::{Scene, SceneResult};
use shared_lib::color::Color;
use shared_lib::gl_font::{FastFontRenderer, Font};

//////////////////////////////////////////////////////////////////////////////
// - FirstText -
//////////////////////////////////////////////////////////////////////////////

pub struct FirstText {
    text_renderer: FastFontRenderer,
}

impl FirstText {
    pub fn new() -> Result<FirstText> {
        let font = Font::from_file("assets/fonts/Roboto-Regular.ttf")?;
        //let font = Font::from_file("assets/fonts/antonio-bold.ttf")?;
        let font_texture_atlas = font.create_texture_atlas(22.0, &Color::RED)?;
        font_texture_atlas.save_texture("texture_atlas.png")?;
        let renderer = FastFontRenderer::new(font_texture_atlas)?;
        Ok(Self {
            text_renderer: renderer,
        })
    }
}

impl Scene<RenderContext> for FirstText {
    fn draw(&mut self, _context: &mut RenderContext) -> SceneResult {
        let text_position = Vector2::new(0.0, 50.0);
        //self.text_renderer.draw_text("H", &text_position, 1.0)?;
        self.text_renderer.render_text("H", 0.0, 50.0, 1.0, [1.0; 3]);
        Ok(())
    }
}
