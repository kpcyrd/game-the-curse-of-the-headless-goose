use crate::{
    action::Move,
    fighter::{self, Fighter},
    input,
    machine::{Campaign, Render},
    random::Rng,
    turn::Turn,
};
use embedded_savegame::storage::Flash;

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
}

impl Battle {
    pub fn update<R: Rng, F: Flash>(
        &mut self,
        rng: &mut R,
        campaign: &mut Campaign<F>,
        event: input::Event,
    ) -> Option<Render> {
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

        fighter::apply_turn(
            rng,
            &mut self.player,
            &mut self.enemy,
            Some(&mv),
            Some(&self.their_move),
        );

        if self.player.defeated() {
            // We have been defeated! Game over.
            self.outcome = Some(Outcome::Lose);
            return Some(Render::Redraw);
        } else if self.enemy.defeated() {
            // They have been defeated! You win!
            self.outcome = Some(Outcome::Win);
            return Some(Render::Redraw);
        }

        // Recharge energy for both fighters
        for fighter in [&mut self.player, &mut self.enemy] {
            fighter.auto_recharge_energy();
        }

        self.their_move = self.enemy.random_move(rng);

        Some(Render::Redraw)
    }
}
