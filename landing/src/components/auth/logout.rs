// components/auth/logout.rs
use dioxus::prelude::*;
use crate::{views::routes::Routes, server::use_auth};

#[component]
pub fn Logout(
    /// Custom button text
    #[props(default = "Logout".into())] 
    label: String,
    /// Additional CSS classes
    #[props(default = "".into())]
    class: String,
) -> Element {
    let auth = use_auth();
    let nav = use_navigator();
    
    let on_logout = async_handler!( [auth, nav], move |_| {
        match auth.logout().await {
            Ok(_) => {
                log::info!("Logged out successfully");
                nav.replace(Routes::Home {});
            },
            Err(e) => {
                log::error!("Logout failed: {}", e);
                // Handle error state if needed
            }
        }
    });

    rsx! {
        button { class: "{class}", onclick: on_logout, {label} }
    }
}