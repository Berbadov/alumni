//! Auth middleware and `AuthenticatedUser` extractor.
//! Resolves `alumni_session` cookies or `Bearer` tokens via `SessionStore` and
//! enforces authentication (`401`) on protected routes while passing public and
//! unmatched routes through to handlers or the 404 guardian.

use std::future::{Future, Ready, ready};
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};

use actix_web::body::EitherBody;
use actix_web::dev::{Payload, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::http::header;
use actix_web::{Error, FromRequest, HttpMessage, HttpRequest, HttpResponse, web};

use crate::AppState;
use crate::error::ApiError;
use crate::models::{ModelError, User};

pub const SESSION_COOKIE_NAME: &str = "alumni_session";

#[derive(Debug, Clone)]
pub struct AuthenticatedUser(pub User);

impl FromRequest for AuthenticatedUser {
    type Error = ApiError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let user = req.extensions().get::<Self>().cloned();
        ready(user.ok_or_else(|| {
            ApiError::from(ModelError::Unauthenticated("authentication required"))
        }))
    }
}

pub fn extract_session_token(req: &HttpRequest) -> Option<String> {
    if let Some(cookie) = req.cookie(SESSION_COOKIE_NAME) {
        let value = cookie.value().trim();
        if !value.is_empty() {
            return Some(value.to_owned());
        }
    }
    if let Some(auth_header) = req.headers().get(header::AUTHORIZATION)
        && let Ok(raw) = auth_header.to_str()
        && let Some(token) = raw.strip_prefix("Bearer ")
    {
        let trimmed = token.trim();
        if !trimmed.is_empty() {
            return Some(trimmed.to_owned());
        }
    }
    None
}

#[must_use]
pub fn is_protected_path(path: &str) -> bool {
    path == "/api/v1/auth/me"
        || path == "/api/v1/admin"
        || path.starts_with("/api/v1/admin/")
        || path == "/api/v1/alumni"
        || path.starts_with("/api/v1/alumni/")
        || path == "/api/v1/announcements"
        || path.starts_with("/api/v1/announcements/")
        || path == "/api/announcements"
        || path.starts_with("/api/announcements/")
        || path == "/api/v1/job-postings"
        || path.starts_with("/api/v1/job-postings/")
        || path == "/announcements"
        || path.starts_with("/announcements/")
}

pub struct AuthMiddleware;

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = AuthMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareService {
            service: Rc::new(service),
        }))
    }
}

pub struct AuthMiddlewareService<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);
        let state = req.app_data::<web::Data<AppState>>().cloned();
        let token = extract_session_token(req.request());
        let protected = is_protected_path(req.path());

        Box::pin(async move {
            let resolved = if let (Some(state), Some(tok)) = (state, token) {
                match state.sessions.resolve_user_id(&tok).await {
                    Ok(user_id) => match state.users.get(user_id).await {
                        Ok(user) if user.is_active => Some(user),
                        _ => None,
                    },
                    Err(_) => None,
                }
            } else {
                None
            };

            if let Some(user) = resolved {
                req.extensions_mut().insert(AuthenticatedUser(user));
                let res = service.call(req).await?;
                return Ok(res.map_into_left_body());
            }

            if protected {
                let response = HttpResponse::Unauthorized()
                    .json(serde_json::json!({ "error": "authentication required" }));
                return Ok(req.into_response(response.map_into_right_body()));
            }

            let res = service.call(req).await?;
            Ok(res.map_into_left_body())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use actix_web::test::{self, TestRequest};
    use actix_web::{App, web};

    use crate::middleware::not_found;

    #[test]
    fn protected_paths_match_resources_only() {
        assert!(is_protected_path("/api/v1/auth/me"));
        assert!(is_protected_path("/api/v1/admin/users"));
        assert!(is_protected_path("/api/v1/admin/users/1/verify"));
        assert!(is_protected_path("/api/v1/alumni"));
        assert!(is_protected_path("/api/v1/alumni/42"));
        assert!(is_protected_path("/api/v1/announcements"));
        assert!(is_protected_path("/api/announcements/1"));
        assert!(is_protected_path("/api/v1/job-postings"));
        assert!(is_protected_path("/announcements/1/edit"));

        assert!(!is_protected_path("/api/health"));
        assert!(!is_protected_path("/api/v1/hello"));
        assert!(!is_protected_path("/api/v1/auth/register"));
        assert!(!is_protected_path("/api/v1/auth/login"));
        assert!(!is_protected_path("/api/v1/auth/logout"));
        assert!(!is_protected_path("/api/nope"));
    }

    #[actix_web::test]
    async fn middleware_blocks_unauthenticated_protected_routes_but_passes_public_and_404() {
        let app = test::init_service(
            App::new()
                .wrap(AuthMiddleware)
                .service(web::resource("/api/health").route(web::get().to(|| async {
                    HttpResponse::Ok().finish()
                })))
                .service(web::resource("/api/v1/alumni").route(web::get().to(|| async {
                    HttpResponse::Ok().finish()
                })))
                .default_service(web::route().to(not_found)),
        )
        .await;

        let health = test::call_service(&app, TestRequest::get().uri("/api/health").to_request()).await;
        assert_eq!(health.status(), StatusCode::OK);

        let alumni = test::call_service(&app, TestRequest::get().uri("/api/v1/alumni").to_request()).await;
        assert_eq!(alumni.status(), StatusCode::UNAUTHORIZED);

        let unknown = test::call_service(&app, TestRequest::get().uri("/api/nope").to_request()).await;
        assert_eq!(unknown.status(), StatusCode::NOT_FOUND);
    }
}
