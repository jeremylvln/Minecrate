use std::net::TcpStream;
use std::io;
use std::io::{Read, Write};
use rand::thread_rng;
use rand::Rng;
use openssl::symm;

use crate::buffer::Buffer;
use crate::packet::{State, Packet};
use crate::serverbound::ServerboundPacket;
use crate::clientbound::ClientboundPacket;

pub struct Stream {
    handle: TcpStream,
    buf: Buffer,
    state: State,
    verify_challenge: [u8; 4],
    in_cipher: Option<symm::Crypter>,
    out_cipher: Option<symm::Crypter>,
    username: String,
}

impl Stream {
    pub fn new(handle: TcpStream) -> io::Result<Self> {
        handle.set_nonblocking(true)?;
        Ok(Self {
            handle,
            buf: Buffer::new(),
            state: State::Handshake,
            verify_challenge: thread_rng().gen::<[u8; 4]>(),
            in_cipher: None,
            out_cipher: None,
            username: String::new(),
        })
    }

    pub fn read_packet(&mut self) -> io::Result<ServerboundPacket> {
        self.read()?;
        if self.buf.len() > 0 {
            self.buf.reset_cursor();
            match ServerboundPacket::deserialize(&mut self.buf, &self.state) {
                Ok(packet) => {
                    self.buf.drain(0..self.buf.cursor());
                    Ok(packet)
                },
                Err(e) => Err(e)
            }
        } else {
            Err(io::Error::from(io::ErrorKind::WouldBlock))
        }
    }

    pub fn send_packet(&mut self, packet: &ClientboundPacket) -> io::Result<()> {
        let mut out_buf = Buffer::new();
        out_buf.write_varint(&packet.get_id())?;
        packet.serialize(&mut out_buf)?;

        let mut final_out_buf = Buffer::new();
        final_out_buf.write_varint(&(out_buf.len() as i32))?;
        final_out_buf.extend(out_buf.as_raw());

        match &mut self.out_cipher {
            Some(cipher) => {
                let mut tmp = vec![0; final_out_buf.len() + 16];
                let n = cipher.update(&final_out_buf.as_raw(), &mut tmp)?;
                self.handle.write_all(&tmp[..n])?
            },
            None => self.handle.write_all(&final_out_buf.as_raw())?
        };

        Ok(())
    }

    pub fn set_state(&mut self, state: State) {
        self.state = state;
    }

    pub fn get_state(&self) -> &State {
        &self.state
    }

    pub fn get_verify_challenge(&self) -> &[u8; 4] {
        &self.verify_challenge
    }

    pub fn set_username(&mut self, username: &str) {
        self.username = username.to_string();
    }

    pub fn get_username(&self) -> &String {
        &self.username
    }

    pub fn set_encryption_key(&mut self, key: &[u8]) {
        self.in_cipher = Some(symm::Crypter::new(
            symm::Cipher::aes_128_cfb8(), symm::Mode::Decrypt,
            key, Some(key)
        ).expect("Failed to create decryption cipher"));

        self.out_cipher = Some(symm::Crypter::new(
            symm::Cipher::aes_128_cfb8(), symm::Mode::Encrypt,
            key, Some(key)
        ).expect("Failed to create encryption cipher"));
    }

    fn read(&mut self) -> io::Result<()> {
        let mut tmp = [0; 1024];
        match self.handle.read(&mut tmp[..]) {
            Ok(count) if count > 0 => {
                self.buf.extend(&tmp[..count]);
                println!("+{} {:?}", count, self.buf);
                Ok(())
            },
            Ok(_) => Ok(()),
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => Ok(()),
            Err(e) => Err(e)
        }
    }
}