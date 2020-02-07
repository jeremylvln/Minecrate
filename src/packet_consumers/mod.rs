use std::io;
use network::serverbound::ServerboundPacket;
use network::stream::Stream;
use network::packet::{State, Packet};

mod status;
mod login;

use crate::server::MinecraftServer;

pub fn packet_process(
    server: &mut MinecraftServer, stream: &mut Stream, packet: &ServerboundPacket
) -> io::Result<()> {
    match &packet.get_state() {
        &State::Status => status::packet_process(server, stream, packet),
        &State::Login => login::packet_process(server, stream, packet),
        _ => Ok(())
    }
}