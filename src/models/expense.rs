use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpenseDTO {
    pub id: Option<i64>,
    pub amount: Option<f64>,
    pub participants: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub room_id: Option<String>,

    #[cfg(feature = "ssr")]
    pub created_at: Option<sqlx::types::chrono::NaiveDateTime>,
}

#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Expense {
    pub id: i64,
    pub amount: f64,
    pub participants: Vec<i64>,
    pub title: String,
    pub description: Option<String>,
    pub room_id: String,

    #[cfg(feature = "ssr")]
    pub created_at: Option<sqlx::types::chrono::NaiveDateTime>,
}

impl Default for Expense {
    fn default() -> Self {
        Self {
            id: -1,
            amount: 0.0,
            participants: vec![],
            title: "".to_string(),
            description: None,
            room_id: "".to_string(),

            #[cfg(feature = "ssr")]
            created_at: None,
        }
    }
}

cfg_if! {
    if #[cfg(feature = "ssr")] {
        impl From<ExpenseDTO> for Expense {
            fn from(expense: ExpenseDTO) -> Self {
                Self {
                    id: expense.id.unwrap(),
                    amount: expense.amount.unwrap(),
                    participants: expense.participants.unwrap().split(",").map(|s| s.parse::<i64>().unwrap()).collect(),
                    title: expense.title.unwrap(),
                    description: expense.description,
                    room_id: expense.room_id.unwrap(),
                    created_at: expense.created_at,
                }
            }
        }
    }
}
