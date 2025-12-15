pub mod battle;
pub mod dialogue;
pub mod intro;

use embedded_graphics::{
    mono_font::{MonoFont, MonoTextStyle, ascii::FONT_7X13},
    pixelcolor::Rgb666,
    prelude::{Point, RgbColor},
};

pub const HEIGHT: u32 = 480;
pub const WIDTH: u32 = 320;

pub const FONT: &MonoFont = &FONT_7X13;
pub const TEXT_STYLE: MonoTextStyle<Rgb666> = MonoTextStyle::new(FONT, Rgb666::WHITE);

pub const fn next_line(font: &MonoFont) -> Point {
    Point::new(0, font.character_size.height as i32)
}
