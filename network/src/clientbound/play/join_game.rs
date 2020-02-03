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

    // pub fn deserialize(buffer: &mut Buffer) -> io::Result<ClientboundPacket> {
    //     match Uuid::parse_str(&buffer.read_string()?) {
    //         Ok(uuid) => Ok(ClientboundPacket::JoinGame(JoinGamePacket {
    //             uuid,
    //             username: buffer.read_string()?,
    //         })),
    //         Err(e) => Err(io::Error::new(io::ErrorKind::InvalidData, e))
    //     }
    // }

    pub fn serialize(&self, buffer: &mut Buffer) -> io::Result<()> {
        let mut final_gamemode = self.gamemode.to_u8();

        if self.hardcore {
            final_gamemode &= 0b00000100;
        }

        buffer.write_int(&self.entity_id)?;
        buffer.write_ubyte(&final_gamemode)?;
        buffer.write_int(&self.dimension.to_u8())?;
        buffer.write_long(&self.hashed_seed)?;
        buffer.write_ubyte(&self.max_players)?;
        buffer.write_string(&self.level_type.to_string())?;
        buffer.write_varint(&self.render_distance)?;
        buffer.write_bool(&self.reduced_debug_info)?;
        buffer.write_bool(&self.enable_respawn_screen)?;
        Ok(())
    }
}