use dioxus::prelude::*;
use dioxus_router::prelude::*;
use server::AuthProvider;
use views::routes::{GuardContext, Routes};

fn main() {
    #[cfg(feature = "web")]
    dioxus_web::launch(App);
}

#[component]
fn App() -> Element {
    use_context_provider(|| Signal::new(GuardContext::default()));
    
    rsx! {
        head {
            script { src: "https://cdn.jsdelivr.net/npm/@supabase/supabase-js@2" }
            link { rel: "icon", href: "/assets/favicon.ico" }
            link { rel: "stylesheet", href: "/assets/styling/main.css" }
            link { rel: "stylesheet", href: "/assets/tailwind.css" }
        }

        AuthProvider {
            Router::<Routes> { config: || RouterConfig::default() }
        }
    }
}