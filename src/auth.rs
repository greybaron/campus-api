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

use crate::types::{CampusLoginData, CdAuthData, CdAuthdataExt, Claims, ResponseError};
use crate::{campus_backend::login::cdlogin_get_jcookie_and_meta, types::LoginResponse};

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

pub fn encode_jwt(cd_auth_data: CdAuthData) -> Result<String, StatusCode> {
    let jwt_secret: String = "tshcnritshmieohnoentshesntsmo".to_string();

    let now = Utc::now();
    let expire: chrono::TimeDelta = Duration::hours(24);
    let exp: usize = (now + expire).timestamp() as usize;
    let iat: usize = now.timestamp() as usize;

    let claim = Claims {
        iat,
        exp,
        cdcookie: cd_auth_data.cookie,
        cduser: cd_auth_data.user,
        cdhash: cd_auth_data.hash,
    };

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

    let user_cookie_hash = CdAuthdataExt {
        cookie: token_data.claims.cdcookie,
        user: token_data.claims.cduser,
        hash: token_data.claims.cdhash,
    };

    req.extensions_mut().insert(user_cookie_hash);

    Ok(next.run(req).await)
}

pub async fn sign_in(
    Json(login_data): Json<CampusLoginData>,
) -> Result<Json<LoginResponse>, StatusCode> {
    // Attempt CD login
    let (cd_auth_data, user_basic_info) = match cdlogin_get_jcookie_and_meta(login_data).await {
        Ok((cd_auth_data, user_basic_info)) => (cd_auth_data, user_basic_info),
        Err(_) => {
            return Err(StatusCode::UNAUTHORIZED); // CD login failed
        }
    };

    // Generate JWT
    let token = encode_jwt(cd_auth_data).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Return jsonized JWT
    Ok(Json(LoginResponse {
        token,
        user: user_basic_info,
    }))
}
