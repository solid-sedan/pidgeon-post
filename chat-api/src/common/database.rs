use sqlx::{postgres::PgPoolOptions, PgPool};

use std::time::Duration;

pub async fn connect() -> PgPool {
    // todo use env variables matched to docker-compose
    let db_connection_str = "postgres://postgres:password@localhost:5432/pidgeon".to_string();

    println!("{}", db_connection_str);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_connection_str)
        .await
        .expect("can't connect to database");

    pool
}
