mod user_tests;

pub fn test_db_url() -> String {
    std::env::var("DATABASE_URL")
        .unwrap_or("postgres://test:test@localhost/test".into())
}