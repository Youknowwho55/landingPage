use crate::server::context::AuthContext;
use crate::views::routes::Routes::Protected;
use dioxus::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;
use web_sys::window;
use log::{info, error};

#[component]
pub fn Callback(auth_context: Arc<AuthContext>) -> Element {
    spawn({
        let auth_context = auth_context.clone();
        async move {
            client! {
                let hash = window().unwrap().location().hash().unwrap();
                let params_parsed: HashMap<String, String> = serde_urlencoded::from_str(
                    hash.strip_prefix("#").unwrap()
                ).unwrap();
                
                if let (Some(access_token), Some(refresh_token)) = (
                    params_parsed.get("access_token"),
                    params_parsed.get("refresh_token")
                ) {
                    // Store tokens in local storage for persistence
                    let window = window().unwrap();
                    let storage = window.local_storage().unwrap().unwrap();
                    
                    storage.set_item("access_token", access_token).unwrap();
                    storage.set_item("refresh_token", refresh_token).unwrap();
                    
                    // Validate the token with our auth system
                    if let Ok(user) = auth_context.validate_session(access_token).await {
                        log::info!("Successfully authenticated user: {}", user.email);
                        let nav = navigator();
                        nav.replace(Protected {});
                    } else {
                        log::error!("Failed to validate tokens");
                        // Optionally redirect to login page on failure
                    }
                }
            }
        }
    });
    
    rsx! {
        div { class: "flex justify-center items-center h-screen",
            p { class: "text-lg", "Processing authentication..." }
        }
    }
}