use leptos::*;

#[derive(Clone, Copy)]
pub enum InputType {
    Text,
    Password,
    Number,
}

impl InputType {
    pub fn as_str(&self) -> &'static str {
        match self {
            InputType::Text => "text",
            InputType::Password => "password",
            InputType::Number => "number",
        }
    }
}

#[derive(Clone, Copy)]
pub struct InputWithControlsParams<T>
where
    T: Fn() -> Option<String>,
{
    pub label: &'static str,
    pub placeholder: &'static str,
    pub name: &'static str,
    pub input_type: InputType,
    pub value: ReadSignal<String>,
    pub set_value: WriteSignal<String>,
    pub value_touched: ReadSignal<bool>,
    pub set_value_touched: WriteSignal<bool>,
    pub value_error: T,
}

#[component]
pub fn InputWithControlsComponent<T: Fn() -> Option<String> + 'static + Clone + Copy>(
    cx: Scope,
    params: InputWithControlsParams<T>,
) -> impl IntoView {
    return view! { cx,
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

                type=params.input_type.as_str()             placeholder=params.placeholder
                name=params.name
                on:input=move |ev| params.set_value.update(|x| *x = event_target_value(&ev))
                on:blur=move |_| params.set_value_touched.set(true)
                required
            />
            <label>
                {move || {
                    if params.value_touched.get() && (params.value_error)().is_some() {
                        view! { cx,
                            <span class="label-text-alt text-error">
                                {move || params.value_error}
                            </span>
                        }
                    } else {

                        view! { cx,
                            <span class="label-text-alt text-transparent">Username error</span>
                        }
                    }
                }}

            </label>
        </div>
    };
}
