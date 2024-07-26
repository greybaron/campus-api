use std::sync::OnceLock;

use jsonwebtoken::{DecodingKey, EncodingKey};

// pub static JWT_SECRET: OnceLock<String> = OnceLock::new();
pub static JWT_ENC_KEY: OnceLock<EncodingKey> = OnceLock::new();
pub static JWT_DEC_KEY: OnceLock<DecodingKey> = OnceLock::new();
pub static AES_KEY: OnceLock<[u8; 32]> = OnceLock::new();
