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
    Intro(Intro),
    Dialogue(Dialogue),
    Battle(Battle),
}

impl Scene {
    pub fn update<R: Rng>(&mut self, rng: &mut R, event: input::Event) -> Option<Render> {
        match self {
            Scene::Intro(intro) => intro.update(event),
            Scene::Dialogue(dialogue) => dialogue.update(event),
            Scene::Battle(battle) => battle.update(rng, event),
        }
    }
}

#[derive(Default)]
pub struct Intro {
    pub confirm_erase: bool,
}

impl Intro {
    pub fn update(&mut self, event: input::Event) -> Option<Render> {
        if !self.confirm_erase {
            match event {
                input::Event::One => {
                    // TODO: Start the game
                    Some(Render::Clear)
                }
                input::Event::Two => {
                    // Show erase menu
                    self.confirm_erase = true;
                    Some(Render::Redraw)
                }
                _ => None,
            }
        } else {
            match event {
                input::Event::Star => {
                    // Cancel erase
                    self.confirm_erase = false;
                    Some(Render::Redraw)
                }
                input::Event::Hash => {
                    // TODO: Erase save data
                    Some(Render::Redraw)
                }
                _ => None,
            }
        }
    }
}

pub struct Dialogue {
    pub text: &'static str,
}

impl Dialogue {
    pub fn update(&mut self, _event: input::Event) -> Option<Render> {
        None
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
