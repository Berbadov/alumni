use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Alumni {
    #[serde(rename = "_id")]
    pub id: String,
    pub full_name: String,
    pub graduation_year: i32,
    pub email: String,
    pub degree: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Default, Deserialize, ToSchema)]
pub struct UpdateUser {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Announcement {
    pub id: u64,
    pub title: String,
    pub body: String,
    pub author: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateAnnouncement {
    pub title: String,
    pub body: String,
    pub author: String,
}

#[derive(Debug, Default, Deserialize, ToSchema)]
pub struct UpdateAnnouncement {
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub body: Option<String>,
    #[serde(default)]
    pub author: Option<String>,
}
