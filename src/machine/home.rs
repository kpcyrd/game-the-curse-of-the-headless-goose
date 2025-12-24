use embedded_savegame::storage::Flash;

use crate::{input, machine::Render, random::Rng};

#[derive(Debug, Default)]
pub struct Home {}

impl Home {
    pub const fn new() -> Self {
        Self {}
    }

    pub fn update<R: Rng, F: Flash>(
        &mut self,
        _rng: &mut R,
        _campaign: &mut crate::machine::Campaign<F>,
        event: input::Event,
    ) -> Option<Render> {
        if event == input::Event::Star {
            panic!("Fastline into usb mode");
        }

        // Home menu logic goes here

        Some(Render::Redraw)
    }
}
