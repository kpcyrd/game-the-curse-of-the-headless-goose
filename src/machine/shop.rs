use crate::{input, machine::Render, random::Rng};
use bitflags::bitflags;
use embedded_savegame::storage::Flash;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Purchase: u128 {
        const MOVE_ONE = 0b1 << 0;
        const MOVE_FOUR = 0b1 << 1;
        const MOVE_FIVE = 0b1 << 2;
        const MOVE_SIX = 0b1 << 3;
        const MOVE_SEVEN = 0b1 << 4;
        const MOVE_EIGHT = 0b1 << 5;
        const MOVE_NINE = 0b1 << 6;
    }
}

impl From<u128> for Purchase {
    fn from(value: u128) -> Self {
        Self::from_bits_truncate(value)
    }
}

impl Purchase {
    pub const fn price(&self) -> u16 {
        match *self {
            Purchase::MOVE_ONE => 50,
            Purchase::MOVE_FOUR => 100,
            Purchase::MOVE_FIVE => 200,
            Purchase::MOVE_SIX => 300,
            Purchase::MOVE_SEVEN => 400,
            Purchase::MOVE_EIGHT => 500,
            Purchase::MOVE_NINE => 600,
            _ => u16::MAX,
        }
    }

    pub const fn label(&self) -> &'static str {
        match *self {
            Purchase::MOVE_ONE => "New move: One (Special)",
            Purchase::MOVE_FOUR => "New move: Four (Strong attack)",
            Purchase::MOVE_FIVE => "New move: Five (Fast attack)",
            Purchase::MOVE_SIX => "New move: Six (Block)",
            Purchase::MOVE_SEVEN => "New move: Seven (Fast attack)",
            Purchase::MOVE_EIGHT => "New move: Eight (Block)",
            Purchase::MOVE_NINE => "New move: Nine (Strong attack)",
            _ => "",
        }
    }

    pub fn from_unlock_level(unlocks: u8) -> Self {
        let mut items = Purchase::empty();

        if unlocks >= 1 {
            items |= Purchase::MOVE_FOUR;
        }

        items
    }
}

#[derive(Debug)]
pub struct Shop {
    pub items: Purchase,
}

impl Shop {
    pub const fn new(items: Purchase) -> Self {
        Self { items }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_purchase_price() {
        assert_eq!(Purchase::MOVE_ONE.price(), 50);
        assert_eq!(Purchase::MOVE_FOUR.price(), 100);
        for purchase in Purchase::all().iter() {
            assert!(purchase.price() < u16::MAX);
        }
    }

    #[test]
    fn test_purchase_label() {
        assert_eq!(Purchase::MOVE_ONE.label(), "New move: One (Special)");
        assert_eq!(
            Purchase::MOVE_FOUR.label(),
            "New move: Four (Strong attack)"
        );
        for purchase in Purchase::all().iter() {
            assert!(!purchase.label().is_empty());
        }
    }
}
