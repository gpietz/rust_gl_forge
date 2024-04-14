use anyhow::Result;
use cgmath::Vector2;

use shared_lib::color::Color;
use shared_lib::gl_font::{FastFontRenderer, Font};

use crate::renderables::Renderable;

//////////////////////////////////////////////////////////////////////////////
// - FirstText -
//////////////////////////////////////////////////////////////////////////////

pub struct FirstText {
    text_renderer: FastFontRenderer,
}

impl FirstText {
    pub fn new() -> Result<FirstText> {
        //let font = Font::from_file("assets/fonts/Roboto-Regular.ttf")?;
        let font = Font::from_file("assets/fonts/antonio-bold.ttf")?;
        let font_texture_atlas = font.create_texture_atlas(22.0, &Color::RED)?;
        let renderer = FastFontRenderer::new(font_texture_atlas)?;
        Ok(Self {
            text_renderer: renderer,
        })
    }
}

impl Renderable for FirstText {
    fn draw(&mut self, _delta_time: f32) -> Result<()> {
        let text_position = Vector2::new(4.0, -2.5);
        self.text_renderer
            .draw_text("Hallo Text", &text_position, 4.0)?;
        Ok(())
    }
}
