use axum::{
    body::Body,
    http::Request,
    routing::{get, post},
    Router,
};
use axum_test::{TestServer, TestServerConfig};
use sqlx::PgPool;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tower_request_id::{RequestId, RequestIdLayer};
use tracing::error_span;

use crate::routes::{health_check, subscription};

struct AppState {
    db_pool: PgPool,
}

pub fn run(pool: &PgPool) -> Router {
    let state = AppState {
        db_pool: pool.clone(),
    };
    let router = Router::new()
        .route("/", get(|| async { "Hello World!" }))
        .route("/health_check", get(health_check::handler))
        .route("/subscriptions", post(subscription::subscribe))
        .layer(ServiceBuilder::new().layer(RequestIdLayer).layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<Body>| {
                let request_id = request
                    .extensions()
                    .get::<RequestId>()
                    .map(ToString::to_string)
                    .unwrap_or_else(|| "unknown".into());

                error_span!(
                    "request",
                    id = %request_id,
                    method = %request.method(),
                    uri = %request.uri(),
                )
            }),
        ))
        .with_state(state.db_pool);

    router
}

pub async fn run_test(pool: &PgPool) -> Result<TestServer, anyhow::Error> {
    let router = run(&pool);
    let config = TestServerConfig::builder().build();
    let server = TestServer::new_with_config(router, config);

    server
}
