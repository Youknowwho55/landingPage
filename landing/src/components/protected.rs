// components/auth.rs
use dioxus::prelude::*;
use crate::server::{Credentials, AuthError};
use crate::views::routes::{GuardContext, Routes};
use crate::components::ui::Button;
use crate::auth::context::AuthContext;
use crate::server::auth::use_auth;
use std::rc::Rc;

#[component]
pub fn Protected() -> Element {
    protected(Routes::Login {}, Routes::Protected {});
    let auth = use_auth();
    let nav = use_navigator();

    let auth = Rc::new(auth);
    let on_logout = {
        let auth = Rc::clone(&auth);
        move |_| {
            let auth = Rc::clone(&auth);
            spawn(async move {
                match auth.logout().await {
                    Ok(_) => {
                        log::info!("Logged out successfully");
                        GuardContext::redirect_next_or_home();
                    },
                    Err(e) => {
                        log::error!("Logout failed: {}", e);
                    }
                }
            });
        }
    };

    rsx! {
        div { class: "max-w-lg mx-auto py-2",
            h1 { class: "text-3xl", "Protected SupaDioxus" }
            Button { text: "Signout", on_click: on_logout }
        }
    }
}

/// Declare a page view protected
///
/// Automatically redirect users to login and back to the page on auth success
pub fn protected(redirect: Routes, next: Routes) {
    #[cfg(target_arch = "wasm32")]
    {
        spawn(async move {
            let user = get_user().await;
            if user.is_err() {
                GuardContext::set_next(next);
                let nav = navigator();
                nav.replace(redirect);
            }
        });
    }
    
    #[cfg(not(target_arch = "wasm32"))]
    {
        // Do nothing or implement alternative behavior for non-WASM targets
    }
}