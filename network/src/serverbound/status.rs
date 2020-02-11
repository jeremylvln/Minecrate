use std::io;

use crate::serverbound::ServerboundPacket;
use crate::buffer::Buffer;

#[derive(Debug)]
pub struct StatusRequestPacket {}

impl StatusRequestPacket {
    pub fn deserialize(_buffer: &mut Buffer) -> io::Result<ServerboundPacket> {
        Ok(ServerboundPacket::StatusRequest(StatusRequestPacket {}))
    }

    pub fn serialize(&self, _buffer: &mut Buffer) -> io::Result<()> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct PingPacket {
    pub payload: i64,
}

impl PingPacket {
    pub fn deserialize(buffer: &mut Buffer) -> io::Result<ServerboundPacket> {
        Ok(ServerboundPacket::Ping(PingPacket {
            payload: buffer.read_long()?,
        }))
    }

    pub fn serialize(&self, buffer: &mut Buffer) -> io::Result<()> {
        buffer.write_long(self.payload)?;
        Ok(())
    }
}