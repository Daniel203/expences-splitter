use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub password: String,
}

impl Default for User {
    fn default() -> Self {
        Self {
            id: -1,
            name: "Guest".to_string(),
            password: "".to_string(),
        }
    }
}

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use sqlx::sqlite::SqlitePool;
        use axum_session_auth::Authentication;

        #[async_trait::async_trait]
        impl Authentication<User, i64, SqlitePool> for User{
            async fn load_user(userid: i64, pool: Option<&SqlitePool>) -> anyhow::Result<User> {
                todo!()
            }

            fn is_authenticated(&self) -> bool {
                todo!()
            }

            fn is_active(&self) -> bool {
                todo!()
            }

            fn is_anonymous(&self) -> bool {
                todo!()
            }
        }
    }
}
