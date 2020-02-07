use std::io;
use uuid::Uuid;
use common::chat::Chat;

use crate::clientbound::ClientboundPacket;
use crate::buffer::Buffer;

#[derive(Debug)]
pub struct DisconnectPacket {
    reason: Chat,
}

impl DisconnectPacket {
    pub fn new(reason: Chat) -> ClientboundPacket {
        ClientboundPacket::Disconnect(DisconnectPacket {
            reason,
        })
    }

    pub fn deserialize(buffer: &mut Buffer) -> io::Result<ClientboundPacket> {
        Ok(ClientboundPacket::Disconnect(DisconnectPacket {
            reason: buffer.read_chat()?,
        }))
    }

    pub fn serialize(&self, buffer: &mut Buffer) -> io::Result<()> {
        buffer.write_chat(&self.reason)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct EncryptionRequestPacket {
    server_id: String,
    public_key_length: i32,
    public_key: Vec<u8>,
    verify_token_length: i32,
    verify_token: Vec<u8>,
}

impl EncryptionRequestPacket {
    pub fn new(server_id: &str, public_key: &[u8], verify_token: &[u8]) -> ClientboundPacket {
        ClientboundPacket::EncryptionRequest(EncryptionRequestPacket {
            server_id: server_id.to_string(),
            public_key_length: public_key.len() as i32,
            public_key: public_key.to_vec(),
            verify_token_length: verify_token.len() as i32,
            verify_token: verify_token.to_vec(),
        })
    }

    pub fn deserialize(buffer: &mut Buffer) -> io::Result<ClientboundPacket> {
        let server_id = buffer.read_string()?;
        let public_key_length = buffer.read_varint()?;
        let public_key = buffer.read_ubyte_array(public_key_length as usize)?;
        let verify_token_length = buffer.read_varint()?;
        let verify_token = buffer.read_ubyte_array(verify_token_length as usize)?;

        Ok(ClientboundPacket::EncryptionRequest(EncryptionRequestPacket {
            server_id,
            public_key_length,
            public_key,
            verify_token_length,
            verify_token,
        }))
    }

    pub fn serialize(&self, buffer: &mut Buffer) -> io::Result<()> {
        buffer.write_string(&self.server_id)?;
        buffer.write_varint(&self.public_key_length)?;
        buffer.write_ubyte_array(&self.public_key)?;
        buffer.write_varint(&self.verify_token_length)?;
        buffer.write_ubyte_array(&self.verify_token)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct LoginSuccessPacket {
    uuid: Uuid,
    username: String,
}

impl LoginSuccessPacket {
    pub fn new(uuid: Uuid, username: &str) -> ClientboundPacket {
        ClientboundPacket::LoginSuccess(LoginSuccessPacket {
            uuid,
            username: username.to_string(),
        })
    }

    pub fn deserialize(buffer: &mut Buffer) -> io::Result<ClientboundPacket> {
        match Uuid::parse_str(&buffer.read_string()?) {
            Ok(uuid) => Ok(ClientboundPacket::LoginSuccess(LoginSuccessPacket {
                uuid,
                username: buffer.read_string()?,
            })),
            Err(e) => Err(io::Error::new(io::ErrorKind::InvalidData, e))
        }
    }

    pub fn serialize(&self, buffer: &mut Buffer) -> io::Result<()> {
        buffer.write_string(self.uuid.to_hyphenated().encode_lower(&mut Uuid::encode_buffer()))?;
        buffer.write_string(&self.username)?;
        Ok(())
    }
}