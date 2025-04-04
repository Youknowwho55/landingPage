use dioxus::prelude::{Router as DRouter, *};
use dioxus_router::prelude::*;
use super::{home::Home, callback::Callback, blog::Blog};
use crate::components::navbar::NavLink;
use crate::components::{Navbar, Login, Protected};
use crate::server::AuthContext;
use std::sync::Arc;

/// Defines all application routes and their associated components.
/// 
/// Each route can optionally specify:
/// - A layout component (using #[layout])
/// - Path parameters (like `/blog/:id`)
/// - Query parameters
/// 
/// The `Wrapper` component provides the common layout for all routes.
#[derive(Clone, Routable, Debug, serde::Serialize)]
pub enum Routes {
    /// Root route - displays the home page
    #[layout(Wrapper)]
    #[route("/")]
    Home {},
    
    /// Login page route
    #[route("/login")]
    Login {},
    
    /// Protected content route (requires authentication)
    #[route("/protected")]
    Protected {},
    
    /// OAuth callback route - handles authentication responses
    #[route("/callback")]
    Callback { 

    },
    
    /// Blog post route with dynamic ID parameter
    #[route("/blog/:id")]
    Blog { 
        /// The ID of the blog post to display
        id: i32 
    },
}

/// The main layout wrapper for all routes.
/// 
/// Provides:
/// - Navigation bar at the top
/// - Consistent page structure
/// - Outlet for route-specific content
#[component]
pub fn Wrapper() -> Element {
    rsx! {
        header { class: "absolute inset-x-0 top-0 z-50",
            Navbar {
                nav_items: vec![
                    rsx! {
                        NavLink { to : NavigationTarget::from(Routes::Home {}), text : "Home"
                        .to_string() }
                    },
                    rsx! {
                        NavLink { to : NavigationTarget::from(Routes::Blog { id : 1 }), text : "Blog"
                        .to_string() }
                    },
                    rsx! {
                        NavLink { to : NavigationTarget::from(Routes::Protected {}), text :
                        "Protected".to_string() }
                    },
                ],
            }
        }
        div { class: "relative isolate pt-16", Outlet::<Routes> {} }
    }
}

/// Determines if a route requires authentication.
/// 
/// Returns:
/// - `true` if the route is protected and requires authentication
/// - `false` if the route is publicly accessible
fn is_guarded(current: &Routes) -> bool {
    match current {
        Routes::Protected {} => true,
        Routes::Callback { } => {
            // The callback route might need custom auth logic
            false
        },
        _ => false, // Home, Login, Blog are public
    }
}

/// The main router component that handles navigation and route guarding.
#[component]
pub fn Router() -> Element {
    rsx! {
        DRouter::<Routes> {
            config: || {
                RouterConfig::default()
                    .on_update(|state| {
                        if is_guarded(&state.current()) {
                            on_not_authorized(move |_| {
                                GuardContext::set_next(state.current().clone());
                            });
                        }
                        None
                    })
            },
        }
    }
}

/// Context for handling protected route navigation.
/// 
/// Stores the originally requested route when authentication is required,
/// allowing redirection back after successful login.
#[derive(Default)]
pub struct GuardContext {
    next: Option<Routes>,
}

impl GuardContext {
    /// Sets the route to redirect to after authentication.
    pub fn set_next(next: Routes) {
        let mut guard = use_context::<Signal<GuardContext>>();
        guard.write().next = Some(next);
    }

    /// Redirects to the stored route or falls back to home.
    pub fn redirect_next_or_home() {
        let nav = navigator();
        let mut guard = use_context::<Signal<GuardContext>>();
        let next_maybe = guard.write().next.take();
        
        match next_maybe {
            Some(next) => { let _ = nav.push(next); },
            None => {
                match nav.push(Routes::Home {}) {
                    Some(_) => {},
                    None => log::error!("Navigation failed"),
                }
            },
        }
    }
}

/// Handles unauthorized access attempts.
/// 
/// On web targets, checks user authentication status.
/// On other platforms, immediately executes the callback.
fn on_not_authorized<F>(f: F)
where
    F: Fn(()) + 'static,
{
    #[cfg(target_arch = "wasm32")]
    {
        spawn(async move {
            if get_user().await.is_err() {
                f(());
            }
        });
    }
    
    #[cfg(not(target_arch = "wasm32"))]
    {
        f(());
    }
}