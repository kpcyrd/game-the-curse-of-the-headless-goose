use crate::{
    action::Move,
    fighter::{self, Fighter},
    input,
    random::Rng,
};
use embedded_savegame::storage::{Flash, Storage};

#[derive(PartialEq, Eq)]
pub enum Render {
    Clear,
    Redraw,
}

const SLOT_SIZE: usize = 64;
// This is half of what we have available (512), but makes scanning faster
const SLOT_COUNT: usize = 256;

/// This holds the state of the higher-order game
pub struct Campaign<F: Flash> {
    flash: Storage<F, SLOT_SIZE, SLOT_COUNT>,
    pending_scene: Option<Scene>,
}

impl<F: Flash> Campaign<F> {
    pub const fn new(flash: Storage<F, SLOT_SIZE, SLOT_COUNT>) -> Self {
        Self {
            flash,
            pending_scene: None,
        }
    }

    pub fn start_game(&mut self) {
        let scene = Scene::Battle(Battle {
            player: {
                let mut us = Fighter::new(fighter::Stats {
                    health: 10,
                    energy: 10,
                    recharge: 1,
                    cooldown: 4,
                    abilities: 5,
                });
                us.cooldown.get_mut(&Move::Zero).unwrap().value = 0;
                us.cooldown.get_mut(&Move::Two).unwrap().value = 1;
                us.cooldown.get_mut(&Move::Three).unwrap().value = 2;
                us.cooldown.get_mut(&Move::Four).unwrap().value = 3;
                us
            },
            enemy: {
                let mut them = Fighter::new(fighter::Stats {
                    health: 30,
                    energy: 10,
                    recharge: 1,
                    cooldown: 2,
                    abilities: 10,
                });
                them.cooldown.get_mut(&Move::Five).unwrap().value = 1;
                them
            },
            their_move: Move::Five,
        });
        self.pending_scene = Some(scene);
    }
}

/// This holds the current scene of the game
pub enum Scene {
    Intro(Intro),
    Dialogue(Dialogue),
    Battle(Battle),
}

impl Scene {
    pub fn update<R: Rng, F: Flash>(
        &mut self,
        rng: &mut R,
        campaign: &mut Campaign<F>,
        event: input::Event,
    ) -> Option<Render> {
        let render = match self {
            Scene::Intro(intro) => intro.update(campaign, event),
            Scene::Dialogue(dialogue) => dialogue.update(campaign, event),
            Scene::Battle(battle) => battle.update(rng, campaign, event),
        };
        if let Some(pending) = campaign.pending_scene.take() {
            *self = pending;
        }
        render
    }

    pub fn tick(&mut self) {
        match self {
            Scene::Intro(_intro) => (),
            Scene::Dialogue(_dialogue) => (),
            Scene::Battle(_battle) => (),
        }
    }
}

#[derive(Default)]
pub struct Intro {
    pub confirm_erase: bool,
}

impl Intro {
    pub fn update<F: Flash>(
        &mut self,
        campaign: &mut Campaign<F>,
        event: input::Event,
    ) -> Option<Render> {
        if !self.confirm_erase {
            match event {
                input::Event::One => {
                    campaign.start_game();
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
                    // Erase all save data
                    campaign.flash.erase_all().unwrap();
                    self.confirm_erase = false;
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
    pub fn update<F: Flash>(
        &mut self,
        _campaign: &mut Campaign<F>,
        _event: input::Event,
    ) -> Option<Render> {
        None
    }
}

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
