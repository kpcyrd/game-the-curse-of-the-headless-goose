use crate::{
    action::Move,
    fighter::{self, Fighter},
    input,
    machine::{Campaign, Render},
    random::Rng,
};
use embedded_savegame::storage::Flash;

pub struct Battle {
    pub player: Fighter,
    pub enemy: Fighter,
    pub their_move: Move,
}

impl Battle {
    pub fn update<R: Rng, F: Flash>(
        &mut self,
        rng: &mut R,
        _campaign: &mut Campaign<F>,
        event: input::Event,
    ) -> Option<Render> {
        let mv = Move::from_input(event)?;

        // Ensure the move is unlocked
        self.player.cooldown.get(&mv)?;

        fighter::turn(
            rng,
            &mut self.player,
            &mut self.enemy,
            Some(&mv),
            Some(&self.their_move),
        );

        if self.player.defeated() {
            // println!("We have been defeated! Game over.");
            // TODO
            return Some(Render::Redraw);
        } else if self.enemy.defeated() {
            // println!("They have been defeated! You win!");
            // TODO
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
