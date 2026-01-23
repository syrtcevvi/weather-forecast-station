use embedded_graphics::{
    Drawable,
    prelude::{DrawTarget, Point, Size},
    primitives::Rectangle,
};
use embedded_layout::{
    View,
    align::{Align, horizontal, vertical},
    layout::linear::LinearLayout,
    prelude::Chain,
};

use crate::sections::{footer::Footer, main_content::MainContent};

pub struct DefaultLayout {
    bounds: Rectangle,
}

impl DefaultLayout {
    pub fn new(size: Size) -> Self {
        Self {
            bounds: Rectangle::new(Point::zero(), size),
        }
    }
}

impl Drawable for DefaultLayout {
    type Color = epd_waveshare::color::Color;
    type Output = ();

    fn draw<D: DrawTarget<Color = Self::Color>>(
        &self,
        target: &mut D,
    ) -> Result<Self::Output, D::Error> {
        LinearLayout::vertical(
            Chain::new(MainContent::new(Size::new(
                self.bounds.size.width,
                (self.bounds.size.height as f32 * 0.9) as u32,
            )))
            .append(Footer::new(Size::new(
                self.bounds.size.width,
                (self.bounds.size.height as f32 * 0.1) as u32,
            ))),
        )
        .arrange()
        .align_to(&self.bounds, horizontal::Left, vertical::Bottom)
        .draw(target)?;

        Ok(())
    }
}

impl View for DefaultLayout {
    #[inline]
    fn translate_impl(&mut self, by: Point) {
        self.bounds.translate_mut(by);
    }

    #[inline]
    fn bounds(&self) -> Rectangle {
        self.bounds
    }
}
