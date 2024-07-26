use anyhow::{Context, Result};
use regex::Regex;
use std::{sync::Arc, time::Instant};

use lazy_static::lazy_static;
use reqwest::Client;
use reqwest_cookie_store::{CookieStore, CookieStoreMutex};
use scraper::{Html, Selector};

use crate::types::{CampusLoginData, CdAuthData, UserBasicInfo};

pub async fn cdlogin_get_jcookie_and_meta(
    login_data: CampusLoginData,
) -> Result<(CdAuthData, UserBasicInfo)> {
    let cookie_store = Arc::new(CookieStoreMutex::new(CookieStore::new(None)));

    let client = reqwest::Client::builder()
        .cookie_provider(cookie_store.clone())
        .build()?;

    campus_login(&client, &login_data).await?;

    let (hash, user_basic_info) = get_hash_and_userinfo(&client).await?;

    let cd_auth_data = CdAuthData {
        cookie: extract_cd_cookie(cookie_store)?,
        hash,
        user: login_data.username,
        password: login_data.password,
    };

    Ok((cd_auth_data, user_basic_info))
}

async fn campus_login(client: &Client, login_data: &CampusLoginData) -> Result<()> {
    let whole_now = Instant::now();
    let resp = client
        .get("https://erp.campus-dual.de/sap/bc/webdynpro/sap/zba_initss?sap-client=100&sap-language=de&uri=https://selfservice.campus-dual.de/index/login")
        .send()
        .await?
        .error_for_status()?;
    println!("CD login req 1: {:.2?}", whole_now.elapsed());

    let now = Instant::now();

    let xsrf = {
        let document = Html::parse_document(&resp.text().await?);
        document
            .select(&Selector::parse(r#"input[name="sap-login-XSRF"]"#).unwrap())
            .next()
            .context("CD login stage 1: XSRF token missing")?
            .value()
            .attr("value")
            .context("CD login stage 1: XSRF token has no value")?
            .to_string()
    };

    let form = [
        ("sap-user", &login_data.username),
        ("sap-password", &login_data.password),
        ("sap-login-XSRF", &xsrf),
    ];

    println!("stage 1 form stuff: {:.2?}", now.elapsed());
    let now = Instant::now();

    let resp = client
        .post("https://erp.campus-dual.de/sap/bc/webdynpro/sap/zba_initss?uri=https%3a%2f%2fselfservice.campus-dual.de%2findex%2flogin&sap-client=100&sap-language=DE")
        .form(&form)
        .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/117.0.0.0 Safari/537.36")
        .send()
        .await?
        .error_for_status()?;

    println!("CD login req 2: {:.2?}", now.elapsed());
    let now = Instant::now();

    // if this cookie is set, the login was successful
    resp.cookies()
        .find(|c| c.domain().unwrap_or_default().contains("campus-dual.de"))
        .context("c-d.de cookie missing")?;

    println!("CD login cookie check: {:.2?}", now.elapsed());

    println!("CD login took {:.2?}", whole_now.elapsed());

    Ok(())
}

fn extract_cd_cookie(cookie_store: Arc<CookieStoreMutex>) -> Result<String> {
    let store = cookie_store.lock().unwrap();
    let cookie: &cookie_store::Cookie = store
        .iter_unexpired()
        .find(|c| c.domain().unwrap_or_default().contains("campus-dual.de"))
        .context("c-d.de cookie missing")?;

    Ok(serde_json::to_string(&cookie)?)
}

pub async fn get_hash_and_userinfo(client: &Client) -> Result<(String, UserBasicInfo)> {
    let whole = Instant::now();
    let mut user_basic_info = UserBasicInfo::default();

    let resp = client
        .get("https://selfservice.campus-dual.de/index/login")
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;

    println!("get hash and user info req: {:.2?}", whole.elapsed());
    let now = Instant::now();

    lazy_static! {
        static ref RE_HASH: Regex = Regex::new(r#"hash="(\w+)";user="(\d+)";"#).unwrap();
        static ref RE_STUDI: Regex = Regex::new(r#"<strong>Name:\s*</strong>(\w+),\s*(\w+).*<strong>\s*Seminargruppe:\s*</strong>([\w-]+).*<br>(.*)"#).unwrap();
    };

    let hash: String;

    if let Some(captures) = RE_HASH.captures(&resp) {
        hash = captures.get(1).unwrap().as_str().to_string();
        user_basic_info.user = captures.get(2).unwrap().as_str().to_string();
    } else {
        return Err(anyhow::anyhow!("Hash not found"));
    }

    if let Some(captures) = RE_STUDI.captures(&resp) {
        user_basic_info.last_name = captures.get(1).unwrap().as_str().to_string();
        user_basic_info.first_name = captures.get(2).unwrap().as_str().to_string();
        user_basic_info.seminar_group = captures.get(3).unwrap().as_str().to_string();
        user_basic_info.seminar_name = captures.get(4).unwrap().as_str().trim().to_string();
    }

    println!("get hash and user info parsing: {:.2?}", now.elapsed());

    Ok((hash, user_basic_info))
}
