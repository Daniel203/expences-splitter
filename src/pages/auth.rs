use cfg_if::cfg_if;
use leptos::*;
use leptos_router::*;

use crate::{
    components::input_component::{
        InputComponent, InputParams, InputType, InputWithControlsComponent, InputWithControlsParams,
    },
    models::user::User,
};

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
    log::info!("fn: get_user()");
    let auth = auth(cx)?;

    let user = auth.current_user;
    log::info!("fn: get_user() - user: {:?}", user);

    return Ok(user);
}

#[server(Logout, "/api")]
pub async fn logout(cx: Scope) -> Result<(), ServerFnError> {
    log::info!("fn: logout()");
    let auth = auth(cx)?;

    log::info!("fn: logout() - logging out user");
    auth.logout_user();

    log::info!("fn: logout() - redirecting to \"/\"");
    leptos_axum::redirect(cx, "/");

    return Ok(());
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
        log::info!("fn: login() - logging in user");
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
pub async fn register(
    cx: Scope,
    username: String,
    password: String,
    confirm_password: String,
) -> Result<(), ServerFnError> {
    log::info!("fn: register()");

    let pool = pool(cx)?;
    let auth = auth(cx)?;

    if password != confirm_password {
        log::info!("fn: register() - passwords do not match");
        return Err(ServerFnError::ServerError(
            "Passwords do not match".to_string(),
        ));
    }

    let hashed_password = hash(password, DEFAULT_COST).unwrap();

    log::info!("fn: register() - creating user on the database");
    sqlx::query("INSERT INTO users (username, password) VALUES (?, ?)")
        .bind(&username)
        .bind(&hashed_password)
        .execute(&pool)
        .await?;

    log::info!("fn: register() - logging in user");
    let user = User::get_user_from_username(username, &pool)
        .await
        .ok_or_else(|| {
            return ServerFnError::ServerError("User not found".to_string());
        })?;

    auth.login_user(user.id);

    log::info!("fn: register() - redirecting to \"/\"");
    leptos_axum::redirect(cx, "/");

    return Ok(());
}

#[component]
pub fn LoginPage(cx: Scope) -> impl IntoView {
    let action = create_server_action::<Login>(cx);

    let (username, set_username) = create_signal(cx, String::new());
    let (password, set_password) = create_signal(cx, String::new());

    let is_form_valid = move || {
        !username.with(String::is_empty) && !password.with(String::is_empty)
    };

    let username_params = InputParams {
        label: "Username",
        placeholder: "username",
        name: "username",
        input_type: InputType::Text,
        value: (username, set_username),
    };

    let password_params = InputParams {
        label: "Password",
        placeholder: "******",
        name: "password",
        input_type: InputType::Password,
        value: (password, set_password),
    };

    view! { cx,
        <div class="flex h-screen justify-center items-center">
            <ActionForm action=action class="space-y-3 w-80">
                <p class="text-3xl font-bold mb-6">"Log In"</p>

                <InputComponent params=username_params />
                <InputComponent params=password_params />

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
    }
}

#[component]
pub fn RegisterPage(cx: Scope) -> impl IntoView {
    let action = create_server_action::<Register>(cx);

    let (username, set_username) = create_signal(cx, String::new());
    let (password, set_password) = create_signal(cx, String::new());
    let (confirm_password, set_confirm_password) = create_signal(cx, String::new());

    const USERNAME_MIN_LENGTH: usize = 5;
    const PASSWORD_MIN_LENGTH: usize = 8;

    let username_error = move || {
        if username.with(String::is_empty) {
            Some("Username cannot be empty".to_string())
        } else if username.with(|x| x.len() < USERNAME_MIN_LENGTH) {
            return Some(format!(
                "Username must be at least {} characters long",
                USERNAME_MIN_LENGTH
            ));
        } else {
            return None;
        }
    };

    let password_error = move || {
        if password.with(String::is_empty) {
            Some("Password cannot be empty".to_string())
        } else if password.with(|x| x.len() < PASSWORD_MIN_LENGTH) {
            return Some(format!(
                "Password must be at least {} characters long",
                PASSWORD_MIN_LENGTH
            ));
        } else {
            return None;
        }
    };

    let confirm_password_error = move || {
        if confirm_password.with(String::is_empty) {
            Some("Password cannot be empty".to_string())
        } else if confirm_password.with(|x| *x != password.get()) {
            return Some("Passwords do not match".to_string());
        } else {
            return None;
        }
    };

    let is_form_valid = move || {
        username_error().is_none()
            && password_error().is_none()
            && confirm_password_error().is_none()
    };

    let username_params = InputWithControlsParams {
        label: "Username",
        placeholder: "Username",
        name: "username",
        input_type: InputType::Text,
        value: (username, set_username),
        value_error: username_error,
    };

    let password_params = InputWithControlsParams {
        label: "Password",
        placeholder: "******",
        name: "password",
        input_type: InputType::Password,
        value: (password, set_password),
        value_error: password_error,
    };

    let confirm_password_params = InputWithControlsParams {
        label: "Confirm password",
        placeholder: "******",
        name: "confirm_password",
        input_type: InputType::Password,
        value: (confirm_password, set_confirm_password),
        value_error: confirm_password_error,
    };

    view! { cx,
        <div class="flex h-screen justify-center items-center">
            <ActionForm action=action class="space-y-3 w-80">
                <p class="text-3xl font-bold mb-6">"Register"</p>

                <InputWithControlsComponent params=username_params/>
                <InputWithControlsComponent params=password_params/>
                <InputWithControlsComponent params=confirm_password_params/>

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
    }
}

#[component]
pub fn LogoutPage(cx: Scope) -> impl IntoView {
    let action = create_server_action::<Logout>(cx);
    action.dispatch(Logout {});

    view! { cx, <div></div> }
}
