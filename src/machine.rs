use crate::{
    action::Move,
    fighter::{self, Fighter},
    input,
    random::Rng,
};
use arrayvec::ArrayString;
use core::fmt::Write;
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
    save_slot: Option<embedded_savegame::Slot>,
    pending_scene: Option<Scene>,
}

impl<F: Flash> Campaign<F> {
    pub const fn new(flash: Storage<F, SLOT_SIZE, SLOT_COUNT>) -> Self {
        Self {
            flash,
            save_slot: None,
            pending_scene: None,
        }
    }

    pub fn intro(&mut self) -> Intro {
        self.save_slot = self.flash.scan().unwrap();
        Intro {
            has_save: self.save_slot.is_some(),
            confirm_erase: None,
            debug_save: None,
        }
    }

    pub fn start_game(&mut self) {
        if let Some(slot) = &mut self.save_slot {
            // TODO: do something with self.save_slot
        } else {
            self.flash.append(&mut [1, 3, 3, 7]).unwrap();
        }

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

#[derive(Clone, Copy)]
pub enum Erase {
    NewGame,
    Flash,
}

#[derive(Default)]
pub struct Intro {
    pub has_save: bool,
    pub confirm_erase: Option<Erase>,
    pub debug_save: Option<ArrayString<85>>,
}

impl Intro {
    pub fn update<F: Flash>(
        &mut self,
        campaign: &mut Campaign<F>,
        event: input::Event,
    ) -> Option<Render> {
        if let Some(_save_info) = &self.debug_save {
            if event == input::Event::Hash {
                self.debug_save = None;
                Some(Render::Clear)
            } else {
                None
            }
        } else if let Some(erase) = self.confirm_erase {
            match event {
                input::Event::Star => {
                    // Cancel erase
                    self.confirm_erase = None;
                    Some(Render::Redraw)
                }
                input::Event::Hash => {
                    // Erase discovered save
                    campaign.save_slot = None;
                    self.has_save = false;
                    self.confirm_erase = None;

                    match erase {
                        Erase::NewGame => {
                            // Start the game after we removed the discovered save
                            campaign.start_game();
                            Some(Render::Clear)
                        }
                        Erase::Flash => {
                            // Clear flash
                            campaign.flash.erase_all().unwrap();
                            Some(Render::Redraw)
                        }
                    }
                }
                _ => None,
            }
        } else {
            match event {
                input::Event::One => {
                    campaign.start_game();
                    Some(Render::Clear)
                }
                input::Event::Two => {
                    if self.has_save {
                        self.confirm_erase = Some(Erase::NewGame);
                        Some(Render::Redraw)
                    } else {
                        campaign.start_game();
                        Some(Render::Clear)
                    }
                }
                input::Event::Three => {
                    // Show erase menu
                    self.confirm_erase = Some(Erase::Flash);
                    Some(Render::Redraw)
                }
                input::Event::Seven => {
                    let mut save_info = ArrayString::<85>::new();
                    write!(&mut save_info, "{:?}", campaign.save_slot).ok();
                    self.debug_save = Some(save_info);
                    Some(Render::Clear)
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
