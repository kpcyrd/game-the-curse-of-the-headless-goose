use crate::{fighter::Fighter, specials::SpecialAbility};
use log::info;

pub struct Heal;

impl SpecialAbility for Heal {
    fn apply(us: &mut Fighter) {
        info!("Healing!");
        us.recharge_health(1);
        us.recharge_energy(1);
    }
}
