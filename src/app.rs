use axum::{Router, http::StatusCode, routing::get};

pub fn app() -> Router {
  Router::new().route("/health_check", get(health_check))
}

pub async fn health_check() -> StatusCode {
  StatusCode::OK
}
