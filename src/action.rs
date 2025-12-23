use crate::input;
use core::fmt;

#[derive(PartialEq)]
pub enum Decision {
    Block,
    FastAttack,
    StrongAttack,
    Special,
}

impl fmt::Debug for Decision {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Decision::Block => write!(f, "Block"),
            Decision::FastAttack => write!(f, "Fast"),
            Decision::StrongAttack => write!(f, "Strong"),
            Decision::Special => write!(f, "Special"),
        }
    }
}

impl Decision {
    pub const fn blocks(&self, other: &Self) -> bool {
        // This is based on `heal`, but probably not true for all specials
        match self {
            Self::Block => matches!(other, Self::FastAttack | Self::Block),
            Self::FastAttack => matches!(other, Self::StrongAttack | Self::Special),
            Self::StrongAttack => matches!(other, Self::Block | Self::Special),
            Self::Special => matches!(other, Self::Block),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Move {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl Move {
    pub const fn to_decision(&self) -> Decision {
        match self {
            Move::Zero | Move::Six | Move::Eight => Decision::Block,
            Move::Two | Move::Three | Move::Five | Move::Seven => Decision::FastAttack,
            Move::Four | Move::Nine => Decision::StrongAttack,
            Move::One => Decision::Special,
        }
    }

    pub const fn damage(&self) -> u16 {
        match self {
            Move::Zero => 0,
            Move::One => 0,
            Move::Two => 2,
            Move::Three => 3,
            Move::Four => 4,
            Move::Five => 5,
            Move::Six => 6,
            Move::Seven => 7,
            Move::Eight => 8,
            Move::Nine => 9,
        }
    }

    pub const fn blocks(&self, other: &Move) -> bool {
        self.to_decision().blocks(&other.to_decision())
    }

    pub const fn from_input(event: input::Event) -> Option<Self> {
        match event {
            input::Event::Zero => Some(Move::Zero),
            input::Event::One => Some(Move::One),
            input::Event::Two => Some(Move::Two),
            input::Event::Three => Some(Move::Three),
            input::Event::Four => Some(Move::Four),
            input::Event::Five => Some(Move::Five),
            input::Event::Six => Some(Move::Six),
            input::Event::Seven => Some(Move::Seven),
            input::Event::Eight => Some(Move::Eight),
            input::Event::Nine => Some(Move::Nine),
            _ => None,
        }
    }
}

impl From<u8> for Move {
    fn from(value: u8) -> Self {
        match value % 10 {
            0 => Move::Zero,
            1 => Move::One,
            2 => Move::Two,
            3 => Move::Three,
            4 => Move::Four,
            5 => Move::Five,
            6 => Move::Six,
            7 => Move::Seven,
            8 => Move::Eight,
            _ => Move::Nine,
        }
    }
}

#[cfg(not(target_os = "none"))]
impl std::str::FromStr for Move {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value: u8 = s.parse()?;
        Ok(Move::from(value))
    }
}
