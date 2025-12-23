use crate::{
    action::Move,
    fighter::{self, Fighter},
    gfx::{
        self,
        glide::{Glider, TextGlider},
    },
    input,
    machine::{Campaign, Render},
    random::Rng,
    timer::Timer,
    turn::{self, Turn},
};
use core::cmp;
use embedded_graphics::text::Text;
use embedded_savegame::storage::Flash;

const TURN_STEP_DELAY: u8 = 1;

pub enum Outcome {
    Win,
    Lose,
}

impl Outcome {
    pub fn message(&self) -> &'static str {
        match self {
            Outcome::Win => "They have been defeated! You win!",
            Outcome::Lose => "We have been defeated!",
        }
    }
}

pub struct Battle {
    pub player: Fighter,
    pub enemy: Fighter,
    pub their_move: Move,
    pub turn: Option<Turn>,
    pub outcome: Option<Outcome>,
    pub glider: Option<TextGlider>,
    pub timer: Timer,
}

impl Battle {
    pub fn new<R: Rng>(rng: &mut R, player: &fighter::Stats, enemy: &fighter::Stats) -> Self {
        let enemy = Fighter::new(enemy);
        let their_move = enemy.random_move(rng);

        Battle {
            player: Fighter::new(player),
            enemy,
            their_move,
            turn: None,
            outcome: None,
            glider: None,
            timer: Timer::new(TURN_STEP_DELAY),
        }
    }

    pub fn update<R: Rng, F: Flash>(
        &mut self,
        rng: &mut R,
        campaign: &mut Campaign<F>,
        event: input::Event,
    ) -> Option<Render> {
        // TODO: This allows me to iterate faster, remove later
        if event == input::Event::Star {
            panic!("Fastline into usb mode");
        }

        // Check if the battle is already over
        if let Some(outcome) = &self.outcome {
            if event == input::Event::Hash {
                match outcome {
                    Outcome::Win => {
                        // Do nothing for now
                        campaign.progress_next(rng);
                    }
                    Outcome::Lose => {
                        // Restart the fight
                        // TODO: maybe offer more options here
                        campaign.pick_scene(rng);
                    }
                }
            }
            return None;
        }

        // Check if the move is valid
        let mv = Move::from_input(event)?;

        // Ensure the move is unlocked
        self.player.cooldown.get(&mv)?;

        // Resolve the move into a turn
        self.turn = Some(fighter::turn(
            rng,
            self.player.clone(),
            self.enemy.clone(),
            Some(&mv),
            Some(&self.their_move),
        ));

        None
    }

    pub fn tick<R: Rng>(&mut self, rng: &mut R, render: &mut Option<Render>) {
        let Some(turn) = &mut self.turn else {
            // Make the timer execute immediately as soon as a turn starts
            self.timer.set_due();
            return;
        };

        // Check timer
        if !self.timer.step() {
            // We do this just to slow down the timer
            *render = cmp::max(*render, Some(Render::Redraw));
            return;
        }

        if let Some(glider) = &mut self.glider {
            if !glider.step() {
                self.glider = None;
            } else {
                *render = cmp::max(*render, Some(Render::Redraw));
                return;
            }
        }

        // Execute the next step and start an animation if needed
        if let Some((source, step)) = turn.next_step() {
            // Apply the step
            let fighter = match source {
                turn::Source::Player => &mut self.player,
                turn::Source::Enemy => &mut self.enemy,
            };
            step.apply(fighter);

            // Setup some animation
            if let Some(style) = step.to_style() {
                self.glider = Some(Glider::new(
                    Text::new(step.to_str(), source.to_point(), style),
                    source.to_glide_direction(),
                    gfx::battle::GLIDE_DISTANCE,
                ));
            }
        } else {
            // Prepare next turn
            if let Some(turn) = &self.turn {
                turn.apply_cooldown(&mut self.player, &mut self.enemy);
            }

            // Recharge energy for both fighters
            for fighter in [&mut self.player, &mut self.enemy] {
                fighter.auto_recharge_energy();
            }

            self.their_move = self.enemy.random_move(rng);

            // End the current turn
            self.turn = None;
        }

        if self.glider.is_none() {
            // Check for defeat
            if self.player.defeated() {
                // We have been defeated! Game over.
                self.outcome = Some(Outcome::Lose);
            } else if self.enemy.defeated() {
                // They have been defeated! You win!
                self.outcome = Some(Outcome::Win);
            }
        }

        *render = cmp::max(*render, Some(Render::Redraw));
    }
}
