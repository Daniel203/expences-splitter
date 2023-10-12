use serde::{Deserialize, Serialize};

#[cfg_attr(feature="ssr", derive(sqlx::FromRow))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Expense {
    pub id: i64,
    pub amount: i64,
    pub participants: Vec<i64>,
    pub title: String,
    pub description: String,
    pub paid_by: i64,
    pub room_id: String,

    #[cfg(feature = "ssr")]
    pub created_at: Option<sqlx::types::chrono::NaiveDateTime>,
}
