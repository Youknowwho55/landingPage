use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
    Extension,
};
use crate::{
    server::auth::{AuthContext, AuthError},
    db::models::UserSession,
};

use crate::utils:session_cache;
/// Authentication middleware for Axum routes
///
/// # Flow
/// 1. Extracts token from headers
/// 2. Validates session
/// 3. Attaches user to request
pub async fn auth_middleware<B>(
    Extension(auth): Extension<Arc<AuthContext>>,
    mut request: Request<B>,
    next: Next<B>,
) -> Result<Response, AuthError> {
    // 1. Extract token from Authorization header
    let token = extract_bearer_token(&request)
        .ok_or(AuthError::InvalidToken)?;

    // 2. Validate session
    let session = validate_session(&auth, &token).await?;

    // 3. Attach user to request extensions
    request.extensions_mut().insert(session.user);

    Ok(next.run(request).await)
}

/// Extracts Bearer token from Authorization header
fn extract_bearer_token<B>(request: &Request<B>) -> Option<String> {
    request.headers()
        .get("Authorization")?
        .to_str()
        .ok()?
        .strip_prefix("Bearer ")
        .map(|s| s.to_string())
}

/// Validates session against database
async fn validate_session(
    auth: &AuthContext,
    token: &str
) -> Result<UserSession, AuthError> {
    // Check cache first
    if let Some(session) = auth.session_cache.get(token).await {
        return Ok(session);
    }

    // Fallback to database
    let session = auth.db.get_session(token).await?;
    
    // Validate expiration
    if session.expires_at < Utc::now() {
        return Err(AuthError::ExpiredToken);
    }

    // Cache valid session
    auth.session_cache.set(token, &session).await;

    Ok(session)
}

/// Role-based access control middleware
pub async fn require_role<B>(
    request: Request<B>,
    next: Next<B>,
    required_role: &str,
) -> Result<Response, AuthError> {
    let user = request.extensions()
        .get::<User>()
        .ok_or(AuthError::Unauthorized)?;

    if !user.roles.contains(required_role) {
        return Err(AuthError::Forbidden);
    }

    Ok(next.run(request).await)
}

/// Rate limiting middleware
pub async fn rate_limit<B>(
    Extension(limiter): Extension<Arc<RateLimiter>>,
    request: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let ip = request.extensions()
        .get::<IpAddr>()
        .ok_or(StatusCode::BAD_REQUEST)?;

    limiter.check(ip)
        .map_err(|_| StatusCode::TOO_MANY_REQUESTS)?;

    Ok(next.run(request).await)
}