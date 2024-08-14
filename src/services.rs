use axum::{Extension, Json};
use chrono::{DateTime, Duration, Utc};
use fnv::FnvHasher;

use std::hash::{Hash, Hasher};

use crate::{
    color_stuff::hex_to_luminance,
    types::{
        CampusDualGrade, CampusDualSignupOption, CampusDualSubGrade, CampusDualVerfahrenOption,
        CampusReminders, CampusTimelineEvent, CdAuthData, CdExamDetails, CdExamStats,
        ExamRegistrationMetadata, ExportTimelineEvent, ExportTimelineEvents, LoginResponse,
        ResponseError, StundenplanItem,
    },
};

pub async fn get_grades(
    Extension(_): Extension<CdAuthData>,
) -> Result<Json<Vec<CampusDualGrade>>, ResponseError> {
    let grades = vec![
        CampusDualGrade {
            name: "Eine Prüfung".to_string(),
            grade: "1,3".to_string(),
            total_passed: Some(true),
            credit_points: 6,
            akad_period: "WS 2021".to_string(),
            subgrades: vec![CampusDualSubGrade {
                name: "Eine Prüfung".to_string(),
                grade: "1,3".to_string(),
                passed: Some(true),
                beurteilung: "32.12.2024".to_string(),
                bekanntgabe: "33.12.2024".to_string(),
                wiederholung: Some("EP".to_string()),
                akad_period: "WS 2021".to_string(),
            }],
        },
        CampusDualGrade {
            name: "Kläglich".to_string(),
            grade: "5,0".to_string(),
            total_passed: Some(false),
            credit_points: 69,
            akad_period: "WS 2021".to_string(),
            subgrades: vec![
                CampusDualSubGrade {
                    name: "Der Tragödie erster Teil".to_string(),
                    grade: "5,0".to_string(),
                    passed: Some(false),
                    beurteilung: "32.12.2024".to_string(),
                    bekanntgabe: "33.12.2024".to_string(),
                    wiederholung: Some("EP".to_string()),
                    akad_period: "WS 2021".to_string(),
                },
                CampusDualSubGrade {
                    name: "Two for two".to_string(),
                    grade: "5,0".to_string(),
                    passed: Some(false),
                    beurteilung: "32.12.2024".to_string(),
                    bekanntgabe: "33.12.2024".to_string(),
                    wiederholung: Some("WP1".to_string()),
                    akad_period: "WS 2021".to_string(),
                },
            ],
        },
    ];

    Ok(Json(grades))
}

pub async fn check_revive_session(
    Extension(_): Extension<CdAuthData>,
) -> Result<Json<Option<LoginResponse>>, ResponseError> {
    println!("checking session...");

    Ok(Json(None))
}

pub async fn get_examsignup(
    Extension(_): Extension<CdAuthData>,
) -> Result<Json<Vec<CampusDualSignupOption>>, ResponseError> {
    let signup_options = vec![
        CampusDualSignupOption {
            name: "Eine Prüfung".to_string(),
            verfahren: "Verfahren".to_string(),
            pruefart: "Prüfungsart".to_string(),
            status: "Status".to_string(),
            signup_information: "Wichtige Info".to_string(),
            exam_date: Some("32.12.2024".to_string()),
            exam_time: Some("12:34".to_string()),
            exam_room: Some("SSR 123".to_string()),
            warning_message: Some("Anmeldung ist nur noch bis gestern möglich".to_string()),
            signup_until: Some("31.12.2024".to_string()),
            internal_metadata: Some(ExamRegistrationMetadata {
                assessment: "".to_string(),
                peryr: "".to_string(),
                perid: "".to_string(),
                offerno: "".to_string(),
            }),
        },
        CampusDualSignupOption {
            name: "Andere Prüfung".to_string(),
            verfahren: "Verfahren".to_string(),
            pruefart: "Prüfungsart".to_string(),
            status: "Status".to_string(),
            signup_information: "Wichtige Info".to_string(),
            exam_date: Some("32.12.2024".to_string()),
            exam_time: Some("12:34".to_string()),
            exam_room: Some("SSR 123".to_string()),
            warning_message: Some("Anmeldung ist nur noch bis gestern möglich".to_string()),
            signup_until: Some("31.12.2024".to_string()),
            internal_metadata: Some(ExamRegistrationMetadata {
                assessment: "".to_string(),
                peryr: "".to_string(),
                perid: "".to_string(),
                offerno: "".to_string(),
            }),
        },
    ];

    Ok(Json(signup_options))
}

pub async fn post_registerexam(
    Extension(_): Extension<CdAuthData>,
    Json(_): Json<ExamRegistrationMetadata>,
) -> Result<String, ResponseError> {
    Ok("bloat".to_string())
}

pub async fn get_examdetails(
    Extension(_): Extension<CdAuthData>,
    Json(_): Json<ExamRegistrationMetadata>,
) -> Result<Json<CdExamDetails>, ResponseError> {
    let exam_details = CdExamDetails {
        ev_agrtype_text: "Aggregationstyp".to_string(),
        ev_audtype_text: "Auditoriumstyp".to_string(),
        ev_continue_indicator: "Fortsetzungsindikator".to_string(),
        ev_dereg_end: "Abmeldefrist".to_string(),
        ev_dereg_endtime: "Abmeldefrist".to_string(),
        ev_duration: "Dauer".to_string(),
        ev_durunit: "Dauer".to_string(),
        ev_exambegtime: "Prüfungsbeginn".to_string(),
        ev_examdate: "Prüfungsdatum".to_string(),
        ev_examendtime: "Prüfungsende".to_string(),
        ev_examorg_text: "Prüfungsorganisation".to_string(),
        ev_examorg_longtext: Some("juckt".to_string()),
        ev_instructor: "Prüfer".to_string(),
        ev_location_short: "Raum".to_string(),
        ev_location_stext: "Raum".to_string(),
        ev_obtype_text: "Objekttyp".to_string(),
        ev_reason: "Grund".to_string(),
        ev_regis_begin: "Anmeldefrist".to_string(),
        ev_regis_begtime: "Anmeldefrist".to_string(),
        ev_regis_end: "Anmeldefrist".to_string(),
        ev_regis_endtime: "Anmeldefrist".to_string(),
        ev_room_short: "Raum".to_string(),
        ev_room_stext: "Raum".to_string(),
        ev_short: "Kurz".to_string(),
        ev_stext: "Lang".to_string(),
    };

    Ok(Json(exam_details))
}

pub async fn post_cancelexam(
    Extension(_): Extension<CdAuthData>,
    Json(_): Json<ExamRegistrationMetadata>,
) -> Result<String, ResponseError> {
    Ok("egal".to_string())
}

pub async fn get_examverfahren(
    Extension(_): Extension<CdAuthData>,
) -> Result<Json<Vec<CampusDualVerfahrenOption>>, ResponseError> {
    let signup_verfahren = vec![
        CampusDualVerfahrenOption {
            name: "Abmeldbare Prüfung".to_string(),
            verfahren: "Verfahren".to_string(),
            pruefart: "Prüfungsart".to_string(),
            status: "Status".to_string(),
            signup_information: "Wichtige Info".to_string(),
            exam_date: Some("32.12.2024".to_string()),
            exam_time: Some("12:34".to_string()),
            exam_room: Some("SSR 123".to_string()),
            warning_message: Some("Abmeldung ist nur noch bis gestern möglich".to_string()),
            signoff_until: Some("31.12.2024".to_string()),
            internal_metadata: Some(ExamRegistrationMetadata {
                assessment: "".to_string(),
                peryr: "".to_string(),
                perid: "".to_string(),
                offerno: "".to_string(),
            }),
        },
        CampusDualVerfahrenOption {
            name: "Andere Prüfung".to_string(),
            verfahren: "Verfahren".to_string(),
            pruefart: "Prüfungsart".to_string(),
            status: "Status".to_string(),
            signup_information: "Wichtige Info".to_string(),
            exam_date: Some("32.12.2024".to_string()),
            exam_time: Some("12:34".to_string()),
            exam_room: Some("SSR 123".to_string()),
            warning_message: Some("Abmeldung ist nur noch bis gestern möglich".to_string()),
            signoff_until: Some("31.12.2024".to_string()),
            internal_metadata: Some(ExamRegistrationMetadata {
                assessment: "".to_string(),
                peryr: "".to_string(),
                perid: "".to_string(),
                offerno: "".to_string(),
            }),
        },
    ];

    Ok(Json(signup_verfahren))
}

pub async fn get_ects(Extension(_): Extension<CdAuthData>) -> Result<String, ResponseError> {
    Ok("155".to_string())
}

pub async fn get_fachsem(Extension(_): Extension<CdAuthData>) -> Result<String, ResponseError> {
    Ok("6".to_string())
}

pub async fn get_examstats(
    Extension(_): Extension<CdAuthData>,
) -> Result<Json<CdExamStats>, ResponseError> {
    // CAMPUSDUAL PIECHART:
    // daten/partitionen: ['erfolgreich', 0], ['nicht bestanden', 0], ['gebucht', 0]
    // farben: ["#0070a3", "#4297d7", "#fcbe04"]

    let resp = CdExamStats {
        total: 100,
        successful: 69,
        unsuccessful: 31,
        unassessed: 0,
        booked: 0,
        finished: 0,
        ronmodus: 0,
    };

    Ok(Json(resp))
}

pub async fn get_stundenplan(
    Extension(_): Extension<CdAuthData>,
) -> Result<Json<Vec<StundenplanItem>>, ResponseError> {
    let mut stundenplan: Vec<StundenplanItem> = vec![];

    let today = Utc::now().date_naive();
    let days_range = -3..=3;

    for offset in days_range {
        let date = today + Duration::days(offset);
        let eight = date.and_hms_opt(8, 0, 0).unwrap().and_utc().timestamp();
        let ninethirty = date.and_hms_opt(9, 30, 0).unwrap().and_utc().timestamp();

        stundenplan.push(StundenplanItem {
            all_day: false,
            color: "egal".to_string(),
            font_color: None,
            description: "Beschreibung".to_string(),
            editable: false,
            end: ninethirty,
            instructor: "Dozent".to_string(),
            remarks: "remarks".to_string(),
            room: "103 Seminarraum".to_string(),
            sinstructor: "DZNT".to_string(),
            sroom: "103 SR".to_string(),
            start: eight,
            title: "ZSPM".to_string(),
        });
    }

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
    Extension(_): Extension<CdAuthData>,
) -> Result<Json<CampusReminders>, ResponseError> {
    let resp = CampusReminders {
        electives: 1,
        exams: 2,
        latest: vec![],
        semester: 7,
        upcoming: vec![],
    };

    Ok(Json(resp))
}

pub async fn get_timeline(
    Extension(_): Extension<CdAuthData>,
) -> Result<Json<ExportTimelineEvents>, ResponseError> {
    // let fachsemester: Vec<ExportTimelineEvent> = events_by_color("#fcbe04", &events);
    // let theoriesemester: Vec<ExportTimelineEvent> = events_by_color("#0070a3", &events);
    // let praxissemester: Vec<ExportTimelineEvent> = events_by_color("#119911", &events);
    // let specials: Vec<ExportTimelineEvent> = events_by_color("#880000", &events);

    let export_events = ExportTimelineEvents {
        fachsemester: vec![ExportTimelineEvent {
            name: "Fachsemester".to_string(),
            description: "Ja das FS halt".to_string(),
            color: "#fcbe04".to_string(),
            start: "gerstenmalz".to_string(),
            end: "alu gobi".to_string(),
        }],
        theoriesemester: vec![ExportTimelineEvent {
            name: "Fachsemester".to_string(),
            description: "Ja das FS halt".to_string(),
            color: "#fcbe04".to_string(),
            start: "gerstenmalz".to_string(),
            end: "alu gobi".to_string(),
        }],
        praxissemester: vec![ExportTimelineEvent {
            name: "Fachsemester".to_string(),
            description: "Ja das FS halt".to_string(),
            color: "#fcbe04".to_string(),
            start: "gerstenmalz".to_string(),
            end: "alu gobi".to_string(),
        }],
        specials: vec![ExportTimelineEvent {
            name: "Fachsemester".to_string(),
            description: "Ja das FS halt".to_string(),
            color: "#fcbe04".to_string(),
            start: "gerstenmalz".to_string(),
            end: "alu gobi".to_string(),
        }],
    };

    Ok(Json(export_events))
}

fn _events_by_color(color: &str, events: &[CampusTimelineEvent]) -> Vec<ExportTimelineEvent> {
    events
        .iter()
        .filter(|event| event.color == color)
        .map(|event| ExportTimelineEvent {
            name: event.title.clone(),
            description: event
                .description
                .replace("<br>", " ")
                .replace("<strong>", "")
                .replace("</strong>", ""),
            color: event.color.clone(),
            start: _campusdate_to_iso8601(&event.start),
            end: _campusdate_to_iso8601(&event.end),
        })
        .collect()
}

fn _campusdate_to_iso8601(input: &str) -> String {
    let format = "%a, %d %b %Y %H:%M:%S %z";

    let date_time = DateTime::parse_from_str(input, format).expect("Failed to parse date");
    date_time.to_rfc3339()
}
