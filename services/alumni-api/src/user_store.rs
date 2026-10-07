use std::collections::BTreeMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Mutex, MutexGuard};

use thiserror::Error;

use crate::models::{CreateUser, UpdateUser, User};

#[derive(Debug, Error, PartialEq, Eq)]
pub enum UserError {
    #[error("{0}")]
    Invalid(&'static str),
    #[error("user not found")]
    NotFound,
    #[error("user store lock poisoned")]
    Poisoned,
}

/// In-memory user model. Ids start at 1 and data resets on restart.
#[derive(Default)]
pub struct UserStore {
    next_id: AtomicU64,
    users: Mutex<BTreeMap<u64, User>>,
}

impl UserStore {
    fn lock(&self) -> Result<MutexGuard<'_, BTreeMap<u64, User>>, UserError> {
        self.users.lock().map_err(|_| UserError::Poisoned)
    }

    pub fn create(&self, input: &CreateUser) -> Result<User, UserError> {
        validate_name(&input.name)?;
        validate_email(&input.email)?;
        let id = self.next_id.fetch_add(1, Ordering::Relaxed) + 1;
        let user = User {
            id,
            name: clean_name(&input.name),
            email: clean_email(&input.email),
        };
        self.lock()?.insert(id, user.clone());
        Ok(user)
    }

    pub fn get(&self, id: u64) -> Result<User, UserError> {
        self.lock()?.get(&id).cloned().ok_or(UserError::NotFound)
    }

    pub fn list(&self) -> Result<Vec<User>, UserError> {
        Ok(self.lock()?.values().cloned().collect())
    }

    pub fn replace(&self, id: u64, input: &CreateUser) -> Result<User, UserError> {
        validate_name(&input.name)?;
        validate_email(&input.email)?;
        let mut users = self.lock()?;
        let user = users.get_mut(&id).ok_or(UserError::NotFound)?;
        user.name = clean_name(&input.name);
        user.email = clean_email(&input.email);
        Ok(user.clone())
    }

    pub fn update(&self, id: u64, input: UpdateUser) -> Result<User, UserError> {
        if let Some(name) = &input.name {
            validate_name(name)?;
        }
        if let Some(email) = &input.email {
            validate_email(email)?;
        }
        let mut users = self.lock()?;
        let user = users.get_mut(&id).ok_or(UserError::NotFound)?;
        if let Some(name) = input.name {
            user.name = clean_name(&name);
        }
        if let Some(email) = input.email {
            user.email = clean_email(&email);
        }
        Ok(user.clone())
    }

    pub fn delete(&self, id: u64) -> Result<(), UserError> {
        self.lock()?
            .remove(&id)
            .map(drop)
            .ok_or(UserError::NotFound)
    }
}

fn validate_name(name: &str) -> Result<(), UserError> {
    if name.trim().is_empty() {
        return Err(UserError::Invalid("name must not be empty"));
    }
    Ok(())
}

fn validate_email(email: &str) -> Result<(), UserError> {
    let Some((local, domain)) = email.trim().split_once('@') else {
        return Err(UserError::Invalid("email must contain '@'"));
    };
    if local.is_empty() || domain.is_empty() {
        return Err(UserError::Invalid("invalid email format"));
    }
    Ok(())
}

fn clean_name(name: &str) -> String {
    name.trim().to_owned()
}

fn clean_email(email: &str) -> String {
    email.trim().to_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_user(name: &str, email: &str) -> CreateUser {
        CreateUser {
            name: name.to_owned(),
            email: email.to_owned(),
        }
    }

    #[test]
    fn create_assigns_sequential_ids_and_normalizes() {
        let store = UserStore::default();
        let first = store
            .create(&new_user("  Ada Lovelace  ", "Ada@Example.com"))
            .unwrap();
        let second = store
            .create(&new_user("Grace", "grace@example.com"))
            .unwrap();
        assert_eq!(first.id, 1);
        assert_eq!(first.name, "Ada Lovelace");
        assert_eq!(first.email, "ada@example.com");
        assert_eq!(second.id, 2);
    }

    #[test]
    fn get_and_list_return_stored_users_in_id_order() {
        let store = UserStore::default();
        store
            .create(&new_user("first", "first@example.com"))
            .unwrap();
        store
            .create(&new_user("second", "second@example.com"))
            .unwrap();
        assert_eq!(store.get(2).unwrap().name, "second");
        let names: Vec<_> = store.list().unwrap().into_iter().map(|u| u.name).collect();
        assert_eq!(names, ["first", "second"]);
        assert_eq!(store.get(99).unwrap_err(), UserError::NotFound);
    }

    #[test]
    fn replace_overwrites_all_fields() {
        let store = UserStore::default();
        store.create(&new_user("Ada", "ada@example.com")).unwrap();
        let user = store
            .replace(1, &new_user("Ada King", "King@Math.org"))
            .unwrap();
        assert_eq!(
            (user.name.as_str(), user.email.as_str()),
            ("Ada King", "king@math.org")
        );
        assert_eq!(
            store.replace(9, &new_user("x", "x@y.z")).unwrap_err(),
            UserError::NotFound
        );
    }

    #[test]
    fn update_changes_only_provided_fields() {
        let store = UserStore::default();
        store.create(&new_user("Ada", "ada@example.com")).unwrap();
        let patch = UpdateUser {
            name: Some("Ada King".to_owned()),
            email: None,
        };
        let user = store.update(1, patch).unwrap();
        assert_eq!(user.name, "Ada King");
        assert_eq!(user.email, "ada@example.com");
        assert_eq!(
            store.update(9, UpdateUser::default()).unwrap_err(),
            UserError::NotFound
        );
    }

    #[test]
    fn delete_removes_user_once() {
        let store = UserStore::default();
        store.create(&new_user("Ada", "ada@example.com")).unwrap();
        assert_eq!(store.delete(1), Ok(()));
        assert_eq!(store.delete(1), Err(UserError::NotFound));
        assert!(store.list().unwrap().is_empty());
    }

    #[test]
    fn invalid_input_is_rejected_without_consuming_an_id() {
        let store = UserStore::default();
        for (name, email) in [
            ("", "a@example.com"),
            ("Ada", "nope"),
            ("Ada", "@example.com"),
        ] {
            assert!(matches!(
                store.create(&new_user(name, email)),
                Err(UserError::Invalid(_))
            ));
        }
        assert_eq!(
            store
                .create(&new_user("Ada", "ada@example.com"))
                .unwrap()
                .id,
            1
        );
        let bad = UpdateUser {
            name: Some("  ".to_owned()),
            email: None,
        };
        assert!(matches!(store.update(1, bad), Err(UserError::Invalid(_))));
    }
}
