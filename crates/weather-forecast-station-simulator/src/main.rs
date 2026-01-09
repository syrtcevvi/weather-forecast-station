use embedded_graphics::{
    mono_font::{MonoTextStyle, ascii::FONT_6X9},
    prelude::*,
    primitives::{Line, PrimitiveStyle},
    text::Text,
};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};
use embedded_layout::{
    align::{Align, horizontal, vertical},
    layout::linear::LinearLayout,
    prelude::Chain,
};

type Color = epd_waveshare::color::Color;
type Display = SimulatorDisplay<Color>;

const DISPLAY_HEIGHT: u32 = 128;
const DISPLAY_WIDTH: u32 = 296;

fn main() -> Result<(), core::convert::Infallible> {
    let mut display = Display::new(Size::new(DISPLAY_WIDTH, DISPLAY_HEIGHT));
    let display_area = display.bounding_box();

    let line_style = PrimitiveStyle::with_stroke(Color::Black, 1);
    let text_style = MonoTextStyle::new(&FONT_6X9, Color::Black);

    LinearLayout::vertical(Chain::new(Text::new(
        "embedded-layout",
        Point::zero(),
        text_style,
    )))
    .with_alignment(horizontal::Center)
    .arrange()
    .align_to(&display_area, horizontal::Center, vertical::Center)
    .draw(&mut display)?;

    let version = env!("CARGO_PKG_VERSION");
    LinearLayout::vertical(
        Chain::new(
            Line::new(Point::zero(), Point::new(DISPLAY_WIDTH as i32, 0)).into_styled(line_style),
        )
        .append(Text::new(
            &format!("v{}", version),
            Point::zero(),
            text_style,
        )),
    )
    .arrange()
    .align_to(&display_area, horizontal::Left, vertical::Bottom)
    .draw(&mut display)?;

    let mut simulator_window = Window::new(
        "Weather forecast station simulator",
        &OutputSettingsBuilder::new().scale(3).build(),
    );
    simulator_window.show_static(&display);

    Ok(())
}
