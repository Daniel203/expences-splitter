use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::{
    error_template::{AppError, ErrorTemplate},
    pages::{
        create_room_page::CreateRoomPage, dashboard_page::DashboardPage, home_page::HomePage,
        join_room_page::JoinRoomPage,
    },
};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! { cx,
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/expences-splitter.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|cx| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { cx,
                <ErrorTemplate outside_errors/>
            }
            .into_view(cx)
        }>
            <main class="h-screen bg-primaryBg">
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                    <Route path="/new" view=|cx| view! { cx, <CreateRoomPage/> }/>
                    <Route path="/join" view=|cx| view! { cx, <JoinRoomPage/> }/>
                    <Route path="/room/:id" view=|cx| view! { cx, <DashboardPage/> }/>
                </Routes>
            </main>
        </Router>
    }
}
