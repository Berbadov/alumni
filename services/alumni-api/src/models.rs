use serde::{Deserialize, Serialize};

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
