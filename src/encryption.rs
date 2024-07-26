use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use anyhow::{anyhow, Result};
use base64::prelude::*;
use http::StatusCode;
use jsonwebtoken::{DecodingKey, EncodingKey};
use rand::Rng;
use std::{env, str};

use crate::constants::AES_KEY;

fn generate_nonce() -> [u8; 12] {
    let mut nonce = [0u8; 12];
    let mut rng = rand::thread_rng();
    rng.fill(&mut nonce);
    nonce
}

pub fn encrypt(plaintext: &str) -> Result<(String, String), StatusCode> {
    let key = AES_KEY.get().unwrap();
    let cipher = Aes256Gcm::new(key.into());

    let nonce = generate_nonce();
    let ciphertext = cipher
        .encrypt(Nonce::from_slice(&nonce), plaintext.as_ref())
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((
        BASE64_STANDARD.encode(nonce),
        BASE64_STANDARD.encode(ciphertext),
    ))
}

pub fn decrypt(nonce: &str, ciphertext: &str) -> Result<String> {
    let key = Key::<Aes256Gcm>::from_slice(AES_KEY.get().unwrap());
    let cipher = Aes256Gcm::new(key);

    let nonce = BASE64_STANDARD.decode(nonce)?;
    let ciphertext = BASE64_STANDARD.decode(ciphertext)?;

    let plaintext = cipher
        .decrypt(Nonce::from_slice(&nonce), ciphertext.as_ref())
        .map_err(|_| anyhow!("decrypt fail"))?;

    Ok(String::from_utf8(plaintext)?)
}

// pub fn ctest() {
//     // let key = "ö example very very secret key."; // Ensure the key is 32 bytes for AES-256
//     let plaintext = "Hello, world!";

//     let (nonce, ciphertext) = encrypt(plaintext);
//     // println!("Nonce: {}", nonce);
//     // println!("Ciphertext: {}", ciphertext);

//     let decrypted_plaintext = decrypt(&nonce, &ciphertext);
//     println!("Decrypted: {}", decrypted_plaintext);
// }

pub fn get_aes_from_env() -> [u8; 32] {
    let key = env::var("AES_KEY").expect("AES_KEY environment variable not set");

    if key.len() < 32 {
        eprintln!("Error: AES_KEY must be at least 32 bytes long.");
        std::process::exit(1);
    }

    let key_bytes = key.as_bytes();
    let mut key_array = [0u8; 32];
    if key_bytes.len() > 32 {
        key_array.copy_from_slice(&key_bytes[..32]);
    } else {
        key_array.copy_from_slice(key_bytes);
    }
    key_array
}

pub fn get_jwt_keys_from_env() -> (EncodingKey, DecodingKey) {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET environment variable not set");
    (
        EncodingKey::from_secret(secret.as_ref()),
        DecodingKey::from_secret(secret.as_ref()),
    )
}
