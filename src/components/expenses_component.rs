use leptos::*;

use crate::models::expense::Expense;

#[component]
pub fn ExpensesComponent() -> impl IntoView {
    let expenses: [Expense; 5] = [
        Expense {
            id: 1,
            amount: 100,
            participants: vec![1, 2, 3],
            title: "Expense 1".to_string(),
            description: "Description 1".to_string(),
            paid_by: 1,
            room_id: "Room 1".to_string(),
            #[cfg(feature = "ssr")]
            created_at: None,
        },
        Expense {
            id: 2,
            amount: 75,
            participants: vec![2, 3],
            title: "Expense 2".to_string(),
            description: "Description 2".to_string(),
            paid_by: 2,
            room_id: "Room 1".to_string(),
            #[cfg(feature = "ssr")]
            created_at: None,
        },
        Expense {
            id: 3,
            amount: 50,
            participants: vec![1, 3],
            title: "Expense 3".to_string(),
            description: "Description 3".to_string(),
            paid_by: 3,
            room_id: "Room 2".to_string(),
            #[cfg(feature = "ssr")]
            created_at: None,
        },
        Expense {
            id: 4,
            amount: 120,
            participants: vec![1, 2],
            title: "Expense 4".to_string(),
            description: "Description 4".to_string(),
            paid_by: 1,
            room_id: "Room 2".to_string(),
            #[cfg(feature = "ssr")]
            created_at: None,
        },
        Expense {
            id: 5,
            amount: 90,
            participants: vec![2, 3],
            title: "Expense 5".to_string(),
            description: "Description 5".to_string(),
            paid_by: 2,
            room_id: "Room 3".to_string(),
            #[cfg(feature = "ssr")]
            created_at: None,
        },
    ];


    view! {
        <div class="overflow-x-auto w-full">
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

                    {expenses
                        .into_iter()
                        .map(|expense| {
                            view! {
                                <tr>
                                    <td>{expense.title}</td>
                                    <td>{expense.paid_by}</td>
                                    <td>{expense.description}</td>
                                    <td>{expense.amount}</td>
                                </tr>
                            }
                        })
                        .collect_view()}

                </tbody>
            </table>
        </div>
    }
}
