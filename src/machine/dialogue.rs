use crate::{
    input,
    machine::{Campaign, Render},
};
use embedded_savegame::storage::Flash;

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
