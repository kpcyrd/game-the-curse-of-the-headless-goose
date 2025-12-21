pub mod battle;
pub mod dialogue;
pub mod intro;

use crate::{
    action::Move,
    fighter::{self, Fighter},
    input,
    machine::{battle::Battle, dialogue::Dialogue, intro::Intro},
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

// Maximum size we are willing to save/load
const SAVE_SIZE: usize = 256;

// Default stats for new games
const DEFAULT_HEALTH: u16 = 10;
const DEFAULT_ENERGY: u16 = 10;
const DEFAULT_RECHARGE: u16 = 1;
const DEFAULT_COOLDOWN: u8 = 4;
const DEFAULT_ABILITIES: u8 = 5;

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

    fn parse_u16(buf: &[u8], default: u16) -> (u16, &[u8]) {
        if let Some((value, buf)) = buf.split_at_checked(2) {
            let value = u16::from_be_bytes(value.try_into().unwrap());
            (value, buf)
        } else {
            (default, &[])
        }
    }

    fn parse_u8(buf: &[u8], default: u8) -> (u8, &[u8]) {
        if let Some((value, buf)) = buf.split_at_checked(1) {
            (value[0], buf)
        } else {
            (default, &[])
        }
    }

    pub fn start_game(&mut self) {
        let mut buf = [0u8; SAVE_SIZE];
        let buf = if let Some(slot) = &mut self.save_slot {
            // Load the savegame
            let loaded = self.flash.read(slot.idx, &mut buf).unwrap();
            // TODO: if this failed, don't silently discard
            loaded.map(|s| &*s).unwrap_or_default()
        } else {
            // Write an empty savegame
            // TODO: it may be enough to just write an empty slice, but no time to experiment right now
            self.flash.append(&mut [0, 0]).unwrap();
            &[]
        };

        let (progress, buf) = Self::parse_u16(buf, 0);
        let (money, buf) = Self::parse_u16(buf, 0);

        let (health, buf) = Self::parse_u16(buf, DEFAULT_HEALTH);
        let (energy, buf) = Self::parse_u16(buf, DEFAULT_ENERGY);
        let (recharge, buf) = Self::parse_u16(buf, DEFAULT_RECHARGE);
        let (cooldown, buf) = Self::parse_u8(buf, DEFAULT_COOLDOWN);
        let (abilities, _buf) = Self::parse_u8(buf, DEFAULT_ABILITIES);

        self.progress = progress;
        self.money = money;
        self.stats = fighter::Stats {
            health,
            energy,
            recharge,
            cooldown,
            abilities,
        };

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
