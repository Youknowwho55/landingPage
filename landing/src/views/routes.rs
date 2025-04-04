use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::{
    components::Navbar,
    views::{home::Home, blog::Blog, not_found::NotFound},
};
use crate::components::auth::login::Login;
use crate::components::protected::Protected;
use crate::server::auth::context::AuthClient;

/// Core application routing configuration
#[derive(Clone, Routable, Debug, serde::Serialize)]
pub enum Routes {
    #[layout(AppLayout)]
    #[route("/")]
    Home {},
    
    #[route("/login")]
    Login {},
    
    #[route("/protected")]
    Protected {},

    #[route("/blog/:id")]
    Blog { id: i32 },
    
    #[route("/404")]
    NotFound {},
}

/// Main application layout with navigation
#[component]
pub fn AppLayout() -> Element {
    rsx! {
        Navbar {
            items: nav_items()
        }
        main { class: "relative isolate pt-16",
            Outlet::<Routes> {}
        }
    }
}

/// Generates navigation items dynamically
fn nav_items() -> Vec<NavItem> {
    vec![
        NavItem {
            to: Routes::Home {},
            text: "Home".into(),
            protected: false,
        },
        NavItem {
            to: Routes::Blog { id: 1 },
            text: "Blog".into(),
            protected: false,

        },
        NavItem {
            to: Routes::Protected {},
            text: "Protected".into(),
            protected: true,
        },
    ]
}

/// Enhanced router with proper error handling
#[component]
pub fn AppRouter() -> Element {
    let auth = use_auth(); // From your auth context

    rsx! {
        Router::<Routes> {
            config: || RouterConfig::default()
                .on_update(move |state| {
                    handle_route_guard(state, &auth)
                })
                .on_error(|err| {
                    log::error!("Routing error: {}", err);
                    Routes::NotFound {}
                })
        }
    }
}

/// Centralized route guard logic
fn handle_route_guard(
    state: &RouterState<Routes>,
    auth: &AuthClient,
) -> Option<NavigationTarget> {
    let current = state.current();
    
    if is_protected_route(¤t) && !auth.is_authenticated() {
        Some(NavigationTarget::from(Routes::Login {}))
    } else {
        None
    }
}

/// Route protection rules
fn is_protected_route(route: &Routes) -> bool {
    matches!(route, Routes::Protected {})
}

/// Navigation item definition
#[derive(Clone)]
pub struct NavItem {
    pub to: Routes,
    pub text: String,
\    pub protected: bool,
}