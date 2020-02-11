use std::convert::TryFrom;
use std::io;
use common::gamemode::Gamemode;
use common::dimension::Dimension;
use common::level_type::LevelType;

use crate::buffer::Buffer;
use crate::clientbound::ClientboundPacket;

#[derive(Debug)]
pub struct JoinGamePacket {
    entity_id: i32,
    gamemode: Gamemode,
    hardcore: bool,
    dimension: Dimension,
    hashed_seed: i64,
    max_players: u8,
    level_type: LevelType,
    render_distance: i32,
    reduced_debug_info: bool,
    enable_respawn_screen: bool,
}

impl JoinGamePacket {
    pub fn new(
        entity_id: i32, gamemode: Gamemode, hardcore: bool,
        dimension: Dimension, hashed_seed: i64, max_players: u8,
        level_type: LevelType, render_distance: i32, reduced_debug_info: bool,
        enable_respawn_screen: bool,
    ) -> ClientboundPacket {
        ClientboundPacket::JoinGame(JoinGamePacket {
            entity_id,
            gamemode,
            hardcore,
            dimension,
            hashed_seed,
            max_players,
            level_type,
            render_distance,
            reduced_debug_info,
            enable_respawn_screen,
        })
    }

    pub fn deserialize(buffer: &mut Buffer) -> io::Result<ClientboundPacket> {
        let entity_id = buffer.read_int()?;
        let mut gamemode_byte = buffer.read_ubyte()?;
        let mut hardcore = false;

        if gamemode_byte & (1 << 3) == 1 {
            hardcore = true;
            gamemode_byte ^= 0b00000100;
        }

        let dimension = Dimension::from(buffer.read_int()?);
        let hashed_seed = buffer.read_long()?;
        let max_players = buffer.read_ubyte()?;

        match Gamemode::try_from(gamemode_byte) {
            Ok(gamemode) => {
                match LevelType::from_string(&buffer.read_string()?) {
                    Some(level_type) => {
                        Ok(ClientboundPacket::JoinGame(JoinGamePacket {
                            entity_id,
                            gamemode,
                            hardcore,
                            dimension,
                            hashed_seed,
                            max_players,
                            level_type,
                            render_distance: buffer.read_varint()?,
                            reduced_debug_info: buffer.read_bool()?,
                            enable_respawn_screen: buffer.read_bool()?,
                        }))
                    },
                    None => Err(io::Error::new(io::ErrorKind::InvalidData, "Unknown level type"))
                }
            }
            Err(error) => Err(io::Error::new(io::ErrorKind::InvalidData, error))
        }
    }

    pub fn serialize(&self, buffer: &mut Buffer) -> io::Result<()> {
        let mut final_gamemode = u8::from(self.gamemode);

        if self.hardcore {
            final_gamemode |= 0b00000100;
        }

        buffer.write_int(self.entity_id)?;
        buffer.write_ubyte(final_gamemode)?;
        buffer.write_int(i32::from(self.dimension))?;
        buffer.write_long(self.hashed_seed)?;
        buffer.write_ubyte(self.max_players)?;
        buffer.write_string(self.level_type.to_string())?;
        buffer.write_varint(self.render_distance)?;
        buffer.write_bool(self.reduced_debug_info)?;
        buffer.write_bool(self.enable_respawn_screen)?;
        Ok(())
    }
}