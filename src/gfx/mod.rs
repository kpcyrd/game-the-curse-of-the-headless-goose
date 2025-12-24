pub mod battle;
pub mod dialogue;
pub mod glide;
pub mod hq;
pub mod intro;
pub mod text;

use arrayvec::ArrayString;
use core::fmt;
use embedded_graphics::{
    Drawable,
    mono_font::{MonoFont, MonoTextStyle, MonoTextStyleBuilder, ascii},
    pixelcolor::Rgb666,
    prelude::{DrawTarget, Point, RgbColor, WebColors},
    primitives::{PrimitiveStyle, PrimitiveStyleBuilder},
    text::{Alignment, Text},
};

pub const HEIGHT: u32 = 480;
pub const WIDTH: u32 = 320;

pub const FONT: &MonoFont = &ascii::FONT_7X13;
pub const FONT_BOLD: &MonoFont = &ascii::FONT_7X13_BOLD;
pub const TEXT_STYLE: MonoTextStyle<Rgb666> = MonoTextStyleBuilder::new()
    .font(FONT)
    .text_color(Rgb666::WHITE)
    .background_color(Rgb666::BLACK)
    .build();

pub const CHAPTER_STYLE: MonoTextStyle<Rgb666> = MonoTextStyleBuilder::new()
    .font(&ascii::FONT_9X15_BOLD)
    .text_color(Rgb666::WHITE)
    .background_color(Rgb666::BLACK)
    .build();

pub const GREEN_TEXT: MonoTextStyle<Rgb666> = MonoTextStyleBuilder::new()
    .font(FONT_BOLD)
    .text_color(Rgb666::new(0, 50, 0))
    .background_color(Rgb666::BLACK)
    .build();

pub const RED_TEXT: MonoTextStyle<Rgb666> = MonoTextStyleBuilder::new()
    .font(FONT_BOLD)
    .text_color(Rgb666::CSS_RED)
    .background_color(Rgb666::BLACK)
    .build();

const BLACK_STYLE: PrimitiveStyle<Rgb666> = PrimitiveStyleBuilder::new()
    .fill_color(Rgb666::BLACK)
    .build();

/*
const WHITE_STYLE: PrimitiveStyle<Rgb666> = PrimitiveStyleBuilder::new()
    .fill_color(Rgb666::WHITE)
    .build();
*/

pub const fn next_line(font: &MonoFont) -> Point {
    Point::new(0, font.character_size.height as i32)
}

pub fn text_fill<const CAP: usize>(buf: &mut ArrayString<CAP>) {
    for _ in 0..buf.remaining_capacity() {
        buf.push(' ');
    }
}

pub fn render_bold<D: DrawTarget<Color = Rgb666>>(display: &mut D, text: &str)
where
    <D as DrawTarget>::Error: fmt::Debug,
{
    Text::with_alignment(
        text,
        display.bounding_box().center(),
        CHAPTER_STYLE,
        Alignment::Center,
    )
    .draw(display)
    .unwrap();
}
