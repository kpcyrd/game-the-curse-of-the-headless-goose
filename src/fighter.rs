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
    our_move: &Move,
    their_move: &Move,
) {
    them.execute(rng, us, their_move, our_move);
    if us.defeated() {
        // If we were defeated, end the round early
        return;
    }
    us.execute(rng, them, our_move, their_move);
}

pub struct Stats {
    pub health: u16,
    pub energy: u16,
    pub recharge: u16,
    pub cooldown: u8,
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
            cooldown: CooldownSet::new(stats.cooldown),
        }
    }

    pub fn random_move<R: Rng>(&self, rng: &mut R) -> Move {
        loop {
            let roll = rng.get_range(0..10);
            let mv = Move::from(roll);
            if self.check_energy_cost(&mv).is_some() {
                return mv;
            }
        }
    }

    pub fn execute<R: Rng>(&mut self, rng: &mut R, other: &mut Self, mv: &Move, their_mv: &Move) {
        // Apply the cost of the move first
        if !self.drain_energy(mv) {
            return;
        }

        // Check cooldown of action
        if self.cooldown.get(mv).attempt(rng) {
            debug!("Move is on cooldown, cannot execute!");
            return;
        }

        // Check if it goes through
        if other.check_energy_cost(their_mv).is_some() && their_mv.blocks(mv) {
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

    const FIGHTER: Fighter = Fighter::new(Stats {
        health: 10,
        energy: 10,
        recharge: 1,
        cooldown: 1,
    });

    #[test]
    fn test_fighter_both_fast() {
        let mut fighter = FIGHTER.clone();
        let mut other = FIGHTER.clone();
        turn(&mut fighter, &mut other, &Move::Three, &Move::Two);

        assert_eq!(
            fighter,
            Fighter {
                health: 8,
                max_health: 10,
                energy: 7,
                max_energy: 10,
                recharge: 1,
                special: Special::Heal,
                cooldown: {
                    let mut cool = CooldownSet::new(1);
                    cool.get(&Move::Three).consume();
                    cool
                }
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
                cooldown: {
                    let mut cool = CooldownSet::new(1);
                    cool.get(&Move::Two).consume();
                    cool
                }
            }
        );
    }

    #[test]
    fn test_fighter_fast_against_block() {
        let mut fighter = FIGHTER.clone();
        let mut other = FIGHTER.clone();
        turn(&mut fighter, &mut other, &Move::Three, &Move::Six);

        assert_eq!(
            fighter,
            Fighter {
                health: 4,
                max_health: 10,
                energy: 7,
                max_energy: 10,
                recharge: 1,
                special: Special::Heal,
                cooldown: {
                    let mut cool = CooldownSet::new(1);
                    cool.get(&Move::Three).consume();
                    cool
                }
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
                cooldown: {
                    let mut cool = CooldownSet::new(1);
                    cool.get(&Move::Six).consume();
                    cool
                }
            }
        );
    }

    #[test]
    fn test_fighter_fast_against_strong() {
        let mut fighter = FIGHTER.clone();
        let mut other = FIGHTER.clone();
        turn(&mut fighter, &mut other, &Move::Three, &Move::Nine);

        assert_eq!(
            fighter,
            Fighter {
                health: 10,
                max_health: 10,
                energy: 7,
                max_energy: 10,
                recharge: 1,
                special: Special::Heal,
                cooldown: {
                    let mut cool = CooldownSet::new(1);
                    cool.get(&Move::Three).consume();
                    cool
                }
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
                cooldown: {
                    let mut cool = CooldownSet::new(1);
                    cool.get(&Move::Nine).consume();
                    cool
                }
            }
        );
    }

    #[test]
    fn test_fighter_both_strong() {
        let mut fighter = FIGHTER.clone();
        let mut other = FIGHTER.clone();
        turn(&mut fighter, &mut other, &Move::Nine, &Move::Four);

        assert_eq!(
            fighter,
            Fighter {
                health: 6,
                max_health: 10,
                energy: 1,
                max_energy: 10,
                recharge: 1,
                special: Special::Heal,
                cooldown: {
                    let mut cool = CooldownSet::new(1);
                    cool.get(&Move::Nine).consume();
                    cool
                }
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
                cooldown: {
                    let mut cool = CooldownSet::new(1);
                    cool.get(&Move::Four).consume();
                    cool
                }
            }
        );
    }

    #[test]
    fn test_fighter_strong_against_block() {
        let mut fighter = FIGHTER.clone();
        let mut other = FIGHTER.clone();
        turn(&mut fighter, &mut other, &Move::Nine, &Move::Six);

        assert_eq!(
            fighter,
            Fighter {
                health: 10,
                max_health: 10,
                energy: 1,
                max_energy: 10,
                recharge: 1,
                special: Special::Heal,
                cooldown: {
                    let mut cool = CooldownSet::new(1);
                    cool.get(&Move::Nine).consume();
                    cool
                }
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
                cooldown: {
                    let mut cool = CooldownSet::new(1);
                    cool.get(&Move::Six).consume();
                    cool
                }
            }
        );
    }

    #[test]
    fn test_fighter_strong_against_fast() {
        let mut fighter = FIGHTER.clone();
        let mut other = FIGHTER.clone();
        turn(&mut fighter, &mut other, &Move::Nine, &Move::Three);

        assert_eq!(
            fighter,
            Fighter {
                health: 7,
                max_health: 10,
                energy: 1,
                max_energy: 10,
                recharge: 1,
                special: Special::Heal,
                cooldown: {
                    let mut cool = CooldownSet::new(1);
                    cool.get(&Move::Nine).consume();
                    cool
                }
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
                cooldown: {
                    let mut cool = CooldownSet::new(1);
                    cool.get(&Move::Three).consume();
                    cool
                }
            }
        );
    }

    #[test]
    fn test_fighter_special_heal_against_block() {
        let mut fighter = FIGHTER.clone();
        fighter.health = 5;
        fighter.energy = 5;
        let mut other = FIGHTER.clone();
        turn(&mut fighter, &mut other, &Move::One, &Move::Six);
        assert_eq!(
            fighter,
            Fighter {
                health: 6,
                max_health: 10,
                energy: 6,
                max_energy: 10,
                recharge: 1,
                special: Special::Heal,
                cooldown: {
                    let mut cool = CooldownSet::new(1);
                    cool.get(&Move::One).consume();
                    cool
                }
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
                cooldown: {
                    let mut cool = CooldownSet::new(1);
                    cool.get(&Move::Six).consume();
                    cool
                }
            }
        );
    }

    #[test]
    fn test_fighter_special_heal_against_fast() {
        let mut fighter = FIGHTER.clone();
        fighter.health = 5;
        fighter.energy = 5;
        let mut other = FIGHTER.clone();
        turn(&mut fighter, &mut other, &Move::One, &Move::Three);
        assert_eq!(
            fighter,
            Fighter {
                health: 2,
                max_health: 10,
                energy: 5,
                max_energy: 10,
                recharge: 1,
                special: Special::Heal,
                cooldown: {
                    let mut cool = CooldownSet::new(1);
                    cool.get(&Move::One).consume();
                    cool
                }
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
                cooldown: {
                    let mut cool = CooldownSet::new(1);
                    cool.get(&Move::Three).consume();
                    cool
                }
            }
        );
    }
}
