use std::collections::BTreeMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Mutex, MutexGuard};

use thiserror::Error;

use crate::models::{Announcement, CreateAnnouncement, UpdateAnnouncement};

#[derive(Debug, Error, PartialEq, Eq)]
pub enum AnnouncementError {
    #[error("{0}")]
    Invalid(&'static str),
    #[error("announcement not found")]
    NotFound,
    #[error("announcement store lock poisoned")]
    Poisoned,
}

/// In-memory announcement model. Ids start at 1 and data resets on restart.
#[derive(Default)]
pub struct AnnouncementStore {
    next_id: AtomicU64,
    announcements: Mutex<BTreeMap<u64, Announcement>>,
}

impl AnnouncementStore {
    fn lock(&self) -> Result<MutexGuard<'_, BTreeMap<u64, Announcement>>, AnnouncementError> {
        self.announcements.lock().map_err(|_| AnnouncementError::Poisoned)
    }

    pub fn create(&self, input: &CreateAnnouncement) -> Result<Announcement, AnnouncementError> {
        validate_field(&input.title, "title must not be empty")?;
        validate_field(&input.body, "body must not be empty")?;
        validate_field(&input.author, "author must not be empty")?;
        let id = self.next_id.fetch_add(1, Ordering::Relaxed) + 1;
        let announcement = Announcement {
            id,
            title: clean(&input.title),
            body: clean(&input.body),
            author: clean(&input.author),
        };
        self.lock()?.insert(id, announcement.clone());
        Ok(announcement)
    }

    pub fn get(&self, id: u64) -> Result<Announcement, AnnouncementError> {
        self.lock()?
            .get(&id)
            .cloned()
            .ok_or(AnnouncementError::NotFound)
    }

    pub fn list(&self) -> Result<Vec<Announcement>, AnnouncementError> {
        Ok(self.lock()?.values().cloned().collect())
    }

    pub fn replace(
        &self,
        id: u64,
        input: &CreateAnnouncement,
    ) -> Result<Announcement, AnnouncementError> {
        validate_field(&input.title, "title must not be empty")?;
        validate_field(&input.body, "body must not be empty")?;
        validate_field(&input.author, "author must not be empty")?;
        let mut announcements = self.lock()?;
        let announcement = announcements
            .get_mut(&id)
            .ok_or(AnnouncementError::NotFound)?;
        announcement.title = clean(&input.title);
        announcement.body = clean(&input.body);
        announcement.author = clean(&input.author);
        Ok(announcement.clone())
    }

    pub fn update(
        &self,
        id: u64,
        input: UpdateAnnouncement,
    ) -> Result<Announcement, AnnouncementError> {
        if let Some(title) = &input.title {
            validate_field(title, "title must not be empty")?;
        }
        if let Some(body) = &input.body {
            validate_field(body, "body must not be empty")?;
        }
        if let Some(author) = &input.author {
            validate_field(author, "author must not be empty")?;
        }
        let mut announcements = self.lock()?;
        let announcement = announcements
            .get_mut(&id)
            .ok_or(AnnouncementError::NotFound)?;
        if let Some(title) = input.title {
            announcement.title = clean(&title);
        }
        if let Some(body) = input.body {
            announcement.body = clean(&body);
        }
        if let Some(author) = input.author {
            announcement.author = clean(&author);
        }
        Ok(announcement.clone())
    }

    pub fn delete(&self, id: u64) -> Result<(), AnnouncementError> {
        self.lock()?
            .remove(&id)
            .map(drop)
            .ok_or(AnnouncementError::NotFound)
    }
}

fn validate_field(value: &str, error: &'static str) -> Result<(), AnnouncementError> {
    if value.trim().is_empty() {
        return Err(AnnouncementError::Invalid(error));
    }
    Ok(())
}

fn clean(value: &str) -> String {
    value.trim().to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_announcement(title: &str, body: &str, author: &str) -> CreateAnnouncement {
        CreateAnnouncement {
            title: title.to_owned(),
            body: body.to_owned(),
            author: author.to_owned(),
        }
    }

    #[test]
    fn create_assigns_sequential_ids_and_trims() {
        let store = AnnouncementStore::default();
        let first = store
            .create(&new_announcement("  Reunion  ", "  Save the date  ", " Ada "))
            .unwrap();
        let second = store
            .create(&new_announcement("Picnic", "Bring food", "Grace"))
            .unwrap();
        assert_eq!(first.id, 1);
        assert_eq!(first.title, "Reunion");
        assert_eq!(first.body, "Save the date");
        assert_eq!(first.author, "Ada");
        assert_eq!(second.id, 2);
    }

    #[test]
    fn get_and_list_return_stored_announcements_in_id_order() {
        let store = AnnouncementStore::default();
        store.create(&new_announcement("first", "one", "Ada")).unwrap();
        store
            .create(&new_announcement("second", "two", "Grace"))
            .unwrap();
        assert_eq!(store.get(2).unwrap().title, "second");
        let titles: Vec<_> = store
            .list()
            .unwrap()
            .into_iter()
            .map(|a| a.title)
            .collect();
        assert_eq!(titles, ["first", "second"]);
        assert_eq!(
            store.get(99).unwrap_err(),
            AnnouncementError::NotFound
        );
    }

    #[test]
    fn replace_overwrites_all_fields() {
        let store = AnnouncementStore::default();
        store
            .create(&new_announcement("Old", "Old body", "Ada"))
            .unwrap();
        let announcement = store
            .replace(1, &new_announcement("New", "New body", "Grace"))
            .unwrap();
        assert_eq!(
            (
                announcement.title.as_str(),
                announcement.body.as_str(),
                announcement.author.as_str()
            ),
            ("New", "New body", "Grace")
        );
        assert_eq!(
            store
                .replace(9, &new_announcement("x", "y", "z"))
                .unwrap_err(),
            AnnouncementError::NotFound
        );
    }

    #[test]
    fn update_changes_only_provided_fields() {
        let store = AnnouncementStore::default();
        store
            .create(&new_announcement("Title", "Body", "Ada"))
            .unwrap();
        let patch = UpdateAnnouncement {
            title: Some("Patched".to_owned()),
            body: None,
            author: None,
        };
        let announcement = store.update(1, patch).unwrap();
        assert_eq!(announcement.title, "Patched");
        assert_eq!(announcement.body, "Body");
        assert_eq!(announcement.author, "Ada");
        assert_eq!(
            store.update(9, UpdateAnnouncement::default()).unwrap_err(),
            AnnouncementError::NotFound
        );
    }

    #[test]
    fn delete_removes_announcement_once() {
        let store = AnnouncementStore::default();
        store
            .create(&new_announcement("Title", "Body", "Ada"))
            .unwrap();
        assert_eq!(store.delete(1), Ok(()));
        assert_eq!(store.delete(1), Err(AnnouncementError::NotFound));
        assert!(store.list().unwrap().is_empty());
    }

    #[test]
    fn invalid_input_is_rejected_without_consuming_an_id() {
        let store = AnnouncementStore::default();
        for (title, body, author) in [("", "body", "Ada"), ("Title", "", "Ada"), ("Title", "body", "")] {
            assert!(matches!(
                store.create(&new_announcement(title, body, author)),
                Err(AnnouncementError::Invalid(_))
            ));
        }
        assert_eq!(
            store
                .create(&new_announcement("Title", "Body", "Ada"))
                .unwrap()
                .id,
            1
        );
        let bad = UpdateAnnouncement {
            title: Some("  ".to_owned()),
            body: None,
            author: None,
        };
        assert!(matches!(store.update(1, bad), Err(AnnouncementError::Invalid(_))));
    }
}
