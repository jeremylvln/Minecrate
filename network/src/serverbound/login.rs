use std::io;

use crate::buffer::Buffer;
use crate::serverbound::ServerboundPacket;

#[derive(Debug)]
pub struct LoginStartPacket {
    pub username: String,
}

impl LoginStartPacket {
    pub fn deserialize(buffer: &mut Buffer) -> io::Result<ServerboundPacket> {
        Ok(ServerboundPacket::LoginStart(LoginStartPacket {
            username: buffer.read_string()?,
        }))
    }

    pub fn serialize(&self, buffer: &mut Buffer) -> io::Result<()> {
        buffer.write_string(&self.username)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct EncryptionResponsePacket {
    pub shared_secret_length: i32,
    pub shared_secret: Vec<u8>,
    pub verify_token_length: i32,
    pub verify_token: Vec<u8>,
}

impl EncryptionResponsePacket {
    pub fn deserialize(buffer: &mut Buffer) -> io::Result<ServerboundPacket> {
        let shared_secret_length = buffer.read_varint()?;
        let shared_secret = buffer.read_ubyte_array(shared_secret_length as usize)?;
        let verify_token_length = buffer.read_varint()?;
        let verify_token = buffer.read_ubyte_array(verify_token_length as usize)?;

        Ok(ServerboundPacket::EncryptionResponse(
            EncryptionResponsePacket {
                shared_secret_length,
                shared_secret,
                verify_token_length,
                verify_token,
            },
        ))
    }

    pub fn serialize(&self, buffer: &mut Buffer) -> io::Result<()> {
        buffer.write_varint(self.shared_secret_length)?;
        buffer.write_ubyte_array(&self.shared_secret)?;
        buffer.write_varint(self.verify_token_length)?;
        buffer.write_ubyte_array(&self.verify_token)?;
        Ok(())
    }
}
