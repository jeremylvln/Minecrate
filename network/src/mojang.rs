use openssl::hash::{self, MessageDigest};
use reqwest;
use serde::{Deserialize, Serialize};
use std::fmt::Write;
use std::io;

const ENDPOINT: &str = "https://sessionserver.mojang.com/session/minecraft/hasJoined";

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    pub id: String,
    pub name: String,
}

pub fn has_joined(
    username: &str,
    server_id: &[u8],
    shared_secret: &[u8],
    server_key: &[u8],
) -> io::Result<Profile> {
    let server_hash = create_server_hash(server_id, shared_secret, server_key);
    let uri = format!(
        "{}?username={}&serverId={}",
        ENDPOINT, username, server_hash
    );

    match reqwest::blocking::get(&uri) {
        Ok(res) => match serde_json::from_str::<Profile>(&res.text().unwrap()) {
            Ok(profile) => Ok(profile),
            Err(_) => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Failed to parse profile",
            )),
        },
        Err(_) => Err(io::Error::new(
            io::ErrorKind::Other,
            "Failed to query profile",
        )),
    }
}

fn create_server_hash(server_id: &[u8], shared_secret: &[u8], key: &[u8]) -> String {
    let mut tmp = server_id.to_vec();
    tmp.extend(shared_secret);
    tmp.extend(key);

    sha1(&tmp)
}

fn sha1(data: &[u8]) -> String {
    let mut digest = hash::hash(MessageDigest::sha1(), data).unwrap();
    let mut negative = false;

    if digest[0] >= 128 {
        negative = true;

        for byte in digest.iter_mut() {
            *byte ^= 0xff;
        }

        for byte in digest.iter_mut().rev() {
            if *byte == 255 {
                *byte = 0;
            } else {
                *byte += 1;
                break;
            }
        }
    }

    let mut ret = String::new();

    if negative {
        write!(&mut ret, "-").unwrap();
    }

    let mut non_zero = false;
    for byte in &*digest {
        if *byte >= 16 {
            non_zero = true;
        } else if !non_zero && *byte > 0 {
            write!(&mut ret, "{:x}", byte).unwrap();
            non_zero = true;
            continue;
        }
        if non_zero {
            write!(&mut ret, "{:02x}", byte).unwrap();
        }
    }
    ret
}
