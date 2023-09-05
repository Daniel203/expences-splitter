use leptos::{ev::SubmitEvent, html::Input, *};
use leptos_router::*;

#[component]
pub fn JoinRoomPage(cx: Scope) -> impl IntoView {
    let (room_id, set_room_id) = create_signal(cx, "".to_string());
    let room_id_input: NodeRef<Input> = create_node_ref(cx);

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let value = room_id_input().expect("<room id> to exist").value();
        set_room_id(value);
    };

    return view! { cx,
    <div class="flex h-screen justify-center items-center">
        <form on:submit=on_submit id="form">

            <div class="grid grid-cols-3 grid-row-2 gap-y-8  w-80">

                <div class="col-span-3">
                    <label class="block text-white text-sm font-bold mb-2" for="room_id">Enter the Room Id</label>
                    <input id="room_id" type="text" placeholder="Room Id" value=room_id node_ref=room_id_input />
                </div>

                <A href="/">
                    <button class="btn-warn btn-lg col-span-1">
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="3" stroke="currentColor" class="w-6 h-6">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M10.5 19.5L3 12m0 0l7.5-7.5M3 12h18" />
                        </svg>
                    </button>
                </A>

                <button class="btn-primary btn-lg col-span-2" type="submit" form="form"><b>JOIN</b></button>

            </div>
        </form>
    </div>
    };
}
