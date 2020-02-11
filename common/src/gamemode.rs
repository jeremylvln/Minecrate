use std::convert::From;
use std::convert::TryFrom;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Gamemode {
    Survival, Creative, Adventure, Spectator
}

impl From<Gamemode> for u8 {
    fn from(value: Gamemode) -> Self {
        match value {
            Gamemode::Survival => 0,
            Gamemode::Creative => 1,
            Gamemode::Adventure => 2,
            Gamemode::Spectator => 3,
        }
    }
}

impl TryFrom<u8> for Gamemode {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Gamemode::Survival),
            1 => Ok(Gamemode::Creative),
            2 => Ok(Gamemode::Adventure),
            3 => Ok(Gamemode::Spectator),
            _ => Err("Unknown Gamemode")
        }
    }
}
