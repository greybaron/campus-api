use anyhow::{Context, Result};
use std::sync::Arc;

use reqwest::Client;
use reqwest_cookie_store::{CookieStore, CookieStoreMutex};
use scraper::{Html, Selector};

use crate::types::CampusLoginData;

pub async fn cdlogin_get_cookie_json(login_data: &CampusLoginData) -> Result<String> {
    let cookie_store = Arc::new(CookieStoreMutex::new(CookieStore::new(None)));

    let client = reqwest::Client::builder()
        .cookie_provider(cookie_store.clone())
        .build()?;

    campus_login(&client, login_data).await?;
    extract_cd_cookie(cookie_store)
}

async fn campus_login(client: &Client, login_data: &CampusLoginData) -> Result<()> {
    let resp = client
        .get("https://erp.campus-dual.de/sap/bc/webdynpro/sap/zba_initss?sap-client=100&sap-language=de&uri=https://selfservice.campus-dual.de/index/login")
        .send()
        .await?
        .error_for_status()?;

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

    let resp = client
        .post("https://erp.campus-dual.de/sap/bc/webdynpro/sap/zba_initss?uri=https%3a%2f%2fselfservice.campus-dual.de%2findex%2flogin&sap-client=100&sap-language=DE")
        .form(&form)
        .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/117.0.0.0 Safari/537.36")
        .send()
        .await?
        .error_for_status()?;

    // check if title of redirect page implicates successful login
    {
        let zba_init_doc = Html::parse_document(&resp.text().await.unwrap());
        match zba_init_doc
            .select(&Selector::parse("title").unwrap())
            .next()
            .unwrap()
            .inner_html()
            .as_str()
        {
            "Initialisierung Selfservices" => Ok(()),
            _ => Err(anyhow::anyhow!("Bad credentials")),
        }
    }
}

fn extract_cd_cookie(cookie_store: Arc<CookieStoreMutex>) -> Result<String> {
    let cookie = {
        let store = cookie_store.lock().unwrap();
        let cookie: cookie_store::Cookie = store
            .iter_unexpired()
            .find(|c| c.domain().unwrap_or_default().contains("campus-dual.de"))
            .context("c-d.de cookie missing")?
            .clone();

        cookie
    };
    dbg!(&cookie);

    Ok(serde_json::to_string(&cookie)?)
}
