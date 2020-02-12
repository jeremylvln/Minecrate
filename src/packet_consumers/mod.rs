use network::packet::{Packet, State};
use network::serverbound::ServerboundPacket;
use network::stream::Stream;
use std::io;

mod login;
mod status;

use crate::server::MinecraftServer;

pub fn packet_process(
    server: &mut MinecraftServer,
    stream: &mut Stream,
    packet: &ServerboundPacket,
) -> io::Result<()> {
    match packet.get_state() {
        State::Status => status::packet_process(server, stream, packet),
        State::Login => login::packet_process(server, stream, packet),
        _ => Ok(()),
    }
}
