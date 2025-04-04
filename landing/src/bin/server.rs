use anyhow::Result;
use axum::{Extension, Json, Router, routing::post, http::StatusCode};
use sqlx::{PgPool, migrate::Migrator};
use std::path::Path;
use tower_http::cors::CorsLayer;

mod db;
mod server;

#[tokio::main]
async fn main() -> Result<()> {
    // 1. Load environment variables
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")?;
    
    // 2. Set up database
    let pool = db::create_pool(&database_url).await?;
    let migrator = Migrator::new(Path::new("./migrations")).await?;
    migrator.run(&pool).await?;

    // 3. Configure routes
    let app = Router::new()
        .route("/api/posts", post(create_post))
        .layer(Extension(pool))
        .layer(CorsLayer::permissive()); // Enable CORS for development

    // 4. Start server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    println!("Server running on http://localhost:8080");
    axum::serve(listener, app).await?;

    Ok(())
}

// Handler for creating posts
async fn create_post(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<serde_json::Value>
) -> Result<(), StatusCode> {
    sqlx::query!(
        "INSERT INTO posts (title, body) VALUES ($1, $2)",
        payload["title"].as_str().unwrap_or_default(),
        payload["body"].as_str().unwrap_or_default()
    )
    .execute(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}