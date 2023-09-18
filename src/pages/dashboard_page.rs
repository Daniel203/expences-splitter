use leptos::*;
use leptos_router::*;

#[derive(Params, PartialEq, Eq, Clone)]
struct DashboardPageParams {
    id: String,
}

#[component]
pub fn DashboardPage(cx: Scope) -> impl IntoView {
    let params = use_params::<DashboardPageParams>(cx);

    let id = move || {
        return params.with(|p| {
           p.clone().map(|p| p.id).unwrap_or_default()
        });
    };

    return view! { cx,
        <div class="flex h-screen justify-center items-center">
            <p>{id}</p>
        </div>
    };
}
