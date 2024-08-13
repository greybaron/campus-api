use std::env;

use tokio::net::TcpListener;
mod routes;
mod services;

#[tokio::main]
async fn main() {
    if env::var(pretty_env_logger::env_logger::DEFAULT_FILTER_ENV).is_err() {
        env::set_var("RUST_LOG", "info");
    }

    pretty_env_logger::init_timed();
    log::info!("Starting headers test API...");

    let listener = TcpListener::bind("0.0.0.0:8080")
        .await
        .expect("Unable to start the server");

    log::info!("Listening on {}", listener.local_addr().unwrap());

    let app = routes::app().await;

    axum::serve(listener, app)
        .await
        .expect("Error serving application");
}
