use common::chat::Chat;
use serde::{Deserialize, Serialize};
use serde_json;
use std::io;

use crate::buffer::Buffer;
use crate::clientbound::ClientboundPacket;
use crate::mojang;

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusResponsePayloadVersion {
    pub name: String,
    pub protocol: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusResponsePayloadPlayers {
    pub max: u32,
    pub online: u32,
    pub sample: Vec<mojang::Profile>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusResponsePayload {
    pub version: StatusResponsePayloadVersion,
    pub players: StatusResponsePayloadPlayers,
    pub description: Chat,
    pub favicon: String,
}

#[derive(Debug)]
pub struct StatusResponsePacket {
    pub payload: StatusResponsePayload,
}

impl StatusResponsePacket {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(payload: StatusResponsePayload) -> ClientboundPacket {
        ClientboundPacket::StatusResponse(StatusResponsePacket { payload })
    }

    pub fn deserialize(buffer: &mut Buffer) -> io::Result<ClientboundPacket> {
        Ok(ClientboundPacket::StatusResponse(StatusResponsePacket {
            payload: serde_json::from_str(&buffer.read_string()?)?,
        }))
    }

    pub fn serialize(&self, buffer: &mut Buffer) -> io::Result<()> {
        buffer.write_string(&serde_json::to_string(&self.payload)?)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct PongPacket {
    pub payload: i64,
}

impl PongPacket {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(payload: i64) -> ClientboundPacket {
        ClientboundPacket::Pong(PongPacket { payload })
    }

    pub fn deserialize(buffer: &mut Buffer) -> io::Result<ClientboundPacket> {
        Ok(ClientboundPacket::Pong(PongPacket {
            payload: buffer.read_long()?,
        }))
    }

    pub fn serialize(&self, buffer: &mut Buffer) -> io::Result<()> {
        buffer.write_long(self.payload)?;
        Ok(())
    }
}
