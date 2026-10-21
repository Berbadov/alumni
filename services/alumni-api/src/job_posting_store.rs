use futures_util::TryStreamExt;
use mongodb::bson::{DateTime, Document, doc};
use mongodb::options::ReturnDocument;
use mongodb::{Collection, Database};

use crate::models::{
    CreateJobPosting, JobPosting, ModelError, UpdateJobPosting, clean_name, validate_apply_url,
    validate_company, validate_description, validate_employment_type, validate_location,
    validate_title,
};

#[derive(Debug, Clone)]
pub struct JobPostingStore {
    alumni: Collection<Document>,
    job_postings: Collection<Document>,
    job_applications: Collection<Document>,
    counters: Collection<Document>,
}

impl JobPostingStore {
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

    async fn check_author(&self, author_id: u64) -> Result<i64, ModelError> {
        let key = i64::try_from(author_id).map_err(ModelError::internal)?;
        let found = self
            .alumni
            .find_one(doc! { "_id": key })
            .await
            .map_err(ModelError::internal)?;
        if found.is_none() {
            return Err(ModelError::Invalid("unknown author_id"));
        }
        Ok(key)
    }

    fn validate(input: &CreateJobPosting) -> Result<(), ModelError> {
        validate_title(&input.title)?;
        validate_company(&input.company)?;
        validate_description(&input.description)?;
        validate_location(&input.location)?;
        validate_employment_type(&input.employment_type)?;
        validate_apply_url(&input.apply_url)
    }

    pub async fn create(&self, input: &CreateJobPosting) -> Result<JobPosting, ModelError> {
        Self::validate(input)?;
        let author = self.check_author(input.author_id).await?;
        let id = self.next_id("job_postings").await?;
        let now = DateTime::now();
        self.job_postings
            .insert_one(doc! {
                "_id": i64::try_from(id).map_err(ModelError::internal)?,
                "title": clean_name(&input.title),
                "company": clean_name(&input.company),
                "description": clean_name(&input.description),
                "location": clean_name(&input.location),
                "employment_type": clean_name(&input.employment_type),
                "apply_url": clean_name(&input.apply_url),
                "author_id": author,
                "is_active": input.is_active.unwrap_or(true),
                "created_at": now,
                "updated_at": now,
            })
            .await
            .map_err(ModelError::internal)?;
        self.get(id).await
    }

    pub async fn get(&self, id: u64) -> Result<JobPosting, ModelError> {
        let key = i64::try_from(id).map_err(ModelError::internal)?;
        let doc = self
            .job_postings
            .find_one(doc! { "_id": key })
            .await
            .map_err(ModelError::internal)?;
        doc.map(|doc| JobPosting::from_doc(&doc))
            .transpose()?
            .ok_or(ModelError::NotFound)
    }

    pub async fn list(&self, active: Option<bool>) -> Result<Vec<JobPosting>, ModelError> {
        let filter = match active {
            Some(value) => doc! { "is_active": value },
            None => Document::new(),
        };
        let mut cursor = self
            .job_postings
            .find(filter)
            .sort(doc! { "_id": 1 })
            .await
            .map_err(ModelError::internal)?;
        let mut postings = Vec::new();
        while let Some(doc) = cursor.try_next().await.map_err(ModelError::internal)? {
            postings.push(JobPosting::from_doc(&doc)?);
        }
        Ok(postings)
    }

    pub async fn replace(
        &self,
        id: u64,
        input: &CreateJobPosting,
    ) -> Result<JobPosting, ModelError> {
        Self::validate(input)?;
        let author = self.check_author(input.author_id).await?;
        let key = i64::try_from(id).map_err(ModelError::internal)?;
        let updated = self
            .job_postings
            .update_one(
                doc! { "_id": key },
                doc! { "$set": {
                    "title": clean_name(&input.title),
                    "company": clean_name(&input.company),
                    "description": clean_name(&input.description),
                    "location": clean_name(&input.location),
                    "employment_type": clean_name(&input.employment_type),
                    "apply_url": clean_name(&input.apply_url),
                    "author_id": author,
                    "is_active": input.is_active.unwrap_or(true),
                    "updated_at": DateTime::now(),
                } },
            )
            .await
            .map_err(ModelError::internal)?;
        if updated.matched_count == 0 {
            return Err(ModelError::NotFound);
        }
        self.get(id).await
    }

    pub async fn update(&self, id: u64, input: UpdateJobPosting) -> Result<JobPosting, ModelError> {
        let mut set = Document::new();
        if let Some(title) = &input.title {
            validate_title(title)?;
            set.insert("title", clean_name(title));
        }
        if let Some(company) = &input.company {
            validate_company(company)?;
            set.insert("company", clean_name(company));
        }
        if let Some(description) = &input.description {
            validate_description(description)?;
            set.insert("description", clean_name(description));
        }
        if let Some(location) = &input.location {
            validate_location(location)?;
            set.insert("location", clean_name(location));
        }
        if let Some(employment_type) = &input.employment_type {
            validate_employment_type(employment_type)?;
            set.insert("employment_type", clean_name(employment_type));
        }
        if let Some(apply_url) = &input.apply_url {
            validate_apply_url(apply_url)?;
            set.insert("apply_url", clean_name(apply_url));
        }
        if let Some(author_id) = input.author_id {
            set.insert("author_id", self.check_author(author_id).await?);
        }
        if let Some(is_active) = input.is_active {
            set.insert("is_active", is_active);
        }
        if set.is_empty() {
            return self.get(id).await;
        }
        set.insert("updated_at", DateTime::now());
        let key = i64::try_from(id).map_err(ModelError::internal)?;
        let updated = self
            .job_postings
            .update_one(doc! { "_id": key }, doc! { "$set": set })
            .await
            .map_err(ModelError::internal)?;
        if updated.matched_count == 0 {
            return Err(ModelError::NotFound);
        }
        self.get(id).await
    }

    pub async fn delete(&self, id: u64) -> Result<(), ModelError> {
        let key = i64::try_from(id).map_err(ModelError::internal)?;
        let used = self
            .job_applications
            .count_documents(doc! { "job_posting_id": key })
            .await
            .map_err(ModelError::internal)?;
        if used > 0 {
            return Err(ModelError::Blocked("job posting has applications"));
        }
        let deleted = self
            .job_postings
            .delete_one(doc! { "_id": key })
            .await
            .map_err(ModelError::internal)?;
        if deleted.deleted_count == 0 {
            return Err(ModelError::NotFound);
        }
        Ok(())
    }
}
