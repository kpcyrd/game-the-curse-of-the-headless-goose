use crate::{action::Move, fighter::Fighter};
use arrayvec::ArrayVec;

// TODO: this is a random number for now
const MAX_STEPS: usize = 32;

#[derive(Default)]
pub struct Turn {
    array: ArrayVec<(Source, Step), MAX_STEPS>,
    current: usize,
}

impl Turn {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, source: Source, step: Step) {
        // XXX: silently discard, bugs are easier to spot if we panic here
        self.array.try_push((source, step)).ok();
    }

    pub fn next_step(&mut self) -> Option<&(Source, Step)> {
        let item = self.array.get(self.current);
        self.current = self.current.saturating_add(1);
        item
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Source {
    Player,
    Enemy,
}

impl Source {
    pub const fn other(&self) -> Self {
        match self {
            Source::Player => Source::Enemy,
            Source::Enemy => Source::Player,
        }
    }
}

pub enum Step {
    TakeDamage(Move),
    AttackFailed(Move),
    SpendEnergy(Move),
    SetCooldown(Move),
    RechargeHealth(u16),
    RechargeEnergy(u16),
    RollSuccess,
    RollFailed,
}

impl Step {
    pub fn apply(&self, fighter: &mut Fighter) {
        match self {
            Step::TakeDamage(mv) => {
                fighter.apply_damage(&mv);
            }
            Step::AttackFailed(_amount) => {}
            Step::SpendEnergy(mv) => {
                fighter.drain_energy(mv);
            }
            Step::SetCooldown(mv) => {
                fighter.cooldown.consume(mv);
            }
            Step::RechargeHealth(amount) => {
                fighter.recharge_health(*amount);
            }
            Step::RechargeEnergy(amount) => {
                fighter.recharge_energy(*amount);
            }
            Step::RollSuccess => {}
            Step::RollFailed => {}
        }
    }
}
