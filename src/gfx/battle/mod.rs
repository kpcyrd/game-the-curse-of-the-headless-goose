pub mod action;
pub mod stats;

use crate::{
    gfx::{self, battle::stats::STATS_WIDTH},
    machine::battle::Battle,
};
use core::fmt;
use embedded_graphics::{Drawable, draw_target::DrawTarget, pixelcolor::Rgb666, prelude::Point};

const PLAYER_STATS_POINT: Point = Point::new(10, gfx::HEIGHT as i32 - 65);
const ENEMY_STATS_POINT: Point = Point::new(gfx::WIDTH as i32 - STATS_WIDTH, 10);

// Adding -15 to x looks nice, but flickers because the chance-percentage text is so wide
pub const PLAYER_ACTIVITY_POINT: Point = Point::new(
    PLAYER_STATS_POINT.x + STATS_WIDTH,
    PLAYER_STATS_POINT.y + 20,
);
pub const ENEMY_ACTIVITY_POINT: Point = Point::new(50, ENEMY_STATS_POINT.y + 5);

pub const GLIDE_DISTANCE: u32 = 16;

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

    // Render turn activity
    if let Some(glider) = &battle.glider {
        glider.draw(display).unwrap();
    }

    // Render player abilities
    for (num, ability) in battle.player.cooldown.iter().enumerate() {
        if let Some(ability) = ability {
            action::render(display, num, ability);
        }
    }

    // Render battle outcome (if any)
    if let Some(outcome) = &battle.outcome {
        gfx::render_bold(display, outcome.message());
    }
}
