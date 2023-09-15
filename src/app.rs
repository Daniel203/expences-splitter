use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::{
    error_template::{AppError, ErrorTemplate},
    pages::{
        auth::{get_user, GetUser, Login, LoginPage, Register, RegisterPage},
        create_room_page::CreateRoomPage,
        dashboard_page::DashboardPage,
        home_page::HomePage,
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
        <Title text="Expences Splitter"/>

        // content for this welcome page
        <Router fallback=|cx| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { cx, <ErrorTemplate outside_errors/> } .into_view(cx)
        }>
            <main class="h-screen bg-primaryBg">
                <Routes>
                    <Route path="" view=|cx| view! { cx, <Page/> }>
                        <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                        <Route path="new" view=|cx| view! { cx, <CreateRoomPage/> }/>
                        <Route path="join" view=|cx| view! { cx, <JoinRoomPage/> }/>
                        <Route path="room/:id" view=|cx| view! { cx, <DashboardPage/> }/>
                        <Route path="register" view=|cx| view! { cx, <RegisterPage /> }/>
                        <Route path="login" view=|cx| view! { cx, <LoginPage /> }/>
                    </Route>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
pub fn Page(cx: Scope) -> impl IntoView {
    let login = create_server_action::<Login>(cx);
    let register = create_server_action::<Register>(cx);

    // get the user every time that the "login" or "register" server functions are called
    let user = create_resource(
        cx,
        move || {
            return (login.version().get(), register.version().get());
        },
        move |_| {
            return get_user(cx);
        },
    );

    return view! {cx,
        <Transition fallback=move || view! { cx, <p>"Loading..."</p>} >
            {move || {
                if let Some(Ok(Some(_))) = user.read(cx) {  
                    return view! {cx, <Outlet />}.into_view(cx);
                } else {  
                    return view! {cx, <LoginPage />}.into_view(cx);
                }
            }}
        </Transition>
    };
}
