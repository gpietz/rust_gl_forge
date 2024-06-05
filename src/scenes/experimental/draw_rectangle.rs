use crate::render_context::RenderContext;
use crate::scene::{Scene, SceneResult};
use shared_lib::color::Color;
use shared_lib::shapes::rectangle::Rectangle;
use shared_lib::shapes::ShapesFactory;
use shared_lib::Drawable;

#[derive(Default)]
pub struct DrawRectangle {
    rectangle: Option<Rectangle>,
}

impl Scene<RenderContext> for DrawRectangle {
    fn activate(&mut self, context: &mut RenderContext) -> SceneResult {
        let window_size = context.window().size();
        let mut rectangle =
            ShapesFactory::new(window_size).create_rectangle(10.0, 10.0, 300, 200, Color::BLACK)?;
        rectangle.set_fill_color(Some(Color::BLACK));
        rectangle.set_corner_radius(Some(5.0));
        //rectangle.set_opacity(1.00);
        //rectangle.set_strength(3.0);

        self.rectangle = Some(rectangle);
        Ok(())
    }

    fn draw(&mut self, _context: &mut RenderContext) -> SceneResult {
        if let Some(ref mut rect) = self.rectangle {
            rect.draw()?;
        }
        Ok(())
    }
}
