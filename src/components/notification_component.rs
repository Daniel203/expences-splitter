use leptos::*;

#[derive(Copy, Clone)]
pub enum NotificationType {
    Error,
    Success,
    Info,
    Warning,
}

impl NotificationType {
    pub fn css_class(&self) -> &'static str {
        match self {
            NotificationType::Error => "alert-error",
            NotificationType::Success => "alert-success",
            NotificationType::Info => "alert-info",
            NotificationType::Warning => "alert-warning",
        }
    }

    pub fn title(&self) -> &'static str {
        match self {
            NotificationType::Error => "Error",
            NotificationType::Success => "Success",
            NotificationType::Info => "Info",
            NotificationType::Warning => "Warning",
        }
    }
}

#[derive(Clone)]
pub struct NotificationParams {
    pub message: String,
    pub notification_type: NotificationType,
}
impl Default for NotificationParams {
    fn default() -> Self {
        Self {
            message: "".to_string(),
            notification_type: NotificationType::Info,
        }
    }
}

#[component]
pub fn NotificationComponent(params: NotificationParams) -> impl IntoView {
    let (is_visible, set_is_visible) = create_signal(true);
    let message = params.message.clone();

    let notification_css_class =
        "alert w-96 whitespace-normal ".to_owned() + params.notification_type.css_class();

    view! {
        {move || {
            if is_visible() {
                view! {
                    <div class="toast">
                        <div class=notification_css_class.clone()>
                            <div class="w-80 self-center">
                                <div class="mb-2 flex justify-between">
                                    <span class="font-bold text-xl">
                                        {params.notification_type.title()}
                                    </span>
                                    <button
                                        class="close-button"
                                        on:click=move |_| set_is_visible(false)
                                    >
                                        <svg
                                            xmlns="http://www.w3.org/2000/svg"
                                            fill="none"
                                            viewBox="0 0 24 24"
                                            stroke-width="1.5"
                                            stroke="currentColor"
                                            class="w-6 h-6"
                                        >
                                            <path
                                                stroke-linecap="round"
                                                stroke-linejoin="round"
                                                d="M6 18L18 6M6 6l12 12"
                                            ></path>
                                        </svg>
                                    </button>
                                </div>
                                <div class="w-80">
                                    <span class="withespace-normal">{message.clone()}</span>
                                </div>
                            </div>
                        </div>
                    </div>
                }
                    .into_view()
            } else {
                view! { <div></div> }.into_view()
            }
        }}
    }
}
