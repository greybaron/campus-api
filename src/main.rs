use std::env;

use constants::{
    AES_KEY, CD_CERT_PEM, JWT_DEC_KEY, JWT_ENC_KEY, RATELIMIT_QUOTA, RATELIMIT_RESTORE_INTERVAL_SEC,
};
use encryption::{get_aes_from_env, get_jwt_keys_from_env};
use tokio::net::TcpListener;

mod auth;
pub mod campus_backend;
mod color_stuff;
mod constants;
mod encryption;
mod ratelimit_keyextractor;
mod routes;
mod services;
mod types;

#[tokio::main]
async fn main() {
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

    if env::var(pretty_env_logger::env_logger::DEFAULT_FILTER_ENV).is_err() {
        env::set_var("RUST_LOG", "info");
    }

    pretty_env_logger::init_timed();
    log::info!("Starting Campus API...");
    log::info!("Rate limit: {}", RATELIMIT_QUOTA.get().unwrap());
    log::info!(
        "RL restore interval: every {} seconds",
        RATELIMIT_RESTORE_INTERVAL_SEC.get().unwrap()
    );

    let buf = include_bytes!("GEANT_OV_RSA_CA_4_tcs-cert3.pem");
    let cert = reqwest::Certificate::from_pem(buf).unwrap();
    CD_CERT_PEM.set(cert).unwrap();

    let listener = TcpListener::bind("0.0.0.0:8080")
        .await
        .expect("Unable to start the server");

    log::info!("Listening on {}", listener.local_addr().unwrap());

    let app = routes::app().await;

    axum::serve(listener, app)
        .await
        .expect("Error serving application");
}
