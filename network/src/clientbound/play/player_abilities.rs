use std::io;

use crate::buffer::Buffer;
use crate::clientbound::ClientboundPacket;

#[derive(Debug)]
pub struct PlayerAbilitiesPacket {
    invulnerability: bool,
    flying: bool,
    allow_flight: bool,
    instant_break: bool,
    flying_speed: f32,
    field_of_view_modifier: f32,
}

impl PlayerAbilitiesPacket {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(
        invulnerability: bool,
        flying: bool,
        allow_flight: bool,
        instant_break: bool,
        flying_speed: f32,
        field_of_view_modifier: f32,
    ) -> ClientboundPacket {
        ClientboundPacket::PlayerAbilities(PlayerAbilitiesPacket {
            invulnerability,
            flying,
            allow_flight,
            instant_break,
            flying_speed,
            field_of_view_modifier,
        })
    }

    pub fn deserialize(buffer: &mut Buffer) -> io::Result<ClientboundPacket> {
        let flags = buffer.read_byte()?;

        Ok(ClientboundPacket::PlayerAbilities(PlayerAbilitiesPacket {
            invulnerability: (flags & 0b0000_0001) != 0,
            flying: (flags & 0b0000_0010) != 0,
            allow_flight: (flags & 0b0000_0100) != 0,
            instant_break: (flags & 0b0000_1000) != 0,
            flying_speed: buffer.read_float()?,
            field_of_view_modifier: buffer.read_float()?,
        }))
    }

    pub fn serialize(&self, buffer: &mut Buffer) -> io::Result<()> {
        let mut flags: i8 = 0;

        if self.invulnerability {
            flags |= 0b0000_0001;
        }
        if self.flying {
            flags |= 0b0000_0010;
        }
        if self.allow_flight {
            flags |= 0b0000_0100;
        }
        if self.instant_break {
            flags |= 0b0000_1000;
        }

        buffer.write_byte(flags)?;
        buffer.write_float(self.flying_speed)?;
        buffer.write_float(self.field_of_view_modifier)?;
        Ok(())
    }
}
