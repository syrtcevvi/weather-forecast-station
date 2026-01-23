mod layouts;
mod sections;
mod widgets;

use embedded_graphics::prelude::*;
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};

use crate::layouts::default_layout::DefaultLayout;

type Color = epd_waveshare::color::Color;
type Display = SimulatorDisplay<Color>;

const DISPLAY_HEIGHT: u32 = 128;
const DISPLAY_WIDTH: u32 = 296;

fn main() -> Result<(), core::convert::Infallible> {
    let mut display = Display::new(Size::new(DISPLAY_WIDTH, DISPLAY_HEIGHT));
    DefaultLayout::new(display.bounding_box().size).draw(&mut display)?;

    let mut simulator_window = Window::new(
        "Weather forecast station simulator",
        &OutputSettingsBuilder::new().scale(3).build(),
    );
    simulator_window.show_static(&display);

    Ok(())
}
