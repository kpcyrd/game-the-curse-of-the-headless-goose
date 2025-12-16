pub mod battle;
pub mod dialogue;
pub mod intro;
pub mod text;

use arrayvec::ArrayString;
use embedded_graphics::{
    mono_font::{MonoFont, MonoTextStyle, MonoTextStyleBuilder, ascii::FONT_7X13},
    pixelcolor::Rgb666,
    prelude::{Point, RgbColor},
    primitives::{PrimitiveStyle, PrimitiveStyleBuilder},
};

pub const HEIGHT: u32 = 480;
pub const WIDTH: u32 = 320;

pub const FONT: &MonoFont = &FONT_7X13;
pub const TEXT_STYLE: MonoTextStyle<Rgb666> = MonoTextStyleBuilder::new()
    .font(FONT)
    .text_color(Rgb666::WHITE)
    .background_color(Rgb666::BLACK)
    .build();

const BLACK_STYLE: PrimitiveStyle<Rgb666> = PrimitiveStyleBuilder::new()
    .fill_color(Rgb666::BLACK)
    .build();

pub const fn next_line(font: &MonoFont) -> Point {
    Point::new(0, font.character_size.height as i32)
}

pub fn text_fill<const CAP: usize>(buf: &mut ArrayString<CAP>) {
    for _ in 0..buf.remaining_capacity() {
        buf.push(' ');
    }
}
