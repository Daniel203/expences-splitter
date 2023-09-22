use leptos::*;
use leptos_router::*;
use crate::components::{input_component::{InputComponent, InputParams, InputType}, notification_component::{NotificationComponent, NotificationParams, NotificationType}};

#[server(CreateRoom, "/api")]
pub async fn create_room(cx: leptos::Scope, room_name: String) -> Result<(), ServerFnError> {
    use crate::state::pool;
    use crate::models::room::Room;

    let pool = pool(cx)?;

    let does_room_exists = sqlx::query_as!(Room, "SELECT * FROM rooms WHERE room_name = $1", room_name)
        .fetch_optional(&pool)
        .await?;

    log!("fn: create_room() - does room exists: {:?}", does_room_exists);

    if let Some(_) = does_room_exists {
        log::info!("fn: create_room() - room already exists");
        return Err(ServerFnError::ServerError("Room already exists".to_string()));
    } 

    log!("fn: create_room() - creating room: {:?}", room_name);

    // insert the room and return the new id
    let res = sqlx::query_as!(Room, "INSERT INTO rooms (room_name, max_participants, owner) VALUES ($1, 20, 3) RETURNING *", room_name)
        .fetch_one(&pool)
        .await;

    match res {
        Ok(room) => {
            log::info!("fn: create_room() - created room: {:?}", room);
            let id = room.id;
            leptos_axum::redirect(cx, &format!("/room/{}", id));
            Ok(())
        },
        Err(e) => {
            log::info!("fn: create_room() - error creating room: {:?}", e);
            Err(ServerFnError::ServerError("Error creating room".to_string()))
        }
    }
}

#[component]
pub fn CreateRoomPage(cx: Scope) -> impl IntoView {
    let create_room = create_server_action::<CreateRoom>(cx);
    let (room_name, set_room_name) = create_signal(cx, String::new());

    let value = create_room.value();
    let has_error = move || value.with(|val| matches!(val, Some(Err(_))));

    let input_params = InputParams {
        label: "Room Name",
        placeholder: "Enter a room name",
        name: "room_name",
        input_type: InputType::Text,
        value: (room_name, set_room_name),
    };

    let notification_params = NotificationParams {
        message: "messaggio di errore normale che non va troppo male",
        notification_type: NotificationType::Error,
    };

    view! { cx,
        <div class="flex h-screen justify-center items-center">
            <ActionForm action=create_room class="space-y-3 w-80">

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
                            <b>CREATE</b>
                        </button>
                    </div>
                </div>

            </ActionForm>

            <Show when=has_error fallback=|_| ()>
                <NotificationComponent params=notification_params/>
            </Show>

        </div>
    }
}
