use std::time::Duration;

use axum::http::StatusCode;
use secrecy::ExposeSecret;
use sqlx::{postgres::PgPoolOptions, types::chrono::Local};
use zero2prod::{
    configuration::get_configuration, routes::subscription::Subscriber, startup::run_test,
};

#[tokio::test]
async fn subscribe_returns_200_for_valid_form_data() {
    // Fetch config and connect to PG
    let config = get_configuration().expect("Unable to fetch configuration");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&config.database.connection_string().expose_secret())
        .await
        .expect("Can't connect to database");

    // Create Test Server
    let server = run_test(&pool).await.unwrap();

    let dt = Local::now();
    let utc = dt.to_utc();
    let ts = utc.timestamp();

    let data = Subscriber {
        name: String::from("Test McTesterson"),
        email: format!("{}@test.com", ts),
    };

    let response = server
        .post("/subscriptions")
        .content_type(&"application/x-www-form-urlencoded")
        .form(&data)
        .await;

    assert_eq!(StatusCode::CREATED, response.status_code());

    let saved = sqlx::query!(
        "SELECT email, name FROM subscriptions WHERE email LIKE $1",
        format!("{}@test.com", ts.to_string())
    )
    .fetch_one(&pool)
    .await
    .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, data.email);
    assert_eq!(saved.name, data.name);
}

#[tokio::test]
async fn subscribe_returns_400_for_missing_data() {
    let config = get_configuration().expect("Unable to fetch configuration");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&config.database.connection_string().expose_secret())
        .await
        .expect("Can't connect to database");
    let server = run_test(&pool).await.unwrap();

    let test_cases = vec![
        (
            Subscriber {
                name: String::from("John Doe"),
                email: String::new(),
            },
            "Missing the email address",
        ),
        (
            Subscriber {
                name: String::new(),
                email: String::from("test@gmail.com"),
            },
            "Missing the name",
        ),
        (
            Subscriber {
                name: String::new(),
                email: String::new(),
            },
            "Missing both name and email",
        ),
    ];

    for (invalid_data, error_message) in test_cases {
        let response = server
            .post("/subscriptions")
            .content_type(&"application/x-www-form-urlencoded")
            .form(&invalid_data)
            .await;

        assert_eq!(
            StatusCode::BAD_REQUEST,
            response.status_code(),
            "The API request failed due to {}",
            error_message
        )
    }
}
