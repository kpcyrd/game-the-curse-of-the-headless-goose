pub mod battle;

use embedded_graphics::{
    mono_font::{MonoFont, MonoTextStyle, ascii::FONT_7X13},
    pixelcolor::Rgb666,
    prelude::RgbColor,
};

pub const HEIGHT: u32 = 240;
pub const WIDTH: u32 = 320;

pub const FONT: &MonoFont = &FONT_7X13;
pub const TEXT_STYLE: MonoTextStyle<Rgb666> = MonoTextStyle::new(FONT, Rgb666::WHITE);
