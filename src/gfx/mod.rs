pub mod battle;
pub mod dialogue;
pub mod intro;

use arrayvec::ArrayString;
use core::fmt;
use embedded_graphics::{
    mono_font::{MonoFont, MonoTextStyle, MonoTextStyleBuilder, ascii::FONT_7X13},
    pixelcolor::Rgb666,
    prelude::{DrawTarget, Point, RgbColor, Size},
    primitives::{PrimitiveStyle, PrimitiveStyleBuilder, Rectangle, StyledDrawable},
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

pub fn clear_remaining_text_box<D: DrawTarget<Color = Rgb666>>(
    display: &mut D,
    point: Point,
    font: &MonoFont,
    text: &str,
    width: usize,
) where
    D::Error: fmt::Debug,
{
    let width = width.saturating_sub(text.len());
    if width == 0 {
        return;
    }

    let delta = Point::new(font.character_size.width as i32 * text.len() as i32, 0);

    Rectangle::new(
        point + delta,
        Size::new(
            font.character_size.width * width as u32,
            font.character_size.height,
        ),
    )
    .draw_styled(&BLACK_STYLE, display)
    .unwrap();
}
