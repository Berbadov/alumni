//! `ApiAuthController`: JSON registration, login, logout, and session check under `/api/v1/auth`.
//! Routes are defined in `routes::api_auth`.

use actix_web::cookie::{Cookie, SameSite, time::Duration as CookieDuration};
use actix_web::{HttpRequest, HttpResponse, web};

use crate::AppState;
use crate::auth_middleware::{AuthenticatedUser, SESSION_COOKIE_NAME, extract_session_token};
use crate::error::ApiError;
use crate::models::{AuthSession, LoginRequest, RegisterUser, UpdateOwnProfileRequest, User};

const SESSION_MAX_AGE_SECS: i64 = 7 * 24 * 60 * 60;

fn build_session_cookie(value: &str, max_age_secs: i64) -> Cookie<'static> {
    Cookie::build(SESSION_COOKIE_NAME, value.to_owned())
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .max_age(CookieDuration::seconds(max_age_secs))
        .finish()
}

/// Registers a new user account from the validated request body.
#[utoipa::path(
    post,
    path = "/api/v1/auth/register",
    tag = "ApiAuthController",
    request_body = RegisterUser,
    responses(
        (status = 201, description = "User registered as unverified", body = User),
        (status = 400, description = "Invalid fields or duplicate email"),
        (status = 403, description = "Role cannot self-register"),
    ),
)]
pub async fn register(
    state: web::Data<AppState>,
    body: web::Json<RegisterUser>,
) -> Result<HttpResponse, ApiError> {
    let user = state.users.register(&body).await?;
    Ok(HttpResponse::Created().json(user))
}

/// Authenticates a user and issues an `HttpOnly` session cookie.
#[utoipa::path(
    post,
    path = "/api/v1/auth/login",
    tag = "ApiAuthController",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Session created", body = AuthSession),
        (status = 400, description = "Invalid email or empty password"),
        (status = 401, description = "Wrong credentials or disabled account"),
    ),
)]
pub async fn login(
    state: web::Data<AppState>,
    body: web::Json<LoginRequest>,
) -> Result<HttpResponse, ApiError> {
    let user = state.users.authenticate(&body.email, &body.password).await?;
    let (raw_token, expires_at) = state.sessions.create(user.id).await?;
    let cookie = build_session_cookie(&raw_token, SESSION_MAX_AGE_SECS);
    Ok(HttpResponse::Ok()
        .cookie(cookie)
        .json(AuthSession { user, expires_at }))
}

/// Revokes the active session and clears the session cookie.
#[utoipa::path(
    post,
    path = "/api/v1/auth/logout",
    tag = "ApiAuthController",
    responses((status = 204, description = "Logged out")),
)]
pub async fn logout(state: web::Data<AppState>, req: HttpRequest) -> Result<HttpResponse, ApiError> {
    if let Some(token) = extract_session_token(&req) {
        state.sessions.revoke(&token).await?;
    }
    let cleared = build_session_cookie("", 0);
    Ok(HttpResponse::NoContent().cookie(cleared).finish())
}

/// Returns the currently authenticated user for the active session cookie.
#[utoipa::path(
    get,
    path = "/api/v1/auth/me",
    tag = "ApiAuthController",
    responses(
        (status = 200, description = "Current authenticated user", body = User),
        (status = 401, description = "Missing, expired, or revoked session"),
    ),
)]
pub async fn me(AuthenticatedUser(user): AuthenticatedUser) -> Result<web::Json<User>, ApiError> {
    Ok(web::Json(user))
}

/// Updates the signed-in user's own profile fields or password.
#[utoipa::path(
    patch,
    path = "/api/v1/auth/me",
    tag = "ApiAuthController",
    request_body = UpdateOwnProfileRequest,
    responses(
        (status = 200, description = "Updated user profile", body = User),
        (status = 400, description = "Invalid profile field or missing current password"),
        (status = 401, description = "Missing session or incorrect current password"),
    ),
)]
pub async fn update_me(
    state: web::Data<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
    body: web::Json<UpdateOwnProfileRequest>,
) -> Result<web::Json<User>, ApiError> {
    let updated = state.users.update_own_profile(user.id, &body).await?;
    Ok(web::Json(updated))
}
