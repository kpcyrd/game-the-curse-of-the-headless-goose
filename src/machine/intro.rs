use crate::{
    input,
    machine::{Campaign, Render},
    random::Rng,
};
use arrayvec::ArrayString;
use core::fmt::Write;
use embedded_savegame::storage::Flash;

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
    pub fn update<R: Rng, F: Flash>(
        &mut self,
        rng: &mut R,
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
                            campaign.start_game(rng);
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
                    campaign.start_game(rng);
                    Some(Render::Clear)
                }
                input::Event::Two => {
                    if self.has_save {
                        self.confirm_erase = Some(Erase::NewGame);
                        Some(Render::Redraw)
                    } else {
                        campaign.start_game(rng);
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
