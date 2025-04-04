# # Start a test PostgreSQL container
# docker run --name test-postgres \
#   -e POSTGRES_USER=test \
#   -e POSTGRES_PASSWORD=test \
#   -e POSTGRES_DB=test \
#   -p 5432:5432 \
#   -d postgres:15-alpine

# # Apply migrations (using sqlx-cli)
# DATABASE_URL=postgres://test:test@localhost/test sqlx migrate run