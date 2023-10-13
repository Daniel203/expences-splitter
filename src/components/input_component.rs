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

#[derive(Clone)]
pub struct InputParams {
    pub label: String,
    pub placeholder: String,
    pub name: String,
    pub input_type: InputType,
    pub value: (ReadSignal<String>, WriteSignal<String>),
}

#[derive(Clone)]
pub struct InputWithControlsParams<T>
where
    T: Fn() -> Option<String>,
{
    pub label: String,
    pub placeholder: String,
    pub name: String,
    pub input_type: InputType,
    pub value: (ReadSignal<String>, WriteSignal<String>),
    pub value_error: T,
}

#[component]
pub fn InputComponent(params: InputParams,) -> impl IntoView {
    view! {
        <div class="form-control w-full">
            <label class="label-text font-bold mb-2">{params.label}</label>
            <input
                class="input input-bordered input-primary w-full"
                type=params.input_type.as_str()
                placeholder=params.placeholder
                name=params.name
                on:input=move |ev| params.value.1.update(|x| *x = event_target_value(&ev))
                required
            />
            <label>
                <span class="label-text-alt text-transparent">Username error</span>
            </label>
        </div>
    }
}

#[component]
pub fn InputWithControlsComponent<T: Fn() -> Option<String> + 'static + Clone + Copy>(
    params: InputWithControlsParams<T>,
) -> impl IntoView {
    let (value_touched, set_value_touched) = create_signal(false);

    view! {
        <div class="form-control w-full">
            <label class="label-text font-bold mb-2">{params.label}</label>
            <input
                class="input input-bordered input-primary w-full"
                class=("input-error", move || value_touched() && (params.value_error)().is_some())

                type=params.input_type.as_str()
                placeholder=params.placeholder
                name=params.name
                on:input=move |ev| params.value.1.update(|x| *x = event_target_value(&ev))
                on:blur=move |_| set_value_touched(true)
                required
            />
            <label>
                {move || {
                    if value_touched() && (params.value_error)().is_some() {
                        view! {
                            <span class="label-text-alt text-error">
                                {move || params.value_error}
                            </span>
                        }
                    } else {
                        view! {
                            <span class="label-text-alt text-transparent">Username error</span>
                        }
                    }
                }}

            </label>
        </div>
    }
}
