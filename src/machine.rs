use crate::{
    action::Move,
    fighter::{self, Fighter},
    input,
    random::Rng,
};

#[derive(PartialEq, Eq)]
pub enum Render {
    Clear,
    Redraw,
}

/// This holds the state of the higher-order game
pub struct Campaign {}

/// This holds the current scene of the game
pub enum Scene {
    Dialogue { text: &'static str },
    Battle(Battle),
}

impl Scene {
    pub fn update<R: Rng>(&mut self, rng: &mut R, event: input::Event) -> Option<Render> {
        match self {
            Scene::Dialogue { .. } => None,
            Scene::Battle(battle) => battle.update(rng, event),
        }
    }
}

pub struct Battle {
    pub player: Fighter,
    pub enemy: Fighter,
    pub their_move: Move,
}

impl Battle {
    pub fn update<R: Rng>(&mut self, rng: &mut R, event: input::Event) -> Option<Render> {
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
        self.their_move = self.enemy.random_move(rng);

        Some(Render::Redraw)
    }
}
