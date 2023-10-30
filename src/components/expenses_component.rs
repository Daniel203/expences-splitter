use leptos::*;

use crate::models::expense::Expense;

#[server(GetExpensesInRoom, "/api")]
pub async fn get_expenses_in_room(room_id: String) -> Result<Vec<Expense>, ServerFnError> {
    use crate::state::pool;
    use crate::models::expense::ExpenseDTO;
    use leptos::logging::log;

    let pool = pool()?;

    log!("fn: get_expenses_in_room() - getting expenses in room");

    let expenses_dto: Vec<ExpenseDTO> =
        sqlx::query_file_as!(ExpenseDTO, "queries/get_expenses_in_room.sql", room_id)
            .fetch_all(&pool)
            .await?;

    let expenses = expenses_dto
        .into_iter()
        .filter(|expense| expense.id.is_some())
        .map(|expense| Expense::from(expense))
        .collect();

    log!("fn: get_expenses_in_room() - expenses: {:?}", expenses);

    Ok(expenses)
}

#[component]
pub fn ExpensesComponent(room_id: String) -> impl IntoView {
    let expenses = create_resource(move || (), move |_| get_expenses_in_room(room_id.clone()));

    let expenses_view = move || {
        expenses.get().map(move |expenses| match expenses {
            Err(_) => view! {<p>"Error"</p>}.into_view(),
            Ok(expenses) => {
                if expenses.is_empty() {
                    return view! {<tr><td>"No expenses"</td></tr>}.into_view();
                }

                expenses
                    .into_iter()
                    .map(|expense| {
                        view! {
                            <tr>
                                <td>{expense.title}</td>
                                <td>{expense.description}</td>
                                <td>{expense.amount}</td>
                            </tr>
                        }
                    })
                    .collect_view()
            }
        })
    };

    view! {
        <div class="overflow-x-auto w-full">
            <Transition fallback=move || view!{<p>"Loading..."</p>}>
                <table class="table table-zebra">
                    <thead>
                        <tr>
                            <th>Title</th>
                            <th>By</th>
                            <th>Descripiton</th>
                            <th>Amount</th>
                        </tr>
                    </thead>
                    <tbody>
                        {move || expenses_view()}
                    </tbody>
                </table>
            </Transition>
        </div>
    }
}
