use leptos::*;
use leptos_router::*;
use crate::components::expenses_component::ExpensesComponent;

#[derive(Params, PartialEq, Eq, Clone)]
struct DashboardPageParams {
    id: String,
}

#[component]
pub fn DashboardPage() -> impl IntoView {
    let params = use_params::<DashboardPageParams>();

    let id = move || {
        params.with(|p| {
           p.clone().map(|p| p.id).unwrap_or_default()
        })
    };

    view! {
        <div class="flex flex-col h-screen justify-center items-center">
            <p class="text-2xl font-bold mb-4">{id}</p>

            <div class="grid grid-cols-2 gap-4">
                <div>
                    <ExpensesComponent/>
                </div>

                <div>
                    <ExpensesComponent/>
                </div>
            </div>
        </div>
    }
}
