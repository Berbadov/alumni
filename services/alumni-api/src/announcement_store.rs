use mongodb::bson::{DateTime, Document, doc};
use futures_util::TryStreamExt;
use mongodb::options::ReturnDocument;
use mongodb::{Collection, Database};

use crate::models::{
    Announcement, CreateAnnouncement, ModelError, UpdateAnnouncement, clean_name, validate_body,
    validate_title,
};

#[derive(Debug, Clone)]
pub struct AnnouncementStore {
    alumni: Collection<Document>,
    announcements: Collection<Document>,
    counters: Collection<Document>,
}

impl AnnouncementStore {
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

    async fn author_exists(&self, author_id: u64) -> Result<bool, ModelError> {
        let key = i64::try_from(author_id).map_err(ModelError::internal)?;
        let found = self
            .alumni
            .find_one(doc! { "_id": key })
            .await
            .map_err(ModelError::internal)?;
        Ok(found.is_some())
    }

    async fn check_author(&self, author_id: u64) -> Result<i64, ModelError> {
        if !self.author_exists(author_id).await? {
            return Err(ModelError::Invalid("unknown author_id"));
        }
        i64::try_from(author_id).map_err(ModelError::internal)
    }

    pub async fn create(&self, input: &CreateAnnouncement) -> Result<Announcement, ModelError> {
        validate_title(&input.title)?;
        validate_body(&input.body)?;
        let author = self.check_author(input.author_id).await?;
        let id = self.next_id("announcements").await?;
        let now = DateTime::now();
        self.announcements
            .insert_one(doc! {
                "_id": i64::try_from(id).map_err(ModelError::internal)?,
                "title": clean_name(&input.title),
                "body": clean_name(&input.body),
                "author_id": author,
                "created_at": now,
                "updated_at": now,
            })
            .await
            .map_err(ModelError::internal)?;
        self.get(id).await
    }

    pub async fn get(&self, id: u64) -> Result<Announcement, ModelError> {
        let key = i64::try_from(id).map_err(ModelError::internal)?;
        let doc = self
            .announcements
            .find_one(doc! { "_id": key })
            .await
            .map_err(ModelError::internal)?;
        doc.map(|doc| Announcement::from_doc(&doc))
            .transpose()?
            .ok_or(ModelError::NotFound)
    }

    pub async fn list(&self) -> Result<Vec<Announcement>, ModelError> {
        let mut cursor = self
            .announcements
            .find(doc! {})
            .sort(doc! { "_id": 1 })
            .await
            .map_err(ModelError::internal)?;
        let mut announcements = Vec::new();
        while let Some(doc) = cursor.try_next().await.map_err(ModelError::internal)? {
            announcements.push(Announcement::from_doc(&doc)?);
        }
        Ok(announcements)
    }

    pub async fn replace(
        &self,
        id: u64,
        input: &CreateAnnouncement,
    ) -> Result<Announcement, ModelError> {
        validate_title(&input.title)?;
        validate_body(&input.body)?;
        let author = self.check_author(input.author_id).await?;
        let key = i64::try_from(id).map_err(ModelError::internal)?;
        let updated = self
            .announcements
            .update_one(
                doc! { "_id": key },
                doc! { "$set": {
                    "title": clean_name(&input.title),
                    "body": clean_name(&input.body),
                    "author_id": author,
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

    pub async fn update(
        &self,
        id: u64,
        input: UpdateAnnouncement,
    ) -> Result<Announcement, ModelError> {
        let mut set = Document::new();
        if let Some(title) = &input.title {
            validate_title(title)?;
            set.insert("title", clean_name(title));
        }
        if let Some(body) = &input.body {
            validate_body(body)?;
            set.insert("body", clean_name(body));
        }
        if let Some(author_id) = input.author_id {
            set.insert("author_id", self.check_author(author_id).await?);
        }
        if set.is_empty() {
            return self.get(id).await;
        }
        set.insert("updated_at", DateTime::now());
        let key = i64::try_from(id).map_err(ModelError::internal)?;
        let updated = self
            .announcements
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
        let deleted = self
            .announcements
            .delete_one(doc! { "_id": key })
            .await
            .map_err(ModelError::internal)?;
        if deleted.deleted_count == 0 {
            return Err(ModelError::NotFound);
        }
        Ok(())
    }
}
