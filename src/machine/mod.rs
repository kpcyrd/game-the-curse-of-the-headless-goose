pub mod battle;
pub mod dialogue;
pub mod home;
pub mod intro;
pub mod moon;
pub mod shop;

use crate::{
    fighter, input,
    machine::{
        battle::Battle, dialogue::Dialogue, home::Home, intro::Intro, moon::Moon, shop::Shop,
    },
    random::Rng,
    save::Save,
    story,
};
use embedded_savegame::storage::{Flash, Storage};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Render {
    Redraw,
    Clear,
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
    purchased: shop::Purchase,
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
            purchased: shop::Purchase::empty(),
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

        self.purchased = shop::Purchase::from(save.pull_u128(0));

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
                    story::Story::Moon(lines) => Scene::Moon(Moon::new(lines)),
                    story::Story::Dialogue(text) => Scene::Dialogue(Dialogue::new(text)),
                    story::Story::Battle { enemy, reward } => Scene::Battle(Battle::new(
                        rng,
                        &self.stats,
                        enemy,
                        *reward,
                        battle::Resolution::ProgressCampaign,
                    )),
                    story::Story::Home { shop_unlocks } => Scene::Home(Home::new(*shop_unlocks)),
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

        save.push_u128(self.purchased.bits());

        self.flash.append(save.slice()).unwrap();
    }

    pub fn progress_next<R: Rng>(&mut self, rng: &mut R) {
        self.progress = self.progress.saturating_add(1);
        self.write_save();
        self.pick_scene(rng);
    }

    pub fn fight_won<R: Rng>(&mut self, rng: &mut R, battle: &Battle) {
        // depending on what has been configured, we may return to the current scene or progress
        match battle.resolution {
            battle::Resolution::ProgressCampaign => {
                self.progress_next(rng);
            }
            battle::Resolution::Return => {
                self.write_save();
                self.pick_scene(rng);
            }
        }
    }

    pub fn open_shop(&mut self, items: shop::Purchase) {
        self.pending_scene = Some(Scene::Shop(Shop::new(items)));
    }

    pub fn add_purchase(&mut self, purchase: shop::Purchase) -> bool {
        if self.purchased.contains(purchase) {
            return false;
        }

        let price = purchase.price();
        let Some(remaining) = self.money.checked_sub(price) else {
            return false;
        };
        self.money = remaining;
        self.purchased.insert(purchase);
        self.write_save();

        true
    }

    #[inline(always)]
    pub const fn money(&self) -> u16 {
        self.money
    }
}

/// This holds the current scene of the game
#[allow(clippy::large_enum_variant)]
pub enum Scene {
    Intro(Intro),
    Moon(Moon),
    Dialogue(Dialogue),
    Battle(Battle),
    Home(Home),
    Shop(Shop),
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
            Scene::Moon(moon) => moon.update(rng, campaign, event),
            Scene::Dialogue(dialogue) => dialogue.update(rng, campaign, event),
            Scene::Battle(battle) => battle.update(rng, campaign, event),
            Scene::Home(home) => home.update(rng, campaign, event),
            Scene::Shop(shop) => shop.update(rng, campaign, event),
        };
        if let Some(pending) = campaign.pending_scene.take() {
            *self = pending;
            Some(Render::Clear)
        } else {
            render
        }
    }

    pub fn tick<R: Rng>(&mut self, rng: &mut R, render: &mut Option<Render>) {
        match self {
            Scene::Intro(_intro) => (),
            Scene::Moon(moon) => moon.tick(rng, render),
            Scene::Dialogue(_dialogue) => (),
            Scene::Battle(battle) => battle.tick(rng, render),
            Scene::Home(_home) => (),
            Scene::Shop(_shop) => (),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type MockFlash = embedded_savegame::mock::MockFlash<512>;

    #[test]
    fn test_sort_render() {
        assert!(Render::Clear > Render::Redraw);
    }

    #[test]
    fn test_purchase_success() {
        let flash = MockFlash::new();
        let mut campaign = Campaign::new(Storage::new(flash));
        campaign.money = u16::MAX;

        assert!(campaign.add_purchase(shop::Purchase::MOVE_FOUR));
        assert_eq!(campaign.purchased, shop::Purchase::MOVE_FOUR);

        assert!(campaign.add_purchase(shop::Purchase::MOVE_ONE));
        assert_eq!(
            campaign.purchased,
            shop::Purchase::MOVE_FOUR | shop::Purchase::MOVE_ONE
        );
    }

    #[test]
    fn test_purchase_fail() {
        let flash = MockFlash::new();
        let mut campaign = Campaign::new(Storage::new(flash));
        assert!(!campaign.add_purchase(shop::Purchase::MOVE_ONE));
        assert_eq!(campaign.purchased, shop::Purchase::empty());
    }
}
