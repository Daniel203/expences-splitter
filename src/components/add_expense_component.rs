use crate::{
    components::{
        input_component::{
            InputComponent, InputParams, InputType, InputWithControlsComponent,
            InputWithControlsParams,
        },
        notification_component::{NotificationComponent, NotificationParams, NotificationType},
        user_in_room_component::get_users_in_room,
    },
    models::{expense::Expense, user::User},
};
use leptos::*;
use leptos_router::{ActionForm, FromFormData};
use std::{collections::HashMap, iter};

type SelectedUsers = HashMap<User, bool>;

#[server(AddExpense, "/api")]
pub async fn add_expense(expense: Expense) -> Result<(), ServerFnError> {
    use crate::state::auth;
    use crate::state::pool;
    use leptos::logging::log;

    let pool = pool()?;

    log!("fn: add_expense() - adding expense: {:?}", expense);

    // add expense
    let res = sqlx::query!(
        "INSERT INTO expense (paid_by, amount, title, description, room_id) VALUES (?, ?, ?, ?, ?) RETURNING id",
        expense.paid_by,
        expense.amount,
        expense.title,
        expense.description,
        expense.room_id
    )
        .fetch_one(&pool)
        .await?;

    log::info!("fn: add_expense() - added expense: {:?}", res);

    log::info!("fn: add_expense() - adding participants: {:?}", expense.participants);

    // add values to user_expense
    for id_participant in expense.participants {
        sqlx::query!(
            "INSERT INTO user_expense (user_id, expense_id) VALUES (?, ?)",
            id_participant,
            res.id
        )
            .execute(&pool)
            .await?;
    }

    log::info!("fn: add_expense() - added participants");

    Ok(())
}

#[component]
pub fn AddExpenseComponent(room_id: String) -> impl IntoView {
    let action = create_server_action::<AddExpense>();

    let value = action.value();
    let has_error = move || value.with(|val| matches!(val, Some(Err(_))));

    let room_id_clone = room_id.clone();
    let users = create_resource(
        move || (),
        move |_| get_users_in_room(room_id_clone.clone()),
    );

    let (who_payed, set_who_payed) = create_signal::<Option<User>>(None);
    let (selected_participants, set_selected_participants) = create_signal(SelectedUsers::new());
    let (amount, set_amount) = create_signal("".to_string());
    let (title, set_title) = create_signal("".to_string());
    let (description, set_description) = create_signal("".to_string());

    let amount_error = move || {
        if amount.with(String::is_empty) {
            return Some("Amount is required".to_string());
        } else if let Err(_) = amount.get().parse::<f64>() {
            return Some("Amount must be a number".to_string());
        } else {
            return None;
        }
    };

    let title_error = move || {
        if title.with(String::is_empty) {
            return Some("Title is required".to_string());
        } else {
            return None;
        }
    };

    let input_title_params = InputWithControlsParams {
        label: "Title".to_string(),
        placeholder: "Title".to_string(),
        name: "title".to_string(),
        input_type: InputType::Text,
        value: (title, set_title),
        value_error: (title_error),
    };

    let input_description_params = InputParams {
        label: "Description".to_string(),
        placeholder: "Description".to_string(),
        name: "description".to_string(),
        input_type: InputType::Text,
        value: (description, set_description),
    };

    let input_amount_params = InputWithControlsParams {
        label: "Amount".to_string(),
        placeholder: "0.00".to_string(),
        name: "amount".to_string(),
        input_type: InputType::Text,
        value: (amount, set_amount),
        value_error: amount_error,
    };

    let get_notification_params = move || {
        let server_message = value().unwrap().unwrap_err().to_string();
        let client_message = server_message.replace("error running server function: ", "");

        NotificationParams {
            message: client_message,
            notification_type: NotificationType::Error,
        }
    };

    let is_form_valid =
        move || title_error().is_none() && amount_error().is_none() && who_payed.get().is_some();

    let add_expense_click = move |_| {
        let room_id = room_id.clone();
        spawn_local(async move {
            let selected_users: Vec<i64> = selected_participants
                .get()
                .iter()
                .filter_map(
                    |(user, is_selected)| {
                        if *is_selected {
                            Some(user.id)
                        } else {
                            None
                        }
                    },
                )
                .collect();

            let mut expense = Expense::default();
            expense.paid_by = who_payed.get().unwrap().id;
            expense.room_id = room_id;
            expense.title = title.get();
            expense.amount = amount.get().parse::<f64>().unwrap();
            expense.participants = selected_users;

            if description.get().len() > 0 {
                expense.description = Some(description.get());
            }

            add_expense(expense).await;
        });
    };

    let paid_by_view = move || match users.get() {
        Some(Ok(users)) => {
            let first_option = view! {
                <option disabled selected required>
                    "Select who paid"
                </option>
            };

            let users_options = users.into_iter().map(|user| {
                let user_clone = user.clone();

                view! {
                    <option on:click=move |_| {
                        set_who_payed(Some(user.clone()))
                    }>{user_clone.username}</option>
                }
            });

            let all_options = iter::once(first_option).chain(users_options).collect_view();

            view! { <select class="select select-primary w-full">{all_options}</select> }
                .into_view()
        }
        _ => view! { <p>"Error"</p> }.into_view(),
    };

    let participants_view = move || match users.get() {
        Some(Ok(users)) => users
            .into_iter()
            .map(|user| {
                let user_clone = user.clone();

                let on_click = move |_| {
                    set_selected_participants.update(|selected_participants| {
                        if let Some(is_selected) = selected_participants.get(&user) {
                            selected_participants.insert(user.clone(), !is_selected);
                        } else {
                            selected_participants.insert(user.clone(), true);
                        }
                    });
                };

                view! {
                    <div class="form-control">
                        <label class="label cursor-pointer">
                            <span class="label-text">{user_clone.username}</span>
                            <input
                                type="checkbox"
                                class="checkbox checkbox-primary"
                                on:click=on_click
                            />
                        </label>
                    </div>
                }
            })
            .collect_view(),
        _ => view! { <p>"Error"</p> }.into_view(),
    };

    view! {
        <div class="mt-10 w-80">
            <Transition fallback=move || view! { <p>"Loading..."</p> }>
                <InputWithControlsComponent params=input_title_params.clone()/>

                <InputComponent params=input_description_params.clone()/>

                <label class="label-text font-bold mb-2">"Who paid?"</label>
                {move || paid_by_view()}

                <div class="mt-6"></div>

                <label class="label-text font-bold mb-2">"Who participated?"</label>
                {move || participants_view()}

                <InputWithControlsComponent params=input_amount_params.clone()/>

            </Transition>
            <button
                class="btn btn-primary btn-lg w-full"
                prop:disabled=move || !is_form_valid()
                on:click=add_expense_click
            >
                <b>ADD EXPENSE</b>
            </button>

            <Show when=has_error fallback=|| ()>
                <NotificationComponent params=get_notification_params()/>
            </Show>
        </div>
    }
}
