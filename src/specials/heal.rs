use crate::{
    fighter::Fighter,
    specials::SpecialAbility,
    turn::{self, Step, Turn},
};
use log::info;

pub struct Heal;

impl SpecialAbility for Heal {
    fn apply(turn: &mut Turn, source: turn::Source, us: &mut Fighter) {
        info!("Healing!");
        turn.push(source, Step::RechargeHealth(1));
        us.recharge_health(1);
        turn.push(source, Step::RechargeEnergy(1));
        us.recharge_energy(1);
    }
}
