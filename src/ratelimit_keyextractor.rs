use std::net::IpAddr;

use http::{request::Request, StatusCode};
use serde::{Deserialize, Serialize};
use tower_governor::{
    errors::GovernorError,
    key_extractor::{KeyExtractor, SmartIpKeyExtractor},
};

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct GovJwtExtractorHashed;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct GovIpOrGlobalExtractorHashed;

impl KeyExtractor for GovJwtExtractorHashed {
    type Key = String;

    fn extract<B>(&self, req: &Request<B>) -> Result<Self::Key, GovernorError> {
        req.headers()
            .get("Authorization")
            .and_then(|token| token.to_str().ok())
            .and_then(|token| token.strip_prefix("Bearer "))
            .map(|token| token.to_string())
            .ok_or(GovernorError::Other {
                code: StatusCode::TOO_MANY_REQUESTS,
                msg: Some("".to_string()),
                headers: None,
            })
    }
}

impl KeyExtractor for GovIpOrGlobalExtractorHashed {
    type Key = Option<IpAddr>;

    fn extract<B>(&self, req: &Request<B>) -> Result<Self::Key, GovernorError> {
        dbg!();
        if let Ok(ip) = SmartIpKeyExtractor.extract(req) {
            Ok(Some(ip))
        } else {
            log::warn!("/signin rate limit: No IP found in request headers, using global limit");
            Ok(None)
        }
    }
}
