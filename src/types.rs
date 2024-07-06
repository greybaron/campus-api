use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CampusLoginData {
    pub username: String,
    pub password: String,
}

// JWT Claims
#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub iat: usize,
    pub cdcookie: String,
}

// API Response type
pub struct ResponseError {
    pub message: String,
    pub status_code: StatusCode,
}

#[derive(serde::Serialize)]
    pub struct Token {
        pub token: String,
    }

// Inserted by the auth middleware into the request extension
#[derive(Clone)]
pub struct UserCookieExt {
    pub cookie: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CampusDualGrade {
    pub name: String,
    pub grade: String,
    pub subgrades: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CampusDualSignupOption {
    pub name: String,
    pub verfahren: String,
    pub status: String,
}
