use core::fmt;

use crate::{action::Move, random::Rng};

#[derive(Clone, Copy, PartialEq, Default)]
pub struct Cooldown {
    pub value: u8,
    pub total: u8,
}

impl Cooldown {
    pub const fn new(duration: u8) -> Self {
        Self {
            value: duration,
            total: duration,
        }
    }

    pub fn increase(&mut self) {
        if self.value < self.total {
            self.value += 1;
        }
    }

    pub fn consume(&mut self) {
        self.value = 0;
    }

    pub fn attempt<R: Rng>(&mut self, rng: &mut R) -> bool {
        if self.value == self.total {
            self.consume();
            true
        } else {
            let chance = u8::MAX
                .saturating_div(self.total)
                .saturating_mul(self.value);
            self.consume();
            let roll = rng.get();
            roll >= chance
        }
    }
}

impl fmt::Debug for Cooldown {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Cooldown {{ {}/{} }}", self.value, self.total)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CooldownSet {
    values: [Cooldown; 10],
}

impl CooldownSet {
    pub const fn new(duration: u8) -> Self {
        CooldownSet {
            values: [Cooldown::new(duration); _],
        }
    }

    pub fn get(&mut self, mv: &Move) -> &mut Cooldown {
        match mv {
            Move::Zero => &mut self.values[0],
            Move::One => &mut self.values[1],
            Move::Two => &mut self.values[2],
            Move::Three => &mut self.values[3],
            Move::Four => &mut self.values[4],
            Move::Five => &mut self.values[5],
            Move::Six => &mut self.values[6],
            Move::Seven => &mut self.values[7],
            Move::Eight => &mut self.values[8],
            Move::Nine => &mut self.values[9],
        }
    }

    pub fn increase(&mut self) {
        for cooldown in &mut self.values {
            cooldown.increase();
        }
    }
}
