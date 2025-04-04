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

dx serve --platform desktop

# Frontend (WASM)

cargo build --bin client --release --target wasm32-unknown-unknown
trunk build --release

# Backend (Native)

cargo build --bin server --release
