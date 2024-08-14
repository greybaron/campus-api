use std::{env, sync::OnceLock};

use jsonwebtoken::{DecodingKey, EncodingKey};
use reqwest::Certificate;

use crate::encryption::{get_aes_from_env, get_jwt_keys_from_env};

pub static JWT_ENC_KEY: OnceLock<EncodingKey> = OnceLock::new();
pub static JWT_DEC_KEY: OnceLock<DecodingKey> = OnceLock::new();
pub static AES_KEY: OnceLock<[u8; 32]> = OnceLock::new();
pub static CD_CERT_PEM: OnceLock<Certificate> = OnceLock::new();
pub static RATELIMIT_QUOTA: OnceLock<u32> = OnceLock::new();
pub static RATELIMIT_RESTORE_INTERVAL_SEC: OnceLock<u64> = OnceLock::new();
pub static LOGIN_RATELIMIT_QUOTA: OnceLock<u32> = OnceLock::new();
pub static LOGIN_RATELIMIT_RESTORE_INTERVAL_SEC: OnceLock<u64> = OnceLock::new();

pub fn set_statics_from_env() {
    AES_KEY.set(get_aes_from_env()).unwrap();
    let (jwt_enc_key, jwt_dec_key) = get_jwt_keys_from_env();
    JWT_ENC_KEY
        .set(jwt_enc_key)
        .unwrap_or_else(|_| panic!("Unable to set JWT enc key"));
    JWT_DEC_KEY
        .set(jwt_dec_key)
        .unwrap_or_else(|_| panic!("Unable to set JWT dec key"));
    RATELIMIT_QUOTA
        .set(
            env::var("RATELIMIT_QUOTA")
                .and_then(|key| key.parse().map_err(|_| env::VarError::NotPresent))
                .unwrap_or(50),
        )
        .unwrap();
    RATELIMIT_RESTORE_INTERVAL_SEC
        .set(
            env::var("RATELIMIT_RESTORE_INTERVAL_SEC")
                .and_then(|key| key.parse().map_err(|_| env::VarError::NotPresent))
                .unwrap_or(2),
        )
        .unwrap();

    LOGIN_RATELIMIT_QUOTA
        .set(
            env::var("LOGIN_RATELIMIT_QUOTA")
                .and_then(|key| key.parse().map_err(|_| env::VarError::NotPresent))
                .unwrap_or(10),
        )
        .unwrap();
    LOGIN_RATELIMIT_RESTORE_INTERVAL_SEC
        .set(
            env::var("LOGIN_RATELIMIT_RESTORE_INTERVAL_SEC")
                .and_then(|key| key.parse().map_err(|_| env::VarError::NotPresent))
                .unwrap_or(10),
        )
        .unwrap();
}
