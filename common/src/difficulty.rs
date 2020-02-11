use std::convert::From;
use std::convert::TryFrom;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Difficulty {
    Peaceful, Easy, Normal, Hard
}

impl From<Difficulty> for u8 {
    fn from(value: Difficulty) -> Self {
        match value {
            Difficulty::Peaceful => 0,
            Difficulty::Easy => 1,
            Difficulty::Normal => 2,
            Difficulty::Hard => 3,
        }
    }
}

impl TryFrom<u8> for Difficulty {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Difficulty::Peaceful),
            1 => Ok(Difficulty::Easy),
            2 => Ok(Difficulty::Normal),
            3 => Ok(Difficulty::Hard),
            _ => Err("Unknown Difficulty")
        }
    }
}
