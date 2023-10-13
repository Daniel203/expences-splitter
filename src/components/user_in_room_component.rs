use crate::models::user::User;
use leptos::*;

#[server(GetUsersInRoom, "/api")]
pub async fn get_users_in_room(room_id: String) -> Result<Vec<User>, ServerFnError> {
    use crate::state::pool;
    use leptos::logging::log;

    let pool = pool()?;

    log!("fn: get_users_in_room() - getting users in room");

    let users: Vec<User> = sqlx::query_file_as!(User, "queries/get_users_in_room.sql", room_id)
        .fetch_all(&pool)
        .await?;

    log!("fn: get_users_in_room() - users: {:?}", users);

    Ok(users)
}

#[component]
pub fn UserInRoomComponent(room_id: String) -> impl IntoView {
    let users = create_resource(move || (), move |_| get_users_in_room(room_id.clone()));

    let users_view = move || {
        users.get().map(move |users| match users {
            Err(_) => view! {<p>"Error"</p>}.into_view(),
            Ok(users) => {
                if users.is_empty() {
                    view! {<tr><td>"No users"</td></tr>}.into_view()
                } else {
                    users
                        .into_iter()
                        .map(|user| {
                            view! {
                                <tr>
                                    <td>{user.username}</td>
                                </tr>
                            }
                        })
                        .collect_view()
                }
            }
        })
    };

    view! {
        <div class="overflow-x-auto w-full">
            <Transition fallback=move || view!{<p>"Loading..."</p>}>
                <table class="table table-zebra">
                    <thead>
                        <tr>
                            <th>User</th>
                        </tr>
                    </thead>
                    <tbody>
                        {move || users_view() }
                    </tbody>
                </table>
            </Transition>
        </div>
    }
}
