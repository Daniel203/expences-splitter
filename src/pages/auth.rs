use leptos::*;
use leptos_router::*;
use cfg_if::cfg_if;

use crate::models::user::User;

cfg_if! {
if #[cfg(feature = "ssr")] {
    use sqlx::SqlitePool;
    use axum_session_auth::{SessionSqlitePool, Authentication, HasPermission};
    use crate::state::auth;

    pub type AuthSession = axum_session_auth::AuthSession<User, i64, SessionSqlitePool, SqlitePool>;
}}

#[server(GetUser, "/api")]
pub async fn get_user(cx: Scope) -> Result<Option<User>, ServerFnError> {
    let auth = auth(cx)?;

    return Ok(auth.current_user);
}

#[server(Login, "/api")]
pub async fn login(cx: Scope) -> Result<(), ServerFnError> {
    todo!()
}

#[server(Register, "/api")]
pub async fn register(cx: Scope) -> Result<(), ServerFnError> {
    todo!()
}


#[component]
pub fn LoginPage(cx: Scope) -> impl IntoView {
    return view! { cx,
        <div class="flex h-screen justify-center items-center">
            <p>LOGIN PAGE</p>
        </div>
    };
}


#[component]
pub fn RegisterPage(cx: Scope) -> impl IntoView {
    return view! { cx,
        <div class="flex h-screen justify-center items-center">
            <p>REGISTER PAGE</p>
        </div>
    };
}
