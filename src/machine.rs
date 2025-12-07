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
    Dialogue {
        text: &'static str,
    },
    Battle {
        player: Fighter,
        enemy: Fighter,
        their_move: Move,
    },
}

impl Scene {
    pub fn update<R: Rng>(&mut self, rng: &mut R, event: input::Event) -> Option<Render> {
        match self {
            Scene::Dialogue { .. } => None,
            Scene::Battle {
                player,
                enemy,
                their_move,
            } => {
                let mv = Move::from_input(event)?;

                // Ensure the move is unlocked
                player.cooldown.get(&mv)?;

                fighter::turn(rng, player, enemy, &mv, their_move);
                *their_move = enemy.random_move(rng);

                Some(Render::Redraw)
            }
        }
    }
}
