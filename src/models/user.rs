use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};
use std::{fmt, string};

#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
#[derive(Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password: String,

    #[cfg(feature = "ssr")]
    pub created_at: Option<sqlx::types::chrono::NaiveDateTime>,
}

impl fmt::Debug for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("User")
            .field("id", &self.id)
            .field("username", &self.username)
            .finish()
    }
}

impl string::ToString for User {
    fn to_string(&self) -> String {
        format!("User: id: {}, username: {}", self.id, self.username)
    }
}

impl Default for User {
    fn default() -> Self {
        Self {
            id: -1,
            username: "Guest".to_string(),
            password: "".to_string(),
             
            #[cfg(feature = "ssr")]
            created_at: None,
        }
    }
}

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use sqlx::sqlite::SqlitePool;
        use axum_session_auth::Authentication;

        impl User {
            pub async fn get(id: i64, pool: &SqlitePool) -> Option<Self> {
                log::info!("fn: get()");

                let user = sqlx::query_as::<_, User>("SELECT * FROM user WHERE id = ?")
                    .bind(id)
                    .fetch_one(pool)
                    .await;

                log::info!("fn: get() - user: {:?}", user);
                return user.ok();
            }

            pub async fn get_user_from_username(username: String, pool: &SqlitePool) -> Option<Self> {
                log::info!("fn: get_user_from_username()");

                let user = sqlx::query_as::<_, User>("SELECT * FROM user WHERE username = ?")
                    .bind(username.trim())
                    .fetch_one(pool)
                    .await;

                log::info!("fn: get_user_from_username() - user: {:?}", user);
                return user.ok();
            }
        }

        #[async_trait::async_trait]
        impl Authentication<User, i64, SqlitePool> for User{
            async fn load_user(userid: i64, pool: Option<&SqlitePool>) -> anyhow::Result<User> {
                log::info!("fn: load_user()");

                let pool = pool.unwrap();
                let user = User::get(userid, pool).await;

                return user.ok_or_else(|| anyhow::anyhow!("Cannot get user"));
            }

            fn is_authenticated(&self) -> bool {
                log::info!("fn: is_authenticated()");
                todo!()
            }

            fn is_active(&self) -> bool {
                log::info!("fn: is_active()");
                todo!()
            }

            fn is_anonymous(&self) -> bool {
                log::info!("fn: is_anonymous()");
                todo!()
            }
        }
    }
}
