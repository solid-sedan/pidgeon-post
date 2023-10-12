use chrono::Utc;
use sqlx::postgres::PgPool;
use uuid::Uuid;

use anyhow::Result;

use crate::model::messages::Message;

pub async fn ingest_message(_: &PgPool, _: Message) -> Result<()> {
    println!("Ingesting message");
    Ok(())
}

pub async fn get_messages(_: &PgPool) -> Result<Vec<Message>, anyhow::Error> {
    println!("Getting messages");
    Ok(vec![Message {
        uuid: Some(Uuid::new_v4()),
        message: "Hello".to_string(),
        created_at: Utc::now().into(),
    }])
}
