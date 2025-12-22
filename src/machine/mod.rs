pub mod battle;
pub mod dialogue;
pub mod intro;

use crate::{
    fighter::{self, Fighter},
    input,
    machine::{battle::Battle, dialogue::Dialogue, intro::Intro},
    random::Rng,
    save::Save,
    story,
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

// Default stats for new games
const DEFAULT_HEALTH: u16 = 10;
const DEFAULT_ENERGY: u16 = 10;
const DEFAULT_RECHARGE: u16 = 1;
const DEFAULT_COOLDOWN: u8 = 4;
const DEFAULT_ABILITIES: u16 = 0b1101;
// const DEFAULT_ABILITIES: u16 = 0b11111; // First 5 abilities unlocked

/// This holds the state of the higher-order game
pub struct Campaign<F: Flash> {
    flash: Storage<F, SLOT_SIZE, SLOT_COUNT>,
    save_slot: Option<embedded_savegame::Slot>,
    progress: u16,
    money: u16,
    stats: fighter::Stats,
    pending_scene: Option<Scene>,
}

impl<F: Flash> Campaign<F> {
    pub const fn new(flash: Storage<F, SLOT_SIZE, SLOT_COUNT>) -> Self {
        Self {
            flash,
            save_slot: None,
            progress: 0,
            money: 0,
            stats: fighter::Stats::zero(),
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

    pub fn start_game<R: Rng>(&mut self, rng: &mut R) {
        let mut save = Save::new();

        if let Some(slot) = &mut self.save_slot {
            // Load the savegame
            // TODO: if this failed, don't silently discard
            if let Some(slice) = self.flash.read(slot.idx, &mut save.buf).unwrap() {
                let len = slice.len();
                save.reset(len);
            }
        } else {
            // Write an empty savegame
            // TODO: it may be enough to just write an empty slice, but no time to experiment right now
            self.flash.append(&mut [0, 0]).unwrap();
        }

        self.progress = save.pull_u16(0);
        self.money = save.pull_u16(0);

        self.stats = fighter::Stats {
            health: save.pull_u16(DEFAULT_HEALTH),
            energy: save.pull_u16(DEFAULT_ENERGY),
            recharge: save.pull_u16(DEFAULT_RECHARGE),
            cooldown: save.pull_u8(DEFAULT_COOLDOWN),
            abilities: save.pull_u16(DEFAULT_ABILITIES),
        };

        /*
        let scene = Scene::Battle(Battle {
            player: Fighter::new(self.stats),
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
        */

        self.pick_scene(rng);
    }

    fn pick_scene<R: Rng>(&mut self, rng: &mut R) {
        self.pending_scene = Some(
            if let Some(scene) = story::SCENES.get(self.progress as usize) {
                match scene {
                    story::Story::Dialogue(text) => Scene::Dialogue(Dialogue::new(text)),
                    story::Story::Battle(enemy) => {
                        let enemy = Fighter::new(*enemy);
                        let their_move = enemy.random_move(rng);

                        Scene::Battle(Battle {
                            player: Fighter::new(self.stats),
                            enemy,
                            their_move,
                        })
                    }
                }
            } else {
                Scene::Intro(self.intro())
            },
        );
    }

    fn write_save(&mut self) {
        let mut save = Save::new();
        save.push_u16(self.progress);
        save.push_u16(self.money);

        save.push_u16(self.stats.health);
        save.push_u16(self.stats.energy);
        save.push_u16(self.stats.recharge);
        save.push_u8(self.stats.cooldown);
        save.push_u16(self.stats.abilities);

        self.flash.append(save.slice()).unwrap();
    }

    pub fn progress_next<R: Rng>(&mut self, rng: &mut R) {
        self.progress = self.progress.saturating_add(1);
        self.write_save();
        self.pick_scene(rng);
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
            Scene::Intro(intro) => intro.update(rng, campaign, event),
            Scene::Dialogue(dialogue) => dialogue.update(rng, campaign, event),
            Scene::Battle(battle) => battle.update(rng, campaign, event),
        };
        if let Some(pending) = campaign.pending_scene.take() {
            *self = pending;
            Some(Render::Clear)
        } else {
            render
        }
    }

    pub fn tick(&mut self) {
        match self {
            Scene::Intro(_intro) => (),
            Scene::Dialogue(_dialogue) => (),
            Scene::Battle(_battle) => (),
        }
    }
}
