use anyhow::Result;
use sqlx::migrate::Migrator;
use std::path::Path;
mod db;
mod server;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")?;
    
    let pool = db::create_pool(&database_url).await?;
    let migrator = Migrator::new(Path::new("./migrations")).await?;
    migrator.run(&pool).await?;

    server::start().await
}