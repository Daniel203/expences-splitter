use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use leptos::{LeptosOptions, ServerFnError, use_context};
        use sqlx::SqlitePool;
        use axum::extract::FromRef;
        use leptos_router::RouteListing;
        use crate::pages::auth::AuthSession;

        #[derive(FromRef, Debug, Clone)]
        pub struct AppState{
            pub leptos_options: LeptosOptions,
            pub pool: SqlitePool,
            pub routes: Vec<RouteListing>,

        }

        pub fn pool() -> Result<SqlitePool, ServerFnError> {
            use_context::<SqlitePool>()
                .ok_or_else(|| ServerFnError::ServerError("Pool missing.".into()))
        }

        pub fn auth() -> Result<AuthSession, ServerFnError> {
            use_context::<AuthSession>()
                .ok_or_else(|| ServerFnError::ServerError("Auth session missing.".into()))
        }

    }
}
