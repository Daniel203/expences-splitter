use leptos::*;
use leptos_router::*;

use crate::components::{
    input_component::{InputComponent, InputParams, InputType},
    notification_component::{NotificationComponent, NotificationParams, NotificationType},
};

use cfg_if::cfg_if;

#[server(JoinRoom, "/api")]
pub async fn join_room(room_name: String) -> Result<(), ServerFnError> {
    let pool = pool()?;
    let auth = auth()?;

    let user = auth.current_user.unwrap();
    let user_id = user.id;

    let does_room_exists =
        sqlx::query_as!(Room, "SELECT * FROM room WHERE room_name = $1", room_name)
            .fetch_optional(&pool)
            .await?;

    log!("fn: join_room() - does room exists: {:?}", does_room_exists);

    match does_room_exists {
        Some(room) => {
            log!("fn: join_room() - room found");

            // TODO: check max_participants

            join_room_sql(user_id, room.id.clone(), &pool).await?;

            log!("fn: join_room() - redirecting to /room/{}", room.id);
            leptos_axum::redirect(&format!("/room/{}", room.id));
            Ok(())
        }
        None => {
            log!("fn: join_room() - room don't exists");
            Err(ServerFnError::ServerError("Room don't exists".to_string()))
        }
    }
}

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use crate::models::room::Room;
        use crate::state::{auth, pool};
        use leptos::logging::log;
        use sqlx::SqlitePool;

        async fn join_room_sql(user_id: i64, room_id: String, pool: &SqlitePool) -> Result<(), ServerFnError> {
            // TODO: check max_participants

            log!("fn: join_room() - checking if the user is already in the room");
            let is_user_in_room = sqlx::query!(
                "SELECT * FROM user_room WHERE room_id = $1 AND user_id = $2",
                room_id,
                user_id,
            )
            .fetch_optional(pool)
            .await?;

            match is_user_in_room {
                Some(_) => {
                    log!("fn: join_room() - user already in the room");
                }
                None => {
                    log!("fn: join_room() - adding the user to the room");
                    sqlx::query!(
                        "INSERT INTO user_room (room_id, user_id) VALUES ($1, $2)",
                        room_id,
                        user_id,
                    )
                    .execute(pool)
                    .await?;
                }
            }

            Ok(())
        }
    }
}

#[component]
pub fn JoinRoomPage() -> impl IntoView {
    let join_room = create_server_action::<JoinRoom>();
    let (room_name, set_room_name) = create_signal(String::new());

    let value = join_room.value();
    let has_error = move || value.with(|val| matches!(val, Some(Err(_))));

    let input_params = InputParams {
        label: "Room Name".to_string(),
        placeholder: "Enter the room name".to_string(),
        name: "room_name".to_string(),
        input_type: InputType::Text,
        value: (room_name, set_room_name),
    };

    let get_notification_params = move || {
        let server_message = value().unwrap().unwrap_err().to_string();
        let client_message = server_message.replace("error running server function: ", "");

        NotificationParams {
            message: client_message,
            notification_type: NotificationType::Error,
        }
    };

    view! {
        <div class="flex h-screen justify-center items-center">
            <ActionForm action=join_room class="space-y-3 w-80">

                <InputComponent params=input_params/>

                <div class="grid grid-cols-3">
                    <div class="col-span-1">
                        <A href="/">
                            <button class="btn btn-ghost btn-lg">
                                <svg
                                    xmlns="http://www.w3.org/2000/svg"
                                    fill="none"
                                    viewBox="0 0 24 24"
                                    stroke-width="3"
                                    stroke="currentColor"
                                    class="w-6 h-6"
                                >
                                    <path
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        d="M10.5 19.5L3 12m0 0l7.5-7.5M3 12h18"
                                    ></path>
                                </svg>
                            </button>
                        </A>
                    </div>

                    <div class="col-span-2">
                        <button class="btn btn-primary btn-lg w-full" type="submit">
                            <b>JOIN</b>
                        </button>
                    </div>
                </div>

            </ActionForm>

            <Show when=has_error fallback=|| ()>
                <NotificationComponent params=get_notification_params()/>
            </Show>

        </div>
    }
}
