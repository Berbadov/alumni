use futures_util::TryStreamExt;
use mongodb::bson::{DateTime, Document, doc};
use mongodb::error::{ErrorKind, WriteFailure};
use mongodb::options::ReturnDocument;
use mongodb::{Collection, Database};

use crate::models::{
    CreateJobApplication, DEFAULT_JOB_APPLICATION_STATUS, JobApplication, JobApplicationFilter,
    ModelError, UpdateJobApplication, clean_name, clean_status, validate_status,
};

fn write_error(err: mongodb::error::Error) -> ModelError {
    if let &ErrorKind::Write(WriteFailure::WriteError(ref failure)) = err.kind.as_ref() {
        if failure.code == 11000 {
            return ModelError::Invalid("application already exists");
        }
    }
    ModelError::internal(err)
}

fn status_or_default(value: Option<&str>) -> Result<String, ModelError> {
    let status = value.map_or_else(|| DEFAULT_JOB_APPLICATION_STATUS.to_owned(), clean_status);
    validate_status(&status)?;
    Ok(status)
}

#[derive(Debug, Clone)]
pub struct JobApplicationStore {
    alumni: Collection<Document>,
    job_postings: Collection<Document>,
    job_applications: Collection<Document>,
    counters: Collection<Document>,
}

impl JobApplicationStore {
    pub fn new(db: &Database) -> Self {
        Self {
            alumni: db.collection("alumni"),
            job_postings: db.collection("job_postings"),
            job_applications: db.collection("job_applications"),
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

    async fn check_posting(&self, job_posting_id: u64) -> Result<i64, ModelError> {
        let key = i64::try_from(job_posting_id).map_err(ModelError::internal)?;
        let found = self
            .job_postings
            .find_one(doc! { "_id": key })
            .await
            .map_err(ModelError::internal)?;
        if found.is_none() {
            return Err(ModelError::Invalid("unknown job_posting_id"));
        }
        Ok(key)
    }

    async fn check_alumni(&self, alumni_id: u64) -> Result<i64, ModelError> {
        let key = i64::try_from(alumni_id).map_err(ModelError::internal)?;
        let found = self
            .alumni
            .find_one(doc! { "_id": key })
            .await
            .map_err(ModelError::internal)?;
        if found.is_none() {
            return Err(ModelError::Invalid("unknown alumni_id"));
        }
        Ok(key)
    }

    pub async fn create(&self, input: &CreateJobApplication) -> Result<JobApplication, ModelError> {
        let posting = self.check_posting(input.job_posting_id).await?;
        let alumni = self.check_alumni(input.alumni_id).await?;
        let status = status_or_default(input.status.as_deref())?;
        let id = self.next_id("job_applications").await?;
        let now = DateTime::now();
        self.job_applications
            .insert_one(doc! {
                "_id": i64::try_from(id).map_err(ModelError::internal)?,
                "job_posting_id": posting,
                "alumni_id": alumni,
                "cover_letter": clean_name(&input.cover_letter),
                "status": status,
                "created_at": now,
                "updated_at": now,
            })
            .await
            .map_err(write_error)?;
        self.get(id).await
    }

    pub async fn get(&self, id: u64) -> Result<JobApplication, ModelError> {
        let key = i64::try_from(id).map_err(ModelError::internal)?;
        let doc = self
            .job_applications
            .find_one(doc! { "_id": key })
            .await
            .map_err(ModelError::internal)?;
        doc.map(|doc| JobApplication::from_doc(&doc))
            .transpose()?
            .ok_or(ModelError::NotFound)
    }

    pub async fn list(
        &self,
        filter: &JobApplicationFilter,
    ) -> Result<Vec<JobApplication>, ModelError> {
        let mut query = Document::new();
        if let Some(job_posting_id) = filter.job_posting_id {
            query.insert(
                "job_posting_id",
                i64::try_from(job_posting_id).map_err(ModelError::internal)?,
            );
        }
        if let Some(alumni_id) = filter.alumni_id {
            query.insert(
                "alumni_id",
                i64::try_from(alumni_id).map_err(ModelError::internal)?,
            );
        }
        if let Some(status) = &filter.status {
            validate_status(status)?;
            query.insert("status", clean_status(status));
        }
        let mut cursor = self
            .job_applications
            .find(query)
            .sort(doc! { "_id": 1 })
            .await
            .map_err(ModelError::internal)?;
        let mut applications = Vec::new();
        while let Some(doc) = cursor.try_next().await.map_err(ModelError::internal)? {
            applications.push(JobApplication::from_doc(&doc)?);
        }
        Ok(applications)
    }

    pub async fn replace(
        &self,
        id: u64,
        input: &CreateJobApplication,
    ) -> Result<JobApplication, ModelError> {
        let posting = self.check_posting(input.job_posting_id).await?;
        let alumni = self.check_alumni(input.alumni_id).await?;
        let status = status_or_default(input.status.as_deref())?;
        let key = i64::try_from(id).map_err(ModelError::internal)?;
        let updated = self
            .job_applications
            .update_one(
                doc! { "_id": key },
                doc! { "$set": {
                    "job_posting_id": posting,
                    "alumni_id": alumni,
                    "cover_letter": clean_name(&input.cover_letter),
                    "status": status,
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

    pub async fn update(
        &self,
        id: u64,
        input: UpdateJobApplication,
    ) -> Result<JobApplication, ModelError> {
        let mut set = Document::new();
        if let Some(job_posting_id) = input.job_posting_id {
            set.insert("job_posting_id", self.check_posting(job_posting_id).await?);
        }
        if let Some(alumni_id) = input.alumni_id {
            set.insert("alumni_id", self.check_alumni(alumni_id).await?);
        }
        if let Some(cover_letter) = &input.cover_letter {
            set.insert("cover_letter", clean_name(cover_letter));
        }
        if let Some(status) = &input.status {
            validate_status(status)?;
            set.insert("status", clean_status(status));
        }
        if set.is_empty() {
            return self.get(id).await;
        }
        set.insert("updated_at", DateTime::now());
        let key = i64::try_from(id).map_err(ModelError::internal)?;
        let updated = self
            .job_applications
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
        let deleted = self
            .job_applications
            .delete_one(doc! { "_id": key })
            .await
            .map_err(ModelError::internal)?;
        if deleted.deleted_count == 0 {
            return Err(ModelError::NotFound);
        }
        Ok(())
    }
}
