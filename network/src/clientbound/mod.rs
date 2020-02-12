use std::io;

pub mod login;
pub mod play;
pub mod status;

use crate::buffer::Buffer;
use crate::packet::{Packet, State};

#[derive(Debug)]
#[allow(clippy::large_enum_variant)]
pub enum ClientboundPacket {
    // Play
    ServerDifficulty(play::ServerDifficultyPacket),
    PluginMessage(play::PluginMessagePacket),
    DisconnectPlay(play::DisconnectPlayPacket),
    JoinGame(play::JoinGamePacket),
    PlayerAbilities(play::PlayerAbilitiesPacket),

    // Status
    StatusResponse(status::StatusResponsePacket),
    Pong(status::PongPacket),

    // Login
    Disconnect(login::DisconnectPacket),
    EncryptionRequest(login::EncryptionRequestPacket),
    LoginSuccess(login::LoginSuccessPacket),
}

impl Packet for ClientboundPacket {
    fn deserialize(buffer: &mut Buffer, state: &State) -> io::Result<Self> {
        let packet_len = buffer.read_varint()?;

        if buffer.has_at_least(packet_len as usize) {
            let cursor_before = buffer.cursor();
            let packet_id = buffer.read_varint()?;
            let payload_size = (packet_len as usize) - (buffer.cursor() - cursor_before);

            match *state {
                State::Handshake => match packet_id {
                    _ => Err(io::Error::new(io::ErrorKind::Other, "Unknown packet id")),
                },
                State::Play => match packet_id {
                    0x0D => play::ServerDifficultyPacket::deserialize(buffer),
                    0x19 => play::PluginMessagePacket::deserialize(buffer, payload_size),
                    0x1B => play::DisconnectPlayPacket::deserialize(buffer),
                    0x26 => play::JoinGamePacket::deserialize(buffer),
                    0x32 => play::PlayerAbilitiesPacket::deserialize(buffer),
                    _ => Err(io::Error::new(io::ErrorKind::Other, "Unknown packet id")),
                },
                State::Status => match packet_id {
                    0x0 => status::StatusResponsePacket::deserialize(buffer),
                    0x1 => status::PongPacket::deserialize(buffer),
                    _ => Err(io::Error::new(io::ErrorKind::Other, "Unknown packet id")),
                },
                State::Login => match packet_id {
                    0x0 => login::DisconnectPacket::deserialize(buffer),
                    0x1 => login::EncryptionRequestPacket::deserialize(buffer),
                    0x2 => login::LoginSuccessPacket::deserialize(buffer),
                    _ => Err(io::Error::new(io::ErrorKind::Other, "Unknown packet id")),
                },
            }
        } else {
            Err(io::Error::from(io::ErrorKind::WouldBlock))
        }
    }

    fn serialize(&self, buffer: &mut Buffer) -> io::Result<()> {
        match *self {
            // Play
            ClientboundPacket::ServerDifficulty(ref x) => x.serialize(buffer),
            ClientboundPacket::PluginMessage(ref x) => x.serialize(buffer),
            ClientboundPacket::DisconnectPlay(ref x) => x.serialize(buffer),
            ClientboundPacket::JoinGame(ref x) => x.serialize(buffer),
            ClientboundPacket::PlayerAbilities(ref x) => x.serialize(buffer),

            // Status
            ClientboundPacket::StatusResponse(ref x) => x.serialize(buffer),
            ClientboundPacket::Pong(ref x) => x.serialize(buffer),

            // Login
            ClientboundPacket::Disconnect(ref x) => x.serialize(buffer),
            ClientboundPacket::EncryptionRequest(ref x) => x.serialize(buffer),
            ClientboundPacket::LoginSuccess(ref x) => x.serialize(buffer),
        }
    }

    fn get_id(&self) -> i32 {
        match *self {
            // Play
            ClientboundPacket::ServerDifficulty(_) => 0x0D,
            ClientboundPacket::PluginMessage(_) => 0x19,
            ClientboundPacket::DisconnectPlay(_) => 0x1B,
            ClientboundPacket::JoinGame(_) => 0x26,
            ClientboundPacket::PlayerAbilities(_) => 0x32,

            // Status
            ClientboundPacket::StatusResponse(_) => 0x0,
            ClientboundPacket::Pong(_) => 0x1,

            // Login
            ClientboundPacket::Disconnect(_) => 0x0,
            ClientboundPacket::EncryptionRequest(_) => 0x1,
            ClientboundPacket::LoginSuccess(_) => 0x2,
        }
    }

    fn get_state(&self) -> State {
        match *self {
            // Play
            ClientboundPacket::ServerDifficulty(_) => State::Play,
            ClientboundPacket::PluginMessage(_) => State::Play,
            ClientboundPacket::DisconnectPlay(_) => State::Play,
            ClientboundPacket::JoinGame(_) => State::Play,
            ClientboundPacket::PlayerAbilities(_) => State::Play,

            // Status
            ClientboundPacket::StatusResponse(_) => State::Status,
            ClientboundPacket::Pong(_) => State::Status,

            // Login
            ClientboundPacket::Disconnect(_) => State::Login,
            ClientboundPacket::EncryptionRequest(_) => State::Login,
            ClientboundPacket::LoginSuccess(_) => State::Login,
        }
    }
}
