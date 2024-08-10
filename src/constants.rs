use std::sync::OnceLock;

use jsonwebtoken::{DecodingKey, EncodingKey};
use reqwest::Certificate;

pub static JWT_ENC_KEY: OnceLock<EncodingKey> = OnceLock::new();
pub static JWT_DEC_KEY: OnceLock<DecodingKey> = OnceLock::new();
pub static AES_KEY: OnceLock<[u8; 32]> = OnceLock::new();
pub static CD_CERT_PEM: OnceLock<Certificate> = OnceLock::new();
pub static RATELIMIT_QUOTA: OnceLock<i64> = OnceLock::new();
pub static RATELIMIT_RESTORE_INTERVAL_SEC: OnceLock<i64> = OnceLock::new();
