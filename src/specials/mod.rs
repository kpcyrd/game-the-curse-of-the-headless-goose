pub mod heal;

use crate::{
    fighter::Fighter,
    specials::heal::Heal,
    turn::{self, Turn},
};

trait SpecialAbility {
    fn apply(turn: &mut Turn, source: turn::Source, us: &mut Fighter);
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Special {
    Heal,
}

impl Special {
    pub fn apply(&self, turn: &mut Turn, source: turn::Source, us: &mut Fighter) {
        match self {
            Special::Heal => Heal::apply(turn, source, us),
        }
    }
}
