use axum::{
    routing::{get, post},
    Router,
};
use axum_test::{TestServer, TestServerConfig};
use sqlx::PgPool;
use tower_http::trace::TraceLayer;

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
        .layer(TraceLayer::new_for_http())
        .with_state(state.db_pool);

    router
}

pub async fn run_test(pool: &PgPool) -> Result<TestServer, anyhow::Error> {
    let router = run(&pool);
    let config = TestServerConfig::builder().build();
    let server = TestServer::new_with_config(router, config);

    server
}
