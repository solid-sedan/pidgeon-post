use sqlx::{postgres::PgPoolOptions, PgPool};

use dotenv::dotenv;

use std::time::Duration;

#[allow(clippy::all)]
pub async fn connect() -> PgPool {
    dotenv().ok();

    let db_connection_str = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.");

    println!("{}", db_connection_str);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_connection_str)
        .await
        .unwrap_or_else(|_| panic!("Error connecting to {}", db_connection_str));

    // migrations (may move into build process)
    sqlx::migrate!("./migrations").
        run(&pool)
        .await
        .unwrap_or_else(|_| panic!("Error running migrations"));

    pool
}
