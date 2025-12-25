use crate::{input, machine::Render, random::Rng, timer::Timer};
use core::cmp;
use embedded_savegame::storage::Flash;

const SCROLL_TARGET: i32 = 50;
const SCROLL_INCREMENT: i32 = 2;

const TEXT_DELAY: u8 = 7;

#[derive(Debug)]
pub struct Moon {
    lines: &'static [&'static str],
    pub idx: usize,
    pub scroll: i32,
    pub timer: Timer,
    init_skip: bool,
}

impl Moon {
    pub const fn new(lines: &'static [&'static str]) -> Self {
        Self {
            lines,
            idx: 0,
            scroll: 0,
            timer: Timer::new(TEXT_DELAY),
            init_skip: false,
        }
    }

    pub fn line(&self) -> Option<&'static str> {
        self.lines.get(self.idx).copied()
    }

    pub fn animation_done(&self) -> bool {
        self.scroll >= SCROLL_TARGET
    }

    pub fn update<R: Rng, F: Flash>(
        &mut self,
        rng: &mut R,
        campaign: &mut crate::machine::Campaign<F>,
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
            input::Event::Hash if self.animation_done() => {
                self.idx = self.idx.saturating_add(1);

                if self.line().is_none() {
                    campaign.progress_next(rng);
                    return Some(Render::Clear);
                } else {
                    Some(Render::Redraw)
                }
            }
            _ => None,
        }
    }

    pub fn tick<R: Rng>(&mut self, _rng: &mut R, render: &mut Option<Render>) {
        if !self.animation_done() {
            self.scroll += SCROLL_INCREMENT;
            *render = cmp::max(*render, Some(Render::Redraw));
        } else if !self.timer.is_due() {
            self.timer.tick();
            *render = cmp::max(*render, Some(Render::Redraw));
        }
    }
}
