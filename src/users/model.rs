use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, sqlx::FromRow,Serialize, Clone)]
pub struct User {
    pub id: uuid::Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: String,
    pub verified: bool,

    #[serde(rename="createdAt")]
    pub created_at: Option<DateTime<Utc>>,

    #[serde(rename="updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}
