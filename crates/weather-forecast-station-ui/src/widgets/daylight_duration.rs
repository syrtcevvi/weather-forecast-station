use embedded_graphics::{
    Drawable,
    prelude::{DrawTarget, Point, Primitive, Size},
    primitives::{Circle, PrimitiveStyle, Rectangle},
};
use embedded_layout::View;

pub struct DaylightDuration {
    bounds: Rectangle,
}

impl DaylightDuration {
    pub fn new(size: Size) -> Self {
        Self {
            bounds: Rectangle::new(Point::zero(), size),
        }
    }
}

impl Drawable for DaylightDuration {
    type Color = epd_waveshare::color::Color;
    type Output = ();

    fn draw<D: DrawTarget<Color = Self::Color>>(
        &self,
        target: &mut D,
    ) -> Result<Self::Output, D::Error> {
        let style = PrimitiveStyle::with_stroke(Self::Color::Black, 1);

        let outer_circle_diameter = self.bounds.size.width;
        Circle::new(self.bounds.top_left, outer_circle_diameter)
            .into_styled(style)
            .draw(target)?;

        let inner_circle_diameter = (self.bounds.size.width as f32 * 0.7) as u32;
        Circle::new(
            Point::new(
                self.bounds.top_left.x
                    + ((outer_circle_diameter - inner_circle_diameter) / 2) as i32,
                self.bounds.top_left.y
                    + ((outer_circle_diameter - inner_circle_diameter) / 2) as i32,
            ),
            inner_circle_diameter,
        )
        .into_styled(style)
        .draw(target)?;

        Ok(())
    }
}

impl View for DaylightDuration {
    #[inline]
    fn translate_impl(&mut self, by: Point) {
        self.bounds.translate_mut(by);
    }

    #[inline]
    fn bounds(&self) -> Rectangle {
        self.bounds
    }
}
