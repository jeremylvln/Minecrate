use std::io;
use common::difficulty::Difficulty;

use crate::buffer::Buffer;
use crate::clientbound::ClientboundPacket;

#[derive(Debug)]
pub struct ServerDifficultyPacket {
    difficulty: Difficulty,
    locked: bool,
}

impl ServerDifficultyPacket {
    pub fn new(difficulty: Difficulty, locked: bool) -> ClientboundPacket {
        ClientboundPacket::ServerDifficulty(ServerDifficultyPacket {
            difficulty,
            locked,
        })
    }

    pub fn deserialize(buffer: &mut Buffer) -> io::Result<ClientboundPacket> {
        match Difficulty::from_u8(&buffer.read_ubyte()?) {
            Some(difficulty) => {
                Ok(ClientboundPacket::ServerDifficulty(ServerDifficultyPacket {
                    difficulty,
                    locked: buffer.read_bool()?,
                }))
            }
            None => Err(io::Error::new(io::ErrorKind::InvalidData, "Unknown difficulty"))
        }
    }

    pub fn serialize(&self, buffer: &mut Buffer) -> io::Result<()> {
        buffer.write_ubyte(&self.difficulty.to_u8())?;
        buffer.write_bool(&self.locked)?;
        Ok(())
    }
}