use crate::{gfx, machine::Intro};
use core::fmt;
use embedded_graphics::{
    Drawable,
    draw_target::DrawTarget,
    pixelcolor::Rgb666,
    prelude::Point,
    text::{Baseline, Text},
};

const LINES: usize = 4;
const WIDTH: usize = 25;

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

    let mut iter = menu.iter();
    let mut point = Point::new(0, 0);
    for _ in 0..LINES {
        if let Some(line) = iter.next() {
            Text::with_baseline(line, point, gfx::TEXT_STYLE, Baseline::Top)
                .draw(display)
                .unwrap();
            gfx::clear_remaining_text_box(display, point, gfx::FONT, line, WIDTH);
        } else {
            gfx::clear_remaining_text_box(display, point, gfx::FONT, "", WIDTH);
        }

        point += gfx::next_line(gfx::FONT);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lines_consts() {
        let calculated = [MENU, CONFIRM].iter().map(|list| list.len()).max();
        assert_eq!(Some(LINES), calculated)
    }

    #[test]
    fn test_width_consts() {
        let width = [MENU, CONFIRM]
            .iter()
            .flat_map(|list| list.iter())
            .map(|line| line.len())
            .max();
        assert_eq!(Some(WIDTH), width)
    }
}
