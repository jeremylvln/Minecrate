use common::chat::Chat;
use std::io;

use crate::buffer::Buffer;
use crate::clientbound::ClientboundPacket;

#[derive(Debug)]
pub struct DisconnectPlayPacket {
    reason: Chat,
}

impl DisconnectPlayPacket {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(reason: Chat) -> ClientboundPacket {
        ClientboundPacket::DisconnectPlay(DisconnectPlayPacket { reason })
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
