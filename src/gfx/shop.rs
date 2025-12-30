use crate::{
    gfx,
    machine::{Campaign, shop::Shop},
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

const PADDING: i32 = 30;
const NEXT_LINE: Point = Point::new(0, gfx::FONT.character_size.height as i32);

const fn align_right(point: Point, text: &str) -> Point {
    let align_right =
        gfx::WIDTH as i32 - PADDING - (gfx::FONT.character_size.width as i32 * text.len() as i32);
    Point::new(align_right, point.y)
}

pub fn render<D: DrawTarget<Color = Rgb666>, F: Flash>(
    display: &mut D,
    shop: &Shop,
    campaign: &Campaign<F>,
) where
    <D as DrawTarget>::Error: fmt::Debug,
{
    let mut point = Point::new(PADDING, PADDING);
    Text::with_baseline("SHOP", point, gfx::TEXT_STYLE, Baseline::Top)
        .draw(display)
        .unwrap();

    let mut buf = ArrayString::<20>::new();
    write!(&mut buf, "Funds: ${}", campaign.money()).unwrap();
    Text::with_baseline(
        &buf,
        align_right(point, &buf),
        gfx::TEXT_STYLE,
        Baseline::Top,
    )
    .draw(display)
    .unwrap();
    point += NEXT_LINE;

    for item in shop.items.iter() {
        point += NEXT_LINE;
        let label = item.label();
        Text::with_baseline(label, point, gfx::TEXT_STYLE, Baseline::Top)
            .draw(display)
            .unwrap();

        let price = item.price();
        let can_afford = campaign.money() >= price;
        let style = if can_afford {
            gfx::GREEN_TEXT
        } else {
            gfx::RED_TEXT
        };

        point += NEXT_LINE;
        let mut buf = ArrayString::<6>::new();
        write!(&mut buf, "${price}").unwrap();
        Text::with_baseline(&buf, align_right(point, &buf), style, Baseline::Top)
            .draw(display)
            .unwrap();
    }

    point += NEXT_LINE * 2;
    Text::with_baseline("#: Exit", point, gfx::TEXT_STYLE, Baseline::Top)
        .draw(display)
        .unwrap();
}
