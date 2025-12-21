use crate::{
    action::{Decision, Move},
    cooldown::CooldownSet,
    random::Rng,
    specials::{self, Special},
};
use log::{debug, info};

pub fn turn<R: Rng>(
    rng: &mut R,
    us: &mut Fighter,
    them: &mut Fighter,
    mut our_move: Option<&Move>,
    mut their_move: Option<&Move>,
) {
    // Apply cooldown checks and rolls
    info!("Their turn (early)");
    them.roll_cooldown_check(rng, &mut their_move);
    info!("Our turn (early)");
    us.roll_cooldown_check(rng, &mut our_move);

    // Apply moves
    info!("Their turn");
    them.execute(us, their_move, our_move);
    if us.defeated() {
        // If we were defeated, end the round early
        return;
    }
    info!("Our turn");
    us.execute(them, our_move, their_move);
}

#[derive(Clone, Copy)]
pub struct Stats {
    pub health: u16,
    pub energy: u16,
    pub recharge: u16,
    pub cooldown: u8,
    pub abilities: u8,
}

impl Stats {
    // This doesn't make much sense, but let's us notice if we don't initialize properly
    pub const fn zero() -> Self {
        Self {
            health: 0,
            energy: 0,
            recharge: 0,
            cooldown: 0,
            abilities: 0,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Fighter {
    pub health: u16,
    pub max_health: u16,
    pub energy: u16,
    pub max_energy: u16,
    pub recharge: u16,
    pub special: Special,
    pub cooldown: CooldownSet,
}

impl Fighter {
    pub const fn new(stats: Stats) -> Self {
        Self {
            health: stats.health,
            max_health: stats.health,
            energy: stats.energy,
            max_energy: stats.energy,
            recharge: stats.recharge,
            special: specials::Special::Heal,
            cooldown: CooldownSet::new(stats.cooldown, stats.abilities as usize),
        }
    }

    pub fn random_move<R: Rng>(&self, rng: &mut R) -> Move {
        let mut cooldown_reroll = true;
        loop {
            let roll = rng.get_range(0..10);
            let mv = Move::from(roll);

            if let Some(cooldown) = self.cooldown.get(&mv) {
                // balance the odds of selecting moves on cooldown
                if !cooldown.full() && cooldown_reroll {
                    cooldown_reroll = false;
                    continue;
                }

                // if we can afford the move, select it
                if self.check_energy_cost(&mv).is_some() {
                    return mv;
                }
            }
        }
    }

    pub fn roll_cooldown_check<R: Rng>(&mut self, rng: &mut R, mv: &mut Option<&Move>) {
        if mv.take_if(|mv| !self.cooldown.attempt(rng, mv)).is_some() {
            debug!("Move is on cooldown, cannot execute!");
        }
    }

    pub fn execute(&mut self, other: &mut Self, mv: Option<&Move>, their_mv: Option<&Move>) {
        let Some(mv) = mv else {
            return;
        };

        // Apply the cost of the move first
        if !self.drain_energy(mv) {
            return;
        }

        // Check if it goes through
        if let Some(their_mv) = their_mv
            && other.check_energy_cost(their_mv).is_some()
            && their_mv.blocks(mv)
        {
            info!("Blocked!");
            return;
        }

        if mv.damage() > 0 {
            info!("Applying damage!");
            other.apply_damage(mv);
        }

        if mv.to_decision() == Decision::Special {
            let special = self.special;
            special.apply(self);
        }
    }

    pub const fn apply_damage(&mut self, mv: &Move) {
        self.health = self.health.saturating_sub(mv.damage());
    }

    pub const fn check_energy_cost(&self, mv: &Move) -> Option<u16> {
        self.energy.checked_sub(mv.damage())
    }

    pub const fn drain_energy(&mut self, mv: &Move) -> bool {
        if let Some(new_energy) = self.check_energy_cost(mv) {
            self.energy = new_energy;
            true
        } else {
            false
        }
    }

    pub const fn defeated(&self) -> bool {
        self.health == 0
    }

    pub fn auto_recharge_energy(&mut self) {
        self.recharge_energy(self.recharge);
        self.cooldown.increase();
    }

    pub fn recharge_energy(&mut self, value: u16) {
        self.energy = (self.energy.saturating_add(value)).min(self.max_energy);
    }

    pub fn recharge_health(&mut self, value: u16) {
        self.health = (self.health.saturating_add(value)).min(self.max_health);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::random::FastRandom;

    const FIGHTER: Fighter = Fighter::new(Stats {
        health: 10,
        energy: 10,
        recharge: 1,
        cooldown: 1,
        abilities: 10,
    });

    #[test]
    fn test_fighter_both_fast() {
        let mut fighter = FIGHTER.clone();
        let mut other = FIGHTER.clone();
        let mut rng = FastRandom::new();
        turn(
            &mut rng,
            &mut fighter,
            &mut other,
            Some(&Move::Three),
            Some(&Move::Two),
        );

        assert_eq!(
            fighter,
            Fighter {
                health: 8,
                max_health: 10,
                energy: 7,
                max_energy: 10,
                recharge: 1,
                special: Special::Heal,
                cooldown: CooldownSet::new(1, 10).and_consumed(&Move::Three),
            }
        );
        assert_eq!(
            other,
            Fighter {
                health: 7,
                max_health: 10,
                energy: 8,
                max_energy: 10,
                recharge: 1,
                special: Special::Heal,
                cooldown: CooldownSet::new(1, 10).and_consumed(&Move::Two),
            }
        );
    }

    #[test]
    fn test_fighter_fast_against_block() {
        let mut fighter = FIGHTER.clone();
        let mut other = FIGHTER.clone();
        let mut rng = FastRandom::new();
        turn(
            &mut rng,
            &mut fighter,
            &mut other,
            Some(&Move::Three),
            Some(&Move::Six),
        );

        assert_eq!(
            fighter,
            Fighter {
                health: 4,
                max_health: 10,
                energy: 7,
                max_energy: 10,
                recharge: 1,
                special: Special::Heal,
                cooldown: CooldownSet::new(1, 10).and_consumed(&Move::Three),
            }
        );
        assert_eq!(
            other,
            Fighter {
                health: 7,
                max_health: 10,
                energy: 4,
                max_energy: 10,
                recharge: 1,
                special: Special::Heal,
                cooldown: CooldownSet::new(1, 10).and_consumed(&Move::Six),
            }
        );
    }

    #[test]
    fn test_fighter_fast_against_strong() {
        let mut fighter = FIGHTER.clone();
        let mut other = FIGHTER.clone();
        let mut rng = FastRandom::new();
        turn(
            &mut rng,
            &mut fighter,
            &mut other,
            Some(&Move::Three),
            Some(&Move::Nine),
        );

        assert_eq!(
            fighter,
            Fighter {
                health: 10,
                max_health: 10,
                energy: 7,
                max_energy: 10,
                recharge: 1,
                special: Special::Heal,
                cooldown: CooldownSet::new(1, 10).and_consumed(&Move::Three),
            }
        );
        assert_eq!(
            other,
            Fighter {
                health: 7,
                max_health: 10,
                energy: 1,
                max_energy: 10,
                recharge: 1,
                special: Special::Heal,
                cooldown: CooldownSet::new(1, 10).and_consumed(&Move::Nine),
            }
        );
    }

    #[test]
    fn test_fighter_both_strong() {
        let mut fighter = FIGHTER.clone();
        let mut other = FIGHTER.clone();
        let mut rng = FastRandom::new();
        turn(
            &mut rng,
            &mut fighter,
            &mut other,
            Some(&Move::Nine),
            Some(&Move::Four),
        );

        assert_eq!(
            fighter,
            Fighter {
                health: 6,
                max_health: 10,
                energy: 1,
                max_energy: 10,
                recharge: 1,
                special: Special::Heal,
                cooldown: CooldownSet::new(1, 10).and_consumed(&Move::Nine),
            }
        );
        assert_eq!(
            other,
            Fighter {
                health: 1,
                max_health: 10,
                energy: 6,
                max_energy: 10,
                recharge: 1,
                special: Special::Heal,
                cooldown: CooldownSet::new(1, 10).and_consumed(&Move::Four),
            }
        );
    }

    #[test]
    fn test_fighter_strong_against_block() {
        let mut fighter = FIGHTER.clone();
        let mut other = FIGHTER.clone();
        let mut rng = FastRandom::new();
        turn(
            &mut rng,
            &mut fighter,
            &mut other,
            Some(&Move::Nine),
            Some(&Move::Six),
        );

        assert_eq!(
            fighter,
            Fighter {
                health: 10,
                max_health: 10,
                energy: 1,
                max_energy: 10,
                recharge: 1,
                special: Special::Heal,
                cooldown: CooldownSet::new(1, 10).and_consumed(&Move::Nine),
            }
        );
        assert_eq!(
            other,
            Fighter {
                health: 1,
                max_health: 10,
                energy: 4,
                max_energy: 10,
                recharge: 1,
                special: Special::Heal,
                cooldown: CooldownSet::new(1, 10).and_consumed(&Move::Six),
            }
        );
    }

    #[test]
    fn test_fighter_strong_against_fast() {
        let mut fighter = FIGHTER.clone();
        let mut other = FIGHTER.clone();
        let mut rng = FastRandom::new();
        turn(
            &mut rng,
            &mut fighter,
            &mut other,
            Some(&Move::Nine),
            Some(&Move::Three),
        );

        assert_eq!(
            fighter,
            Fighter {
                health: 7,
                max_health: 10,
                energy: 1,
                max_energy: 10,
                recharge: 1,
                special: Special::Heal,
                cooldown: CooldownSet::new(1, 10).and_consumed(&Move::Nine),
            }
        );
        assert_eq!(
            other,
            Fighter {
                health: 10,
                max_health: 10,
                energy: 7,
                max_energy: 10,
                recharge: 1,
                special: Special::Heal,
                cooldown: CooldownSet::new(1, 10).and_consumed(&Move::Three),
            }
        );
    }

    #[test]
    fn test_fighter_special_heal_against_block() {
        let mut fighter = FIGHTER.clone();
        fighter.health = 5;
        fighter.energy = 5;
        let mut other = FIGHTER.clone();
        let mut rng = FastRandom::new();
        turn(
            &mut rng,
            &mut fighter,
            &mut other,
            Some(&Move::One),
            Some(&Move::Six),
        );
        assert_eq!(
            fighter,
            Fighter {
                health: 6,
                max_health: 10,
                energy: 6,
                max_energy: 10,
                recharge: 1,
                special: Special::Heal,
                cooldown: CooldownSet::new(1, 10).and_consumed(&Move::One),
            }
        );
        assert_eq!(
            other,
            Fighter {
                health: 10,
                max_health: 10,
                energy: 4,
                max_energy: 10,
                recharge: 1,
                special: Special::Heal,
                cooldown: CooldownSet::new(1, 10).and_consumed(&Move::Six),
            }
        );
    }

    #[test]
    fn test_fighter_special_heal_against_fast() {
        let mut fighter = FIGHTER.clone();
        fighter.health = 5;
        fighter.energy = 5;
        let mut other = FIGHTER.clone();
        let mut rng = FastRandom::new();
        turn(
            &mut rng,
            &mut fighter,
            &mut other,
            Some(&Move::One),
            Some(&Move::Three),
        );
        assert_eq!(
            fighter,
            Fighter {
                health: 2,
                max_health: 10,
                energy: 5,
                max_energy: 10,
                recharge: 1,
                special: Special::Heal,
                cooldown: CooldownSet::new(1, 10).and_consumed(&Move::One),
            }
        );
        assert_eq!(
            other,
            Fighter {
                health: 10,
                max_health: 10,
                energy: 7,
                max_energy: 10,
                recharge: 1,
                special: Special::Heal,
                cooldown: CooldownSet::new(1, 10).and_consumed(&Move::Three),
            }
        );
    }

    #[test]
    fn test_fighter_cooldown_fail_cant_block() {
        let mut fighter = FIGHTER.clone();
        let mut other = FIGHTER.clone();

        let cd = other.cooldown.get_mut(&Move::Six).unwrap();
        cd.value = 0;
        cd.total = u8::MAX;
        let cd = other.cooldown.clone();

        let mut rng = FastRandom::new();
        turn(
            &mut rng,
            &mut fighter,
            &mut other,
            Some(&Move::Three),
            Some(&Move::Six),
        );

        assert_eq!(
            fighter,
            Fighter {
                health: 10,
                max_health: 10,
                energy: 7,
                max_energy: 10,
                recharge: 1,
                special: Special::Heal,
                cooldown: CooldownSet::new(1, 10).and_consumed(&Move::Three),
            }
        );
        assert_eq!(
            other,
            Fighter {
                health: 7,
                max_health: 10,
                energy: 10,
                max_energy: 10,
                recharge: 1,
                special: Special::Heal,
                cooldown: cd.and_consumed(&Move::Six),
            }
        );
    }
}
