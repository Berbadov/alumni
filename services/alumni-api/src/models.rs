use mongodb::bson::{Bson, Document};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub const MIN_GRADUATION_YEAR: i32 = 1900;
pub const MAX_GRADUATION_YEAR: i32 = 2100;

#[derive(Debug, Clone)]
pub enum ModelError {
    Invalid(&'static str),
    NotFound,
    Duplicate,
    Blocked(&'static str),
    Internal(String),
}

impl ModelError {
    pub fn internal(err: impl std::fmt::Display) -> Self {
        Self::Internal(err.to_string())
    }
}

fn corrupt(what: &str) -> ModelError {
    ModelError::Internal(format!("corrupt {what} document"))
}

fn read_int(doc: &Document, key: &str, what: &str) -> Result<i64, ModelError> {
    match doc.get(key) {
        Some(Bson::Int32(value)) => Ok(i64::from(*value)),
        Some(Bson::Int64(value)) => Ok(*value),
        Some(Bson::Double(value))
            if value.is_finite()
                && value.fract() == 0.0
                && *value >= -9.223_372_036_854_776e18
                && *value < 9.223_372_036_854_776e18 =>
        {
            #[allow(clippy::cast_possible_truncation)]
            Ok(*value as i64)
        }
        _ => Err(corrupt(what)),
    }
}

fn read_u64(doc: &Document, key: &str, what: &str) -> Result<u64, ModelError> {
    let raw = read_int(doc, key, what)?;
    u64::try_from(raw).map_err(|_| corrupt(what))
}

fn read_string(doc: &Document, key: &str, what: &str) -> Result<String, ModelError> {
    doc.get_str(key).map_err(|_| corrupt(what)).map(str::to_owned)
}

fn read_time(doc: &Document, key: &str, what: &str) -> Result<String, ModelError> {
    doc.get_datetime(key)
        .map_err(|_| corrupt(what))?
        .try_to_rfc3339_string()
        .map_err(|_| corrupt(what))
}

fn read_password(doc: &Document) -> Option<String> {
    match doc.get("password_hash") {
        Some(Bson::String(hash)) => Some(hash.clone()),
        _ => None,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Alumni {
    #[serde(rename = "_id")]
    pub id: u64,
    pub full_name: String,
    pub email: String,
    pub graduation_year: i32,
    pub degree: String,
    #[serde(skip_serializing, default)]
    pub password_hash: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl Alumni {
    pub fn from_doc(doc: &Document) -> Result<Self, ModelError> {
        Ok(Self {
            id: read_u64(doc, "_id", "alumni")?,
            full_name: read_string(doc, "full_name", "alumni")?,
            email: read_string(doc, "email", "alumni")?,
            graduation_year: i32::try_from(read_int(doc, "graduation_year", "alumni")?)
                .map_err(|_| corrupt("alumni"))?,
            degree: read_string(doc, "degree", "alumni")?,
            password_hash: read_password(doc),
            created_at: read_time(doc, "created_at", "alumni")?,
            updated_at: read_time(doc, "updated_at", "alumni")?,
        })
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateAlumni {
    pub full_name: String,
    pub email: String,
    pub graduation_year: i32,
    pub degree: String,
}

#[derive(Debug, Default, Deserialize, ToSchema)]
pub struct UpdateAlumni {
    #[serde(default)]
    pub full_name: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub graduation_year: Option<i32>,
    #[serde(default)]
    pub degree: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Announcement {
    #[serde(rename = "_id")]
    pub id: u64,
    pub title: String,
    pub body: String,
    pub author_id: u64,
    pub created_at: String,
    pub updated_at: String,
}

impl Announcement {
    pub fn from_doc(doc: &Document) -> Result<Self, ModelError> {
        Ok(Self {
            id: read_u64(doc, "_id", "announcement")?,
            title: read_string(doc, "title", "announcement")?,
            body: read_string(doc, "body", "announcement")?,
            author_id: read_u64(doc, "author_id", "announcement")?,
            created_at: read_time(doc, "created_at", "announcement")?,
            updated_at: read_time(doc, "updated_at", "announcement")?,
        })
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateAnnouncement {
    pub title: String,
    pub body: String,
    pub author_id: u64,
}

#[derive(Debug, Default, Deserialize, ToSchema)]
pub struct UpdateAnnouncement {
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub body: Option<String>,
    #[serde(default)]
    pub author_id: Option<u64>,
}

pub fn validate_full_name(value: &str) -> Result<(), ModelError> {
    if value.trim().is_empty() {
        return Err(ModelError::Invalid("full_name must not be empty"));
    }
    Ok(())
}

pub fn validate_email(value: &str) -> Result<(), ModelError> {
    let Some((local, domain)) = value.trim().split_once('@') else {
        return Err(ModelError::Invalid("email must contain '@'"));
    };
    if local.is_empty() || domain.is_empty() {
        return Err(ModelError::Invalid("invalid email format"));
    }
    Ok(())
}

pub fn validate_graduation_year(value: i32) -> Result<(), ModelError> {
    if !(MIN_GRADUATION_YEAR..=MAX_GRADUATION_YEAR).contains(&value) {
        return Err(ModelError::Invalid(
            "graduation_year must be between 1900 and 2100",
        ));
    }
    Ok(())
}

pub fn validate_degree(value: &str) -> Result<(), ModelError> {
    if value.trim().is_empty() {
        return Err(ModelError::Invalid("degree must not be empty"));
    }
    Ok(())
}

pub fn validate_title(value: &str) -> Result<(), ModelError> {
    if value.trim().is_empty() {
        return Err(ModelError::Invalid("title must not be empty"));
    }
    Ok(())
}

pub fn validate_body(value: &str) -> Result<(), ModelError> {
    if value.trim().is_empty() {
        return Err(ModelError::Invalid("body must not be empty"));
    }
    Ok(())
}

pub fn clean_name(value: &str) -> String {
    value.trim().to_owned()
}

pub fn clean_email(value: &str) -> String {
    value.trim().to_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_reject_blank_but_keep_content() {
        assert!(validate_full_name("Ada Lovelace").is_ok());
        assert!(validate_full_name("  Ada  ").is_ok());
        assert!(matches!(
            validate_full_name("   "),
            Err(ModelError::Invalid(_))
        ));
    }

    #[test]
    fn emails_need_local_and_domain_parts() {
        assert!(validate_email("ada@example.com").is_ok());
        for bad in ["nope", "@example.com", "local@", "  "] {
            assert!(
                matches!(validate_email(bad), Err(ModelError::Invalid(_))),
                "{bad} must fail"
            );
        }
    }

    #[test]
    fn graduation_year_stays_in_range() {
        assert!(validate_graduation_year(2027).is_ok());
        assert!(validate_graduation_year(1900).is_ok());
        assert!(validate_graduation_year(2100).is_ok());
        for bad in [1899, 2101, -5, 99999] {
            assert!(
                matches!(validate_graduation_year(bad), Err(ModelError::Invalid(_))),
                "{bad} must fail"
            );
        }
    }

    #[test]
    fn degree_title_and_body_reject_blank() {
        assert!(validate_degree("Web Development").is_ok());
        assert!(validate_title("Reunion").is_ok());
        assert!(validate_body("Save the date").is_ok());
        assert!(matches!(
            validate_degree(" "),
            Err(ModelError::Invalid(_))
        ));
        assert!(matches!(validate_title(""), Err(ModelError::Invalid(_))));
        assert!(matches!(validate_body("  "), Err(ModelError::Invalid(_))));
    }

    #[test]
    fn cleaning_trims_and_lowercases_email() {
        assert_eq!(clean_name("  Ada  "), "Ada");
        assert_eq!(clean_email("Ada@Example.COM "), "ada@example.com");
    }
}
