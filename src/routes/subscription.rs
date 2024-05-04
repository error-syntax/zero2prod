use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Form,
};
use serde::{Deserialize, Serialize};
use sqlx::{types::chrono::Utc, PgPool};
use tracing::{error, warn};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct Subscriber {
    pub name: String,
    pub email: String,
}

#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(input, pool),
    fields(
        reques_id = %Uuid::new_v4(),
        subscriber_email = %input.email,
        subscriber_name = %input.name
    )
)]
pub async fn subscribe(State(pool): State<PgPool>, Form(input): Form<Subscriber>) -> Response {
    if input.name.is_empty() || input.email.is_empty() {
        warn!(
            "Unable to subscribe due to missing field(s): Name {}; Email {}",
            input.name, input.email
        );
        return (
            StatusCode::BAD_REQUEST,
            "Unable to subscribe due to missing field(s)",
        )
            .into_response();
    }

    match insert_subscriber(&pool, &input).await {
        Ok(_) => {
            return (
                StatusCode::CREATED,
                format!("Congratulations {}, you're subscribed!", input.name),
            )
                .into_response()
        }
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Uh-oh, an error occurred on our end, try again"),
            )
                .into_response()
        }
    }
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(input, pool)
)]
pub async fn insert_subscriber(pool: &PgPool, input: &Subscriber) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO subscriptions (id, email, name, subscribed_at) VALUES ($1, $2, $3, $4)",
        Uuid::new_v4(),
        input.email,
        input.name,
        Utc::now(),
    )
    .execute(pool)
    .await
    .map_err(|e| {
        error!("Failed to execute query: {:?}", e);
        e
    })?;

    Ok(())
}
