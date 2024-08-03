use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CampusLoginData {
    pub username: String,
    pub password: String,
}

// JWT Claims
#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,     // expiration time
    pub iat: usize,     // issued at
    pub nonce: String,  // AES nonce
    pub cipher: String, // AES cipher (CdAuthData)
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CdAuthData {
    pub cookie: String,
    pub hash: String,
    pub user: String,
    pub password: String,
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
    pub signup_until: Option<String>,
    pub internal_metadata: Option<ExamRegistrationMetadata>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExamRegistrationMetadata {
    pub assessment: String,
    pub peryr: String,
    pub perid: String,
    pub offerno: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CampusDualVerfahrenOption {
    pub name: String,
    pub verfahren: String,
    pub pruefart: String,
    pub status: String,
    pub signup_information: String,
    pub exam_date: Option<String>,
    pub exam_time: Option<String>,
    pub exam_room: Option<String>,
    pub warning_message: Option<String>,
    pub signoff_until: Option<String>,
    pub internal_metadata: Option<ExamRegistrationMetadata>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct UserBasicInfo {
    pub first_name: String,
    pub last_name: String,
    pub seminar_group: String,
    pub seminar_name: String,
    pub user: String,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct StundenplanItem {
    #[serde(rename = "allDay")]
    all_day: bool,
    pub color: String,
    pub white_font_recommended: Option<bool>,
    description: String,
    editable: bool,
    pub end: i64,
    instructor: String,
    remarks: String,
    room: String,
    sinstructor: String,
    sroom: String,
    pub start: i64,
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct LatestReminder {
    #[serde(rename(deserialize = "ACAD_SESSION"))]
    acad_session: String,
    #[serde(rename(deserialize = "ACAD_YEAR"))]
    acad_year: String,
    #[serde(rename(deserialize = "AGRDATE"))]
    agrdate: String,
    #[serde(rename(deserialize = "AGRTYPE"))]
    agrtype: String,
    #[serde(rename(deserialize = "AWOBJECT"))]
    awobject: String,
    #[serde(rename(deserialize = "AWOBJECT_SHORT"))]
    awobject_short: String,
    #[serde(rename(deserialize = "AWOTYPE"))]
    awotype: String,
    #[serde(rename(deserialize = "AWSTATUS"))]
    awstatus: String,
    #[serde(rename(deserialize = "BOOKDATE"))]
    bookdate: String,
    #[serde(rename(deserialize = "BOOKREASON"))]
    bookreason: String,
    #[serde(rename(deserialize = "CPGRADED"))]
    cpgraded: String,
    #[serde(rename(deserialize = "CPUNIT"))]
    cpunit: String,
    #[serde(rename(deserialize = "GRADESYMBOL"))]
    gradesymbol: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct UpcomingReminder {
    #[serde(rename(deserialize = "BEGUZ"))]
    beguz: String,
    #[serde(rename(deserialize = "COMMENT"))]
    comment: String,
    #[serde(rename(deserialize = "ENDUZ"))]
    enduz: String,
    #[serde(rename(deserialize = "EVDAT"))]
    evdat: String,
    #[serde(rename(deserialize = "INSTRUCTOR"))]
    instructor: String,
    #[serde(rename(deserialize = "LOCATION"))]
    location: String,
    #[serde(rename(deserialize = "OBJID"))]
    objid: String,
    #[serde(rename(deserialize = "ROOM"))]
    room: String,
    #[serde(rename(deserialize = "SINSTRUCTOR"))]
    sinstructor: String,
    #[serde(rename(deserialize = "SM_SHORT"))]
    sm_short: String,
    #[serde(rename(deserialize = "SM_STEXT"))]
    sm_stext: String,
    #[serde(rename(deserialize = "SROOM"))]
    sroom: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CampusReminders {
    #[serde(rename(deserialize = "ELECTIVES"))]
    electives: i64,
    #[serde(rename(deserialize = "EXAMS"))]
    exams: i64,
    #[serde(rename(deserialize = "LATEST"))]
    latest: Vec<LatestReminder>,
    #[serde(rename(deserialize = "SEMESTER"))]
    semester: i64,
    #[serde(rename(deserialize = "UPCOMING"))]
    upcoming: Vec<UpcomingReminder>,
}
