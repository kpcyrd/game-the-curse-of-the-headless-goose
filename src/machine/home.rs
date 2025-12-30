use crate::{
    input,
    machine::{Render, shop::Purchase},
    random::Rng,
};
use embedded_savegame::storage::Flash;

#[derive(Debug)]
pub struct Home {
    shop_items: Purchase,
}

impl Home {
    pub fn new(shop_unlocks: u8) -> Self {
        let shop_items = Purchase::from_unlock_level(shop_unlocks);
        Self { shop_items }
    }

    pub fn update<R: Rng, F: Flash>(
        &mut self,
        _rng: &mut R,
        campaign: &mut crate::machine::Campaign<F>,
        event: input::Event,
    ) -> Option<Render> {
        match event {
            input::Event::Star => {
                panic!("Fastline into usb mode");
            }
            input::Event::One => {
                campaign.open_shop(self.shop_items);
                Some(Render::Clear)
            }
            _ => None,
        }
    }
}
