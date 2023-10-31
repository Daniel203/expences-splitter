use std::collections::HashMap;

use crate::{components::{user_in_room_component::get_users_in_room, input_component::{InputComponent, InputType, InputParams}}, models::user::User};
use leptos::*;

type UsersSelection = HashMap<User, bool>;

fn populate_users_selection(
    set_users_selection: WriteSignal<UsersSelection>,
    users: Vec<User>,
) -> () {
    let mut users_selection = UsersSelection::new();
    for user in users {
        users_selection.insert(user, false);
    }
    set_users_selection(users_selection);
}

#[component]
pub fn AddExpenseComponent(room_id: String) -> impl IntoView {
    let users = create_resource(move || (), move |_| get_users_in_room(room_id.clone()));

    let (payers_selection, set_payers_selection) = create_signal(UsersSelection::new());
    let (participants_selection, set_participants_selection) = create_signal(UsersSelection::new());

    let (amount, set_amount) = create_signal("0.0".to_string());

    let input_amount_params = InputParams {
        label: "Amount".to_string(),
        placeholder: "0.00".to_string(),
        name: "amount".to_string(),
        input_type: InputType::Number,
        value: (amount, set_amount),
    };

    let users_selection_view = move |users_selection: ReadSignal<UsersSelection>, set_users_selection: WriteSignal<UsersSelection>| {
        match users.get() {
            Some(Ok(users)) => {
                if users.is_empty() {
                    return view! { <p>"No users"</p> }.into_view();
                }

                populate_users_selection(set_users_selection, users.clone());

                users
                    .into_iter()
                    .map(|user| {
                        let user_tmp = user.clone();

                        view! {
                            <div class="form-control">
                                <label class="label cursor-pointer">
                                    <span class="label-text">{user_tmp.username}</span>
                                    <input
                                        type="checkbox"
                                        class="checkbox checkbox-primary"
                                        on:click=move |_| {
                                            let mut users_selection_tmp = users_selection.get();
                                            let selected = users_selection_tmp.get(&user).unwrap();
                                            users_selection_tmp.insert(user.clone(), !selected);
                                            logging::log!("{:?}", users_selection_tmp.clone());
                                            set_users_selection(users_selection_tmp);
                                        }
                                    />

                                </label>
                            </div>
                        }
                    })
                    .collect_view()
            }
            _ => view! { <p>"Error"</p> }.into_view(),
        }
    };

    view! {
        <div class="mt-10 w-80">
            <Transition fallback=move || view! { <p>"Loading..."</p> }>
                <p class="text-xl font-bold">Who paid?</p>
                {move || users_selection_view(payers_selection, set_payers_selection)}

                <div class="mt-6"></div>

                <p class="text-xl font-bold">Who participated?</p>
                {move || users_selection_view(participants_selection, set_participants_selection)}

                <InputComponent params=input_amount_params.clone() />
            </Transition>
        </div>
    }
}
