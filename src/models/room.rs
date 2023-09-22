use serde::{Deserialize, Serialize};

#[cfg_attr(feature="ssr", derive(sqlx::FromRow))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Room {
    pub id: i64,
    pub room_name: String,
    pub max_participants: i64,
    pub owner: i64,

    #[cfg(feature = "ssr")]
    pub created_at: Option<sqlx::types::chrono::NaiveDateTime>,
}
