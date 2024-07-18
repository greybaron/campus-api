use axum::{
    middleware,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use http::{header::CONTENT_TYPE, Method};
use tower_http::cors::{Any, CorsLayer};

use crate::{auth, services};

pub async fn app() -> Router {
    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Any)
        .allow_headers([CONTENT_TYPE]);

    Router::new()
        .route("/", get(|| async { "API is reachable".into_response() }))
        .route("/signin", post(auth::sign_in))
        .route(
            "/check_session_alive",
            get(services::check_session_alive).layer(middleware::from_fn(auth::authorize)),
        )
        .route(
            "/get_grades",
            get(services::get_grades).layer(middleware::from_fn(auth::authorize)),
        )
        .route(
            "/get_examsignup",
            get(services::get_signup_options).layer(middleware::from_fn(auth::authorize)),
        )
        .route(
            "/get_ects",
            get(services::get_ects).layer(middleware::from_fn(auth::authorize)),
        )
        .route(
            "/get_fachsem",
            get(services::get_fachsem).layer(middleware::from_fn(auth::authorize)),
        )
        .route(
            "/get_examstats",
            get(services::get_examstats).layer(middleware::from_fn(auth::authorize)),
        )
        .route(
            "/get_stundenplan",
            get(services::get_stundenplan).layer(middleware::from_fn(auth::authorize)),
        )
        .layer(cors)
}
