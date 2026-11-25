use mongodb::bson::{Bson, Document};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub const MIN_GRADUATION_YEAR: i32 = 1900;
pub const MAX_GRADUATION_YEAR: i32 = 2100;
pub const MAX_JOB_TITLE_LEN: usize = 120;
pub const MIN_USER_AGE: i32 = 16;
pub const MAX_USER_AGE: i32 = 120;
pub const MIN_PASSWORD_LEN: usize = 8;
pub const MAX_PASSWORD_LEN: usize = 128;

#[derive(Debug, Clone)]
pub enum ModelError {
    Invalid(&'static str),
    Unauthenticated(&'static str),
    Forbidden(&'static str),
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

fn read_bool(doc: &Document, key: &str, what: &str) -> Result<bool, ModelError> {
    doc.get_bool(key).map_err(|_| corrupt(what))
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

fn read_optional_u64(doc: &Document, key: &str, what: &str) -> Result<Option<u64>, ModelError> {
    match doc.get(key) {
        None | Some(Bson::Null) => Ok(None),
        Some(_) => read_u64(doc, key, what).map(Some),
    }
}

fn read_optional_string(doc: &Document, key: &str, what: &str) -> Result<Option<String>, ModelError> {
    match doc.get(key) {
        None | Some(Bson::Null) => Ok(None),
        Some(Bson::String(value)) => Ok(Some(value.clone())),
        _ => Err(corrupt(what)),
    }
}

fn read_optional_role(doc: &Document, key: &str, what: &str) -> Result<Option<Role>, ModelError> {
    match read_optional_string(doc, key, what)? {
        Some(raw) => Role::parse(&raw).map(Some).map_err(|_| corrupt(what)),
        None => Ok(None),
    }
}

fn read_roles(doc: &Document) -> Result<Vec<Role>, ModelError> {
    let items = doc.get_array("roles").map_err(|_| corrupt("user"))?;
    let mut roles = Vec::with_capacity(items.len());
    for item in items {
        let raw = item.as_str().ok_or_else(|| corrupt("user"))?;
        roles.push(Role::parse(raw).map_err(|_| corrupt("user"))?);
    }
    if roles.is_empty() {
        return Err(corrupt("user"));
    }
    Ok(roles)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    Unverified,
    Undergrad,
    Grad,
    Phd,
    Alumni,
    Academic,
    Professor,
    Admin,
}

impl Role {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unverified => "unverified",
            Self::Undergrad => "undergrad",
            Self::Grad => "grad",
            Self::Phd => "phd",
            Self::Alumni => "alumni",
            Self::Academic => "academic",
            Self::Professor => "professor",
            Self::Admin => "admin",
        }
    }

    pub fn parse(value: &str) -> Result<Self, ModelError> {
        match value.trim().to_ascii_lowercase().as_str() {
            "unverified" => Ok(Self::Unverified),
            "undergrad" => Ok(Self::Undergrad),
            "grad" => Ok(Self::Grad),
            "phd" => Ok(Self::Phd),
            "alumni" => Ok(Self::Alumni),
            "academic" => Ok(Self::Academic),
            "professor" => Ok(Self::Professor),
            "admin" => Ok(Self::Admin),
            _ => Err(ModelError::Invalid("unknown role")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum VerificationStatus {
    Pending,
    Verified,
    Rejected,
}

impl VerificationStatus {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Verified => "verified",
            Self::Rejected => "rejected",
        }
    }

    pub fn parse(value: &str) -> Result<Self, ModelError> {
        match value.trim().to_ascii_lowercase().as_str() {
            "pending" => Ok(Self::Pending),
            "verified" => Ok(Self::Verified),
            "rejected" => Ok(Self::Rejected),
            _ => Err(ModelError::Invalid("unknown verification status")),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: u64,
    pub first_name: String,
    pub last_name: String,
    pub full_name: String,
    pub age: i32,
    pub email: String,
    pub roles: Vec<Role>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_role: Option<Role>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graduation_date: Option<String>,
    pub verification_status: VerificationStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    pub email_verified: bool,
    pub phone_verified: bool,
    pub is_active: bool,
    #[serde(skip_serializing, default)]
    pub password_hash: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl User {
    #[must_use]
    pub fn has_role(&self, role: Role) -> bool {
        self.roles.contains(&role)
    }

    #[must_use]
    pub fn is_admin(&self) -> bool {
        self.has_role(Role::Admin)
    }

    #[must_use]
    pub fn is_verified_member(&self) -> bool {
        self.roles.iter().any(|role| *role != Role::Unverified)
    }

    pub fn can_read_directory(&self) -> Result<(), ModelError> {
        if self.is_verified_member() {
            return Ok(());
        }
        Err(ModelError::Forbidden("account verification pending"))
    }

    pub fn can_create_alumni(&self) -> Result<(), ModelError> {
        if self.is_admin() {
            return Ok(());
        }
        Err(ModelError::Forbidden("admin role required"))
    }

    pub fn can_update_alumni(&self, alumni_user_id: Option<u64>) -> Result<(), ModelError> {
        if self.is_admin() || (self.has_role(Role::Alumni) && alumni_user_id == Some(self.id)) {
            return Ok(());
        }
        Err(ModelError::Forbidden(
            "only admins or the linked alumnus can update this record",
        ))
    }

    pub fn can_delete_alumni(&self) -> Result<(), ModelError> {
        if self.is_admin() {
            return Ok(());
        }
        Err(ModelError::Forbidden("admin role required"))
    }

    pub fn can_create_announcement(&self) -> Result<(), ModelError> {
        if self.is_admin() || self.has_role(Role::Academic) || self.has_role(Role::Professor) {
            return Ok(());
        }
        Err(ModelError::Forbidden(
            "academic, professor, or admin role required",
        ))
    }

    pub fn can_modify_announcement(&self, author_id: u64) -> Result<(), ModelError> {
        if self.is_admin()
            || ((self.has_role(Role::Academic) || self.has_role(Role::Professor))
                && author_id == self.id)
        {
            return Ok(());
        }
        Err(ModelError::Forbidden(
            "only admins or the announcement author can modify it",
        ))
    }

    pub fn can_create_job_posting(&self) -> Result<(), ModelError> {
        if self.is_admin()
            || self.has_role(Role::Alumni)
            || self.has_role(Role::Academic)
            || self.has_role(Role::Professor)
        {
            return Ok(());
        }
        Err(ModelError::Forbidden(
            "alumni, academic, professor, or admin role required",
        ))
    }

    pub fn can_modify_job_posting(&self, author_id: u64) -> Result<(), ModelError> {
        if self.is_admin()
            || ((self.has_role(Role::Alumni)
                || self.has_role(Role::Academic)
                || self.has_role(Role::Professor))
                && author_id == self.id)
        {
            return Ok(());
        }
        Err(ModelError::Forbidden(
            "only admins or the posting author can modify it",
        ))
    }

    pub fn can_read_inactive_job_posting(&self, author_id: u64) -> Result<(), ModelError> {
        if self.is_admin() || author_id == self.id {
            return Ok(());
        }
        Err(ModelError::Forbidden("inactive posting is restricted"))
    }

    pub fn can_act_as_author(&self, author_id: u64) -> Result<(), ModelError> {
        if self.is_admin() || author_id == self.id {
            return Ok(());
        }
        Err(ModelError::Forbidden(
            "non-admin users can only publish under their own id",
        ))
    }

    pub fn can_manage_users(&self) -> Result<(), ModelError> {
        if self.is_admin() {
            return Ok(());
        }
        Err(ModelError::Forbidden("admin role required"))
    }

    pub fn from_doc(doc: &Document) -> Result<Self, ModelError> {
        let verification_raw = read_string(doc, "verification_status", "user")?;
        Ok(Self {
            id: read_u64(doc, "_id", "user")?,
            first_name: read_string(doc, "first_name", "user")?,
            last_name: read_string(doc, "last_name", "user")?,
            full_name: read_string(doc, "full_name", "user")?,
            age: i32::try_from(read_int(doc, "age", "user")?).map_err(|_| corrupt("user"))?,
            email: read_string(doc, "email", "user")?,
            roles: read_roles(doc)?,
            requested_role: read_optional_role(doc, "requested_role", "user")?,
            graduation_date: read_optional_string(doc, "graduation_date", "user")?,
            verification_status: VerificationStatus::parse(&verification_raw)
                .map_err(|_| corrupt("user"))?,
            verification_method: read_optional_string(doc, "verification_method", "user")?,
            phone_number: read_optional_string(doc, "phone_number", "user")?,
            email_verified: read_bool(doc, "email_verified", "user")?,
            phone_verified: read_bool(doc, "phone_verified", "user")?,
            is_active: read_bool(doc, "is_active", "user")?,
            password_hash: read_password(doc),
            created_at: read_time(doc, "created_at", "user")?,
            updated_at: read_time(doc, "updated_at", "user")?,
        })
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct RegisterUser {
    pub first_name: String,
    pub last_name: String,
    pub age: i32,
    pub email: String,
    #[serde(default)]
    pub academic_status: Option<Role>,
    #[serde(default)]
    pub graduation_date: Option<String>,
    #[serde(default)]
    pub phone_number: Option<String>,
    pub password: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AuthSession {
    pub user: User,
    pub expires_at: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct VerifyUserRequest {
    pub verification_status: VerificationStatus,
    #[serde(default)]
    pub roles: Option<Vec<Role>>,
    #[serde(default)]
    pub verification_method: Option<String>,
}

#[derive(Debug, Default, Deserialize, ToSchema)]
pub struct AdminUpdateUserRequest {
    #[serde(default)]
    pub roles: Option<Vec<Role>>,
    #[serde(default)]
    pub is_active: Option<bool>,
}

#[derive(Debug, Default, Deserialize, ToSchema)]
pub struct UpdateOwnProfileRequest {
    #[serde(default)]
    pub first_name: Option<String>,
    #[serde(default)]
    pub last_name: Option<String>,
    #[serde(default)]
    pub phone_number: Option<String>,
    #[serde(default)]
    pub graduation_date: Option<String>,
    #[serde(default)]
    pub current_password: Option<String>,
    #[serde(default)]
    pub new_password: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Alumni {
    #[serde(rename = "_id")]
    pub id: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<u64>,
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
            user_id: read_optional_u64(doc, "user_id", "alumni")?,
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

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
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
    pub updated_at: String,
}

impl JobPosting {
    pub fn from_doc(doc: &Document) -> Result<Self, ModelError> {
        Ok(Self {
            id: read_u64(doc, "_id", "job posting")?,
            title: read_string(doc, "title", "job posting")?,
            company: read_string(doc, "company", "job posting")?,
            description: read_string(doc, "description", "job posting")?,
            location: read_string(doc, "location", "job posting")?,
            employment_type: read_string(doc, "employment_type", "job posting")?,
            apply_url: read_string(doc, "apply_url", "job posting")?,
            author_id: read_u64(doc, "author_id", "job posting")?,
            is_active: read_bool(doc, "is_active", "job posting")?,
            created_at: read_time(doc, "created_at", "job posting")?,
            updated_at: read_time(doc, "updated_at", "job posting")?,
        })
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateJobPosting {
    pub title: String,
    pub company: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub location: Option<String>,
    pub employment_type: String,
    pub apply_url: String,
    pub author_id: u64,
    #[serde(default)]
    pub is_active: Option<bool>,
}

#[derive(Debug, Default, Deserialize, ToSchema)]
pub struct UpdateJobPosting {
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub company: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub location: Option<String>,
    #[serde(default)]
    pub employment_type: Option<String>,
    #[serde(default)]
    pub apply_url: Option<String>,
    #[serde(default)]
    pub author_id: Option<u64>,
    #[serde(default)]
    pub is_active: Option<bool>,
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

pub fn validate_job_title(value: &str) -> Result<(), ModelError> {
    validate_title(value)?;
    if value.trim().chars().count() > MAX_JOB_TITLE_LEN {
        return Err(ModelError::Invalid("title must be at most 120 characters"));
    }
    Ok(())
}

pub fn validate_body(value: &str) -> Result<(), ModelError> {
    if value.trim().is_empty() {
        return Err(ModelError::Invalid("body must not be empty"));
    }
    Ok(())
}

pub fn validate_company(value: &str) -> Result<(), ModelError> {
    if value.trim().is_empty() {
        return Err(ModelError::Invalid("company must not be empty"));
    }
    Ok(())
}

pub fn validate_employment_type(value: &str) -> Result<(), ModelError> {
    if value.trim().is_empty() {
        return Err(ModelError::Invalid("employment_type must not be empty"));
    }
    Ok(())
}

pub fn validate_apply_url(value: &str) -> Result<(), ModelError> {
    let url = value.trim();
    if url.is_empty() {
        return Err(ModelError::Invalid("apply_url must not be empty"));
    }
    if !url.starts_with("http://") && !url.starts_with("https://") {
        return Err(ModelError::Invalid(
            "apply_url must start with http:// or https://",
        ));
    }
    Ok(())
}

pub fn clean_name(value: &str) -> String {
    value.trim().to_owned()
}

pub fn clean_email(value: &str) -> String {
    value.trim().to_lowercase()
}

pub fn normalize_assigned_roles(roles: &[Role]) -> Result<Vec<Role>, ModelError> {
    let mut unique = Vec::with_capacity(roles.len());
    for role in roles {
        if *role == Role::Unverified {
            return Err(ModelError::Invalid(
                "cannot assign unverified alongside active roles",
            ));
        }
        if !unique.contains(role) {
            unique.push(*role);
        }
    }
    if unique.is_empty() {
        return Err(ModelError::Invalid("at least one role is required"));
    }
    Ok(unique)
}

pub fn validate_first_name(value: &str) -> Result<(), ModelError> {
    if value.trim().is_empty() {
        return Err(ModelError::Invalid("first_name must not be empty"));
    }
    Ok(())
}

pub fn validate_last_name(value: &str) -> Result<(), ModelError> {
    if value.trim().is_empty() {
        return Err(ModelError::Invalid("last_name must not be empty"));
    }
    Ok(())
}

pub fn validate_age(value: i32) -> Result<(), ModelError> {
    if !(MIN_USER_AGE..=MAX_USER_AGE).contains(&value) {
        return Err(ModelError::Invalid("age must be between 16 and 120"));
    }
    Ok(())
}

pub fn validate_password(value: &str) -> Result<(), ModelError> {
    let count = value.chars().count();
    if value.trim().is_empty() || count < MIN_PASSWORD_LEN {
        return Err(ModelError::Invalid(
            "password must be at least 8 characters",
        ));
    }
    if count > MAX_PASSWORD_LEN {
        return Err(ModelError::Invalid(
            "password must be at most 128 characters",
        ));
    }
    Ok(())
}

const fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 {
                29
            } else {
                28
            }
        }
        _ => 0,
    }
}

pub fn validate_graduation_date(value: &str) -> Result<String, ModelError> {
    let trimmed = value.trim();
    let bytes = trimmed.as_bytes();
    if bytes.len() != 10 || bytes[4] != b'-' || bytes[7] != b'-' {
        return Err(ModelError::Invalid(
            "graduation_date must use YYYY-MM-DD format",
        ));
    }
    let year: i32 = trimmed[0..4]
        .parse()
        .map_err(|_| ModelError::Invalid("graduation_date must use YYYY-MM-DD format"))?;
    let month: u32 = trimmed[5..7]
        .parse()
        .map_err(|_| ModelError::Invalid("graduation_date must use YYYY-MM-DD format"))?;
    let day: u32 = trimmed[8..10]
        .parse()
        .map_err(|_| ModelError::Invalid("graduation_date must use YYYY-MM-DD format"))?;
    if !(MIN_GRADUATION_YEAR..=MAX_GRADUATION_YEAR).contains(&year) {
        return Err(ModelError::Invalid(
            "graduation_date year must be between 1900 and 2100",
        ));
    }
    if !(1..=12).contains(&month) || day == 0 || day > days_in_month(year, month) {
        return Err(ModelError::Invalid("graduation_date is not a valid date"));
    }
    Ok(trimmed.to_owned())
}

pub fn resolve_registration_role(
    academic_status: Option<Role>,
    graduation_date: Option<&str>,
) -> Result<(Role, Option<String>), ModelError> {
    if academic_status == Some(Role::Admin) {
        return Err(ModelError::Forbidden(
            "admin accounts cannot self-register",
        ));
    }
    if academic_status == Some(Role::Unverified) {
        return Err(ModelError::Invalid(
            "academic_status or graduation_date is required",
        ));
    }
    let cleaned_date = match graduation_date {
        Some(raw) if !raw.trim().is_empty() => Some(validate_graduation_date(raw)?),
        _ => None,
    };
    match (academic_status, cleaned_date) {
        (Some(Role::Alumni), None) => Err(ModelError::Invalid(
            "graduation_date is required for alumni",
        )),
        (Some(role), date) => Ok((role, date)),
        (None, Some(date)) => Ok((Role::Alumni, Some(date))),
        (None, None) => Err(ModelError::Invalid(
            "academic_status or graduation_date is required",
        )),
    }
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

    #[test]
    fn job_posting_fields_reject_blank() {
        for result in [
            validate_job_title(" "),
            validate_company(""),
            validate_employment_type(" "),
        ] {
            assert!(matches!(result, Err(ModelError::Invalid(_))));
        }
        assert!(validate_company("Acme").is_ok());
        assert!(validate_employment_type("Full-time").is_ok());
    }

    #[test]
    fn job_title_rejects_over_120_chars() {
        assert!(validate_job_title("Rust Engineer").is_ok());
        assert!(validate_job_title(&"x".repeat(MAX_JOB_TITLE_LEN)).is_ok());
        assert!(matches!(
            validate_job_title(&"x".repeat(MAX_JOB_TITLE_LEN + 1)),
            Err(ModelError::Invalid(_))
        ));
    }

    #[test]
    fn apply_url_needs_http_prefix() {
        assert!(validate_apply_url("https://example.com/jobs/1").is_ok());
        assert!(validate_apply_url("http://example.com").is_ok());
        for bad in ["", "ftp://example.com", "example.com", "//example.com"] {
            assert!(
                matches!(validate_apply_url(bad), Err(ModelError::Invalid(_))),
                "{bad} must fail"
            );
        }
    }

    #[test]
    fn user_age_and_password_stay_in_bounds() {
        assert!(validate_age(16).is_ok());
        assert!(validate_age(120).is_ok());
        for bad in [15, 0, -1, 121] {
            assert!(matches!(validate_age(bad), Err(ModelError::Invalid(_))));
        }

        assert!(validate_password("secret12").is_ok());
        assert!(matches!(
            validate_password("short"),
            Err(ModelError::Invalid(_))
        ));
        assert!(matches!(
            validate_password("        "),
            Err(ModelError::Invalid(_))
        ));
    }

    #[test]
    fn graduation_date_validates_calendar() {
        assert_eq!(
            validate_graduation_date(" 2024-02-29 ").unwrap(),
            "2024-02-29"
        );
        for bad in ["2025-02-29", "2024-13-01", "24-01-01", "1899-01-01", ""] {
            assert!(
                matches!(validate_graduation_date(bad), Err(ModelError::Invalid(_))),
                "{bad} must fail"
            );
        }
    }

    #[test]
    fn registration_role_handles_status_or_graduation_date() {
        assert_eq!(
            resolve_registration_role(Some(Role::Undergrad), None).unwrap(),
            (Role::Undergrad, None)
        );
        assert_eq!(
            resolve_registration_role(None, Some("2025-06-15")).unwrap(),
            (Role::Alumni, Some("2025-06-15".to_owned()))
        );
        assert!(matches!(
            resolve_registration_role(Some(Role::Alumni), None),
            Err(ModelError::Invalid(_))
        ));
        assert!(matches!(
            resolve_registration_role(Some(Role::Admin), None),
            Err(ModelError::Forbidden(_))
        ));
        assert!(matches!(
            resolve_registration_role(None, None),
            Err(ModelError::Invalid(_))
        ));
    }

    fn user_with_roles(id: u64, roles: Vec<Role>) -> User {
        User {
            id,
            first_name: "Test".to_owned(),
            last_name: "User".to_owned(),
            full_name: "Test User".to_owned(),
            age: 24,
            email: "test@example.edu".to_owned(),
            roles,
            requested_role: None,
            graduation_date: None,
            verification_status: VerificationStatus::Verified,
            verification_method: Some("government_papers".to_owned()),
            phone_number: None,
            email_verified: false,
            phone_verified: false,
            is_active: true,
            password_hash: None,
            created_at: "2026-11-25T00:00:00Z".to_owned(),
            updated_at: "2026-11-25T00:00:00Z".to_owned(),
        }
    }

    #[test]
    fn authorization_policies_enforce_roles_and_ownership() {
        let unverified = user_with_roles(1, vec![Role::Unverified]);
        assert!(matches!(
            unverified.can_read_directory(),
            Err(ModelError::Forbidden(_))
        ));
        assert!(matches!(
            unverified.can_create_job_posting(),
            Err(ModelError::Forbidden(_))
        ));

        let undergrad = user_with_roles(2, vec![Role::Undergrad]);
        assert!(undergrad.can_read_directory().is_ok());
        assert!(matches!(
            undergrad.can_create_job_posting(),
            Err(ModelError::Forbidden(_))
        ));
        assert!(matches!(
            undergrad.can_create_announcement(),
            Err(ModelError::Forbidden(_))
        ));

        let alumnus = user_with_roles(3, vec![Role::Alumni]);
        assert!(alumnus.can_create_job_posting().is_ok());
        assert!(alumnus.can_modify_job_posting(3).is_ok());
        assert!(matches!(
            alumnus.can_modify_job_posting(9),
            Err(ModelError::Forbidden(_))
        ));
        assert!(alumnus.can_update_alumni(Some(3)).is_ok());
        assert!(matches!(
            alumnus.can_update_alumni(Some(9)),
            Err(ModelError::Forbidden(_))
        ));

        let professor = user_with_roles(4, vec![Role::Professor]);
        assert!(professor.can_create_announcement().is_ok());
        assert!(professor.can_modify_announcement(4).is_ok());
        assert!(matches!(
            professor.can_modify_announcement(3),
            Err(ModelError::Forbidden(_))
        ));

        let admin = user_with_roles(5, vec![Role::Admin]);
        assert!(admin.can_create_alumni().is_ok());
        assert!(admin.can_delete_alumni().is_ok());
        assert!(admin.can_modify_announcement(3).is_ok());
        assert!(admin.can_modify_job_posting(3).is_ok());
        assert!(admin.can_manage_users().is_ok());
        assert!(matches!(
            professor.can_manage_users(),
            Err(ModelError::Forbidden(_))
        ));
    }

    #[test]
    fn assigned_roles_normalize_and_reject_unverified_or_empty() {
        assert_eq!(
            normalize_assigned_roles(&[Role::Professor, Role::Admin, Role::Professor]).unwrap(),
            vec![Role::Professor, Role::Admin]
        );
        assert!(matches!(
            normalize_assigned_roles(&[]),
            Err(ModelError::Invalid(_))
        ));
        assert!(matches!(
            normalize_assigned_roles(&[Role::Unverified]),
            Err(ModelError::Invalid(_))
        ));
    }
}
