use std::time::Duration;

use leptos::*;

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

pub struct NotificationParams {
    pub message: &'static str,
    pub notification_type: NotificationType,
}

#[component]
pub fn NotificationComponent(cx: Scope, params: NotificationParams) -> impl IntoView {
    let (is_visible, set_is_visible) = create_signal(cx, true);

    view! {cx,
        { move || if is_visible() {
            view!{cx,<div class="toast">
                <div class="alert max-w-md whitespace-normal" class={params.notification_type.css_class()}>
                    <div>
                        <div class="mb-2 flex justify-between">
                            <span class="font-bold text-xl">{params.notification_type.title()}</span>
                            <button
                                class="close-button"
                                on:click=move |_| set_is_visible(false)
                            >
                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
                                </svg>
                            </button>
                        </div>
                        <div><span class="withespace-normal">{params.message}</span></div>
                    </div>
                </div>
            </div>}.into_view(cx)
        } else {
            view! {cx, <div></div>}.into_view(cx)
        }}
    }
}
