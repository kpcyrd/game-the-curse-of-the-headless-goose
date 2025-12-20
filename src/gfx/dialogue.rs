use crate::{gfx, machine::dialogue::Dialogue};
use core::fmt;
use embedded_graphics::{
    Drawable,
    draw_target::DrawTarget,
    pixelcolor::Rgb666,
    prelude::Point,
    text::{Baseline, Text},
};

const LINE_WIDTH: u32 = gfx::WIDTH / gfx::FONT.character_size.width;

pub fn render<D: DrawTarget<Color = Rgb666>>(display: &mut D, dialogue: &Dialogue)
where
    <D as DrawTarget>::Error: fmt::Debug,
{
    let mut point = Point::new(0, 0);
    let mut text = dialogue.text;

    while !text.is_empty() && point.y < gfx::HEIGHT as i32 {
        let max = text
            .find('\n')
            .unwrap_or(text.len())
            .min(LINE_WIDTH as usize);

        let remaining = if max == 0 {
            text.strip_prefix('\n').unwrap_or(text)
        } else {
            let (line, remaining) = text.split_at(max);

            Text::with_baseline(line, point, gfx::TEXT_STYLE, Baseline::Top)
                .draw(display)
                .unwrap();

            remaining
        };

        point += gfx::next_line(gfx::FONT);
        text = remaining.strip_prefix('\n').unwrap_or(remaining);
    }
}
