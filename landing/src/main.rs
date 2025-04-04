use dioxus::prelude::*;

use server::AuthProvider;
// use components::Navbar;
use views::{Blog, Home};
mod components;
mod views;
use sqlx::migrate::Migrator;
mod db;
use std::path::Path;
mod server;
use anyhow::Result;

// use client::auth::AuthorizedClient;
use crate::views::routes::{GuardContext, Router as AppRouter};


#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    // #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },

}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");



#[tokio::main]

 async fn main() -> anyhow::Result<()> {

    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")?;
    
    // Create connection pool
    let pool = db::create_pool(&database_url).await?;

    // Run migrations
    let migrator = Migrator::new(Path::new("./migrations")).await?;
    migrator.run(&pool).await?;
    dioxus::logger::initialize_default();

    server_only!({
        dotenv::dotenv().ok();
        info!("loaded env variables");
    });
    dioxus::launch(App);
    Ok(())

}


#[component]
fn App() -> Element {
    let _guard_context = use_context_provider(|| Signal::new(GuardContext::default()));

    // Build cool things 
    rsx! {
        head {
            script { src: "https://cdn.jsdelivr.net/npm/@supabase/supabase-js@2" }
        }
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        // WILL NEED TO SEE WHAT ROUTER TO USE
        AuthProvider::new(AppRouter {})
    }
}



