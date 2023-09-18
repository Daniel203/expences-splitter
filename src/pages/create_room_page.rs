use crate::models::room::Room;
use leptos::*;
use leptos_router::*;

#[server(CreateRoom, "/api")]
pub async fn create_room(cx: leptos::Scope, room_name: String) -> Result<(), ServerFnError> {
    todo!()
    // use crate::state::pool;
    // let pool = pool(cx)?;
    //
    // let does_room_exists = sqlx::query_as!(Room, "SELECT * FROM rooms WHERE room_name = $1", room_name)
    //     .fetch_optional(&pool)
    //     .await?;
    //
    // log!("does room exists: {:?}", does_room_exists);
    //
    // if let Some(_) = does_room_exists {
    //     return Err(ServerFnError::ServerError("Room already exists".to_string()));
    // } 
    //
    // log!("creating room: {:?}", room_name);
    //
    // // insert the room and return the new id
    // let res = sqlx::query_as!(Room, "INSERT INTO rooms (room_name, max_participants, owner) VALUES ($1, 20, 3) RETURNING *", room_name)
    //     .fetch_one(&pool)
    //     .await;
    //
    // match res {
    //     Ok(room) => {
    //         log!("created room: {:?}", room);
    //         let id = room.id;
    //         leptos_axum::redirect(cx, &format!("/room/{}", id));
    //         return Ok(());
    //     },
    //     Err(e) => {
    //         log!("error creating room: {:?}", e);
    //         return Err(ServerFnError::ServerError("Error creating room".to_string()));
    //     }
    // }
}

#[component]
pub fn CreateRoomPage(cx: Scope) -> impl IntoView {
    let create_room = create_server_action::<CreateRoom>(cx);
    let value = create_room.value();
    let has_error = move || value.with(|val| matches!(val, Some(Err(_))));

    return view! { cx,
        <div class="flex h-screen justify-center items-center">
            <ActionForm action=create_room>

                <div class="grid grid-cols-3 grid-row-2 gap-y-8 w-80">

                    <div class="col-span-3">
                        <label class="block text-white text-sm font-bold mb-2" for="room_name">
                            Enter the Room Name
                        </label>
                        <input id="room_name" type="text" placeholder="Room Name" name="room_name"/>
                    </div>

                    <A href="/">
                        <button class="btn-warn btn-lg col-span-1">
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

                    <button class="btn-primary btn-lg col-span-2" type="submit">
                        <b>CREATE</b>
                    </button>

                </div>

            </ActionForm>

            <Show when=has_error fallback=|_| ()>
                // TODO: create message that says error
                <div></div>
            </Show>

        </div>
    };
}
