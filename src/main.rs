use tokio::net::TcpListener;

mod auth;
pub mod campus_backend;
mod routes;
mod services;
mod types;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080")
        .await
        .expect("Unable to conne to connect to the server");

    println!("Listening on {}", listener.local_addr().unwrap());

    let app = routes::app().await;

    axum::serve(listener, app)
        .await
        .expect("Error serving application");
}
