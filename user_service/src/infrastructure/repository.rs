use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use anyhow::anyhow;

use crate::domain::User;

#[allow(unused)]
pub trait UserRepository: Send + Sync {
    async fn get_by_username(&self, username: &str) -> anyhow::Result<User>;
    async fn create_user(&self, user: User) -> anyhow::Result<User>;
}

#[derive(Default)]
pub struct FakeUserRepository {
    users: Arc<Mutex<HashMap<String, User>>>,
}

impl UserRepository for FakeUserRepository {
    async fn get_by_username(&self, username: &str) -> anyhow::Result<User> {
        self.users
            .lock()
            .unwrap()
            .get(username)
            .cloned()
            .ok_or_else(|| anyhow!("Failed to find user with username:{}", username))
    }

    async fn create_user(&self, user: User) -> anyhow::Result<User> {
        self.users
            .lock()
            .unwrap()
            .insert(user.username.to_string(), user.clone());
        Ok(user)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_utils::user_factory::create_domain_user;

    #[tokio::test]
    async fn fake_user_repository_creates_user() {
        let user = create_domain_user();
        let repo = FakeUserRepository::default();

        let result_user = repo.create_user(user.clone()).await.unwrap();

        assert_eq!(user, result_user);
    }

    #[tokio::test]
    async fn fake_user_repository_get_user() {
        let user = create_domain_user();
        let repo = FakeUserRepository::default();
        repo.create_user(user.clone()).await.unwrap();

        let result_user = repo.get_by_username(&user.username).await.unwrap();

        assert_eq!(user, result_user);
    }
}
