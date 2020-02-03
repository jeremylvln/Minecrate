use std::io;

pub mod handshake;
pub mod status;
pub mod login;

use crate::buffer::Buffer;
use crate::packet::{Packet, State};

#[derive(Debug)]
pub enum ServerboundPacket {
    // Handshake
    Handshake(handshake::HandshakePacket),

    // Play

    // Status
    StatusRequest(status::StatusRequestPacket),
    Ping(status::PingPacket),

    // Login
    LoginStart(login::LoginStartPacket),
    EncryptionResponse(login::EncryptionResponsePacket),
}

impl Packet for ServerboundPacket {
    fn deserialize(buffer: &mut Buffer, state: &State) -> io::Result<Self> {
        let packet_len = buffer.read_varint()?;

        if buffer.has_at_least(packet_len as usize) {
            let packet_id = buffer.read_varint()?;

            match state {
                &State::Handshake => match packet_id {
                    0x0 => handshake::HandshakePacket::deserialize(buffer),
                    _ => Err(io::Error::new(io::ErrorKind::Other, "Unknown packet id"))
                },
                &State::Play => match packet_id {
                    _ => Err(io::Error::new(io::ErrorKind::Other, "Unknown packet id"))
                },
                &State::Status => match packet_id {
                    0x0 => status::StatusRequestPacket::deserialize(buffer),
                    0x1 => status::PingPacket::deserialize(buffer),
                    _ => Err(io::Error::new(io::ErrorKind::Other, "Unknown packet id"))
                },
                &State::Login => match packet_id {
                    0x0 => login::LoginStartPacket::deserialize(buffer),
                    0x1 => login::EncryptionResponsePacket::deserialize(buffer),
                    _ => Err(io::Error::new(io::ErrorKind::Other, "Unknown packet id"))
                }
            }
        } else {
            Err(io::Error::from(io::ErrorKind::WouldBlock))
        }
    }

    fn serialize(&self, buffer: &mut Buffer) -> io::Result<()> {
        match self {
            &ServerboundPacket::Handshake(ref x) => x.serialize(buffer),
            &ServerboundPacket::StatusRequest(ref x) => x.serialize(buffer),
            &ServerboundPacket::Ping(ref x) => x.serialize(buffer),
            &ServerboundPacket::LoginStart(ref x) => x.serialize(buffer),
            &ServerboundPacket::EncryptionResponse(ref x) => x.serialize(buffer),
        }
    }

    fn get_id(&self) -> i32 {
        match self {
            &ServerboundPacket::Handshake(_) => 0x0,
            &ServerboundPacket::StatusRequest(_) => 0x0,
            &ServerboundPacket::Ping(_) => 0x1,
            &ServerboundPacket::LoginStart(_) => 0x0,
            &ServerboundPacket::EncryptionResponse(_) => 0x1,
        }
    }
}