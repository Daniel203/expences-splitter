use cfg_if::cfg_if;
use leptos::*;
use leptos_router::*;

use crate::models::user::User;

cfg_if! {
if #[cfg(feature = "ssr")] {
    use sqlx::SqlitePool;
    use axum_session_auth::{SessionSqlitePool, Authentication, HasPermission};
    use bcrypt::{verify, hash, DEFAULT_COST};
    use crate::state::{auth, pool};

    pub type AuthSession = axum_session_auth::AuthSession<User, i64, SessionSqlitePool, SqlitePool>;
}}

#[server(GetUser, "/api")]
pub async fn get_user(cx: Scope) -> Result<Option<User>, ServerFnError> {
    let auth = auth(cx)?;

    return Ok(auth.current_user);
}

#[server(Login, "/api")]
pub async fn login(cx: Scope, username: String, password: String) -> Result<(), ServerFnError> {
    log::info!("fn: login()");

    let pool = pool(cx)?;
    let auth = auth(cx)?;

    let mut user = User::get_user_from_username(username, &pool)
        .await
        .ok_or_else(|| {
            log::info!("fn: login() - user does not exist");
            return ServerFnError::ServerError("User does not exist".to_string());
        })?;

    if verify(&password, &user.password)? {
        log::info!("fn: login() - password is correct");
        auth.login_user(user.id);

        log::info!("fn: login() - redirecting to \"/\"");
        leptos_axum::redirect(cx, "/");
        return Ok(());
    } else {
        log::info!("fn: login() - password is incorrect");
        return Err(ServerFnError::ServerError(
            "Password is incorrect".to_string(),
        ));
    }
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
                        <input id="password" type="password" placeholder="******" name="password"/>
                    </div>

                    <button class="btn-primary btn-lg col-span-3" type="submit"><b>LOGIN</b></button>

                    <div class="col-span-3 w-80">
                        <p class="text-center">"Don't have an account? "
                            <A href="/register"><b><u>"Register now!"</u></b></A>
                        </p>
                    </div>


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
