use core::fmt::Write;

use console::kprintln;
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};
use ssd1306::{mode::BufferedGraphicsMode, prelude::*, I2CDisplayInterface, Ssd1306};

pub fn display() -> Result<(), &'static str> {
    let mut i2c = pi::i2c::I2C::new(250_000_000, false);

    let interface = I2CDisplayInterface::new(i2c);
    let mut display =
        Ssd1306::new(interface, DisplaySize128x32, DisplayRotation::Rotate0).into_terminal_mode();
    match display.init() {
        Ok(_) => {
            kprintln!("Display initialized");
        }
        Err(e) => {
            kprintln!("Display initialization failed: {:?}", e);
            return Err("Display initialization failed");
        }
    }

    display.write_str("Hello!");

    /*let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();

    Text::with_baseline("Hello world!", Point::zero(), text_style, Baseline::Top)
        .draw(&mut display)
        .ok();

    Text::with_baseline("Hello Rust!", Point::new(0, 16), text_style, Baseline::Top)
        .draw(&mut display)
        .ok();*/

    //display.flush().unwrap();

    Ok(())
}
