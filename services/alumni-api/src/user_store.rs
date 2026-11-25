use base64::{Engine as _, engine::general_purpose::STANDARD_NO_PAD};
use futures_util::TryStreamExt;
use hmac::Hmac;
use mongodb::bson::{DateTime, Document, doc};
use mongodb::error::{ErrorKind, WriteFailure};
use mongodb::options::ReturnDocument;
use mongodb::{Collection, Database};
use pbkdf2::pbkdf2;
use rand::RngCore;
use sha2::Sha256;

use crate::models::{
    AdminUpdateUserRequest, ModelError, RegisterUser, Role, UpdateOwnProfileRequest, User,
    VerificationStatus, VerifyUserRequest, clean_email, clean_name, normalize_assigned_roles,
    resolve_registration_role, validate_age, validate_email, validate_first_name,
    validate_graduation_date, validate_last_name, validate_password,
};
use crate::verification::{VerificationPolicy, validate_phone_number};

const PBKDF2_ROUNDS: u32 = 100_000;
const SALT_LEN: usize = 16;
const HASH_LEN: usize = 32;

fn write_error(err: mongodb::error::Error) -> ModelError {
    if let &ErrorKind::Write(WriteFailure::WriteError(ref failure)) = err.kind.as_ref()
        && failure.code == 11000
    {
        return ModelError::Duplicate;
    }
    ModelError::internal(err)
}

fn hash_password(password: &str) -> Result<String, ModelError> {
    let mut salt = [0_u8; SALT_LEN];
    rand::rng().fill_bytes(&mut salt);
    let mut derived = [0_u8; HASH_LEN];
    pbkdf2::<Hmac<Sha256>>(password.as_bytes(), &salt, PBKDF2_ROUNDS, &mut derived)
        .map_err(ModelError::internal)?;
    Ok(format!(
        "$pbkdf2-sha256$i={PBKDF2_ROUNDS}${}${}",
        STANDARD_NO_PAD.encode(salt),
        STANDARD_NO_PAD.encode(derived),
    ))
}

fn verify_password(password: &str, encoded: &str) -> bool {
    let parts: Vec<&str> = encoded.split('$').collect();
    if parts.len() != 5 || !parts[0].is_empty() || parts[1] != "pbkdf2-sha256" {
        return false;
    }
    let Some(rounds_raw) = parts[2].strip_prefix("i=") else {
        return false;
    };
    let Ok(rounds) = rounds_raw.parse::<u32>() else {
        return false;
    };
    let Ok(salt) = STANDARD_NO_PAD.decode(parts[3]) else {
        return false;
    };
    let Ok(expected) = STANDARD_NO_PAD.decode(parts[4]) else {
        return false;
    };
    if expected.is_empty() {
        return false;
    }
    let mut derived = vec![0_u8; expected.len()];
    if pbkdf2::<Hmac<Sha256>>(password.as_bytes(), &salt, rounds, &mut derived).is_err() {
        return false;
    }
    let mut diff = 0_u8;
    for (a, b) in derived.iter().zip(expected.iter()) {
        diff |= a ^ b;
    }
    diff == 0
}

#[derive(Debug, Clone)]
pub struct UserStore {
    users: Collection<Document>,
    counters: Collection<Document>,
    verification: VerificationPolicy,
}

impl UserStore {
    pub fn new(db: &Database) -> Self {
        Self::with_policy(db, VerificationPolicy::default())
    }

    pub fn with_policy(db: &Database, verification: VerificationPolicy) -> Self {
        Self {
            users: db.collection("users"),
            counters: db.collection("counters"),
            verification,
        }
    }

    async fn next_id(&self, key: &str) -> Result<u64, ModelError> {
        let updated = self
            .counters
            .find_one_and_update(doc! { "_id": key }, doc! { "$inc": { "seq": 1_i64 } })
            .upsert(true)
            .return_document(ReturnDocument::After)
            .await
            .map_err(ModelError::internal)?
            .ok_or_else(|| ModelError::Internal(format!("counter {key} missing after upsert")))?;
        let seq = updated.get_i64("seq").map_err(ModelError::internal)?;
        u64::try_from(seq).map_err(ModelError::internal)
    }

    /// Validates and persists a newly registered user as `unverified` pending
    /// government-paper verification. Email and SMS checks stay inactive unless
    /// enabled in `VerificationPolicy`.
    pub async fn register(&self, input: &RegisterUser) -> Result<User, ModelError> {
        validate_first_name(&input.first_name)?;
        validate_last_name(&input.last_name)?;
        validate_age(input.age)?;
        validate_email(&input.email)?;
        validate_password(&input.password)?;
        let (requested_role, graduation_date) =
            resolve_registration_role(input.academic_status, input.graduation_date.as_deref())?;
        let phone_number = match input.phone_number.as_deref() {
            Some(raw) if !raw.trim().is_empty() => Some(validate_phone_number(raw)?),
            _ => None,
        };

        let first_name = clean_name(&input.first_name);
        let last_name = clean_name(&input.last_name);
        let full_name = format!("{first_name} {last_name}");
        let email = clean_email(&input.email);
        let password_hash = hash_password(&input.password)?;

        let _pending_checks =
            self.verification
                .pending_channels(VerificationStatus::Pending, false, false);

        let id = self.next_id("users").await?;
        let now = DateTime::now();
        let mut doc = doc! {
            "_id": i64::try_from(id).map_err(ModelError::internal)?,
            "first_name": first_name,
            "last_name": last_name,
            "full_name": full_name,
            "age": input.age,
            "email": email,
            "password_hash": password_hash,
            "roles": [Role::Unverified.as_str()],
            "requested_role": requested_role.as_str(),
            "verification_status": VerificationStatus::Pending.as_str(),
            "email_verified": false,
            "phone_verified": false,
            "is_active": true,
            "created_at": now,
            "updated_at": now,
        };
        if let Some(date) = graduation_date {
            doc.insert("graduation_date", date);
        }
        if let Some(phone) = phone_number {
            doc.insert("phone_number", phone);
        }

        self.users.insert_one(doc).await.map_err(write_error)?;
        self.get(id).await
    }

    /// Loads one user record by id.
    pub async fn get(&self, id: u64) -> Result<User, ModelError> {
        let key = i64::try_from(id).map_err(ModelError::internal)?;
        let doc = self
            .users
            .find_one(doc! { "_id": key })
            .await
            .map_err(ModelError::internal)?;
        doc.map(|doc| User::from_doc(&doc))
            .transpose()?
            .ok_or(ModelError::NotFound)
    }

    /// Lists all users, ordered by `_id`.
    pub async fn list(&self) -> Result<Vec<User>, ModelError> {
        let mut cursor = self
            .users
            .find(doc! {})
            .sort(doc! { "_id": 1 })
            .await
            .map_err(ModelError::internal)?;
        let mut users = Vec::new();
        while let Some(doc) = cursor.try_next().await.map_err(ModelError::internal)? {
            users.push(User::from_doc(&doc)?);
        }
        Ok(users)
    }

    /// Approves or rejects a user's government-paper verification.
    pub async fn verify_user(
        &self,
        admin_id: u64,
        user_id: u64,
        input: &VerifyUserRequest,
    ) -> Result<User, ModelError> {
        let existing = self.get(user_id).await?;
        let now = DateTime::now();
        let verifier = i64::try_from(admin_id).map_err(ModelError::internal)?;
        let key = i64::try_from(user_id).map_err(ModelError::internal)?;

        let set = match input.verification_status {
            VerificationStatus::Pending => {
                return Err(ModelError::Invalid(
                    "verification_status must be verified or rejected",
                ));
            }
            VerificationStatus::Verified => {
                let raw_roles = match &input.roles {
                    Some(roles) => roles.clone(),
                    None => match existing.requested_role {
                        Some(role) => vec![role],
                        None => {
                            return Err(ModelError::Invalid("roles are required to verify user"));
                        }
                    },
                };
                let roles = normalize_assigned_roles(&raw_roles)?;
                let role_strings: Vec<&str> = roles.iter().map(|r| r.as_str()).collect();
                let method = input
                    .verification_method
                    .as_deref()
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                    .unwrap_or("government_papers");
                doc! {
                    "roles": role_strings,
                    "verification_status": VerificationStatus::Verified.as_str(),
                    "verification_method": method,
                    "verified_at": now,
                    "verified_by": verifier,
                    "updated_at": now,
                }
            }
            VerificationStatus::Rejected => {
                doc! {
                    "roles": [Role::Unverified.as_str()],
                    "verification_status": VerificationStatus::Rejected.as_str(),
                    "verification_method": "government_papers",
                    "verified_at": now,
                    "verified_by": verifier,
                    "updated_at": now,
                }
            }
        };

        self.users
            .update_one(doc! { "_id": key }, doc! { "$set": set })
            .await
            .map_err(ModelError::internal)?;
        self.get(user_id).await
    }

    /// Updates a user's roles and/or active state from the admin panel.
    /// Returns `(updated_user, revoke_sessions)`.
    pub async fn admin_update(
        &self,
        admin_id: u64,
        user_id: u64,
        input: &AdminUpdateUserRequest,
    ) -> Result<(User, bool), ModelError> {
        let existing = self.get(user_id).await?;
        let mut set = Document::new();
        let mut revoke_sessions = false;

        if let Some(raw_roles) = &input.roles {
            let roles = normalize_assigned_roles(raw_roles)?;
            if admin_id == user_id && !roles.contains(&Role::Admin) {
                return Err(ModelError::Blocked(
                    "cannot remove admin role from your own account",
                ));
            }
            let role_strings: Vec<&str> = roles.iter().map(|r| r.as_str()).collect();
            set.insert("roles", role_strings);
            revoke_sessions = true;
        }

        if let Some(active) = input.is_active {
            if admin_id == user_id && !active {
                return Err(ModelError::Blocked("cannot disable your own admin account"));
            }
            set.insert("is_active", active);
            if !active {
                revoke_sessions = true;
            }
        }

        if set.is_empty() {
            return Ok((existing, false));
        }

        set.insert("updated_at", DateTime::now());
        let key = i64::try_from(user_id).map_err(ModelError::internal)?;
        self.users
            .update_one(doc! { "_id": key }, doc! { "$set": set })
            .await
            .map_err(ModelError::internal)?;
        Ok((self.get(user_id).await?, revoke_sessions))
    }

    /// Ensures a bootstrap admin user exists or promotes an existing account
    /// matching `BOOTSTRAP_ADMIN_EMAIL`.
    pub async fn ensure_bootstrap_admin(
        &self,
        email: &str,
        password: Option<&str>,
    ) -> Result<(), ModelError> {
        validate_email(email)?;
        let cleaned = clean_email(email);
        if let Some(doc) = self
            .users
            .find_one(doc! { "email": &cleaned })
            .await
            .map_err(ModelError::internal)?
        {
            let existing = User::from_doc(&doc)?;
            let mut roles: Vec<Role> = existing
                .roles
                .into_iter()
                .filter(|role| *role != Role::Unverified)
                .collect();
            if !roles.contains(&Role::Admin) {
                roles.push(Role::Admin);
            }
            let role_strings: Vec<&str> = roles.iter().map(|role| role.as_str()).collect();
            let now = DateTime::now();
            self.users
                .update_one(
                    doc! { "_id": i64::try_from(existing.id).map_err(ModelError::internal)? },
                    doc! { "$set": {
                        "roles": role_strings,
                        "verification_status": VerificationStatus::Verified.as_str(),
                        "verification_method": "bootstrap",
                        "verified_at": now,
                        "is_active": true,
                        "updated_at": now,
                    } },
                )
                .await
                .map_err(ModelError::internal)?;
            return Ok(());
        }

        let Some(raw_password) = password else {
            return Ok(());
        };
        validate_password(raw_password)?;

        let password_hash = hash_password(raw_password)?;
        let id = self.next_id("users").await?;
        let now = DateTime::now();
        self.users
            .insert_one(doc! {
                "_id": i64::try_from(id).map_err(ModelError::internal)?,
                "first_name": "Beraat",
                "last_name": "Yildirim",
                "full_name": "Beraat Yildirim",
                "age": 24_i32,
                "email": cleaned,
                "password_hash": password_hash,
                "roles": [Role::Admin.as_str()],
                "verification_status": VerificationStatus::Verified.as_str(),
                "verification_method": "bootstrap",
                "verified_at": now,
                "email_verified": true,
                "phone_verified": false,
                "is_active": true,
                "created_at": now,
                "updated_at": now,
            })
            .await
            .map_err(write_error)?;
        Ok(())
    }

    /// Verifies email and password credentials and returns the active `User`.
    pub async fn authenticate(&self, email: &str, password: &str) -> Result<User, ModelError> {
        validate_email(email)?;
        if password.is_empty() {
            return Err(ModelError::Invalid("password must not be empty"));
        }
        let cleaned = clean_email(email);
        let doc = self
            .users
            .find_one(doc! { "email": cleaned })
            .await
            .map_err(ModelError::internal)?
            .ok_or(ModelError::Unauthenticated("invalid email or password"))?;

        let user = User::from_doc(&doc)?;
        let Some(hash) = user.password_hash.as_deref() else {
            return Err(ModelError::Unauthenticated("invalid email or password"));
        };
        if !verify_password(password, hash) {
            return Err(ModelError::Unauthenticated("invalid email or password"));
        }
        if !user.is_active {
            return Err(ModelError::Unauthenticated("account is disabled"));
        }
        Ok(user)
    }

    /// Updates the signed-in user's own profile fields and optionally changes
    /// their password after checking `current_password`.
    pub async fn update_own_profile(
        &self,
        user_id: u64,
        input: &UpdateOwnProfileRequest,
    ) -> Result<User, ModelError> {
        let existing = self.get(user_id).await?;
        let mut set = Document::new();
        let mut unset = Document::new();

        let next_first = match &input.first_name {
            Some(raw) => {
                validate_first_name(raw)?;
                let cleaned = clean_name(raw);
                set.insert("first_name", &cleaned);
                cleaned
            }
            None => existing.first_name.clone(),
        };
        let next_last = match &input.last_name {
            Some(raw) => {
                validate_last_name(raw)?;
                let cleaned = clean_name(raw);
                set.insert("last_name", &cleaned);
                cleaned
            }
            None => existing.last_name.clone(),
        };
        if input.first_name.is_some() || input.last_name.is_some() {
            set.insert("full_name", format!("{next_first} {next_last}"));
        }

        if let Some(raw_phone) = &input.phone_number {
            if raw_phone.trim().is_empty() {
                unset.insert("phone_number", "");
                set.insert("phone_verified", false);
            } else {
                let phone = validate_phone_number(raw_phone)?;
                set.insert("phone_number", phone);
            }
        }

        if let Some(raw_date) = &input.graduation_date {
            if raw_date.trim().is_empty() {
                if existing.has_role(Role::Alumni)
                    || existing.requested_role == Some(Role::Alumni)
                {
                    return Err(ModelError::Invalid(
                        "graduation_date is required for alumni",
                    ));
                }
                unset.insert("graduation_date", "");
            } else {
                let date = validate_graduation_date(raw_date)?;
                set.insert("graduation_date", date);
            }
        }

        if let Some(new_password) = &input.new_password {
            let current = input
                .current_password
                .as_deref()
                .filter(|s| !s.is_empty())
                .ok_or(ModelError::Invalid("current_password is required"))?;
            let Some(hash) = existing.password_hash.as_deref() else {
                return Err(ModelError::Invalid("current password is not set"));
            };
            if !verify_password(current, hash) {
                return Err(ModelError::Unauthenticated("current password is incorrect"));
            }
            validate_password(new_password)?;
            set.insert("password_hash", hash_password(new_password)?);
        }

        if set.is_empty() && unset.is_empty() {
            return Ok(existing);
        }

        set.insert("updated_at", DateTime::now());
        let mut update_doc = doc! { "$set": set };
        if !unset.is_empty() {
            update_doc.insert("$unset", unset);
        }

        let key = i64::try_from(user_id).map_err(ModelError::internal)?;
        self.users
            .update_one(doc! { "_id": key }, update_doc)
            .await
            .map_err(ModelError::internal)?;
        self.get(user_id).await
    }
}
