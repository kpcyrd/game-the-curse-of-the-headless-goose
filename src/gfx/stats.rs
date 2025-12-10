use crate::{action::Move, fighter::Fighter, gfx};
use arrayvec::ArrayString;
use core::fmt;
use core::fmt::Write;
use embedded_graphics::{
    Drawable,
    draw_target::DrawTarget,
    pixelcolor::Rgb666,
    prelude::Point,
    text::{Baseline, Text},
};

const CAP: usize = 14;
const CHAR_WIDTH: i32 = gfx::TEXT_STYLE.font.character_size.width as i32;
const CHAR_HEIGHT: i32 = gfx::TEXT_STYLE.font.character_size.height as i32;

pub fn render<D: DrawTarget<Color = Rgb666>>(
    display: &mut D,
    point: Point,
    fighter: &Fighter,
    _mv: Option<&Move>,
) where
    <D as DrawTarget>::Error: fmt::Debug,
{
    let point = Point::new(
        point.x.min(gfx::WIDTH as i32 - CAP as i32 * CHAR_WIDTH),
        point.y,
    );

    let mut text = ArrayString::<CAP>::new_const();
    for (i, (label, value)) in [("Health", fighter.health), ("Energy", fighter.energy)]
        .iter()
        .enumerate()
    {
        text.clear();
        write!(&mut text, "{label}: {value}").unwrap();
        Text::with_baseline(
            &text,
            point + Point::new(0, i as i32 * CHAR_HEIGHT),
            gfx::TEXT_STYLE,
            Baseline::Top,
        )
        .draw(display)
        .unwrap();
    }
}
