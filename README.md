<!-- @format -->

# landingPage

Learning to make landing page with Dioxus

Email verification
Password reset flows
Role-based access control
Two-factor authentication
Audit logging

cargo run --bin migrate

# Run all auth tests

DATABASE_URL=postgres://test:test@localhost/test cargo test auth

# Run specific test

DATABASE_URL=postgres://test:test@localhost/test cargo test test_user_creation
