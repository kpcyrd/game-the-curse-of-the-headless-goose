use crate::{gfx, machine::dialogue::Dialogue, story::Decoration};
use core::fmt;
use embedded_graphics::{
    Drawable,
    draw_target::DrawTarget,
    image::ImageRaw,
    pixelcolor::{BinaryColor, Rgb666},
    prelude::{Point, Primitive, RgbColor, Size},
    primitives::Rectangle,
    text::{Alignment, Baseline, Text},
};
use embedded_graphics_colorcast::Image;

const LINE_WIDTH: u32 = gfx::WIDTH / gfx::FONT.character_size.width;
const AVATAR_SIZE: Size = Size::new(150, 200);
const AVATAR_POINT: Point = Point::new(0, (gfx::HEIGHT - AVATAR_SIZE.height) as i32);

const SLOTH: ImageRaw<BinaryColor> =
    ImageRaw::new(include_bytes!("../../art/sloth.raw"), AVATAR_SIZE.width);

pub fn render<D: DrawTarget<Color = Rgb666>>(display: &mut D, dialogue: &mut Dialogue)
where
    <D as DrawTarget>::Error: fmt::Debug,
{
    if dialogue.clear_previous_text {
        Rectangle::new(
            Point::new(0, 0),
            Size::new(gfx::WIDTH, gfx::HEIGHT - AVATAR_SIZE.height),
        )
        .into_styled(gfx::BLACK_STYLE)
        .draw(display)
        .unwrap();
        dialogue.clear_previous_text = false;
    }

    // Render text
    let mut point = Point::new(0, 0);
    let Some((decoration, mut text)) = dialogue.text.get(dialogue.progress).copied() else {
        return;
    };

    // Render some type of decoration (or the text as headline)
    match decoration {
        Decoration::Blank => {}
        Decoration::Chapter => {
            Text::with_alignment(
                text,
                display.bounding_box().center(),
                gfx::CHAPTER_STYLE,
                Alignment::Center,
            )
            .draw(display)
            .unwrap();
            return;
        }
        Decoration::Sloth => {
            Image::new(&SLOTH, AVATAR_POINT, Rgb666::WHITE)
                .draw(display)
                .unwrap();
        }
    }

    // Render "normal" text
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
