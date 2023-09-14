use cfg_if::cfg_if;
use leptos::*;
use leptos_router::*;

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
    let action = create_server_action::<Login>(cx);

    return view! { cx,
        <div class="flex h-screen justify-center items-center">
            <ActionForm action=action>
                <h1>"Log In"</h1>
                // <label> "User ID:" <input type="text" placeholder="User ID" maxlength="32" name="username" class="auth-input" /> </label>
                // <br/>
                // <label> "Password:" <input type="password" placeholder="Password" name="password" class="auth-input" /> </label>
                // <br/>
                // <label> <input type="checkbox" name="remember" class="auth-input" />"Remember me?"</label>
                // <br/>
                // <button type="submit" class="button">"Log In"</button>
            </ActionForm>
        </div>
    };
}

#[component]
pub fn RegisterPage(cx: Scope) -> impl IntoView {
    let action = create_server_action::<Register>(cx);

    return view! { cx,
        <div class="flex h-screen justify-center items-center">
            <p>REGISTER PAGE</p>
        </div>
    };
}
