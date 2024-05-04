use std::time::Duration;

use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() {
    let subscriber = get_subscriber("zero2Prod".into(), "info".into());
    init_subscriber(subscriber);

    let config = get_configuration().expect("Failed to read configuration");

    let address = format!("127.0.0.1:{}", config.application_port);
    let listener = TcpListener::bind(address).await.unwrap();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&config.database.connection_string().expose_secret())
        .await
        .expect("Can't connect to database");

    let router = run(&pool);

    println!(
        "The app is running on http://localhost:{}",
        config.application_port
    );

    axum::serve(listener, router).await.unwrap()
}
