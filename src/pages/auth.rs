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

    let (username, set_username) = create_signal(cx, String::new());
    let (password, set_password) = create_signal(cx, String::new());

    let is_form_valid = move || {
        return !username.with(String::is_empty) && !password.with(String::is_empty);
    };

    return view! { cx,
        <div class="flex h-screen justify-center items-center">
            <ActionForm action=action class="space-y-6 w-80">
                <p class="text-3xl font-bold">"Log In"</p>

                <div class="form-control w-full">
                    <label class="label-text font-bold mb-2">Username</label>
                    <input
                        class="input input-bordered input-primary w-full"
                        type="text"
                        placeholder="Username"
                        name="username"
                        on:input=move |ev| set_username.update(|x| *x = event_target_value(&ev))
                    />
                </div>

                <div class="form-control w-full">
                    <label class="label-text font-bold mb-2">Password</label>
                    <input
                        class="input input-bordered input-primary w-full"
                        type="password"
                        placeholder="******"
                        name="password"
                        on:input=move |ev| set_password.update(|x| *x = event_target_value(&ev))
                    />
                </div>

                <button
                    class="btn btn-primary btn-lg w-full"
                    type="submit"
                    prop:disabled=move || !is_form_valid()
                >
                    <b>LOGIN</b>
                </button>

                <div class="w-full">
                    <p class="text-center">
                        "Don't have an account? " <A href="/register">
                            <b>
                                <u>"Register now!"</u>
                            </b>
                        </A>
                    </p>
                </div>

            </ActionForm>
        </div>
    };
}

#[component]
pub fn RegisterPage(cx: Scope) -> impl IntoView {
    let action = create_server_action::<Register>(cx);

    let (username, set_username) = create_signal(cx, String::new());
    let (password, set_password) = create_signal(cx, String::new());
    let (password_2, set_password_2) = create_signal(cx, String::new());

    let is_form_valid = move || {
        // TODO: control also that the username and password are valid (min_length, etc.)
        return !username.with(String::is_empty)
            && !password.with(String::is_empty)
            && !password_2.with(String::is_empty)
            && password() == password_2();
    };

    // let username_error = move || {
    //     if username.with(String::is_empty) {
    //         return Some("Username cannot be empty");
    //     } else if username.with(|x| x.len() < 5) {
    //         return Some("Username must be at least 5 characters long");
    //     } else {
    //         return None;
    //     }
    // };
    //
    // let password_error = move || {
    //     if password.with(String::is_empty) {
    //         return Some("Password cannot be empty");
    //     } else if password.with(|x| x.len() < 8) {
    //         return Some("Password must be at least 8 characters long");
    //     } else {
    //         return None;
    //     }
    // };

    return view! { cx,
        <div class="flex h-screen justify-center items-center">
            <ActionForm action=action class="space-y-6 w-80">
                <p class="text-3xl font-bold">"Register"</p>

                <div class="form-control w-full">
                    <label class="label-text font-bold mb-2">Username</label>
                    <input
                        class="input input-bordered input-primary w-full"
                        type="text"
                        placeholder="Username"
                        name="username"
                        on:input=move |ev| set_username.update(|x| *x = event_target_value(&ev))
                    />
                </div>

                <div class="form-control w-full">
                    <label class="label-text font-bold mb-2">Password</label>
                    <input
                        class="input input-bordered input-primary w-full"
                        type="password"
                        placeholder="******"
                        name="password"
                        on:input=move |ev| set_password.update(|x| *x = event_target_value(&ev))
                    />
                </div>

                <div class="form-control w-full">
                    <label class="label-text font-bold mb-2">Repeat Password</label>
                    <input
                        class="input input-bordered input-primary w-full"
                        type="password"
                        placeholder="******"
                        name="password_2"
                        on:input=move |ev| set_password_2.update(|x| *x = event_target_value(&ev))
                    />
                </div>

                <button
                    class="btn btn-primary btn-lg w-full"
                    type="submit"
                    prop:disabled=move || !is_form_valid()
                >
                    <b>REGISTER</b>
                </button>

                <div class="w-full">
                    <p class="text-center">
                        "Already have an account? " <A href="/login">
                            <b>
                                <u>"Login now!"</u>
                            </b>
                        </A>
                    </p>
                </div>

            </ActionForm>
        </div>
    };
}
