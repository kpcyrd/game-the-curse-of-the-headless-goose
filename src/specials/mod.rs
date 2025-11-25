pub mod heal;

use crate::{fighter::Fighter, specials::heal::Heal};

trait SpecialAbility {
    fn apply(us: &mut Fighter);
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Special {
    Heal,
}

impl Special {
    pub fn apply(&self, us: &mut Fighter) {
        match self {
            Special::Heal => Heal::apply(us),
        }
    }
}
