use crate::{action::Move, fighter::Fighter, gfx};
use arrayvec::ArrayString;
use core::fmt;
use core::fmt::Write;
use embedded_graphics::{
    Drawable,
    draw_target::DrawTarget,
    pixelcolor::Rgb666,
    prelude::Point,
    text::{Baseline, Text},
};

// The longest possible string (technicaly not possible but still)
const CAP: usize = "Strong: Seven (99%) ".len();
const CHAR_WIDTH: i32 = gfx::TEXT_STYLE.font.character_size.width as i32;
const CHAR_HEIGHT: i32 = gfx::TEXT_STYLE.font.character_size.height as i32;

pub fn render<D: DrawTarget<Color = Rgb666>>(
    display: &mut D,
    point: Point,
    fighter: &Fighter,
    mv: Option<&Move>,
) where
    <D as DrawTarget>::Error: fmt::Debug,
{
    let point = Point::new(
        point.x.min(gfx::WIDTH as i32 - CAP as i32 * CHAR_WIDTH),
        point.y,
    );

    let mut text = ArrayString::<CAP>::new_const();
    for (i, (label, value)) in [("Health", fighter.health), ("Energy", fighter.energy)]
        .iter()
        .enumerate()
    {
        write!(&mut text, "{label}: {value}").unwrap();
        Text::with_baseline(
            &text,
            point + Point::new(0, i as i32 * CHAR_HEIGHT),
            gfx::TEXT_STYLE,
            Baseline::Top,
        )
        .draw(display)
        .unwrap();

        text.clear();
    }

    if let Some(mv) = mv {
        write!(&mut text, "{:?}: {:?}", mv.to_decision(), mv).unwrap();
        if let Some(cd) = fighter.cooldown.get(mv)
            && !cd.full()
        {
            let percent = (cd.value as u32 * 100) / cd.total as u32;
            write!(&mut text, " ({percent}%)").unwrap();
        }

        Text::with_baseline(
            &text,
            point + Point::new(0, 3 * CHAR_HEIGHT),
            gfx::TEXT_STYLE,
            Baseline::Top,
        )
        .draw(display)
        .unwrap();
    }
}
