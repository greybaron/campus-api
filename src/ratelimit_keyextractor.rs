use http::{request::Request, StatusCode};
use serde::{Deserialize, Serialize};
use tower_governor::{errors::GovernorError, key_extractor::KeyExtractor};

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct GovJwtExtractorHashed;

// #[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
// pub struct GovUnameExtractorHashed;

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

// impl KeyExtractor for GovUnameExtractorHashed {
//     type Key = String;

//     fn extract<B>(&self, req: &Request<B>) -> Result<Self::Key, GovernorError> {
//         let huh = req.body().TryInto::<String>::try_into(req.body())

//         req.headers()
//             .get("Authorization")
//             .and_then(|token| token.to_str().ok())
//             .and_then(|token| token.strip_prefix("Bearer "))
//             .map(|token| token.to_string())
//             .ok_or(GovernorError::Other {
//                 code: StatusCode::INTERNAL_SERVER_ERROR,
//                 msg: Some("".to_string()),
//                 headers: None,
//             })
//     }
// }
