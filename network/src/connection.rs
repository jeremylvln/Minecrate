use std::net::TcpListener;
use std::io;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::ops::DerefMut;
use std::thread;
use std::time::Duration;
use log::{debug, info, error};
use openssl::rsa::Rsa;
use openssl::pkey;
use uuid::Uuid;

use crate::packet::State;
use crate::stream::Stream;
use crate::serverbound::ServerboundPacket;
use crate::serverbound;
use crate::clientbound::ClientboundPacket;
use crate::clientbound;
use crate::mojang;
use crate::utils::{rsa_decrypt};

pub const PROTOCOL_VERSION: i32 = 701;
pub const PROTOCOL_NAME: &'static str = "20w06a";

pub struct ConnectionHandler {
    streams: Arc<Mutex<Vec<Stream>>>,
    rsa: Rsa<pkey::Private>,
    rsa_pub_der: Vec<u8>,
}

impl ConnectionHandler {
    pub fn new() -> Self {
        info!("Generating key-pair...");
        let rsa = Rsa::generate(1024).unwrap();
        let rsa_pub_der = rsa.public_key_to_der().unwrap();

        ConnectionHandler {
            streams: Arc::new(Mutex::new(vec![])),
            rsa,
            rsa_pub_der,
        }
    }

    pub fn listen<T, P>(
        &mut self, run: Arc<AtomicBool>, host: &str, port: u16,
        mut tick_cb: T, mut packet_cb: P,
    ) -> io::Result<()>
        where T: FnMut(), P: FnMut(&mut Stream, &ServerboundPacket) -> io::Result<()> {

        let run_cpy = run.clone();
        thread::spawn({
            let listener = TcpListener::bind(format!("{}:{}", host, port))?;
            listener.set_nonblocking(true)?;

            let streams_cpy = Arc::clone(&self.streams);
            info!("Listening on {}:{}.", host, port);

            move || while run_cpy.load(Ordering::SeqCst) {
                for stream in listener.incoming() {
                    match stream {
                        Ok(s) => {
                            streams_cpy.lock().unwrap().push(
                                Stream::new(s).expect("Failed to create a stream handle")
                            );
                        },
                        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                            continue;
                        },
                        Err(_) => {},
                    }
                }

                std::thread::sleep(Duration::from_millis(50));
            }
        });

        while run.load(Ordering::SeqCst) {
            {
                let mut streams_lock = self.streams.lock().unwrap();
                let streams_ref = streams_lock.deref_mut();

                let mut i = 0;
                while i < streams_ref.len() {
                    let remove = match streams_ref[i].read_packet() {
                        Ok(packet) => {
                            debug!("Received packet: {:?}.", packet);
                            match &packet {
                                &ServerboundPacket::Handshake(ref p) => {
                                    self.handle_handshake(&mut streams_ref[i], p)?;
                                    packet_cb(&mut streams_ref[i], &packet)?
                                },
                                &ServerboundPacket::LoginStart(ref p) => {
                                    self.handle_login_start(&mut streams_ref[i], p)?;
                                    packet_cb(&mut streams_ref[i], &packet)?
                                },
                                &ServerboundPacket::EncryptionResponse(ref p) => {
                                    self.handle_encryption_response(&mut streams_ref[i], p)?;
                                    packet_cb(&mut streams_ref[i], &packet)?
                                },
                                _ => packet_cb(&mut streams_ref[i], &packet)?,
                            }
                            false
                        },
                        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => false,
                        Err(e) => {
                            error!("Disconnecting player: {}", e);
                            true
                        }
                    };

                    if remove {
                        streams_ref.remove(i);
                    } else {
                        i += 1;
                    }
                }
            }

            tick_cb();
            std::thread::sleep(Duration::from_millis(50));
        }

        Ok(())
    }

    pub fn broadcast_packet(&mut self, packet: &ClientboundPacket) -> io::Result<()> {
        let mut streams_lock = self.streams.lock().unwrap();
        let streams_ref = streams_lock.deref_mut();

        for stream in streams_ref {
            stream.send_packet(packet)?;
        }
        Ok(())
    }

    fn handle_handshake(
        &self, stream: &mut Stream, packet: &serverbound::handshake::HandshakePacket
    ) -> io::Result<()> {
        if packet.protocol != PROTOCOL_VERSION {
            // TODO
            Ok(())
        } else {
            if packet.next == 1 {
                stream.set_state(State::Status);
            } else {
                stream.set_state(State::Login);
            }
            Ok(())
        }
    }
    
    fn handle_login_start(
        &self, stream: &mut Stream, packet: &serverbound::login::LoginStartPacket
    ) -> io::Result<()> {
        stream.set_username(&packet.username);

        let res = clientbound::login::EncryptionRequestPacket::new(
            "",
            &self.rsa_pub_der,
            stream.get_verify_challenge(),
        );

        stream.send_packet(&res)?;
        Ok(())
    }

    fn handle_encryption_response(
        &self, stream: &mut Stream, packet: &serverbound::login::EncryptionResponsePacket
    ) -> io::Result<()> {
        let decrypted_shared = rsa_decrypt(&self.rsa, &packet.shared_secret)?;
        let decrypted_verify = rsa_decrypt(&self.rsa, &packet.verify_token)?;

        if decrypted_verify != stream.get_verify_challenge() {
            Err(io::Error::new(io::ErrorKind::InvalidData, "Failed to verify challenge"))
        } else {
            match mojang::has_joined(
                stream.get_username(), &vec![], &decrypted_shared, &self.rsa_pub_der
            ) {
                Ok(profile) => {
                    stream.set_encryption_key(&decrypted_shared);

                    let res_login = clientbound::login::LoginSuccessPacket::new(
                        Uuid::parse_str(&profile.id).unwrap(),
                        stream.get_username(),
                    );

                    stream.send_packet(&res_login)?;
                    stream.set_state(State::Play);
                    Ok(())
                },
                Err(e) => Err(io::Error::new(io::ErrorKind::InvalidData, e))
            }
        }
    }
}