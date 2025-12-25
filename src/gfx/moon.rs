use crate::{gfx, machine::moon::Moon};
use core::fmt;
use embedded_graphics::{
    Drawable, Pixel,
    draw_target::DrawTarget,
    image::{GetPixel, ImageRaw},
    pixelcolor::{BinaryColor, Rgb666},
    prelude::{Dimensions, Point, PointsIter, RgbColor, Size},
    primitives::{Rectangle, StyledDrawable},
    text::{Alignment, Text},
};
use embedded_graphics_colorcast::Image;

const MOON: ImageRaw<BinaryColor> = ImageRaw::new(include_bytes!("../../art/moon.raw"), 150);

const MOON_DELTA_POINT: Point = Point::new(0, -75);
const DIALOGUE_OFFSET: Point = Point::new(0, 25);
const HELP_HINT_OFFSET: Point = Point::new(0, 150);

pub fn render<D: DrawTarget<Color = Rgb666>>(display: &mut D, moon: &mut Moon)
where
    <D as DrawTarget>::Error: fmt::Debug,
{
    let center = display.bounding_box().center();

    let img = Image::new(&MOON, Point::new(0, 0), Rgb666::WHITE);
    let point =
        center - img.bounding_box().center() + MOON_DELTA_POINT - Point::new(0, moon.scroll);

    // the colorcast library currently only writes 'on' pixels, but we need both
    display
        .draw_iter(img.bounding_box().points().map(|offset| {
            if MOON.pixel(offset) == Some(BinaryColor::On) {
                Pixel(point + offset, Rgb666::WHITE)
            } else {
                Pixel(point + offset, Rgb666::BLACK)
            }
        }))
        .unwrap();

    // Wait until the animation is done, plus some delay
    if moon.animation_done() && moon.timer.is_due() {
        if let Some(line) = moon.line() {
            // Show current line of dialogue
            let text = Text::with_alignment(
                line,
                center + DIALOGUE_OFFSET,
                gfx::TEXT_STYLE,
                Alignment::Center,
            );

            // Clear space on the left
            let textbox = text.bounding_box();
            Rectangle::new(
                Point::new(0, textbox.top_left.y),
                Size::new(textbox.top_left.x as u32, textbox.size.height as u32),
            )
            .draw_styled(&gfx::BLACK_STYLE, display)
            .unwrap();

            // Render the middle text
            text.draw(display).unwrap();

            // Clear space on the right
            Rectangle::with_corners(
                textbox.top_left + textbox.size,
                Point::new(display.bounding_box().size.width as i32, textbox.top_left.y),
            )
            .draw_styled(&gfx::BLACK_STYLE, display)
            .unwrap();

            // Teach how to navigate cut-scenes
            Text::with_alignment(
                "Press # to continue",
                center + HELP_HINT_OFFSET,
                gfx::HINT_TEXT,
                Alignment::Center,
            )
            .draw(display)
            .unwrap();
        }
    }
}
