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
    pub exp: usize,       // expiration time
    pub iat: usize,       // issued at
    pub cdcookie: String, // Campus Dual cookie (for first type of CD requests)
    pub cduser: String,   // Campus Dual user   (for second type)
    pub cdhash: String,   // Campus Dual hash   (for second type)
}

// API Response type
pub struct ResponseError {
    pub message: String,
    pub status_code: StatusCode,
}

#[derive(serde::Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: UserBasicInfo,
}

// Inserted by the auth middleware into the request extension
#[derive(Debug, Clone)]
pub struct CdAuthdataExt {
    pub cookie: String,
    pub user: String,
    pub hash: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CampusDualGrade {
    pub name: String,
    pub grade: String,
    pub total_passed: Option<bool>,
    pub credit_points: i32,
    pub akad_period: String,
    pub subgrades: Vec<CampusDualSubGrade>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CampusDualSubGrade {
    pub name: String,
    pub grade: String,
    pub passed: Option<bool>,
    pub beurteilung: String,
    pub bekanntgabe: String,
    pub wiederholung: Option<String>,
    pub akad_period: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CampusDualSignupOption {
    pub name: String,
    pub verfahren: String,
    pub pruefart: String,
    pub status: String,
    pub signup_information: String,
    pub exam_date: Option<String>,
    pub exam_time: Option<String>,
    pub exam_room: Option<String>,
    pub warning_message: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct UserBasicInfo {
    pub first_name: String,
    pub last_name: String,
    pub seminar_group: String,
    pub seminar_name: String,
    pub user: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CdAuthData {
    pub cookie: String,
    pub user: String,
    pub hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CdExamStats {
    #[serde(rename(deserialize = "EXAMS"))]
    pub total: i64,

    #[serde(rename(deserialize = "SUCCESS"))]
    pub successful: i64,

    #[serde(rename(deserialize = "FAILURE"))]
    pub unsuccessful: i64,

    #[serde(rename(deserialize = "BOOKED"))]
    pub unassessed: i64,

    #[serde(rename(deserialize = "MBOOKED"))]
    pub booked: i64,

    #[serde(rename(deserialize = "MODULES"))]
    pub finished: i64,

    #[serde(rename(deserialize = "WPCOUNT"))]
    pub ronmodus: i64,
}
