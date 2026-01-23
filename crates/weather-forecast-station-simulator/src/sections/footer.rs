use embedded_graphics::{
    Drawable,
    mono_font::{MonoTextStyle, ascii::FONT_6X9},
    prelude::{DrawTarget, Point, Primitive, Size},
    primitives::{Line, PrimitiveStyle, Rectangle},
    text::Text,
};
use embedded_layout::{
    View,
    align::{Align, horizontal, vertical},
    layout::linear::LinearLayout,
    prelude::Chain,
};
use epd_waveshare::color::Color;

pub struct Footer {
    bounds: Rectangle,
}

impl Footer {
    pub fn new(size: Size) -> Self {
        Self {
            bounds: Rectangle::new(Point::zero(), size),
        }
    }
}

impl Drawable for Footer {
    type Color = epd_waveshare::color::Color;
    type Output = ();

    fn draw<D: DrawTarget<Color = Self::Color>>(
        &self,
        target: &mut D,
    ) -> Result<Self::Output, D::Error> {
        let text_style = MonoTextStyle::new(&FONT_6X9, Color::Black);
        let line_style = PrimitiveStyle::with_stroke(Color::Black, 1);

        LinearLayout::vertical(
            Chain::new(
                Line::new(Point::zero(), Point::new(self.bounds.size.width as i32, 0))
                    .into_styled(line_style),
            )
            .append(Text::new(
                &format!("v{}", env!("CARGO_PKG_VERSION")),
                Point::zero(),
                text_style,
            )),
        )
        .arrange()
        .align_to(&self.bounds, horizontal::Left, vertical::Bottom)
        .draw(target)?;

        Ok(())
    }
}

impl View for Footer {
    #[inline]
    fn translate_impl(&mut self, by: Point) {
        self.bounds.translate_mut(by);
    }

    #[inline]
    fn bounds(&self) -> Rectangle {
        self.bounds
    }
}
