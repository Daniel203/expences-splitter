use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::{
    error_template::{AppError, ErrorTemplate},
    pages::{
        auth::{get_user, Login, LoginPage, Logout, LogoutPage, Register, RegisterPage},
        create_room_page::CreateRoomPage,
        dashboard_page::DashboardPage,
        home_page::HomePage,
        join_room_page::JoinRoomPage,
    },
};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/expenses-splitter.css"/>

        // sets the document title
        <Title text="Expenses Splitter"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <main class="h-screen">
                <Routes>
                    <Route path="register" view=|| view! { <RegisterPage/> }/>
                    <Route path="login" view=|| view! { <LoginPage/> }/>
                    <Route path="" view=|| view! { <Page/> }>
                        <Route path="" view=|| view! { <HomePage/> }/>
                        <Route path="new" view=|| view! { <CreateRoomPage/> }/>
                        <Route path="join" view=|| view! { <JoinRoomPage/> }/>
                        <Route path="room/:id" view=|| view! { <DashboardPage/> }/>
                        <Route path="logout" view=|| view! { <LogoutPage/> }/>
                    </Route>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
pub fn Page() -> impl IntoView {
    let login = create_server_action::<Login>();
    let register = create_server_action::<Register>();
    let logout = create_server_action::<Logout>();

    // get the user every time that the "login" or "register" server functions are called
    let user = create_resource(
        move || {
            (
                login.version().get(),
                register.version().get(),
                logout.version().get(),
            )
        },
        move |_| {
            get_user()
        },
    );

    view! {
        <Transition fallback=move || {
            view! { <p>"Loading..."</p> }
        }>
            {move || {
                if let Some(Ok(Some(_))) = user.get() {
                    view! { <Outlet/> }.into_view()
                } else {
                    view! { <UserNotAuthenticated/> }.into_view()
                }
            }}

        </Transition>
    }
}

#[component]
pub fn UserNotAuthenticated() -> impl IntoView {
    view! {
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
    }
}
