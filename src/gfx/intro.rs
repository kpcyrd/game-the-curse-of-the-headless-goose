use crate::{gfx, machine::Intro};
use core::fmt;
use embedded_graphics::{
    Drawable,
    draw_target::DrawTarget,
    pixelcolor::Rgb666,
    prelude::Point,
    text::{Baseline, Text},
};

const MENU: &[&str] = &["1: Start New Game", "2: Erase Save Data", "3: todo"];
const CONFIRM: &[&str] = &[
    "Are you sure?",
    "",
    "#: Yes, erase my savegame",
    "*: Cancel",
];

pub fn render<D: DrawTarget<Color = Rgb666>>(display: &mut D, intro: &Intro)
where
    <D as DrawTarget>::Error: fmt::Debug,
{
    let menu = if intro.confirm_erase { CONFIRM } else { MENU };

    let mut point = Point::new(0, 0);
    for line in menu {
        Text::with_baseline(line, point, gfx::TEXT_STYLE, Baseline::Top)
            .draw(display)
            .unwrap();

        point += gfx::next_line(gfx::FONT);
    }
}
