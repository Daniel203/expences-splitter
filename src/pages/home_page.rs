use leptos::*;
use leptos_router::*;

#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
    return view! { cx,
        <div class="flex h-screen justify-center items-center">
            <div class="flex flex-col space-y-8">
                <A href="join">
                    <button class="btn-primary btn-xxl w-80" >
                        <b>ENTER</b> an existing room
                    </button>
                </A>

                <A href="new">
                    <button class="btn-primary btn-xxl w-80">
                        <b>CREATE</b> a new room
                    </button>
                </A>
            </div>
        </div>
    };
}
