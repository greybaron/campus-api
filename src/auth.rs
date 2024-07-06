use axum::{
    body::Body,
    extract::{Json, Request},
    http,
    http::{Response, StatusCode},
    middleware::Next,
    response::IntoResponse,
};

use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde_json::json;

use crate::{campus_backend::login::cdlogin_get_cookie_json, types::Token};
use crate::types::{CampusLoginData, Claims, ResponseError, UserCookieExt};

impl IntoResponse for ResponseError {
    fn into_response(self) -> Response<Body> {
        let body = Json(json!({
            "error": self.message,
        }));

        (self.status_code, body).into_response()
    }
}

impl From<anyhow::Error> for ResponseError {
    fn from(_: anyhow::Error) -> Self {
        ResponseError {
            message: "Internal Server Error".to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<reqwest::Error> for ResponseError {
    fn from(_: reqwest::Error) -> Self {
        ResponseError {
            message: "CampusDual is not reachable".to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

pub fn encode_jwt(cdcookie: String) -> Result<String, StatusCode> {
    let jwt_secret: String = "tshcnritshmieohnoentshesntsmo".to_string();

    let now = Utc::now();
    let expire: chrono::TimeDelta = Duration::hours(24);
    let exp: usize = (now + expire).timestamp() as usize;
    let iat: usize = now.timestamp() as usize;

    let claim = Claims { iat, exp, cdcookie };

    encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn decode_jwt(jwt: String) -> Result<TokenData<Claims>, StatusCode> {
    let jwt_secret = "tshcnritshmieohnoentshesntsmo".to_string();

    let result: Result<TokenData<Claims>, StatusCode> = decode(
        &jwt,
        &DecodingKey::from_secret(jwt_secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);
    result
}

pub async fn authorize(mut req: Request, next: Next) -> Result<Response<Body>, ResponseError> {
    let auth_header = req.headers().get(http::header::AUTHORIZATION);

    let auth_header = match auth_header {
        Some(header) => header.to_str().map_err(|_| ResponseError {
            message: "Empty header is not allowed".to_string(),
            status_code: StatusCode::FORBIDDEN,
        })?,
        None => {
            return Err(ResponseError {
                message: "JWT token is missing".to_string(),
                status_code: StatusCode::FORBIDDEN,
            })
        }
    };

    let mut header = auth_header.split_whitespace();

    // maybe i'll need bearer idk
    let (_, token) = (header.next(), header.next());

    let token_data = match decode_jwt(token.unwrap().to_string()) {
        Ok(data) => data,
        Err(_) => {
            return Err(ResponseError {
                message: "Invalid JWT".to_string(),
                status_code: StatusCode::UNAUTHORIZED,
            })
        }
    };

    let user_cookie = UserCookieExt {
        cookie: token_data.claims.cdcookie,
    };

    req.extensions_mut().insert(user_cookie);

    Ok(next.run(req).await)
}

pub async fn sign_in(Json(login_data): Json<CampusLoginData>) -> Result<Json<Token>, StatusCode> {
    // Attempt CD login
    let j_cookie = match cdlogin_get_cookie_json(&login_data).await {
        Ok(j_cookie) => j_cookie,
        Err(_) => {
            return Err(StatusCode::UNAUTHORIZED); // CD login failed
        }
    };

    // Generate JWT
    let token = encode_jwt(j_cookie.to_string()).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Return jsonized JWT
    Ok(Json(Token { token }))
}
