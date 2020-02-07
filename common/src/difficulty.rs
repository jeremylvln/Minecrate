#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Difficulty {
    Peaceful, Easy, Normal, Hard
}

impl Difficulty {
    pub fn from_u8(value: &u8) -> Option<Self> {
        match value {
            0 => Some(Difficulty::Peaceful),
            1 => Some(Difficulty::Easy),
            2 => Some(Difficulty::Normal),
            3 => Some(Difficulty::Hard),
            _ => None
        }
    }

    pub fn to_u8(&self) -> u8 {
        match self {
            &Difficulty::Peaceful => 0,
            &Difficulty::Easy => 1,
            &Difficulty::Normal => 2,
            &Difficulty::Hard => 3,
        }
    }
}

impl Eq for Difficulty {}