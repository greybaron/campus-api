use std::sync::Arc;

use anyhow::{Context, Result};
use cookie_store::CookieStore;
use reqwest::Url;
use reqwest_cookie_store::CookieStoreMutex;
use scraper::{Html, Selector};

use crate::types::{CampusDualGrade, CampusDualSignupOption};

pub fn get_client_with_cd_cookie(j_cookie: String) -> Result<reqwest::Client> {
    let cookie: cookie_store::Cookie = serde_json::from_str(&j_cookie)?;
    let cookie_store = Arc::new(CookieStoreMutex::new(CookieStore::new(None)));
    {
        let mut store = cookie_store.lock().unwrap();
        store.insert(cookie, &Url::parse("https://campus-dual.de")?)?;
    }

    Ok(reqwest::Client::builder()
        .cookie_provider(cookie_store)
        .build()?)
}

pub fn extract_grades(html_text: String) -> anyhow::Result<Vec<CampusDualGrade>> {
    let mut grades = Vec::new();

    let document = Html::parse_document(&html_text);
    let table = document
        .select(&Selector::parse("#acwork tbody").unwrap())
        .next()
        .context("CD grades page: #acwork tbody missing")?;
    let top_level_line_selector = Selector::parse(".child-of-node-0").unwrap();
    let top_level_lines = table.select(&top_level_line_selector);
    for line in top_level_lines {
        let l_id = line
            .value()
            .attr("id")
            .context("CD: grades table line has no ID")?;
        let content_selector = &Selector::parse("td").unwrap();
        let mut content = line.select(content_selector);
        let name = content.next().unwrap().text().next().unwrap();
        let grade = content.next().unwrap().text().next().unwrap();

        let subline_selector = &Selector::parse(&format!(".child-of-{}", l_id)).unwrap();
        let sub_count = table.select(subline_selector).count();

        grades.push(CampusDualGrade {
            name: name.to_string(),
            grade: grade.to_string(),
            subgrades: sub_count,
        });
    }

    Ok(grades)
}

pub async fn extract_exam_signup_options(
    html_text: String,
) -> anyhow::Result<Vec<CampusDualSignupOption>> {
    let mut signup_options = Vec::new();

    let document = Html::parse_document(&html_text);
    let table = document
        .select(&Selector::parse("#expproc tbody").unwrap())
        .next()
        .unwrap();
    let top_level_line_selector = Selector::parse(".child-of-node-0").unwrap();
    let top_level_lines = table.select(&top_level_line_selector);
    for line in top_level_lines {
        let l_id = line.value().attr("id").unwrap();
        let content_selector = &Selector::parse("td").unwrap();
        let mut content = line.select(content_selector);
        let class = content.next().unwrap().text().next().unwrap();
        let verfahren = content.next().unwrap().text().next().unwrap();

        let subline_selector = &Selector::parse(&format!(".child-of-{l_id}")).unwrap();
        let status_icon_url = table
            .select(subline_selector)
            .next()
            .unwrap()
            .select(&Selector::parse("img").unwrap())
            .next()
            .unwrap()
            .value()
            .attr("src")
            .unwrap();

        let status = match status_icon_url {
            "/images/missed.png" => "🚫",
            "/images/yellow.png" => "📝",
            "/images/exclamation.jpg" => "⚠️",
            _ => "???",
        };

        signup_options.push(CampusDualSignupOption {
            name: class.to_string(),
            verfahren: verfahren.to_string(),
            status: status.to_string(),
        });
    }

    Ok(signup_options)
}
