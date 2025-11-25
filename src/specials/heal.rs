use crate::{fighter::Fighter, specials::SpecialAbility};

pub struct Heal;

impl SpecialAbility for Heal {
    fn apply(us: &mut Fighter) {
        println!("Healing!");
        us.recharge_health(1);
        us.recharge_energy(1);
    }
}
