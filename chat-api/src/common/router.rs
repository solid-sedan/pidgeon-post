use axum::{Extension, Router};
use sqlx::postgres::PgPool;

pub async fn new(pool: PgPool) -> Router {
    Router::new()
        .merge(crate::handler::messages::router())
        .layer(Extension(pool))
}
