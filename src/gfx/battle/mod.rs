pub mod action;
pub mod stats;

use crate::{
    gfx::{self, battle::stats::STATS_WIDTH},
    machine::battle::Battle,
};
use core::fmt;
use embedded_graphics::{
    Drawable,
    draw_target::DrawTarget,
    pixelcolor::Rgb666,
    prelude::Point,
    text::{Baseline, Text},
};

const PLAYER_STATS_POINT: Point = Point::new(10, gfx::HEIGHT as i32 - 65);
const ENEMY_STATS_POINT: Point = Point::new(gfx::WIDTH as i32 - STATS_WIDTH, 10);

const PLAYER_ACTIVITY_POINT: Point = Point::new(
    PLAYER_STATS_POINT.x + STATS_WIDTH - 15,
    PLAYER_STATS_POINT.y + 10,
);
const ENEMY_ACTIVITY_POINT: Point = Point::new(50, ENEMY_STATS_POINT.y + 5);

pub fn render<D: DrawTarget<Color = Rgb666>>(display: &mut D, battle: &Battle)
where
    <D as DrawTarget>::Error: fmt::Debug,
{
    // Render fighter stats
    stats::render(display, PLAYER_STATS_POINT, &battle.player, None);
    stats::render(
        display,
        ENEMY_STATS_POINT,
        &battle.enemy,
        Some(&battle.their_move),
    );

    // Render step activity
    Text::with_baseline(
        "asdf",
        PLAYER_ACTIVITY_POINT,
        gfx::GREEN_TEXT,
        Baseline::Top,
    )
    .draw(display)
    .unwrap();

    Text::with_baseline("foobar", ENEMY_ACTIVITY_POINT, gfx::RED_TEXT, Baseline::Top)
        .draw(display)
        .unwrap();

    // Render player abilities
    for (num, ability) in battle.player.cooldown.iter().enumerate() {
        if let Some(ability) = ability {
            action::render(display, num, ability);
        }
    }

    // Render upcoming turn step (if any)
    if let Some((source, step)) = &battle.upcoming_step {
        // turn::render_step(display, Point::new(0, gfx::HEIGHT as i32 / 2), *source, *step);
        let mut text = arrayvec::ArrayString::<128>::new_const();
        use core::fmt::Write;
        write!(&mut text, "{source:?} - {step:?}").ok();
        gfx::render_bold(display, &text);
    }

    // Render battle outcome (if any)
    if let Some(outcome) = &battle.outcome {
        gfx::render_bold(display, outcome.message());
    }
}
