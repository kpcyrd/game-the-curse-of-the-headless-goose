use crate::{
    gfx,
    machine::{Campaign, hq::Hq},
};
use arrayvec::ArrayString;
use core::fmt::{self, Write};
use embedded_graphics::{
    Drawable,
    draw_target::DrawTarget,
    pixelcolor::Rgb666,
    prelude::Point,
    text::{Baseline, Text},
};
use embedded_savegame::storage::Flash;

pub fn render<D: DrawTarget<Color = Rgb666>, F: Flash>(
    display: &mut D,
    _hq: &Hq,
    campaign: &Campaign<F>,
) where
    <D as DrawTarget>::Error: fmt::Debug,
{
    let mut buf = ArrayString::<20>::new();
    write!(&mut buf, "Funds: ${}", campaign.money()).unwrap();

    let mut point = Point::new(50, 100);
    Text::with_baseline(
        "HQ SCENE PLACEHOLDER",
        point,
        gfx::TEXT_STYLE,
        Baseline::Top,
    )
    .draw(display)
    .unwrap();

    point.y += gfx::FONT.character_size.height as i32 * 2;
    Text::with_baseline(&buf, point, gfx::TEXT_STYLE, Baseline::Top)
        .draw(display)
        .unwrap();
}
