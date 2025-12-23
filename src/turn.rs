use crate::{
    action::Move,
    fighter::Fighter,
    gfx::{self, glide},
};
use arrayvec::ArrayVec;
use core::fmt;
use embedded_graphics::{mono_font::MonoTextStyle, pixelcolor::Rgb666, prelude::Point};

// TODO: this is a random number for now
const MAX_STEPS: usize = 32;

#[derive(Default)]
pub struct Turn {
    array: ArrayVec<(Source, Step), MAX_STEPS>,
    current: usize,
    player_cooldown: Option<Move>,
    enemy_cooldown: Option<Move>,
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

    pub fn queue_cooldown(&mut self, source: Source, mv: Move) {
        match source {
            Source::Player => self.player_cooldown = Some(mv),
            Source::Enemy => self.enemy_cooldown = Some(mv),
        }
    }

    pub fn apply_cooldown(&self, player: &mut Fighter, enemy: &mut Fighter) {
        if let Some(mv) = self.player_cooldown {
            player.cooldown.consume(&mv);
        }
        if let Some(mv) = self.enemy_cooldown {
            enemy.cooldown.consume(&mv);
        }
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

    pub const fn to_point(&self) -> Point {
        match self {
            Source::Player => gfx::battle::PLAYER_ACTIVITY_POINT,
            Source::Enemy => gfx::battle::ENEMY_ACTIVITY_POINT,
        }
    }

    pub const fn to_glide_direction(&self) -> glide::Direction {
        match self {
            Source::Player => glide::Direction::Up,
            Source::Enemy => glide::Direction::Down,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Step {
    TakeDamage(Move),
    AttackFailed(Move),
    SpendEnergy(Move),
    RechargeHealth(u16),
    RechargeEnergy(u16),
    RollSuccess,
    RollFailed,
}

impl fmt::Display for Step {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Step::TakeDamage(mv) => write!(f, "DAMAGE +{}", mv.damage()),
            Step::AttackFailed(_) => write!(f, "MISS"),
            // This isn't really displayed anywhere
            Step::SpendEnergy(mv) => write!(f, "ENERGY -{}", mv.damage()),
            Step::RechargeHealth(amount) => write!(f, "HEALTH +{}", amount),
            Step::RechargeEnergy(amount) => write!(f, "ENERGY +{}", amount),
            Step::RollSuccess => write!(f, "ROLL SUCCESS"),
            Step::RollFailed => write!(f, "ROLL FAILED"),
        }
    }
}

impl Step {
    pub fn apply(&self, fighter: &mut Fighter) {
        match self {
            Step::TakeDamage(mv) => {
                fighter.apply_damage(mv);
            }
            Step::AttackFailed(_amount) => {}
            Step::SpendEnergy(mv) => {
                fighter.drain_energy(mv);
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

    pub const fn to_style(&self) -> Option<MonoTextStyle<'static, Rgb666>> {
        let style = match self {
            Self::TakeDamage(_) => gfx::RED_TEXT,
            Self::AttackFailed(_) => gfx::RED_TEXT,
            Self::SpendEnergy(_) => return None,
            Self::RechargeHealth(_) => gfx::GREEN_TEXT,
            Self::RechargeEnergy(_) => gfx::GREEN_TEXT,
            Self::RollSuccess => gfx::GREEN_TEXT,
            Self::RollFailed => gfx::RED_TEXT,
        };
        Some(style)
    }
}
