use axum::http::StatusCode;

pub async fn handler() -> (StatusCode, String) {
    (StatusCode::OK, String::from("API Live!"))
}
