use axum::{Extension, Json};
use fnv::FnvHasher;
use std::{
    hash::{Hash, Hasher},
    time::Instant,
};

use crate::{
    campus_backend::req_client_funcs::{
        extract_exam_signup_options, extract_grades, get_client_with_cd_cookie,
    },
    types::{
        CampusDualGrade, CampusDualSignupOption, CdAuthdataExt, CdExamStats, ResponseError,
        StundenplanItem,
    },
};

pub async fn get_grades(
    Extension(cd_cookie_and_hash): Extension<CdAuthdataExt>,
) -> Result<Json<Vec<CampusDualGrade>>, ResponseError> {
    let now = Instant::now();
    let client = get_client_with_cd_cookie(cd_cookie_and_hash.cookie)?;
    println!("Time to get client: {:.2?}", now.elapsed());

    let now = Instant::now();

    let grade_html = client
        .get("https://selfservice.campus-dual.de/acwork/index")
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;
    println!("get grades req: {:.2?}", now.elapsed());

    let now = Instant::now();

    let grades = extract_grades(grade_html)?;
    println!("extract grades: {:.2?}", now.elapsed());

    Ok(Json(grades))
}

pub async fn get_signup_options(
    Extension(cd_cookie_and_hash): Extension<CdAuthdataExt>,
) -> Result<Json<Vec<CampusDualSignupOption>>, ResponseError> {
    let client = get_client_with_cd_cookie(cd_cookie_and_hash.cookie)?;
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

pub async fn get_ects(
    Extension(cd_authdata): Extension<CdAuthdataExt>,
) -> Result<String, ResponseError> {
    // dbg!(cd_cookie_and_hash);
    let client = reqwest::Client::new();

    let user = cd_authdata.user;
    let hash = cd_authdata.hash;

    let resp = client
        .get(format!(
            "https://selfservice.campus-dual.de/dash/getcp?user={user}&hash={hash}"
        ))
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;

    // todo!();
    Ok(resp)
}

pub async fn get_fachsem(
    Extension(cd_authdata): Extension<CdAuthdataExt>,
) -> Result<String, ResponseError> {
    let client = reqwest::Client::new();

    let user = cd_authdata.user;
    let hash = cd_authdata.hash;

    let resp = client
        .get(format!(
            "https://selfservice.campus-dual.de/dash/getfs?user={user}&hash={hash}"
        ))
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;

    // Remove the quotes from the string, parse number
    let whyisthisnecessary = resp.replace('"', "");
    let number = whyisthisnecessary.trim().parse::<u32>();

    match number {
        Ok(num) => Ok(num.to_string()),
        Err(_) => Err(ResponseError {
            message: "CampusDual returned garbage".to_string(),
            status_code: http::StatusCode::INTERNAL_SERVER_ERROR,
        }),
    }
}

pub async fn get_examstats(
    Extension(cd_authdata): Extension<CdAuthdataExt>,
) -> Result<Json<CdExamStats>, ResponseError> {
    // CAMPUSDUAL PIECHART:
    // daten/partitionen: ['erfolgreich', 0], ['nicht bestanden', 0], ['gebucht', 0]
    // farben: ["#0070a3", "#4297d7", "#fcbe04"]

    let client = reqwest::Client::new();

    let user = cd_authdata.user;
    let hash = cd_authdata.hash;

    let resp = client
        .get(format!(
            "https://selfservice.campus-dual.de/dash/getexamstats?user={user}&hash={hash}"
        ))
        .send()
        .await?
        .error_for_status()?
        .json::<CdExamStats>()
        .await?;

    Ok(Json(resp))
}

pub async fn get_stundenplan(
    Extension(cd_authdata): Extension<CdAuthdataExt>,
) -> Result<Json<Vec<StundenplanItem>>, ResponseError> {
    let client = reqwest::Client::new();

    let user = cd_authdata.user;
    let hash = cd_authdata.hash;

    let mut stundenplan: Vec<StundenplanItem> = client
        .get(format!(
            "https://selfservice.campus-dual.de/room/json?userid={user}&hash={hash}&start=1720735200&end=1720821600"
        ))
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    for item in &mut stundenplan {
        item.start *= 1000;
        item.end *= 1000;
        item.color = match item.color.as_str() {
            "dunkelrot" => "#8B0000".to_string(),
            _ => string_to_rgb(&item.title),
        }
    }

    Ok(Json(stundenplan))
}

fn string_to_rgb(input: &str) -> String {
    // Create a hasher
    let mut hasher = FnvHasher::default();

    // Hash the input string
    input.hash(&mut hasher);
    let hash = hasher.finish();

    // Extract RGB components from the hash
    let r = (hash & 0xFF) as u8;
    let g = ((hash >> 8) & 0xFF) as u8;
    let b = ((hash >> 16) & 0xFF) as u8;

    format!("#{:02X}{:02X}{:02X}", r, g, b)
}
