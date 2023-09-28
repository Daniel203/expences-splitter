use leptos::*;
use leptos_router::*;

use crate::components::{
    input_component::{InputComponent, InputParams, InputType},
    notification_component::{NotificationComponent, NotificationParams, NotificationType},
};

#[server(JoinRoom, "/api")]
pub async fn join_room(cx: leptos::Scope, room_name: String) -> Result<(), ServerFnError> {
    use crate::models::room::Room;
    use crate::state::pool;

    let pool = pool(cx)?;

    let does_room_exists =
        sqlx::query_as!(Room, "SELECT * FROM room WHERE room_name = $1", room_name)
            .fetch_optional(&pool)
            .await?;

    log!("fn: join_room() - does room exists: {:?}", does_room_exists);

    match does_room_exists {
        Some(room) => {
            log!("fn: join_room() - room found");
            log!("fn: join_room() - redirecting to /room/{}", room.id);
            leptos_axum::redirect(cx, &format!("/room/{}", room.id));
            Ok(())
        }
        None => {
            log!("fn: join_room() - room don't exists");
            Err(ServerFnError::ServerError(
                "Room don't exists".to_string(),
            ))
        }
    }
}

#[component]
pub fn JoinRoomPage(cx: Scope) -> impl IntoView {
    let join_room = create_server_action::<JoinRoom>(cx);
    let (room_name, set_room_name) = create_signal(cx, String::new());

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

    view! {cx,
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

            <Show when=has_error fallback=|_| ()>
                <NotificationComponent params=get_notification_params()/>
            </Show>

        </div>
    }
}
