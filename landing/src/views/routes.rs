use dioxus::prelude::{Router as DRouter, *};
use dioxus_router::prelude::*;

// login::Login
use super::{home::Home, callback::Callback, blog::Blog};
use crate::components::navbar::NavLink;

use crate::components::{Navbar, Login, Protected};
use crate::server::AuthContext;
use std::sync::Arc;

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize,)]
pub enum Routes {
    #[layout(Wrapper)]
    #[route("/")]
    Home {},
    #[route("/login")]
    Login {},
    #[route("/protected")]
    Protected {},
    #[route("/callback")]
    Callback { auth_context: Arc<AuthContext> },
    #[route("/blog/:id")]
    Blog { id: i32 },
}

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

/// Register the protected state of routes here
fn is_guarded(current: &Routes) -> bool {
    // guard routes
    match current {
        Routes::Home {} => false,
        Routes::Login {} => false,
        Routes::Blog { id: _ } => false,
        Routes::Protected {} => true,
        Routes::Callback { auth_context } => {
            // Add logic if needed to determine if this route is guarded
            false
        },
    }
}

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

#[derive(Default)]
pub struct GuardContext {
    next: Option<Routes>,
}

impl GuardContext {
    pub fn set_next(next: Routes) {
        let mut guard = use_context::<Signal<GuardContext>>();
        guard.write().next = Some(next);
    }

    pub fn redirect_next_or_home() {
        let nav = navigator();
        let guard = use_context::<Signal<GuardContext>>();
        let next_maybe = guard.write().next.take();
        if let Some(next) = next_maybe {
            nav.push(next);
        } else {
            nav.push(Routes::Home {});
        }
    }
}

fn on_not_authorized<F>(f: F)
where
    F: Fn(()) + 'static,
{
    #[cfg(target_arch = "wasm32")]
    {
        spawn(async move {
            let user = get_user().await;
            if user.is_err() {
                f(());
            }
        });
    }
    
    #[cfg(not(target_arch = "wasm32"))]
    {
        f(());
    }
}

