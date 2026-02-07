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

use crate::widgets::daylight_duration::DaylightDuration;

pub struct MainContent {
    bounds: Rectangle,
}

impl MainContent {
    pub fn new(size: Size) -> Self {
        Self {
            bounds: Rectangle::new(Point::zero(), size),
        }
    }
}

impl Drawable for MainContent {
    type Color = epd_waveshare::color::Color;
    type Output = ();

    fn draw<D: DrawTarget<Color = Self::Color>>(
        &self,
        target: &mut D,
    ) -> Result<Self::Output, D::Error> {
        let daylight_duration = DaylightDuration::new(Size::new(
            (self.bounds.size.height as f32 * 0.9) as u32,
            (self.bounds.size.height as f32 * 0.9) as u32,
        ));
        LinearLayout::vertical(Chain::new(daylight_duration))
            .arrange()
            .align_to(&self.bounds, horizontal::Center, vertical::Center)
            .draw(target)?;

        Ok(())
    }
}

impl View for MainContent {
    #[inline]
    fn translate_impl(&mut self, by: Point) {
        self.bounds.translate_mut(by);
    }

    #[inline]
    fn bounds(&self) -> Rectangle {
        self.bounds
    }
}
