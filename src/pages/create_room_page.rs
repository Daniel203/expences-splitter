use leptos::*;
use leptos_router::*;

#[server(CreateRoom, "/api")]
pub async fn create_room(room_name: String) -> Result<(), ServerFnError> {
    use crate::db;
    use crate::models::room::Room;

    let mut conn = db().await?;

    // TODO: replace the select with insert of course
    let rooms: Vec<Room> = sqlx::query_as!(
        Room,
        "SELECT * FROM rooms",
    )
    .fetch_all(&mut conn)
    .await?;

    
    return Ok(());
}

#[component]
pub fn CreateRoomPage(cx: Scope) -> impl IntoView {
    let create_room = create_server_action::<CreateRoom>(cx);
    let value = create_room.value();
    let has_error = move || value.with(|val| matches!(val, Some(Err(_))));

    return view! { cx,
    <div class="flex h-screen justify-center items-center">
        <ActionForm action=create_room>

            <div class="grid grid-cols-3 grid-row-2 gap-y-8  w-80">

                <div class="col-span-3">
                    <label class="block text-white text-sm font-bold mb-2" for="room_name">Enter the Room Name</label>
                    <input id="room_name" type="text" placeholder="Room Name" name="room_name"/>
                </div>

                <A href="/">
                    <button class="btn-warn btn-lg col-span-1">
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="3" stroke="currentColor" class="w-6 h-6">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M10.5 19.5L3 12m0 0l7.5-7.5M3 12h18" />
                        </svg>
                    </button>
                </A>

                <button class="btn-primary btn-lg col-span-2" type="submit"><b>CREATE</b></button>

            </div>

        </ActionForm>
    </div>
    };
}
