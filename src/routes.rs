use std::{sync::Arc, time::Duration};

use axum::{
    middleware,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use http::{header::CONTENT_TYPE, Method};
use tower_governor::{governor::GovernorConfigBuilder, GovernorLayer};
use tower_http::cors::{Any, CorsLayer};

use crate::{
    auth,
    constants::{
        LOGIN_RATELIMIT_QUOTA, LOGIN_RATELIMIT_RESTORE_INTERVAL_SEC, RATELIMIT_QUOTA,
        RATELIMIT_RESTORE_INTERVAL_SEC,
    },
    ratelimit_keyextractor::{GovIpOrGlobalExtractorHashed, GovJwtExtractorHashed},
    services,
};

pub async fn app() -> Router {
    // simulate state by allowing STDs signup
    let shared_state = std::sync::Arc::new(crate::types::SimulatedState {
        stds_signup_state: std::sync::Mutex::new(false),
    });

    // Bucket rate limiting:
    // Budget of 20 requests
    // Each request consumes 1 budget
    // Increase budget by 1 every second
    let governor_conf_jwt = Arc::new(
        GovernorConfigBuilder::default()
            .burst_size(*RATELIMIT_QUOTA.get().unwrap())
            .per_second(*RATELIMIT_RESTORE_INTERVAL_SEC.get().unwrap())
            .key_extractor(GovJwtExtractorHashed)
            .finish()
            .unwrap(),
    );

    let governor_conf_signin = Arc::new(
        GovernorConfigBuilder::default()
            .burst_size(*LOGIN_RATELIMIT_QUOTA.get().unwrap())
            .per_second(*LOGIN_RATELIMIT_RESTORE_INTERVAL_SEC.get().unwrap())
            .key_extractor(GovIpOrGlobalExtractorHashed)
            .finish()
            .unwrap(),
    );

    let governor_limiter_jwt = governor_conf_jwt.limiter().clone();
    let governor_limiter_signin = governor_conf_signin.limiter().clone();

    // a separate background task to clean up
    let interval = Duration::from_secs(60);
    std::thread::spawn(move || loop {
        std::thread::sleep(interval);
        governor_limiter_jwt.retain_recent();
        governor_limiter_signin.retain_recent();
    });

    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Any)
        .allow_headers([CONTENT_TYPE]);

    Router::new()
        .route("/check_revive_session", get(services::check_revive_session))
        .route("/get_grades", get(services::get_grades))
        .route("/get_gradestats", post(services::get_gradestats))
        .route("/get_examsignup", get(services::get_examsignup))
        .with_state(shared_state.clone())
        .route("/registerexam", post(services::post_registerexam))
        .with_state(shared_state.clone())
        .route("/get_examdetails", post(services::get_examdetails))
        .route("/cancelexam", post(services::post_cancelexam))
        .with_state(shared_state.clone())
        .route("/get_examverfahren", get(services::get_examverfahren))
        .with_state(shared_state.clone())
        .route("/get_ects", get(services::get_ects))
        .route("/get_fachsem", get(services::get_fachsem))
        .route("/get_examstats", get(services::get_examstats))
        .route("/get_stundenplan", get(services::get_stundenplan))
        .route("/get_reminders", get(services::get_reminders))
        .with_state(shared_state.clone())
        .route("/get_timeline", get(services::get_timeline))
        // apply auth and jwt rate limiting to all previous (jwt is only stored as hash)
        .layer(GovernorLayer {
            config: governor_conf_jwt,
        })
        .layer(middleware::from_fn(auth::authorize))
        // sign in rate limiting (based on username, only stored as hash)
        .route(
            "/signin",
            post(auth::sign_in).layer(GovernorLayer {
                config: governor_conf_signin,
            }),
        )
        .route("/", get(|| async { "API is reachable".into_response() }))
        .layer(cors)
}
