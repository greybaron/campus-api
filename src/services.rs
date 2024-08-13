use axum::extract::Request;
use http::StatusCode;

pub async fn print_headers(req: Request) -> Result<String, StatusCode> {
    for h in req.headers().iter() {
        println!("{:?}", h);
    }
    Ok("done".to_string())
}
