use crate::{cooldown::Cooldown, gfx};
use core::fmt;
use embedded_graphics::{
    Drawable,
    draw_target::DrawTarget,
    mono_font::{MonoTextStyle, ascii::FONT_6X10},
    pixelcolor::{Rgb666, RgbColor},
    prelude::{Dimensions, Point, Primitive, Size},
    primitives::{PrimitiveStyle, PrimitiveStyleBuilder, Rectangle, StrokeAlignment},
    text::{Baseline, Text},
};

const NUM: u32 = 10;
const POINT: Point = Point::new(
    (gfx::WIDTH - (NUM * (RECT_SIZE + RECT_PAD as u32))) as i32 - RECT_PAD,
    (gfx::HEIGHT - RECT_SIZE) as i32 - RECT_PAD,
);
const RECT_SIZE: u32 = 20;
const RECT_PAD: i32 = 5;

const STYLE: MonoTextStyle<Rgb666> = MonoTextStyle::new(&FONT_6X10, Rgb666::WHITE);

const BORDER_STROKE: PrimitiveStyle<Rgb666> = PrimitiveStyleBuilder::new()
    .stroke_color(Rgb666::WHITE)
    .stroke_width(1)
    .stroke_alignment(StrokeAlignment::Inside)
    .build();

pub fn render<D: DrawTarget<Color = Rgb666>>(display: &mut D, num: usize, cd: &Cooldown)
where
    <D as DrawTarget>::Error: fmt::Debug,
{
    let mut buf = itoa::Buffer::new();
    let text = buf.format(num);

    let rect = Rectangle::new(
        POINT + Point::new((RECT_SIZE as i32 + RECT_PAD) * (num as i32), 0),
        Size::new(RECT_SIZE, RECT_SIZE),
    );

    rect.into_styled(BORDER_STROKE).draw(display).unwrap();

    Text::with_baseline(
        text,
        rect.bounding_box().center() - Point::new(2, 4),
        STYLE,
        Baseline::Top,
    )
    .draw(display)
    .unwrap();
}
