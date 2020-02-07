#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Gamemode {
    Survival, Creative, Adventure, Spectator
}

impl Gamemode {
    pub fn from_u8(value: &u8) -> Option<Self> {
        match value {
            0 => Some(Gamemode::Survival),
            1 => Some(Gamemode::Creative),
            2 => Some(Gamemode::Adventure),
            3 => Some(Gamemode::Spectator),
            _ => None
        }
    }

    pub fn to_u8(&self) -> u8 {
        match self {
            &Gamemode::Survival => 0,
            &Gamemode::Creative => 1,
            &Gamemode::Adventure => 2,
            &Gamemode::Spectator => 3,
        }
    }
}

impl Eq for Gamemode {}