use leptos::*;

#[derive(Clone, Copy)]
pub struct InputParams<T> 
    where T: Fn() -> Option<String>
{
    pub label: &'static str,
    pub placeholder: &'static str,
    pub name: &'static str,
    pub value: ReadSignal<String>,
    pub set_value: WriteSignal<String>,
    pub value_touched: ReadSignal<bool>,
    pub set_value_touched: WriteSignal<bool>,
    pub value_error: T,
}

#[component]
// pub fn InputCompnent<T: std::ops::Fn<()>>(cx: Scope, params: InputParams<T>) -> impl IntoView {
pub fn InputComponent<T: Fn() -> Option<String> + 'static + Clone, std::marker::Copy>(cx: Scope, params: InputParams<T>) -> impl IntoView {
    return view! {cx,
        <div class="form-control w-full">
            <label class="label-text font-bold mb-2">{params.label}</label>
            <input
                class="input input-bordered input-primary w-full"
                class=(
                    "input-error",
                    move || params.value_touched.get() && (params.clone().value_error)().is_some(),
                )
                class=(
                    "input-primary",
                    move || !params.value_touched.get() || (params.value_error)().is_none(),
                )
                type="text"
                placeholder=params.placeholder
                name=params.name
                on:input=move |ev| params.set_value.update(|x| *x = event_target_value(&ev))
                on:blur=move |_| params.set_value_touched.set(true)
                required
            />
            <label>
                // {move || {
                //     if params.value_touched.get() && (params.value_error)().is_some() {
                //         view! { cx,
                //             <span class="label-text-alt text-error">
                //                 // {move || params.value_error}
                //                 Test error
                //             </span>
                //         }
                //     } else {
                //         view! { cx,
                //             <span class="label-text-alt text-transparent">
                //                 Username error
                //             </span>
                //         }
                //     }
                // }}
            </label>
        </div>
    };
}
