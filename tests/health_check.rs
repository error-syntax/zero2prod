use std::time::Duration;

use axum::http::StatusCode;
use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;
use zero2prod::{configuration::get_configuration, startup::run_test};

#[tokio::test]
async fn test_health_check() {
    let config = get_configuration().expect("Unable to fetch configuration");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&config.database.connection_string().expose_secret())
        .await
        .expect("Can't connect to database");
    let server = run_test(&pool).await.unwrap();

    let response = server.get("/health_check").await;

    assert_eq!(response.status_code(), StatusCode::OK);
    assert_eq!(response.text(), "API Live!");
}
