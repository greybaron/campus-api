use std::sync::Arc;

use anyhow::{Context, Result};
use chrono::NaiveDate;
use cookie_store::CookieStore;
use lazy_static::lazy_static;
use regex::Regex;
use reqwest::Url;
use reqwest_cookie_store::CookieStoreMutex;
use scraper::{selectable::Selectable, Html, Selector};

use crate::{
    constants::CD_CERT_PEM,
    types::{
        CampusDualGrade, CampusDualSignupOption, CampusDualSubGrade, CampusDualVerfahrenOption,
        ExamRegistrationMetadata, SubGradeMetadata,
    },
};

pub fn get_client_default() -> reqwest::Client {
    reqwest::ClientBuilder::new()
        .add_root_certificate(CD_CERT_PEM.get().unwrap().clone())
        .use_rustls_tls()
        .build()
        .unwrap()
}

pub fn get_client_with_cd_cookie(j_cookie: String) -> Result<reqwest::Client> {
    let cookie: cookie_store::Cookie = serde_json::from_str(&j_cookie)?;
    let cookie_store = Arc::new(CookieStoreMutex::new(CookieStore::new(None)));
    {
        let mut store = cookie_store.lock().unwrap();
        store.insert(cookie, &Url::parse("https://campus-dual.de")?)?;
    }

    Ok(reqwest::Client::builder()
        .add_root_certificate(CD_CERT_PEM.get().unwrap().clone())
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

            let internal_metadata_opt = subline
                .select(&Selector::parse("td>div#mscore>a").unwrap())
                .next();

            let internal_metadata = internal_metadata_opt.and_then(|internal_metadata| {
                let module = internal_metadata.attr("data-module")?;
                let peryr = internal_metadata.attr("data-peryr")?;
                let perid = internal_metadata.attr("data-perid")?;

                Some(SubGradeMetadata {
                    module: module.to_string(),
                    peryr: peryr.to_string(),
                    perid: perid.to_string(),
                })
            });

            let bloat = CampusDualSubGrade {
                name: sub_name,
                grade: sub_grade.to_string(),
                passed,
                beurteilung,
                bekanntgabe,
                wiederholung,
                akad_period,
                internal_metadata,
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

    // get teilpruefungen
    let teilpruefung_toplevel_selector = Selector::parse(".child-of-node-1000").unwrap();
    for line in table.select(&teilpruefung_toplevel_selector) {
        let content_selector = &Selector::parse("td").unwrap();
        let mut content = line.select(content_selector);
        let name = content.next().unwrap().text().next().unwrap().trim();
        let grade = content.next().unwrap().text().next().unwrap();

        let total_passed_sel = Selector::parse("img").unwrap();
        let total_passed_el_opt = &content.next().unwrap().select(&total_passed_sel).next();

        let total_passed = total_passed_el_opt
            .as_ref()
            .map(|passed_el| passed_el.value().attr("src").unwrap().contains("green.png"));
        let beurteilung = content.nth(1).unwrap().text().next().unwrap().to_string();
        let bekanntgabe = content.next().unwrap().text().next().unwrap().to_string();

        let credit_points = 0;
        let akad_period = content.nth(1).unwrap().text().next().unwrap().to_string();

        let subgrades = vec![CampusDualSubGrade {
            name: name.to_string(),
            grade: grade.to_string(),
            passed: total_passed,
            beurteilung,
            bekanntgabe,
            wiederholung: None,
            akad_period: akad_period.clone(),
            internal_metadata: None,
        }];

        grades.push(CampusDualGrade {
            name: name.to_string(),
            grade: grade.to_string(),
            total_passed,
            credit_points,
            akad_period,
            subgrades,
        });
    }

    grades.sort_by(|grade_a, grade_b| {
        get_newest_subgrade_date(grade_b).cmp(&get_newest_subgrade_date(grade_a))
    });

    Ok(grades)
}

fn get_newest_subgrade_date(grade: &CampusDualGrade) -> NaiveDate {
    let newest_subgrade = grade
        .subgrades
        .iter()
        .max_by(|a, b| a.bekanntgabe.cmp(&b.bekanntgabe))
        .unwrap();

    NaiveDate::parse_from_str(&newest_subgrade.bekanntgabe, "%d.%m.%Y").unwrap()
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

        let name = content.next().unwrap().text().next().unwrap().to_string();
        let verfahren = content.next().unwrap().text().next().unwrap().to_string();
        let pruefart = content.next().unwrap().text().next().unwrap().to_string();

        let subline_selector = &Selector::parse(&format!(".child-of-{l_id}")).unwrap();
        let mut sublines = table.select(subline_selector);
        let main_subline = sublines.next().unwrap();

        let internal_metadata = main_subline
            .select(&Selector::parse("td>a.booking").unwrap())
            .next()
            .map(|meta_el| ExamRegistrationMetadata {
                assessment: meta_el.value().attr("data-evob_objid").unwrap().to_string(),
                peryr: meta_el.value().attr("data-peryr").unwrap().to_string(),
                perid: meta_el.value().attr("data-perid").unwrap().to_string(),
                offerno: meta_el.value().attr("data-offerno").unwrap().to_string(),
            });

        let status_icon_url = main_subline
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
            _ => "⁉️",
        }
        .to_string();

        // my shoddy code demands that the iterator is over owned values and not references,
        // else the iterator doesn't consume the values and causes wrapping after the first None
        let mut main_subline_texts = main_subline.text().collect::<Vec<_>>().into_iter();

        if main_subline_texts.len() == 0 {
            signup_options.push(CampusDualSignupOption {
                name,
                verfahren,
                pruefart,
                status,
                signup_information: "Daten konnten nicht extrahiert werden".to_string(),
                exam_date: None,
                exam_time: None,
                exam_room: None,
                warning_message: None,
                signup_until: None,
                internal_metadata: None,
            });

            continue;
        }

        let signup_information_messy = main_subline_texts.next().unwrap().trim_start();
        let signup_information =
            if let Some(stripped) = signup_information_messy.strip_suffix(", Prüfungstermin: ") {
                stripped
            } else {
                signup_information_messy
            }
            .to_string();

        let exam_date = main_subline_texts.next().map(|el| el.to_string());
        let exam_time = main_subline_texts.nth(1).map(|el| el.to_string());
        let exam_room = main_subline_texts
            .next()
            .map(|el| el.strip_prefix(", ").unwrap_or(el).to_string());

        let warning_message = sublines.next().map(|second_subline| {
            second_subline
                .text()
                .fold(String::new(), |a, b| a + b)
                .trim_start()
                // campusdual html is fucked in case of no time, so replace that but leave 1 space
                .replace("   :  ", "")
                .to_string()
        });
        lazy_static! {
            static ref RE: Regex = Regex::new(r"bis (\d{2}\.\d{2}\.\d{4})").unwrap();
        }
        let signup_until = warning_message.as_ref().and_then(|msg| {
            RE.captures(msg)
                .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()))
        });

        signup_options.push(CampusDualSignupOption {
            name,
            verfahren,
            pruefart,
            status,
            signup_information,
            exam_date,
            exam_time,
            exam_room,
            warning_message,
            signup_until,
            internal_metadata,
        });
    }

    Ok(signup_options)
}

pub async fn extract_exam_verfahren_options(
    html_text: String,
) -> Result<Vec<CampusDualVerfahrenOption>> {
    let mut signup_options = Vec::new();

    let document = Html::parse_document(&html_text);
    let table = document
        .select(&Selector::parse("#exopen tbody").unwrap())
        .next()
        .unwrap();
    let top_level_line_selector = Selector::parse(".child-of-node-0").unwrap();
    let top_level_lines = table.select(&top_level_line_selector);
    for line in top_level_lines {
        let l_id = line.value().attr("id").unwrap();
        let content_selector = &Selector::parse("td").unwrap();
        let mut content = line.select(content_selector);

        let name = content.next().unwrap().text().next().unwrap().to_string();
        let verfahren = content.next().unwrap().text().next().unwrap().to_string();
        let pruefart = content.next().unwrap().text().next().unwrap().to_string();

        let subline_selector = &Selector::parse(&format!(".child-of-{l_id}")).unwrap();
        let mut sublines = table.select(subline_selector);
        let main_subline = sublines.next().unwrap();

        let internal_metadata = main_subline
            .select(&Selector::parse("td>a.booking").unwrap())
            .next()
            .map(|meta_el| ExamRegistrationMetadata {
                assessment: meta_el.value().attr("data-evob_objid").unwrap().to_string(),
                peryr: meta_el.value().attr("data-peryr").unwrap().to_string(),
                perid: meta_el.value().attr("data-perid").unwrap().to_string(),
                offerno: meta_el.value().attr("data-offerno").unwrap().to_string(),
            });

        let status_icon_url = main_subline
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
            _ => "⁉️",
        }
        .to_string();

        // my shoddy code demands that the iterator is over owned values and not references,
        // else the iterator doesn't consume the values and causes wrapping after the first None
        let mut main_subline_texts = main_subline.text().collect::<Vec<_>>().into_iter();

        if main_subline_texts.len() == 0 {
            signup_options.push(CampusDualVerfahrenOption {
                name,
                verfahren,
                pruefart,
                status,
                signup_information: "Daten konnten nicht extrahiert werden".to_string(),
                exam_date: None,
                exam_time: None,
                exam_room: None,
                warning_message: None,
                signoff_until: None,
                internal_metadata: None,
            });

            continue;
        }

        let signup_information_messy = main_subline_texts.next().unwrap().trim_start();
        let signup_information = if let Some(stripped) = signup_information_messy
            .split_once("Prüfungstermin")
            .map(|split| split.0.replace(", ", ""))
        {
            stripped
        } else {
            signup_information_messy.to_string()
        };

        let exam_date = main_subline_texts.next().map(|el| el.to_string());
        let exam_time = main_subline_texts.nth(1).map(|el| el.to_string());
        let exam_room = main_subline_texts
            .next()
            .map(|el| el.strip_prefix(", ").unwrap_or(el).to_string());

        let warning_message = sublines.next().map(|second_subline| {
            second_subline
                .text()
                .fold(String::new(), |a, b| a + b)
                .trim_start()
                // campusdual html is fucked in case of no time, so replace that but leave 1 space
                .replace("   :  ", "")
                .to_string()
        });
        lazy_static! {
            static ref RE: Regex = Regex::new(r"bis zum (\d{2}\.\d{2}\.\d{4})").unwrap();
        }
        let signoff_until = warning_message.as_ref().and_then(|msg| {
            RE.captures(msg)
                .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()))
        });

        signup_options.push(CampusDualVerfahrenOption {
            name,
            verfahren,
            pruefart,
            status,
            signup_information,
            exam_date,
            exam_time,
            exam_room,
            warning_message,
            signoff_until,
            internal_metadata,
        });
    }

    Ok(signup_options)
}
