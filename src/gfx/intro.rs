use crate::{
    gfx::{self, text::TextBox},
    machine::Intro,
};
use core::fmt;
use embedded_graphics::{
    draw_target::DrawTarget,
    pixelcolor::Rgb666,
    prelude::{Point, Size},
    primitives::{Rectangle, StyledDrawable},
};

const LINES: usize = 4;
const WIDTH: usize = 25;

const TEXT_OFFSET: Point = Point::new(
    (gfx::WIDTH - (gfx::FONT.character_size.width * WIDTH as u32)) as i32 / 2,
    300,
);

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
    let menu = if intro.confirm_erase {
        CONFIRM
    } else {
        Rectangle::with_center(Point::new(gfx::WIDTH as i32 / 2, 150), Size::new(200, 200))
            .draw_styled(&gfx::WHITE_STYLE, display)
            .unwrap();
        MENU
    };

    let mut iter = menu.iter();
    let mut point = TEXT_OFFSET;
    for _ in 0..LINES {
        TextBox::new(point, &gfx::TEXT_STYLE, WIDTH).render_and_clear(display, iter.next());
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
