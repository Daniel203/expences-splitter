use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::{
    error_template::{AppError, ErrorTemplate},
    pages::{
        auth::{get_user, GetUser, Login, LoginPage, Logout, LogoutPage, Register, RegisterPage},
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
            view! { cx, <ErrorTemplate outside_errors/> }.into_view(cx)
        }>
            <main class="h-screen">
                <Routes>
                    <Route path="register" view=|cx| view! { cx, <RegisterPage/> }/>
                    <Route path="login" view=|cx| view! { cx, <LoginPage/> }/>
                    <Route path="" view=|cx| view! { cx, <Page/> }>
                        <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                        <Route path="new" view=|cx| view! { cx, <CreateRoomPage/> }/>
                        <Route path="join" view=|cx| view! { cx, <JoinRoomPage/> }/>
                        <Route path="room/:id" view=|cx| view! { cx, <DashboardPage/> }/>
                        <Route path="logout" view=|cx| view! { cx, <LogoutPage/> }/>
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
    let logout = create_server_action::<Logout>(cx);

    // get the user every time that the "login" or "register" server functions are called
    let user = create_resource(
        cx,
        move || {
            return (
                login.version().get(),
                register.version().get(),
                logout.version().get(),
            );
        },
        move |_| {
            return get_user(cx);
        },
    );

    return view! { cx,
        <Transition fallback=move || {
            view! { cx, <p>"Loading..."</p> }
        }>
            {move || {
                if let Some(Ok(Some(_))) = user.read(cx) {
                    return view! { cx, <Outlet/> }.into_view(cx);
                } else {
                    return view! { cx, <UserNotAuthenticated/> }.into_view(cx);
                }
            }}

        </Transition>
    };
}

#[component]
pub fn UserNotAuthenticated(cx: Scope) -> impl IntoView {
    return view! { cx,
        <div class="flex h-screen justify-center items-center">
            <div>
                <p class="font-bold text-3xl mb-6">"You are not logged in"</p>
                <p class="mb-2 text-center">
                    "Already have an account? " <A href="/login">
                        <b>
                            <u>"Login now!"</u>
                        </b>
                    </A>
                </p>
                <p class="text-center">
                    "Don't have an account? " <A href="/register">
                        <b>
                            <u>"Register now!"</u>
                        </b>
                    </A>
                </p>
            </div>
        </div>
    };
}
