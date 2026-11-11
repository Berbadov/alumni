//! Middleware: the request logger prints one line per request on the terminal
//! and appends error responses to the error log file. The 404 guardian answers
//! unmatched routes with status 404 and a reason.

use std::fs::OpenOptions;
use std::future::{Future, Ready, ready};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::http::StatusCode;
use actix_web::{Error, HttpRequest, HttpResponse};

use crate::views;

pub const DEFAULT_ERROR_LOG: &str = "logs/error.log";

pub struct RequestLogger {
    error_log: PathBuf,
}

impl RequestLogger {
    pub fn new(error_log: PathBuf) -> Self {
        Self { error_log }
    }
}

impl<S, B> Transform<S, ServiceRequest> for RequestLogger
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = RequestLoggerService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RequestLoggerService {
            service,
            error_log: self.error_log.clone(),
        }))
    }
}

pub struct RequestLoggerService<S> {
    service: S,
    error_log: PathBuf,
}

impl<S, B> Service<ServiceRequest> for RequestLoggerService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let start = Instant::now();
        let method = req.method().clone();
        let path = req.path().to_owned();
        let error_log = self.error_log.clone();
        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            let status = res.status();
            let line = format_log_line(&now_hms(), method.as_str(), &path, status, start.elapsed());
            println!("{line}");
            if status.as_u16() >= 400 {
                append_error_log(&error_log, &line);
            }
            Ok(res)
        })
    }
}

fn now_hms() -> String {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_secs());
    let tod = secs % 86_400;
    format!("{:02}:{:02}:{:02}", tod / 3_600, (tod % 3_600) / 60, tod % 60)
}

fn format_log_line(
    hms: &str,
    method: &str,
    path: &str,
    status: StatusCode,
    elapsed: Duration,
) -> String {
    format!(
        "{hms} {method} {path} {} {}ms",
        status.as_u16(),
        elapsed.as_millis()
    )
}

fn append_error_log(path: &Path, line: &str) {
    let write = || -> std::io::Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let mut file = OpenOptions::new().create(true).append(true).open(path)?;
        writeln!(file, "{line}")
    };
    let _ = write();
}

/// 404 guardian for unmatched routes: JSON with a reason under `/api`,
/// the generic HTML 404 page for every other path.
pub async fn not_found(req: HttpRequest) -> HttpResponse {
    let reason = format!("no route matches {} {}", req.method(), req.path());
    if req.path().starts_with("/api") {
        HttpResponse::NotFound().json(serde_json::json!({ "error": reason }))
    } else {
        HttpResponse::NotFound()
            .content_type("text/html; charset=utf-8")
            .body(views::error::not_found(req.path()).into_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test::{self, TestRequest};
    use actix_web::{App, web};

    #[test]
    fn log_line_matches_the_whiteboard_format() {
        let line = format_log_line(
            "10:15:30",
            "POST",
            "/api/job",
            StatusCode::CREATED,
            Duration::from_millis(18),
        );
        assert_eq!(line, "10:15:30 POST /api/job 201 18ms");
    }

    #[actix_web::test]
    async fn guardian_returns_json_404_with_reason_for_api_paths() {
        let app = test::init_service(
            App::new()
                .service(web::resource("/api/health").route(web::get().to(|| async {
                    HttpResponse::Ok().finish()
                })))
                .default_service(web::route().to(not_found)),
        )
        .await;

        let req = TestRequest::get().uri("/api/nope").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
        let body: serde_json::Value = test::read_body_json(resp).await;
        assert_eq!(body["error"], "no route matches GET /api/nope");
    }

    #[actix_web::test]
    async fn guardian_returns_html_404_page_for_other_paths() {
        let app = test::init_service(App::new().default_service(web::route().to(not_found))).await;

        let req = TestRequest::post().uri("/no-such-page").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
        let content_type = resp
            .headers()
            .get("content-type")
            .unwrap()
            .to_str()
            .unwrap();
        assert!(content_type.starts_with("text/html"));
        let body = String::from_utf8(test::read_body(resp).await.to_vec()).unwrap();
        assert!(body.contains("404"));
        assert!(body.contains("/no-such-page"));
    }

    #[actix_web::test]
    async fn logger_passes_requests_through_and_appends_errors_to_the_log_file() {
        let dir = std::env::temp_dir().join(format!("alumni_log_test_{}", std::process::id()));
        let log = dir.join("error.log");
        let app = test::init_service(
            App::new()
                .wrap(RequestLogger::new(log.clone()))
                .service(web::resource("/ok").route(web::get().to(|| async {
                    HttpResponse::Ok().finish()
                })))
                .default_service(web::route().to(not_found)),
        )
        .await;

        let resp = test::call_service(&app, TestRequest::get().uri("/ok").to_request()).await;
        assert_eq!(resp.status(), StatusCode::OK);
        assert!(!log.exists());

        let resp = test::call_service(&app, TestRequest::get().uri("/missing").to_request()).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
        let contents = std::fs::read_to_string(&log).unwrap();
        assert!(contents.contains("GET /missing 404"), "{contents}");
        let _ = std::fs::remove_dir_all(&dir);
    }
}
