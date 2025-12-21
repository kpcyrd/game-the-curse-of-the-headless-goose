use crate::{
    input,
    machine::{Campaign, Render},
    story::Decoration,
};
use embedded_savegame::storage::Flash;

pub struct Dialogue {
    pub text: &'static [(Decoration, &'static str)],
    pub progress: usize,
}

impl Dialogue {
    pub fn update<F: Flash>(
        &mut self,
        campaign: &mut Campaign<F>,
        event: input::Event,
    ) -> Option<Render> {
        if event == input::Event::Hash {
            self.progress = self.progress.saturating_add(1);
            if self.progress >= self.text.len() {
                campaign.progress_next();
            }
            Some(Render::Clear)
        } else {
            None
        }
    }
}
