pub mod action;
pub mod stats;

use crate::{gfx, machine::battle::Battle};
use core::fmt;
use embedded_graphics::{draw_target::DrawTarget, pixelcolor::Rgb666, prelude::Point};

pub fn render<D: DrawTarget<Color = Rgb666>>(display: &mut D, battle: &Battle)
where
    <D as DrawTarget>::Error: fmt::Debug,
{
    // Render fighter stats
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
