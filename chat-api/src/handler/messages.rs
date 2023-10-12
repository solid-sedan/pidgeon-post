use axum::{
    extract,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Extension, Json, Router,
};
// use anyhow::Result;

use sqlx::postgres::PgPool;

use crate::controller::messages::{get_messages, ingest_message};
use crate::model::messages::Message;

pub fn router() -> Router {
    Router::new()
        .route("/messages", get(get_all))
        .route("/messages", post(post_message))
}

pub async fn get_all(Extension(pool): Extension<PgPool>) -> impl IntoResponse {
    match get_messages(&pool).await {
        Ok(res) => (StatusCode::OK, Json(res).into_response()),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(err.to_string()).into_response(),
        ),
    }
}

pub async fn post_message(
    Extension(pool): Extension<PgPool>,
    extract::Json(payload): extract::Json<Message>,
) -> StatusCode {
    match ingest_message(&pool, payload).await {
        Ok(()) => StatusCode::CREATED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
