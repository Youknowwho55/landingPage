// components/auth.rs
use dioxus::prelude::*;
use crate::server::{Credentials, AuthError};
use crate::views::routes::{GuardContext, Routes};
use crate::components::ui::Button;

#[component]
pub fn Protected() -> Element {
    protected(Routes::Login {}, Routes::Protected {});

    let logout = move |_| {
        spawn(async move {
            client! {
              let _ = logout().await;
                GuardContext::redirect_next_or_home();
            }
        });
    };

    rsx! {
        div { class: "max-w-lg mx-auto py-2",
            h1 { class: "text-3xl", "Protected SupaDioxus" }
            Button { text: "Signout", on_click: logout }
        }
    }
}