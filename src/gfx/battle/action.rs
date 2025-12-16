use crate::{cooldown::Cooldown, gfx::{self, BLACK_STYLE}};
use core::fmt;
use embedded_graphics::{
    Drawable,
    draw_target::DrawTarget,
    mono_font::{MonoTextStyle, ascii::FONT_6X10},
    pixelcolor::{Rgb666, RgbColor},
    prelude::{Dimensions, Point, Primitive, Size, WebColors},
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
const RECT_INNER: u32 = RECT_SIZE - BORDER_STROKE.stroke_width * 2;

const STYLE_FULL: MonoTextStyle<Rgb666> = MonoTextStyle::new(&FONT_6X10, Rgb666::WHITE);
const STYLE_CHARGING: MonoTextStyle<Rgb666> = MonoTextStyle::new(&FONT_6X10, Rgb666::CSS_GRAY);

const BORDER_STROKE: PrimitiveStyle<Rgb666> = PrimitiveStyleBuilder::new()
    .stroke_color(Rgb666::WHITE)
    .stroke_width(1)
    .stroke_alignment(StrokeAlignment::Inside)
    .build();

const COOLDOWN_WIDTH: u32 = 3;
const COOLDOWN_STYLE: PrimitiveStyle<Rgb666> = PrimitiveStyleBuilder::new()
    .fill_color(Rgb666::CSS_ORANGE_RED)
    .build();

fn render_cooldown<D: DrawTarget<Color = Rgb666>>(display: &mut D, rect: &Rectangle, cd: &Cooldown)
where
    <D as DrawTarget>::Error: fmt::Debug,
{
    let cooldown_height = if !cd.full() {
        ((cd.value as u32 * RECT_INNER) / cd.total as u32).clamp(2, RECT_INNER)
    } else {
        0
    };

    for (delta, size, color) in [
        (
            Point::new(0, 0),
            Size::new(COOLDOWN_WIDTH, RECT_INNER - cooldown_height),
            BLACK_STYLE,
        ),
        (
            Point::new(0, (RECT_INNER - cooldown_height) as i32),
            Size::new(COOLDOWN_WIDTH, cooldown_height),
            COOLDOWN_STYLE,
        ),
    ] {
        let point = rect.top_left
            + Point::new(
                (RECT_SIZE - COOLDOWN_WIDTH - BORDER_STROKE.stroke_width) as i32,
                BORDER_STROKE.stroke_width as i32,
            )
            + delta;

        let cooldown_rect = Rectangle::new(point, size);
        cooldown_rect.into_styled(color).draw(display).unwrap();
    }
}

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
        rect.bounding_box().center() - Point::new(1, 4),
        if cd.full() {
            STYLE_FULL
        } else {
            STYLE_CHARGING
        },
        Baseline::Top,
    )
    .draw(display)
    .unwrap();

    render_cooldown(display, &rect, cd);
}
