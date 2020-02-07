use std::io;

use crate::buffer::Buffer;
use crate::clientbound::ClientboundPacket;

#[derive(Debug)]
pub struct PluginMessagePacket {
    channel: String,
    data: Vec<u8>,
}

impl PluginMessagePacket {
    pub fn new(channel: &str, data: &[u8]) -> ClientboundPacket {
        ClientboundPacket::PluginMessage(PluginMessagePacket {
            channel: channel.to_string(),
            data: data.to_vec(),
        })
    }

    pub fn new_minecraft_brand(vendor: &str) -> io::Result<ClientboundPacket> {
        let mut buffer = Buffer::new();
        buffer.write_string(vendor)?;

        Ok(ClientboundPacket::PluginMessage(PluginMessagePacket {
            channel: "minecraft:brand".to_string(),
            data: buffer.as_raw().to_vec(),
        }))
    }

    pub fn deserialize(buffer: &mut Buffer, payload_size: usize) -> io::Result<ClientboundPacket> {
        let cursor_before = buffer.cursor();
        let channel = buffer.read_string()?;
        let channel_len = buffer.cursor() - cursor_before;
        let data_size = payload_size - channel_len;

        Ok(ClientboundPacket::PluginMessage(PluginMessagePacket {
            channel,
            data: buffer.read_ubyte_array(data_size)?,
        }))
    }

    pub fn serialize(&self, buffer: &mut Buffer) -> io::Result<()> {
        buffer.write_string(&self.channel)?;
        buffer.write_ubyte_array(&self.data)?;
        Ok(())
    }
}