use leptos::*;
use leptos_router::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="flex h-screen justify-center items-center">
            <div class="flex flex-col space-y-6 w-80">
                <A href="join">
                    <button class="btn btn-primary btn-lg w-full">
                        <b>ENTER</b>
                        an existing room
                    </button>
                </A>

                <A href="new">
                    <button class="btn btn-primary btn-lg w-full">
                        <b>CREATE</b>
                        a new room
                    </button>
                </A>
            </div>
        </div>
    }
}
