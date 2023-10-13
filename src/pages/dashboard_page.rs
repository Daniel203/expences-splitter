use crate::components::{
    expenses_component::ExpensesComponent, user_in_room_component::UserInRoomComponent,
};
use leptos::*;
use leptos_router::*;

#[derive(Params, PartialEq, Eq, Clone)]
struct DashboardPageParams {
    id: String,
}

#[server(GetRoomId, "/api")]
pub async fn get_room_name(room_id: String) -> Result<String, ServerFnError> {
    use crate::state::pool;
    use leptos::logging::log;

    let pool = pool()?;

    log!("fn: get_room_name() - getting the room name");

    let room_name: String = sqlx::query_file!("queries/get_room_name_by_id.sql", room_id)
        .map(|row| row.room_name)
        .fetch_one(&pool)
        .await?;

    log!("fn: get_users_in_room() - room_name: {:?}", room_name);

    Ok(room_name)
}

#[component]
pub fn DashboardPage() -> impl IntoView {
    let params = use_params::<DashboardPageParams>();

    let id = move || params.with(|p| p.clone().map(|p| p.id).unwrap_or_default());
    let room_name = create_resource(move || (), move |_| get_room_name(id()));

    view! {
        <div class="flex flex-col h-screen justify-center items-center">
            <Transition fallback=move || view!{<p>"Loading..."</p>}>
                {move || 
                    view!{
                        <p class="text-2xl font-bold mb-4">{room_name.get()}</p>
                    }.into_view()
                }
            </Transition>

            <div class="grid grid-cols-2 gap-4">
                <div>
                    <ExpensesComponent/>
                </div>

                <div>
                    <UserInRoomComponent room_id=id() />
                </div>
            </div>
        </div>
    }
}
