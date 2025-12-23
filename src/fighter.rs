use crate::{
    action::{Decision, Move},
    cooldown::CooldownSet,
    random::Rng,
    specials::{self, Special},
    turn::{self, Step, Turn},
};
use log::{debug, info};

pub fn turn<R: Rng>(
    rng: &mut R,
    mut player: Fighter,
    mut enemy: Fighter,
    mut our_move: Option<&Move>,
    mut their_move: Option<&Move>,
) -> Turn {
    let mut turn = Turn::new();

    // Apply cooldown checks and rolls
    info!("Their turn (early)");
    enemy.roll_cooldown_check(rng, &mut turn, turn::Source::Enemy, &mut their_move);
    info!("Our turn (early)");
    player.roll_cooldown_check(rng, &mut turn, turn::Source::Player, &mut our_move);

    // Apply moves
    info!("Their turn");
    enemy.execute(
        &mut turn,
        turn::Source::Enemy,
        &mut player,
        their_move,
        our_move,
    );
    if player.defeated() {
        // If we were defeated, end the round early
        return turn;
    }
    info!("Our turn");
    player.execute(
        &mut turn,
        turn::Source::Player,
        &mut enemy,
        our_move,
        their_move,
    );

    // Done
    turn
}

// This function is only used in unit tests and the cli
pub fn apply_turn<R: Rng>(
    rng: &mut R,
    player: &mut Fighter,
    enemy: &mut Fighter,
    our_move: Option<&Move>,
    their_move: Option<&Move>,
) {
    let mut turn = turn(rng, player.clone(), enemy.clone(), our_move, their_move);

    while let Some((source, step)) = turn.next_step() {
        let fighter = match source {
            turn::Source::Player => &mut *player,
            turn::Source::Enemy => &mut *enemy,
        };
        step.apply(fighter);
    }

    turn.apply_cooldown(player, enemy);
}

#[derive(Clone, Copy)]
pub struct Stats {
    pub health: u16,
    pub energy: u16,
    pub recharge: u16,
    pub cooldown: u8,
    pub abilities: u16,
}

impl Stats {
    // This doesn't make much sense, but let's us notice if we don't initialize properly
    // Allows us to make Campaign::new() const fn
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
    pub const fn new(stats: &Stats) -> Self {
        Self {
            health: stats.health,
            max_health: stats.health,
            energy: stats.energy,
            max_energy: stats.energy,
            recharge: stats.recharge,
            special: specials::Special::Heal,
            cooldown: CooldownSet::new(stats.cooldown, stats.abilities),
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

    pub fn roll_cooldown_check<R: Rng>(
        &mut self,
        rng: &mut R,
        turn: &mut Turn,
        source: turn::Source,
        mv: &mut Option<&Move>,
    ) {
        let Some(cooldown) = mv.map(|mv| self.cooldown.get_mut(mv)).flatten() else {
            // Move is not available, discard
            // We don't record this as turn event because this illegal move should never happen
            *mv = None;
            return;
        };

        if cooldown.full() {
            // Everything is fine, no roll needed
            return;
        }

        let step = if cooldown.attempt(rng) {
            // Cooldown roll succeeded
            Step::RollSuccess
        } else {
            // Cooldown roll failed
            debug!("Move is on cooldown, cannot execute!");
            *mv = None;
            Step::RollFailed
        };
        turn.push(source, step);
    }

    pub fn execute(
        &mut self,
        turn: &mut Turn,
        source: turn::Source,
        other: &mut Self,
        mv: Option<&Move>,
        their_mv: Option<&Move>,
    ) {
        let Some(mv) = mv else {
            return;
        };

        // Apply the cost of the move first
        if !self.drain_energy(mv) {
            return;
        }
        turn.push(source, Step::SpendEnergy(*mv));

        // Check if it goes through
        if let Some(their_mv) = their_mv
            && other.check_energy_cost(their_mv).is_some()
            && their_mv.blocks(mv)
        {
            info!("Blocked!");
            turn.push(source, Step::AttackFailed(*mv));
            turn.queue_cooldown(source, *mv);
            return;
        }

        if mv.damage() > 0 {
            info!("Applying damage!");
            turn.push(source.other(), Step::TakeDamage(*mv));
            other.apply_damage(mv);
        }

        if mv.to_decision() == Decision::Special {
            let special = self.special;
            special.apply(turn, source, self);
        }

        turn.queue_cooldown(source, *mv);
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
    use crate::{cooldown::Cooldown, random::FastRandom};

    const FIGHTER: Fighter = Fighter::new(&Stats {
        health: 10,
        energy: 10,
        recharge: 1,
        cooldown: 1,
        abilities: ALL_ABILITIES,
    });
    const ALL_ABILITIES: u16 = 0b1111111111;

    #[test]
    fn test_abilities_5() {
        let fighter = Fighter::new(&Stats {
            health: 10,
            energy: 10,
            recharge: 1,
            cooldown: 1,
            abilities: 0b11111,
        });

        assert_eq!(
            fighter.cooldown,
            CooldownSet::from([
                Some(Cooldown::new(1)),
                Some(Cooldown::new(1)),
                Some(Cooldown::new(1)),
                Some(Cooldown::new(1)),
                Some(Cooldown::new(1)),
                None,
                None,
                None,
                None,
                None,
            ])
        );
    }

    #[test]
    fn test_abilities_all() {
        let fighter = Fighter::new(&Stats {
            health: 10,
            energy: 10,
            recharge: 1,
            cooldown: 1,
            abilities: 0b1111111111,
        });

        assert_eq!(
            fighter.cooldown,
            CooldownSet::from([Some(Cooldown::new(1)); 10])
        );
    }

    #[test]
    fn test_abilities_none() {
        let fighter = Fighter::new(&Stats {
            health: 10,
            energy: 10,
            recharge: 1,
            cooldown: 1,
            abilities: 0b0,
        });

        assert_eq!(fighter.cooldown, CooldownSet::from([None; 10]));
    }

    #[test]
    fn test_abilities_block_only() {
        let fighter = Fighter::new(&Stats {
            health: 10,
            energy: 10,
            recharge: 1,
            cooldown: 1,
            abilities: 0b101000001,
        });

        assert_eq!(
            fighter.cooldown,
            CooldownSet::from([
                Some(Cooldown::new(1)),
                None,
                None,
                None,
                None,
                None,
                Some(Cooldown::new(1)),
                None,
                Some(Cooldown::new(1)),
                None,
            ])
        );
    }

    #[test]
    fn test_fighter_both_fast() {
        let mut fighter = FIGHTER.clone();
        let mut other = FIGHTER.clone();
        let mut rng = FastRandom::new();
        apply_turn(
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
                cooldown: CooldownSet::new(1, ALL_ABILITIES).and_consumed(&Move::Three),
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
                cooldown: CooldownSet::new(1, ALL_ABILITIES).and_consumed(&Move::Two),
            }
        );
    }

    #[test]
    fn test_fighter_fast_against_block() {
        let mut fighter = FIGHTER.clone();
        let mut other = FIGHTER.clone();
        let mut rng = FastRandom::new();
        apply_turn(
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
                cooldown: CooldownSet::new(1, ALL_ABILITIES).and_consumed(&Move::Three),
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
                cooldown: CooldownSet::new(1, ALL_ABILITIES).and_consumed(&Move::Six),
            }
        );
    }

    #[test]
    fn test_fighter_fast_against_strong() {
        let mut fighter = FIGHTER.clone();
        let mut other = FIGHTER.clone();
        let mut rng = FastRandom::new();
        apply_turn(
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
                cooldown: CooldownSet::new(1, ALL_ABILITIES).and_consumed(&Move::Three),
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
                cooldown: CooldownSet::new(1, ALL_ABILITIES).and_consumed(&Move::Nine),
            }
        );
    }

    #[test]
    fn test_fighter_both_strong() {
        let mut fighter = FIGHTER.clone();
        let mut other = FIGHTER.clone();
        let mut rng = FastRandom::new();
        apply_turn(
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
                cooldown: CooldownSet::new(1, ALL_ABILITIES).and_consumed(&Move::Nine),
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
                cooldown: CooldownSet::new(1, ALL_ABILITIES).and_consumed(&Move::Four),
            }
        );
    }

    #[test]
    fn test_fighter_strong_against_block() {
        let mut fighter = FIGHTER.clone();
        let mut other = FIGHTER.clone();
        let mut rng = FastRandom::new();
        apply_turn(
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
                cooldown: CooldownSet::new(1, ALL_ABILITIES).and_consumed(&Move::Nine),
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
                cooldown: CooldownSet::new(1, ALL_ABILITIES).and_consumed(&Move::Six),
            }
        );
    }

    #[test]
    fn test_fighter_strong_against_fast() {
        let mut fighter = FIGHTER.clone();
        let mut other = FIGHTER.clone();
        let mut rng = FastRandom::new();
        apply_turn(
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
                cooldown: CooldownSet::new(1, ALL_ABILITIES).and_consumed(&Move::Nine),
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
                cooldown: CooldownSet::new(1, ALL_ABILITIES).and_consumed(&Move::Three),
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
        apply_turn(
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
                cooldown: CooldownSet::new(1, ALL_ABILITIES).and_consumed(&Move::One),
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
                cooldown: CooldownSet::new(1, ALL_ABILITIES).and_consumed(&Move::Six),
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
        apply_turn(
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
                cooldown: CooldownSet::new(1, ALL_ABILITIES).and_consumed(&Move::One),
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
                cooldown: CooldownSet::new(1, ALL_ABILITIES).and_consumed(&Move::Three),
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
        apply_turn(
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
                cooldown: CooldownSet::new(1, ALL_ABILITIES).and_consumed(&Move::Three),
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
