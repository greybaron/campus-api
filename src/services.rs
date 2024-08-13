use http::StatusCode;

pub async fn check_revive_session() -> Result<String, StatusCode> {
    Ok("done".to_string())
}
