use dioxus::prelude::*;
use dioxus_router::prelude::*;
use server::AuthProvider;
use views::{Blog, Home};
mod components;
mod views;
use sqlx::migrate::Migrator;
mod db;
use std::path::Path;
mod server;
use anyhow::Result;
use crate::views::routes::{GuardContext, Router as AppRouter, Routes};

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
        info!("loaded env variables");
    });
    
    dioxus::launch(App);
    Ok(())
}

#[component]
fn App() -> Element {
    // Provide guard context for route protection
    use_context_provider(|| Signal::new(GuardContext::default()));
    
    // Provide auth context for the entire app
    rsx! {
        head {
            script { src: "https://cdn.jsdelivr.net/npm/@supabase/supabase-js@2" }
            link { rel: "icon", href: "/assets/favicon.ico" }
            link { rel: "stylesheet", href: "/assets/styling/main.css" }
            link { rel: "stylesheet", href: "/assets/tailwind.css" }
        }

        Router::<Routes> { config: || RouterConfig::default().on_update(|state| { None }) }
    }
}