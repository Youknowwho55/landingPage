use sqlx::migrate::Migrator;
use std::path::Path;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")?;
    let pool = sqlx::postgres::PgPool::connect(&database_url).await?;
    
    let migrator = Migrator::new(Path::new("./migrations")).await?;
    migrator.run(&pool).await?;
    
    println!("Migrations completed successfully");
    Ok(())
}