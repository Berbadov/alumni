use mongodb::bson::{DateTime, Document, doc};
use futures_util::TryStreamExt;
use mongodb::error::{ErrorKind, WriteFailure};
use mongodb::options::ReturnDocument;
use mongodb::{Collection, Database};

use crate::models::{
    Alumni, CreateAlumni, ModelError, UpdateAlumni, clean_email, clean_name, validate_degree,
    validate_email, validate_full_name, validate_graduation_year,
};

fn write_error(err: mongodb::error::Error) -> ModelError {
    if let &ErrorKind::Write(WriteFailure::WriteError(ref failure)) = err.kind.as_ref() {
        if failure.code == 11000 {
            return ModelError::Duplicate;
        }
    }
    ModelError::internal(err)
}

#[derive(Debug, Clone)]
pub struct AlumniStore {
    alumni: Collection<Document>,
    announcements: Collection<Document>,
    counters: Collection<Document>,
}

impl AlumniStore {
    pub fn new(db: &Database) -> Self {
        Self {
            alumni: db.collection("alumni"),
            announcements: db.collection("announcements"),
            counters: db.collection("counters"),
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

    pub async fn create(&self, input: &CreateAlumni) -> Result<Alumni, ModelError> {
        validate_full_name(&input.full_name)?;
        validate_email(&input.email)?;
        validate_graduation_year(input.graduation_year)?;
        validate_degree(&input.degree)?;
        let id = self.next_id("alumni").await?;
        let now = DateTime::now();
        self.alumni
            .insert_one(doc! {
                "_id": i64::try_from(id).map_err(ModelError::internal)?,
                "full_name": clean_name(&input.full_name),
                "email": clean_email(&input.email),
                "graduation_year": input.graduation_year,
                "degree": clean_name(&input.degree),
                "created_at": now,
                "updated_at": now,
            })
            .await
            .map_err(write_error)?;
        self.get(id).await
    }

    pub async fn get(&self, id: u64) -> Result<Alumni, ModelError> {
        let key = i64::try_from(id).map_err(ModelError::internal)?;
        let doc = self
            .alumni
            .find_one(doc! { "_id": key })
            .await
            .map_err(ModelError::internal)?;
        doc.map(|doc| Alumni::from_doc(&doc)).transpose()?.ok_or(ModelError::NotFound)
    }

    pub async fn list(&self) -> Result<Vec<Alumni>, ModelError> {
        let mut cursor = self
            .alumni
            .find(doc! {})
            .sort(doc! { "_id": 1 })
            .await
            .map_err(ModelError::internal)?;
        let mut alumni = Vec::new();
        while let Some(doc) = cursor.try_next().await.map_err(ModelError::internal)? {
            alumni.push(Alumni::from_doc(&doc)?);
        }
        Ok(alumni)
    }

    pub async fn replace(&self, id: u64, input: &CreateAlumni) -> Result<Alumni, ModelError> {
        validate_full_name(&input.full_name)?;
        validate_email(&input.email)?;
        validate_graduation_year(input.graduation_year)?;
        validate_degree(&input.degree)?;
        let key = i64::try_from(id).map_err(ModelError::internal)?;
        let updated = self
            .alumni
            .update_one(
                doc! { "_id": key },
                doc! { "$set": {
                    "full_name": clean_name(&input.full_name),
                    "email": clean_email(&input.email),
                    "graduation_year": input.graduation_year,
                    "degree": clean_name(&input.degree),
                    "updated_at": DateTime::now(),
                } },
            )
            .await
            .map_err(write_error)?;
        if updated.matched_count == 0 {
            return Err(ModelError::NotFound);
        }
        self.get(id).await
    }

    pub async fn update(&self, id: u64, input: UpdateAlumni) -> Result<Alumni, ModelError> {
        let mut set = Document::new();
        if let Some(name) = &input.full_name {
            validate_full_name(name)?;
            set.insert("full_name", clean_name(name));
        }
        if let Some(email) = &input.email {
            validate_email(email)?;
            set.insert("email", clean_email(email));
        }
        if let Some(year) = input.graduation_year {
            validate_graduation_year(year)?;
            set.insert("graduation_year", year);
        }
        if let Some(degree) = &input.degree {
            validate_degree(degree)?;
            set.insert("degree", clean_name(degree));
        }
        if set.is_empty() {
            return self.get(id).await;
        }
        set.insert("updated_at", DateTime::now());
        let key = i64::try_from(id).map_err(ModelError::internal)?;
        let updated = self
            .alumni
            .update_one(doc! { "_id": key }, doc! { "$set": set })
            .await
            .map_err(write_error)?;
        if updated.matched_count == 0 {
            return Err(ModelError::NotFound);
        }
        self.get(id).await
    }

    pub async fn delete(&self, id: u64) -> Result<(), ModelError> {
        let key = i64::try_from(id).map_err(ModelError::internal)?;
        let used = self
            .announcements
            .count_documents(doc! { "author_id": key })
            .await
            .map_err(ModelError::internal)?;
        if used > 0 {
            return Err(ModelError::Blocked("alumni has announcements"));
        }
        let deleted = self
            .alumni
            .delete_one(doc! { "_id": key })
            .await
            .map_err(ModelError::internal)?;
        if deleted.deleted_count == 0 {
            return Err(ModelError::NotFound);
        }
        Ok(())
    }
}
