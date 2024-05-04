use axum::http::StatusCode;
use tracing::instrument;

#[instrument(name = "Health Check")]
pub async fn handler() -> (StatusCode, String) {
    (StatusCode::OK, String::from("API Live!"))
}
