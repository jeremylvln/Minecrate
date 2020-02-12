use common::chat::Chat;
use network::clientbound;
use network::connection::{PROTOCOL_NAME, PROTOCOL_VERSION};
use network::serverbound::ServerboundPacket;
use network::stream::Stream;
use std::io;

use crate::server::MinecraftServer;

pub fn packet_process(
    server: &mut MinecraftServer,
    stream: &mut Stream,
    packet: &ServerboundPacket,
) -> io::Result<()> {
    match packet {
        ServerboundPacket::StatusRequest(_) => {
            let payload = clientbound::status::StatusResponsePayload {
                version: clientbound::status::StatusResponsePayloadVersion {
                    name: PROTOCOL_NAME.to_string(),
                    protocol: PROTOCOL_VERSION,
                },
                players: clientbound::status::StatusResponsePayloadPlayers {
                    max: server.config.max_players,
                    online: 0,
                    sample: vec![],
                },
                description: Chat::new_text(&server.config.motd),
                favicon: "".to_string(),
            };
            let res = clientbound::status::StatusResponsePacket::new(payload);

            stream.send_packet(&res)?;
            Ok(())
        }
        ServerboundPacket::Ping(ref x) => {
            let res = clientbound::status::PongPacket::new(x.payload);
            stream.send_packet(&res)?;
            Ok(())
        }
        _ => Ok(()),
    }
}
