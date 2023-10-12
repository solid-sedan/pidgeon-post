use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Message {
    pub uuid: Option<Uuid>,
    pub message: String,
    pub created_at: Option<DateTime<Utc>>,
}
