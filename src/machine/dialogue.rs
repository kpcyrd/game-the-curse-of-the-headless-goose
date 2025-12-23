use crate::{
    input,
    machine::{Campaign, Render},
    random::Rng,
    story::Decoration,
};
use embedded_savegame::storage::Flash;

pub struct Dialogue {
    pub text: &'static [(Decoration, &'static str)],
    pub progress: usize,
    pub clear_previous_text: bool,
    init_skip: bool,
}

impl Dialogue {
    pub const fn new(text: &'static [(Decoration, &'static str)]) -> Self {
        Self {
            text,
            progress: 0,
            clear_previous_text: false,
            init_skip: false,
        }
    }

    pub fn update<R: Rng, F: Flash>(
        &mut self,
        rng: &mut R,
        campaign: &mut Campaign<F>,
        event: input::Event,
    ) -> Option<Render> {
        match event {
            // Allow skipping dialogue by pressing star twice
            input::Event::Star if self.init_skip => {
                campaign.progress_next(rng);
                Some(Render::Clear)
            }
            input::Event::Star => {
                self.init_skip = true;
                None
            }
            _ if self.init_skip => {
                self.init_skip = false;
                None
            }
            // Advance dialogue on hash press
            input::Event::Hash => {
                let (current, _) = self.text.get(self.progress).unwrap();
                self.progress = self.progress.saturating_add(1);
                if let Some((next, _)) = self.text.get(self.progress) {
                    if next == current {
                        self.clear_previous_text = true;
                        Some(Render::Redraw)
                    } else {
                        Some(Render::Clear)
                    }
                } else {
                    campaign.progress_next(rng);
                    Some(Render::Clear)
                }
            }
            _ => None,
        }
    }
}
