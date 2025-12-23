use crate::{
    action::Move,
    fighter::{self, Fighter},
    input,
    machine::{Campaign, Render},
    random::Rng,
    timer::Timer,
    turn::{self, Turn},
};
use core::cmp;
use embedded_savegame::storage::Flash;

const TURN_STEP_DELAY: u8 = u8::MAX;

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
    pub upcoming_step: Option<(turn::Source, turn::Step)>,
    pub outcome: Option<Outcome>,
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
            upcoming_step: None,
            outcome: None,
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
            return;
        }

        // Execute the upcoming step we've render for a moment
        if let Some((source, step)) = self.upcoming_step.take() {
            let fighter = match source {
                turn::Source::Player => &mut self.player,
                turn::Source::Enemy => &mut self.enemy,
            };
            step.apply(fighter);
        } else if let Some((source, step)) = turn.next_step() {
            // Select the next step for rendering
            self.upcoming_step = Some((*source, *step));
        } else {
            // Prepare next turn

            // Recharge energy for both fighters
            for fighter in [&mut self.player, &mut self.enemy] {
                fighter.auto_recharge_energy();
            }

            self.their_move = self.enemy.random_move(rng);

            // End the current turn
            self.turn = None;
        }

        // Check for defeat
        if self.player.defeated() {
            // We have been defeated! Game over.
            self.outcome = Some(Outcome::Lose);
        } else if self.enemy.defeated() {
            // They have been defeated! You win!
            self.outcome = Some(Outcome::Win);
        }

        *render = cmp::max(*render, Some(Render::Redraw));
    }
}
