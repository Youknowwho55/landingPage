use axum::{routing::post, Json, Router};
use sqlx::PgPool;

async fn create_post(
    pool: Extension<PgPool>,
    Json(payload): Json<serde_json::Value>
) -> Result<(), StatusCode> {
    sqlx::query!(
        "INSERT INTO posts (title, body) VALUES ($1, $2)",
        payload["title"].as_str(),
        payload["body"].as_str()
    )
    .execute(&*pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}

#[tokio::main]
async fn main() {
    let pool = db::create_pool().await.unwrap();
    let app = Router::new()
        .route("/api/posts", post(create_post))
        .layer(Extension(pool));

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

use tower_http::cors::CorsLayer;

let app = Router::new()
    // ... routes ...
    .layer(CorsLayer::permissive()); // Allow all origins in dev