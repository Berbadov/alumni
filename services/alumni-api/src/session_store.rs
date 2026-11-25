use mongodb::bson::{DateTime, Document, doc};
use mongodb::{Collection, Database};
use rand::RngCore;
use sha2::{Digest, Sha256};

use crate::models::ModelError;

const TOKEN_BYTES: usize = 32;
const SESSION_TTL_MILLIS: i64 = 7 * 24 * 60 * 60 * 1000;

fn hash_token(raw_token: &str) -> String {
    let digest = Sha256::digest(raw_token.as_bytes());
    hex::encode(digest)
}

#[derive(Debug, Clone)]
pub struct SessionStore {
    sessions: Collection<Document>,
}

impl SessionStore {
    pub fn new(db: &Database) -> Self {
        Self {
            sessions: db.collection("sessions"),
        }
    }

    /// Creates a new 7-day session for `user_id` and returns `(raw_token, expires_at_rfc3339)`.
    pub async fn create(&self, user_id: u64) -> Result<(String, String), ModelError> {
        let mut bytes = [0_u8; TOKEN_BYTES];
        rand::rng().fill_bytes(&mut bytes);
        let raw_token = hex::encode(bytes);
        let token_hash = hash_token(&raw_token);

        let now = DateTime::now();
        let expires_at = DateTime::from_millis(now.timestamp_millis() + SESSION_TTL_MILLIS);
        let expires_iso = expires_at
            .try_to_rfc3339_string()
            .map_err(ModelError::internal)?;

        let uid = i64::try_from(user_id).map_err(ModelError::internal)?;
        self.sessions
            .insert_one(doc! {
                "_id": token_hash,
                "user_id": uid,
                "expires_at": expires_at,
                "created_at": now,
            })
            .await
            .map_err(ModelError::internal)?;

        Ok((raw_token, expires_iso))
    }

    /// Resolves an active, unrevoked, unexpired session token to its `user_id`.
    pub async fn resolve_user_id(&self, raw_token: &str) -> Result<u64, ModelError> {
        if raw_token.trim().is_empty() {
            return Err(ModelError::Unauthenticated("missing session token"));
        }
        let token_hash = hash_token(raw_token.trim());
        let now = DateTime::now();
        let doc = self
            .sessions
            .find_one(doc! {
                "_id": token_hash,
                "revoked_at": { "$exists": false },
                "expires_at": { "$gt": now },
            })
            .await
            .map_err(ModelError::internal)?
            .ok_or(ModelError::Unauthenticated("invalid or expired session"))?;

        let raw_id = doc
            .get_i64("user_id")
            .map_err(|_| ModelError::Internal("corrupt session document".to_owned()))?;
        u64::try_from(raw_id)
            .map_err(|_| ModelError::Internal("corrupt session document".to_owned()))
    }

    /// Marks an existing session as revoked. Missing tokens are ignored so logout is idempotent.
    pub async fn revoke(&self, raw_token: &str) -> Result<(), ModelError> {
        if raw_token.trim().is_empty() {
            return Ok(());
        }
        let token_hash = hash_token(raw_token.trim());
        self.sessions
            .update_one(
                doc! { "_id": token_hash, "revoked_at": { "$exists": false } },
                doc! { "$set": { "revoked_at": DateTime::now() } },
            )
            .await
            .map_err(ModelError::internal)?;
        Ok(())
    }

    /// Revokes every active session belonging to `user_id`.
    pub async fn revoke_for_user(&self, user_id: u64) -> Result<(), ModelError> {
        let uid = i64::try_from(user_id).map_err(ModelError::internal)?;
        self.sessions
            .update_many(
                doc! { "user_id": uid, "revoked_at": { "$exists": false } },
                doc! { "$set": { "revoked_at": DateTime::now() } },
            )
            .await
            .map_err(ModelError::internal)?;
        Ok(())
    }
}
