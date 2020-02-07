use std::io;
use common::gamemode::Gamemode;
use common::dimension::Dimension;
use common::level_type::LevelType;
use network::serverbound::ServerboundPacket;
use network::clientbound;
use network::stream::Stream;

use crate::server::MinecraftServer;

const SERVER_VENDOR: &'static str = "minecrate";

pub fn packet_process(
    server: &mut MinecraftServer, stream: &mut Stream, packet: &ServerboundPacket
) -> io::Result<()> {
    match packet {
        ServerboundPacket::EncryptionResponse(_) => {
            stream.send_packet(&clientbound::play::JoinGamePacket::new(
                0, Gamemode::Survival, false, Dimension::Overworld,
                0, 0, LevelType::Default, 32, false, true,
            ))?;

            stream.send_packet(&clientbound::play::PluginMessagePacket::new_minecraft_brand(
                SERVER_VENDOR,
            )?)?;

            stream.send_packet(&clientbound::play::ServerDifficultyPacket::new(
                server.difficulty, server.difficulty_locked,
            ))?;

            stream.send_packet(&clientbound::play::PlayerAbilitiesPacket::new(
                false, false, false, false, 0.05, 0.1,
            ))?;
            Ok(())
        },
        _ => Ok(())
    }
}