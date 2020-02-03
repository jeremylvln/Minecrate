use openssl::rsa::{Rsa, Padding};
use openssl::pkey::Private;

pub fn rsa_encrypt(key: &Rsa<Private>, data: &[u8]) -> Result<Vec<u8>, String> {
    let mut ret = vec![0; 128];
    match key.public_encrypt(data, &mut ret, Padding::PKCS1) {
        Ok(128) => (),
        _ => return Err(String::from("Failed to encrypt data"))
    }

    Ok(ret)
}

pub fn rsa_decrypt(key: &Rsa<Private>, data: &[u8]) -> Result<Vec<u8>, String> {
    let mut ret = vec![0; 128];

    let len = match key.private_decrypt(data, &mut ret, Padding::PKCS1) {
        Ok(x) if x <= 128 => x,
        _ => return Err(String::from("Failed to decrypt data"))
    };
    ret.truncate(len);
    Ok(ret)
}