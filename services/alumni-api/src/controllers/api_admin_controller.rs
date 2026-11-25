//! `ApiAdminController`: admin user listing, paper verification, role updates,
//! and session revocation under `/api/v1/admin`.
//! Routes are defined in `routes::api_admin`.

use actix_web::{HttpResponse, web};

use crate::AppState;
use crate::auth_middleware::AuthenticatedUser;
use crate::error::ApiError;
use crate::models::{AdminUpdateUserRequest, User, VerifyUserRequest};

/// Lists all registered users, ordered by id.
#[utoipa::path(
    get,
    path = "/api/v1/admin/users",
    tag = "ApiAdminController",
    responses(
        (status = 200, description = "All user accounts", body = [User]),
        (status = 401, description = "Authentication required"),
        (status = 403, description = "Admin role required"),
    ),
)]
pub async fn list_users(
    state: web::Data<AppState>,
    AuthenticatedUser(actor): AuthenticatedUser,
) -> Result<web::Json<Vec<User>>, ApiError> {
    actor.can_manage_users()?;
    Ok(web::Json(state.users.list().await?))
}

/// Approves or rejects a user's government-paper verification.
#[utoipa::path(
    post,
    path = "/api/v1/admin/users/{id}/verify",
    tag = "ApiAdminController",
    params(("id" = u64, Path, description = "User id")),
    request_body = VerifyUserRequest,
    responses(
        (status = 200, description = "Verification updated", body = User),
        (status = 400, description = "Invalid verification request"),
        (status = 401, description = "Authentication required"),
        (status = 403, description = "Admin role required"),
        (status = 404, description = "No user with that id"),
    ),
)]
pub async fn verify_user(
    state: web::Data<AppState>,
    AuthenticatedUser(actor): AuthenticatedUser,
    path: web::Path<u64>,
    body: web::Json<VerifyUserRequest>,
) -> Result<web::Json<User>, ApiError> {
    actor.can_manage_users()?;
    let target_id = path.into_inner();
    let updated = state.users.verify_user(actor.id, target_id, &body).await?;
    state.sessions.revoke_for_user(target_id).await?;
    Ok(web::Json(updated))
}

/// Updates a user's assigned roles and/or active state.
#[utoipa::path(
    patch,
    path = "/api/v1/admin/users/{id}",
    tag = "ApiAdminController",
    params(("id" = u64, Path, description = "User id")),
    request_body = AdminUpdateUserRequest,
    responses(
        (status = 200, description = "User updated", body = User),
        (status = 400, description = "Invalid roles or self-lockout blocked"),
        (status = 401, description = "Authentication required"),
        (status = 403, description = "Admin role required"),
        (status = 404, description = "No user with that id"),
    ),
)]
pub async fn update_user(
    state: web::Data<AppState>,
    AuthenticatedUser(actor): AuthenticatedUser,
    path: web::Path<u64>,
    body: web::Json<AdminUpdateUserRequest>,
) -> Result<web::Json<User>, ApiError> {
    actor.can_manage_users()?;
    let target_id = path.into_inner();
    let (updated, revoke_sessions) = state
        .users
        .admin_update(actor.id, target_id, &body)
        .await?;
    if revoke_sessions {
        state.sessions.revoke_for_user(target_id).await?;
    }
    Ok(web::Json(updated))
}

/// Revokes all active sessions for one user.
#[utoipa::path(
    post,
    path = "/api/v1/admin/users/{id}/revoke-sessions",
    tag = "ApiAdminController",
    params(("id" = u64, Path, description = "User id")),
    responses(
        (status = 204, description = "Active sessions revoked"),
        (status = 401, description = "Authentication required"),
        (status = 403, description = "Admin role required"),
        (status = 404, description = "No user with that id"),
    ),
)]
pub async fn revoke_sessions(
    state: web::Data<AppState>,
    AuthenticatedUser(actor): AuthenticatedUser,
    path: web::Path<u64>,
) -> Result<HttpResponse, ApiError> {
    actor.can_manage_users()?;
    let target_id = path.into_inner();
    let _ = state.users.get(target_id).await?;
    state.sessions.revoke_for_user(target_id).await?;
    Ok(HttpResponse::NoContent().finish())
}
