use gloo_net::http::{Request, Response};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Clone, Debug, Deserialize)]
pub struct Alumni {
    #[serde(rename = "_id")]
    pub id: u64,
    pub full_name: String,
    pub email: String,
    pub graduation_year: i32,
    pub degree: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct JobPosting {
    #[serde(rename = "_id")]
    pub id: u64,
    pub title: String,
    pub company: String,
    pub description: String,
    pub location: String,
    pub employment_type: String,
    pub apply_url: String,
    pub author_id: u64,
    pub is_active: bool,
    pub created_at: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct JobApplication {
    #[serde(rename = "_id")]
    pub id: u64,
    pub job_posting_id: u64,
    pub alumni_id: u64,
    pub cover_letter: String,
    pub status: String,
    pub updated_at: String,
}

#[derive(Serialize)]
struct CreateApplication<'a> {
    job_posting_id: u64,
    alumni_id: u64,
    cover_letter: &'a str,
}

pub const APPLICATION_STATUSES: [&str; 4] = ["submitted", "reviewed", "accepted", "rejected"];

fn api_error(status: u16, body: &str) -> String {
    let message = serde_json::from_str::<serde_json::Value>(body)
        .ok()
        .and_then(|value| {
            value
                .get("error")
                .and_then(|error| error.as_str())
                .map(str::to_owned)
        })
        .unwrap_or_else(|| body.trim().to_owned());
    if message.is_empty() {
        format!("Request failed ({status})")
    } else {
        message
    }
}

async fn read_json<T: DeserializeOwned>(
    response: Result<Response, gloo_net::Error>,
) -> Result<T, String> {
    let response = response.map_err(|err| format!("Network error: {err}"))?;
    let status = response.status();
    let body = response
        .text()
        .await
        .map_err(|err| format!("Could not read the response: {err}"))?;
    if !(200..300).contains(&status) {
        return Err(api_error(status, &body));
    }
    serde_json::from_str(&body).map_err(|err| format!("Invalid response: {err}"))
}

async fn get<T: DeserializeOwned>(path: &str) -> Result<T, String> {
    read_json(Request::get(path).send().await).await
}

pub async fn fetch_alumni() -> Result<Vec<Alumni>, String> {
    get("/api/v1/alumni").await
}

pub async fn fetch_job_postings() -> Result<Vec<JobPosting>, String> {
    get("/api/v1/job-postings").await
}

pub async fn fetch_job_posting(id: u64) -> Result<JobPosting, String> {
    get(&format!("/api/v1/job-postings/{id}")).await
}

pub async fn fetch_applications(
    job_posting_id: Option<u64>,
    alumni_id: Option<u64>,
    status: Option<&str>,
) -> Result<Vec<JobApplication>, String> {
    let mut query = Vec::new();
    if let Some(id) = job_posting_id {
        query.push(format!("job_posting_id={id}"));
    }
    if let Some(id) = alumni_id {
        query.push(format!("alumni_id={id}"));
    }
    if let Some(status) = status {
        query.push(format!("status={status}"));
    }
    let path = if query.is_empty() {
        "/api/v1/job-applications".to_owned()
    } else {
        format!("/api/v1/job-applications?{}", query.join("&"))
    };
    get(&path).await
}

pub async fn create_application(
    job_posting_id: u64,
    alumni_id: u64,
    cover_letter: &str,
) -> Result<JobApplication, String> {
    let body = CreateApplication {
        job_posting_id,
        alumni_id,
        cover_letter,
    };
    let request = Request::post("/api/v1/job-applications")
        .json(&body)
        .map_err(|err| format!("Could not build the request: {err}"))?;
    read_json(request.send().await).await
}

pub async fn update_application_status(id: u64, status: &str) -> Result<JobApplication, String> {
    let request = Request::patch(&format!("/api/v1/job-applications/{id}"))
        .json(&json!({ "status": status }))
        .map_err(|err| format!("Could not build the request: {err}"))?;
    read_json(request.send().await).await
}
