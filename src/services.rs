use axum::{Extension, Json};
use serde::{Deserialize, Serialize};
// use anyhow::Result;

use crate::{
    campus_backend::req_client_funcs::{
        extract_exam_signup_options, extract_grades, get_client_with_cd_cookie,
    },
    types::{CampusDualGrade, CampusDualSignupOption, ResponseError, UserCookieExt},
};

#[derive(Serialize, Deserialize)]
struct UserResponse {
    test: String,
}

pub async fn get_grades(
    Extension(cd_cookie): Extension<UserCookieExt>,
) -> Result<Json<Vec<CampusDualGrade>>, ResponseError> {
    let client = get_client_with_cd_cookie(cd_cookie.cookie)?;

    let grade_html = client
        .get("https://selfservice.campus-dual.de/acwork/index")
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;

    let grades = extract_grades(grade_html)?;

    Ok(Json(grades))
}

pub async fn get_signup_options(
    Extension(cd_cookie): Extension<UserCookieExt>,
) -> Result<Json<Vec<CampusDualSignupOption>>, ResponseError> {
    let client = get_client_with_cd_cookie(cd_cookie.cookie)?;
    let exam_signup_html = client
        .get("https://selfservice.campus-dual.de/acwork/expproc")
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;

    let signup_options = extract_exam_signup_options(exam_signup_html).await?;

    Ok(Json(signup_options))
}
