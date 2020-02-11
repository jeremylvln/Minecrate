use std::io;

use crate::serverbound::ServerboundPacket;
use crate::buffer::Buffer;

#[derive(Debug)]
pub struct HandshakePacket {
    pub protocol: i32,
    pub address: String,
    pub port: u16,
    pub next: i32,
}

impl HandshakePacket {
    pub fn deserialize(buffer: &mut Buffer) -> io::Result<ServerboundPacket> {
        Ok(ServerboundPacket::Handshake(HandshakePacket {
            protocol: buffer.read_varint()?,
            address: buffer.read_string()?,
            port: buffer.read_ushort()?,
            next: buffer.read_varint()?,
        }))
    }

    pub fn serialize(&self, buffer: &mut Buffer) -> io::Result<()> {
        buffer.write_varint(self.protocol)?;
        buffer.write_string(&self.address)?;
        buffer.write_ushort(self.port)?;
        buffer.write_varint(self.next)?;
        Ok(())
    }
}
