use std::io;

use crate::buffer::Buffer;

#[derive(Debug)]
pub enum State {
    Handshake,
    Play,
    Status,
    Login,
}

pub trait Packet: Sized {
    fn deserialize(buffer: &mut Buffer, state: &State) -> io::Result<Self>;
    fn serialize(&self, buffer: &mut Buffer) -> io::Result<()>;
    fn get_id(&self) -> i32;
    fn get_state(&self) -> State;
}
