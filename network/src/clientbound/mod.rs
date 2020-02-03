use std::io;

pub mod play;
pub mod status;
pub mod login;

use crate::buffer::Buffer;
use crate::packet::{Packet, State};

#[derive(Debug)]
pub enum ClientboundPacket {
    // Play
    JoinGame(play::join_game::JoinGamePacket),

    // Status
    StatusResponse(status::StatusResponsePacket),
    Pong(status::PongPacket),

    // Login
    EncryptionRequest(login::EncryptionRequestPacket),
    LoginSuccess(login::LoginSuccessPacket),
}

impl Packet for ClientboundPacket {
    fn deserialize(buffer: &mut Buffer, state: &State) -> io::Result<Self> {
        let packet_len = buffer.read_varint()?;

        if buffer.has_at_least(packet_len as usize) {
            let packet_id = buffer.read_varint()?;

            match state {
                &State::Handshake => match packet_id {
                    _ => Err(io::Error::new(io::ErrorKind::Other, "Unknown packet id"))
                },
                &State::Play => match packet_id {
                    //25 => play::join_game::JoinGamePacket::deserialize(buffer),
                    _ => Err(io::Error::new(io::ErrorKind::Other, "Unknown packet id"))
                },
                &State::Status => match packet_id {
                    0x0 => status::StatusResponsePacket::deserialize(buffer),
                    0x1 => status::PongPacket::deserialize(buffer),
                    _ => Err(io::Error::new(io::ErrorKind::Other, "Unknown packet id"))
                },
                &State::Login => match packet_id {
                    0x1 => login::EncryptionRequestPacket::deserialize(buffer),
                    0x2 => login::LoginSuccessPacket::deserialize(buffer),
                    _ => Err(io::Error::new(io::ErrorKind::Other, "Unknown packet id"))
                }
            }
        } else {
            Err(io::Error::from(io::ErrorKind::WouldBlock))
        }
    }

    fn serialize(&self, buffer: &mut Buffer) -> io::Result<()> {
        match self {
            &ClientboundPacket::JoinGame(ref x) => x.serialize(buffer),
            &ClientboundPacket::StatusResponse(ref x) => x.serialize(buffer),
            &ClientboundPacket::Pong(ref x) => x.serialize(buffer),
            &ClientboundPacket::EncryptionRequest(ref x) => x.serialize(buffer),
            &ClientboundPacket::LoginSuccess(ref x) => x.serialize(buffer),
        }
    }

    fn get_id(&self) -> i32 {
        match self {
            &ClientboundPacket::JoinGame(_) => 0x26,
            &ClientboundPacket::StatusResponse(_) => 0x0,
            &ClientboundPacket::Pong(_) => 0x1,
            &ClientboundPacket::EncryptionRequest(_) => 0x1,
            &ClientboundPacket::LoginSuccess(_) => 0x2,
        }
    }
}