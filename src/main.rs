use std::io;
use env_logger::Env;
use common::chat::Chat;
use network::connection::{self, ConnectionHandler};
use network::clientbound;
use network::serverbound::ServerboundPacket;
use network::stream::Stream;

fn main() {
    println!("Welcome to Minecrate!");
    env_logger::from_env(Env::default().default_filter_or("info")).init();
    
    let mut connection = ConnectionHandler::new();
    connection.listen("127.0.0.1", 25565, &packet_process)
        .expect("Failed to start the server");
}

fn packet_process(stream: &mut Stream, packet: &ServerboundPacket) -> io::Result<()> {
    match packet {
        ServerboundPacket::StatusRequest(_) => {
            let payload = clientbound::status::StatusResponsePayload {
                version: clientbound::status::StatusResponsePayloadVersion {
                    name: connection::PROTOCOL_NAME.to_string(),
                    protocol: connection::PROTOCOL_VERSION,
                },
                players: clientbound::status::StatusResponsePayloadPlayers {
                    max: 20,
                    online: 0,
                    sample: vec![],
                },
                description: Chat::new_text("Minecrate!!"),
                favicon: "".to_string(),
            };
            let res = clientbound::status::StatusResponsePacket::new(payload);

            stream.send_packet(&res)?;
            Ok(())
        },
        ServerboundPacket::Ping(ref x) => {
            let res = clientbound::status::PongPacket::new(x.payload);
            stream.send_packet(&res)?;
            Ok(())
        },
        _ => Ok(())
    }
}