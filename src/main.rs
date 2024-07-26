use constants::{AES_KEY, JWT_DEC_KEY, JWT_ENC_KEY};
use encryption::{get_aes_from_env, get_jwt_keys_from_env};
use tokio::net::TcpListener;

mod auth;
pub mod campus_backend;
mod constants;
mod encryption;
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

    let listener = TcpListener::bind("0.0.0.0:8080")
        .await
        .expect("Unable to connect to the server");

    println!("Listening on {}", listener.local_addr().unwrap());

    let app = routes::app().await;

    axum::serve(listener, app)
        .await
        .expect("Error serving application");
}
