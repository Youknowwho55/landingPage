use your_crate::db::{self, queries::UserQueries};
use sqlx::PgPool;
use fake::{Fake, faker::internet::en::SafeEmail};

#[sqlx::test]
async fn test_user_creation(pool: PgPool) -> anyhow::Result<()> {
    // Arrange
    let email: String = SafeEmail().fake();
    let password_hash = "$2b$12$..."; // Fake bcrypt hash
    
    // Act
    let user = UserQueries::create(&pool, &email, password_hash).await?;
    
    // Assert
    assert_eq!(user.email, email);
    assert!(!user.password_hash.is_empty());
    
    // Verify in database
    let fetched = UserQueries::get_by_email(&pool, &email).await?;
    assert!(fetched.is_some());
    
    Ok(())
}

#[sqlx::test]
async fn test_duplicate_email(pool: PgPool) -> anyhow::Result<()> {
    let email = "test@example.com";
    let password = "hash";
    
    // First creation should succeed
    UserQueries::create(&pool, email, password).await?;
    
    // Second attempt should fail
    let result = UserQueries::create(&pool, email, password).await;
    assert!(result.is_err());
    
    Ok(())
}