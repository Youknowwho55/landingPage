use crate::server::utils::generate_session_token;
use crate::views::routes::Routes;
use dioxus::prelude::*;
use log::{error, info};
use web_sys::window;

#[component]
pub fn Callback() -> Element {
    let nav = use_navigator();

    spawn(async move {
        // 1. Extract and parse hash from URL
        let hash = match window()
            .and_then(|w| w.location().hash().ok())
            .filter(|h| !h.is_empty())
        {
            Some(h) => h,
            None => {
                error!("No hash found in URL");
                nav.replace(Routes::Login {});
                return;
            }
        };

        // 2. Parse URL parameters
        let params: std::collections::HashMap<String, String> = 
            serde_urlencoded::from_str(hash.strip_prefix('#').unwrap_or(&hash))
                .map_err(|e| {
                    error!("Failed to parse URL parameters: {}", e);
                    e
                })
                .unwrap_or_default();

        // 3. Extract tokens
        let (access_token, refresh_token) = match (
            params.get("access_token").cloned(),
            params.get("refresh_token").cloned()
        ) {
            (Some(access), Some(refresh)) => (access, refresh),
            _ => {
                error!("Missing tokens in URL");
                nav.replace(Routes::Login {});
                return;
            }
        };

        // 4. Generate and store session token
        match generate_session_token().await {
            Ok(_) => {
                info!("Authentication successful");
                nav.replace(Routes::Protected {});
            },
            Err(e) => {
                error!("Authentication failed: {}", e);
                nav.replace(Routes::Login {});
            }
        }
    });

    rsx! {
        div { class: "flex justify-center items-center h-screen",
            p { class: "text-lg", "Processing authentication..." }
        }
    }
}