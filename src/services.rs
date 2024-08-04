use anyhow::Context;
use axum::{Extension, Json};
use fnv::FnvHasher;
use http::StatusCode;
use scraper::{node, Html, Selector};
use std::{
    hash::{Hash, Hasher},
    time::Instant,
};

use crate::{
    auth::sign_in,
    campus_backend::req_client_funcs::{
        extract_exam_signup_options, extract_exam_verfahren_options, extract_grades,
        get_client_with_cd_cookie,
    },
    color_stuff::hex_to_luminance,
    types::{
        CampusDualGrade, CampusDualSignupOption, CampusDualVerfahrenOption, CampusLoginData,
        CampusReminders, CampusTimeline, CdAuthData, CdExamStats, DocumentOption,
        ExamRegistrationMetadata, LoginResponse, ResponseError, StundenplanItem, TimelineEvent,
    },
};

pub async fn get_grades(
    Extension(cd_auth_data): Extension<CdAuthData>,
) -> Result<Json<Vec<CampusDualGrade>>, ResponseError> {
    let now = Instant::now();
    let client = get_client_with_cd_cookie(cd_auth_data.cookie)?;
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

pub async fn check_revive_session(
    Extension(cd_auth_data): Extension<CdAuthData>,
) -> Result<Json<Option<LoginResponse>>, ResponseError> {
    println!("checking session...");

    let client = get_client_with_cd_cookie(cd_auth_data.cookie)?;

    let resp = client
        .get("https://erp.campus-dual.de/sap/bc/webdynpro/sap/zba_initss?sap-client=100&sap-language=de&uri=https://selfservice.campus-dual.de/index/login")
        .send()
        .await?;

    match resp.status().as_u16() {
        // 200 means the old session is not alive anymore
        200 => {
            println!("session was dead");
            let new_login_response = sign_in(Json(CampusLoginData {
                username: cd_auth_data.user,
                password: cd_auth_data.password,
            }))
            .await;

            println!("revive ok?={}", new_login_response.is_ok());

            match new_login_response {
                Ok(Json(login_response)) => Ok(Json(Some(login_response))),
                Err(_) => Err(ResponseError {
                    message: "CD healthcheck failed".to_string(),
                    status_code: StatusCode::UNAUTHORIZED,
                }),
            }
        }
        500 => Ok(Json(None)),
        _ => Err(ResponseError {
            message: "CD healthcheck failed".to_string(),
            status_code: resp.status(),
        }),
    }
}

pub async fn get_examsignup(
    Extension(cd_auth_data): Extension<CdAuthData>,
) -> Result<Json<Vec<CampusDualSignupOption>>, ResponseError> {
    let client = get_client_with_cd_cookie(cd_auth_data.cookie)?;
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

pub async fn post_registerexam(
    Extension(cd_auth_data): Extension<CdAuthData>,
    Json(examregist_meta): Json<ExamRegistrationMetadata>,
) -> Result<String, ResponseError> {
    let client = reqwest::Client::new();
    let exam_regist_resp = client
        .get(format!(
            "https://selfservice.campus-dual.de/acwork/registerexam?userid={}&assessment={}&peryr={}&perid={}&offerno={}&hash={}",
            cd_auth_data.user,
            examregist_meta.assessment,
            examregist_meta.peryr,
            examregist_meta.perid,
            examregist_meta.offerno,
            cd_auth_data.hash,
        ))
        .send()
        .await?
        .error_for_status()?;

    dbg!(exam_regist_resp.status());

    Ok(exam_regist_resp.text().await?)
}

pub async fn post_cancelexam(
    Extension(cd_auth_data): Extension<CdAuthData>,
    Json(examregist_meta): Json<ExamRegistrationMetadata>,
) -> Result<String, ResponseError> {
    let client = reqwest::Client::new();
    let exam_regist_resp = client
        .get(format!(
            "https://selfservice.campus-dual.de/acwork/cancelexam?userid={}&objid={}&hash={}",
            cd_auth_data.user, examregist_meta.assessment, cd_auth_data.hash
        ))
        .send()
        .await?
        .error_for_status()?;

    dbg!(exam_regist_resp.status());

    Ok(exam_regist_resp.text().await?)
}

pub async fn get_examverfahren(
    Extension(cd_auth_data): Extension<CdAuthData>,
) -> Result<Json<Vec<CampusDualVerfahrenOption>>, ResponseError> {
    let client = get_client_with_cd_cookie(cd_auth_data.cookie)?;
    let exam_verfahren_html = client
        .get("https://selfservice.campus-dual.de/acwork/cancelproc")
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;

    let signup_verfahren = extract_exam_verfahren_options(exam_verfahren_html).await?;

    Ok(Json(signup_verfahren))
}

pub async fn get_ects(
    Extension(cd_authdata): Extension<CdAuthData>,
) -> Result<String, ResponseError> {
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
    Extension(cd_authdata): Extension<CdAuthData>,
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
    Extension(cd_authdata): Extension<CdAuthData>,
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
    Extension(cd_authdata): Extension<CdAuthData>,
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
            "darkred" => "#D41610".to_string(),
            _ => string_to_rgb(&format!("0{}0", item.title)),
        };
        item.font_color = Some(
            if hex_to_luminance(&item.color) < 128.0 {
                "#FFFFFF"
            } else {
                "#000000"
            }
            .to_string(),
        );
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

pub async fn get_reminders(
    Extension(cd_authdata): Extension<CdAuthData>,
) -> Result<Json<CampusReminders>, ResponseError> {
    let client = reqwest::Client::new();

    let user = cd_authdata.user;
    let hash = cd_authdata.hash;

    let resp = client
        .get(format!(
            "https://selfservice.campus-dual.de/dash/getreminders?user={user}&hash={hash}"
        ))
        .send()
        .await?
        .error_for_status()?
        .json::<CampusReminders>()
        .await?;

    Ok(Json(resp))
}

pub async fn get_timeline(
    Extension(cd_authdata): Extension<CdAuthData>,
) -> Result<Json<Vec<TimelineEvent>>, ResponseError> {
    let client = reqwest::Client::new();
    let resp: CampusTimeline = client
        .get(format!(
            "https://selfservice.campus-dual.de/dash/gettimeline?user={}",
            cd_authdata.user
        ))
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    Ok(Json(resp.events))
}

pub async fn get_documentlist(
    Extension(cd_auth_data): Extension<CdAuthData>,
) -> Result<Json<Vec<DocumentOption>>, ResponseError> {
    let client = get_client_with_cd_cookie(cd_auth_data.cookie)?;

    let stage1 = client
        .get("https://erp.campus-dual.de/sap/bc/webdynpro/sap/zba_printss?sap-language=DE&sap-wd-configId=ZBA_AC_SOAP_PSS")
        .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:128.0) Gecko/20100101 Firefox/128.0")
        .send()
        .await?
        .error_for_status()?
        .text().await?;
    dbg!();
    std::fs::write("stage1fucked.html", &stage1).unwrap();

    let (action, secure_id) = {
        let document = Html::parse_document(&stage1);
        let form = document
            .select(&Selector::parse(r#"form[name="sap.client.SsrClient.form"]"#).unwrap())
            .next()
            .unwrap();
            // .context("Documents stage 1: form missing")?;
        let action = form
            .value()
            .attr("action")
            .context("Documents stage 1: form-action missing")?
            .to_string();
        let input_wdsecid = form
            .select(&Selector::parse("input#sap-wd-secure-id").unwrap())
            .next()
            .unwrap();
            // .context("sap-wd-secure-id not found")?;
        let secure_id = input_wdsecid.value().attr("value").unwrap().to_string();

        (action, secure_id)
    };

    dbg!(&action, secure_id);

    let stage2 = client
        .post(format!("https://erp.campus-dual.de{action}"))
        .header(
            "User-Agent",
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:128.0) Gecko/20100101 Firefox/128.0",
        )
        .send()
        .await?;
        // .text()
        // .await?;

    println!("{:#?}", stage2.headers());
    let stage2= stage2.text().await?;

    let options: Vec<DocumentOption> = {
        let document = Html::parse_document(&stage2);
        dbg!();

        document
            .select(&Selector::parse(".lsLink__text").unwrap())
            .map(|el| {
                dbg!(el.attr("id"));

                DocumentOption {
                name: el.inner_html(),
                id: el.value().attr("id").unwrap().replace("-text", "").to_string(),
            }
        })
            .collect()
    };

    let action2 = {
        let document = Html::parse_document(&stage2);
        document
            .select(&Selector::parse(r#"form[name="sap.client.SsrClient.form"]"#).unwrap())
            .next()
            .context("Documents stage 1: form missing")?
            .value()
            .attr("action")
            .context("Documents stage 1: form-action missing")?
            .to_string()
    };

    dbg!(&action2);

    let stage3 = client
        .post(format!("https://erp.campus-dual.de{action2}"))
        .header(
            "User-Agent",
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:128.0) Gecko/20100101 Firefox/128.0",
        )
        .header("Prority", "u=0")
        .header("SAP-Perf-FESRec", "F0D23546D21241D59ADAD85168CB1617,C031904A47BDB0ECE96C15F228000000,2,2396,3149,2,START_0,102,2396,mac_10.15,SAP_WDA")
        .header("SAP-Perf-FESRec-opt", "ZBA_PRINTSS,START,,ff_128,0,0,,,751,,,,,,,,2,,20240804205241102,ZBA_PRINTSS")
        .send()
        .await?;

    dbg!(stage3.status(), stage3.text().await?);

   

    Ok(Json(options))
}
