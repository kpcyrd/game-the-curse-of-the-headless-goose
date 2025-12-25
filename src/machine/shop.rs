use embedded_savegame::storage::Flash;

use crate::{input, machine::Render, random::Rng};

#[derive(Debug, Default)]
pub struct Shop {}

impl Shop {
    pub const fn new() -> Self {
        Self {}
    }

    pub fn update<R: Rng, F: Flash>(
        &mut self,
        rng: &mut R,
        campaign: &mut crate::machine::Campaign<F>,
        event: input::Event,
    ) -> Option<Render> {
        match event {
            input::Event::Star => {
                panic!("Fastline into usb mode");
            }
            input::Event::Hash => {
                campaign.pick_scene(rng);
                Some(Render::Clear)
            }
            _ => None,
        }
    }
}
