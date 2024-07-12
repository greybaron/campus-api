use std::sync::Arc;

use anyhow::{Context, Result};
use cookie_store::CookieStore;
use reqwest::Url;
use reqwest_cookie_store::CookieStoreMutex;
use scraper::{Html, Selector};

use crate::types::{CampusDualGrade, CampusDualSignupOption, CampusDualSubGrade};

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

pub fn extract_grades(html_text: String) -> Result<Vec<CampusDualGrade>> {
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

        let total_passed_sel = Selector::parse("img").unwrap();
        let total_passed_el_opt = &content.next().unwrap().select(&total_passed_sel).next();

        let total_passed = total_passed_el_opt
            .as_ref()
            .map(|passed_el| passed_el.value().attr("src").unwrap().contains("green.png"));

        let credit_points = content
            .next()
            .unwrap()
            .text()
            .next()
            .unwrap()
            .trim_start()
            .parse::<i32>()
            .unwrap_or_default();
        let akad_period = content.nth(3).unwrap().text().next().unwrap().to_string();

        let subline_selector = &Selector::parse(&format!(".child-of-{}", l_id)).unwrap();
        let subgrade_elements = table.select(subline_selector);

        let mut subgrades: Vec<CampusDualSubGrade> = Vec::new();
        for subline in subgrade_elements {
            let content_selector = &Selector::parse("td").unwrap();
            let mut content = subline.select(content_selector);
            let sub_name = content
                .next()
                .unwrap()
                .text()
                .next()
                .unwrap()
                .trim_start()
                .to_string();
            let sub_grade = content.next().unwrap().text().next().unwrap();

            let passed_sel = Selector::parse("img").unwrap();
            let passed_el_opt = &content.next().unwrap().select(&passed_sel).next();
            let passed = passed_el_opt
                .as_ref()
                .map(|passed_el| passed_el.value().attr("src").unwrap().contains("green.png"));

            let beurteilung = content.nth(1).unwrap().text().next().unwrap().to_string();
            let bekanntgabe = content.next().unwrap().text().next().unwrap().to_string();
            let wiederholung = content.next().unwrap().text().next().map(|s| s.to_string());
            let akad_period = content.next().unwrap().text().next().unwrap().to_string();

            let bloat = CampusDualSubGrade {
                name: sub_name,
                grade: sub_grade.to_string(),
                passed,
                beurteilung,
                bekanntgabe,
                wiederholung,
                akad_period,
            };
            subgrades.push(bloat);
        }

        grades.push(CampusDualGrade {
            name: name.to_string(),
            grade: grade.to_string(),
            total_passed,
            credit_points,
            akad_period,
            subgrades,
        });
    }

    println!("{:#?}", grades);

    Ok(grades)
}

pub async fn extract_exam_signup_options(html_text: String) -> Result<Vec<CampusDualSignupOption>> {
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
