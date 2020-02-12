use common::difficulty::Difficulty;
use std::convert::TryFrom;
use std::io;

use crate::buffer::Buffer;
use crate::clientbound::ClientboundPacket;

#[derive(Debug)]
pub struct ServerDifficultyPacket {
    difficulty: Difficulty,
    locked: bool,
}

impl ServerDifficultyPacket {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(difficulty: Difficulty, locked: bool) -> ClientboundPacket {
        ClientboundPacket::ServerDifficulty(ServerDifficultyPacket { difficulty, locked })
    }

    pub fn deserialize(buffer: &mut Buffer) -> io::Result<ClientboundPacket> {
        match Difficulty::try_from(buffer.read_ubyte()?) {
            Ok(difficulty) => Ok(ClientboundPacket::ServerDifficulty(
                ServerDifficultyPacket {
                    difficulty,
                    locked: buffer.read_bool()?,
                },
            )),
            Err(error) => Err(io::Error::new(io::ErrorKind::InvalidData, error)),
        }
    }

    pub fn serialize(&self, buffer: &mut Buffer) -> io::Result<()> {
        buffer.write_ubyte(u8::from(self.difficulty))?;
        buffer.write_bool(self.locked)?;
        Ok(())
    }
}
