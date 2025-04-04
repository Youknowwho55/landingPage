// components/auth.rs
use dioxus::prelude::*;
use crate::views::routes::Routes;
use crate::server::use_auth;
use dioxus::prelude::use_navigator;  // If you're using the main prelude


#[component]
pub fn Logout() -> Element 
{
    let auth = use_auth();
    let nav = use_navigator();
    let on_logout = move |_| {
        let auth = auth.clone();
        let nav = nav.clone();

        spawn(async move {
            match auth.logout().await {
                Ok(_) => {
                    log::info!("Logged out successfully");
                    nav.replace(Routes::Home {});
                },
                Err(e) => {
                    log::error!("Logout failed: {}", e);
                    // Optionally show error to user
                }
            }
        });
    };

    rsx! {
        button { onclick: on_logout, "Logout" }
    }
}

// components/auth.rs