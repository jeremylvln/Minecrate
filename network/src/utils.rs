use openssl::pkey::Private;
use openssl::rsa::{Padding, Rsa};
use std::io;

pub fn rsa_encrypt(key: &Rsa<Private>, data: &[u8]) -> io::Result<Vec<u8>> {
    let mut ret = vec![0; 128];
    match key.public_encrypt(data, &mut ret, Padding::PKCS1) {
        Ok(128) => Ok(ret),
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Failed to encrypt data",
        )),
    }
}

pub fn rsa_decrypt(key: &Rsa<Private>, data: &[u8]) -> io::Result<Vec<u8>> {
    let mut ret = vec![0; 128];

    let len = match key.private_decrypt(data, &mut ret, Padding::PKCS1) {
        Ok(x) if x <= 128 => x,
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Failed to decrypt data",
            ))
        }
    };
    ret.truncate(len);
    Ok(ret)
}
