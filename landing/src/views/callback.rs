use crate::server::utils::generate_session_token;
use crate::views::routes::Routes;
use dioxus::prelude::*;
use log::{error, info};
use web_sys::window;

#[component]
pub fn Callback() -> Element {
    let nav = use_navigator();

    spawn(async move {
        // 1. Extract hash from URL
        let hash = match window()
            .and_then(|w| w.location().hash().ok())
            .filter(|h| !h.is_empty())
        {
            Some(h) => h,
            None => {
                error!("No hash found in URL");
                return;
            }
        };

        // 2. Parse URL parameters
        let params: std::collections::HashMap<String, String> = match serde_urlencoded::from_str::<Vec<(String, String)>>(
            hash.strip_prefix('#').unwrap_or(&hash)
        ) {
            Ok(p) => p.into_iter().collect(),
            Err(e) => {
                error!("Failed to parse URL parameters: {}", e);
                return;
            }
        };

        // 3. Extract tokens
        let (access_token, refresh_token) = match (
            params.get("access_token"),
            params.get("refresh_token")
        ) {
            (Some(access), Some(refresh)) => (access.to_owned(), refresh.to_owned()),
            _ => {
                error!("Missing access_token or refresh_token");
                return;
            }
        };

        // 4. Generate and store session token
        match generate_session_token(access_token, refresh_token).await {
            Ok(_) => {
                info!("Session tokens generated successfully");
                nav.replace(Routes::Protected {});
            },
            Err(e) => {
                error!("Failed to generate session token: {}", e);
                // Consider redirecting to login page here
            }
        }
    });

    rsx! {
        div { class: "flex justify-center items-center h-screen",
            p { class: "text-lg", "Processing authentication..." }
        }
    }
}