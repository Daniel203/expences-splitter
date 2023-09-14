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
    let auth = auth(cx)?;

    auth.login_user(3);

    log!("sono nella funzione di login");
    return Ok(());
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
                <div class="grid grid-cols-3 grid-row-2 gap-y-8 w-80">
                    <h1>"Log In"</h1>

                    <div class="col-span-3 ">
                        <label class="block text-white text-sm font-bold mb-2" for="username">Username</label>
                        <input id="username" type="text" placeholder="Username" name="username"/>
                    </div>

                    <div class="col-span-3 ">
                        <label class="block text-white text-sm font-bold mb-2" for="password">Password</label>
                        <input id="password" type="password" placeholder="******" name="Password"/>
                    </div>

                    <button class="btn-primary btn-lg col-span-2" type="submit"><b>LOGIN</b></button>
                </div>
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
