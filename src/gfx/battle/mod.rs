pub mod action;
pub mod stats;

use crate::{gfx, machine::battle::Battle};
use core::fmt;
use embedded_graphics::{draw_target::DrawTarget, pixelcolor::Rgb666, prelude::Point};

pub fn render<D: DrawTarget<Color = Rgb666>>(display: &mut D, battle: &Battle)
where
    <D as DrawTarget>::Error: fmt::Debug,
{
    stats::render(
        display,
        Point::new(10, gfx::HEIGHT as i32 - 65),
        &battle.player,
        None,
    );
    stats::render(
        display,
        Point::new(gfx::WIDTH as i32, 10),
        &battle.enemy,
        Some(&battle.their_move),
    );

    for (num, ability) in battle.player.cooldown.iter().enumerate() {
        if let Some(ability) = ability {
            action::render(display, num, ability);
        }
    }
}
