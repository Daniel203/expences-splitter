use leptos::*;
use crate::components::user_in_room_component::get_users_in_room;

#[component]
pub fn AddExpenseComponent(room_id: String) -> impl IntoView {
    let users = create_resource(move || (), move |_| get_users_in_room(room_id.clone()));

    let checkboxes_view = move || {
        users.get().map(move |users| match users {
            Err(_) => view! {<p>"Error"</p>}.into_view(),
            Ok(users) => {
                if users.is_empty() {
                    return view! {<p>"No users"</p>}.into_view();
                }

                users
                    .into_iter()
                    .map(|user| {
                        view! {
                            <div class="form-control">
                                <label class="label cursor-pointer">
                                    <span class="label-text">{user.username}</span> 
                                    <input type="checkbox" checked="checked" class="checkbox" />
                                </label>
                            </div>
                        }
                    })
                    .collect_view()
            }
        })
    };

    view! {
        <div class="mt-10 w-80">
            <Transition fallback=move || view!{<p>"Loading..."</p>}>
                {move || checkboxes_view()}
            </Transition>
            // <button class="btn btn-primary btn-lg w-full">
            //     <b>CREATE</b>
            //     a new expense
            // </button>

        </div>
    }
}
