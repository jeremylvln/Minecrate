use std::io;
use common::chat::Chat;

use crate::clientbound::ClientboundPacket;
use crate::buffer::Buffer;

#[derive(Debug)]
pub struct DisconnectPlayPacket {
    reason: Chat,
}

impl DisconnectPlayPacket {
    pub fn new(reason: Chat) -> ClientboundPacket {
        ClientboundPacket::DisconnectPlay(DisconnectPlayPacket {
            reason,
        })
    }

    pub fn deserialize(buffer: &mut Buffer) -> io::Result<ClientboundPacket> {
        Ok(ClientboundPacket::DisconnectPlay(DisconnectPlayPacket {
            reason: buffer.read_chat()?,
        }))
    }

    pub fn serialize(&self, buffer: &mut Buffer) -> io::Result<()> {
        buffer.write_chat(&self.reason)?;
        Ok(())
    }
}