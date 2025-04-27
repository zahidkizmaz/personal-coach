use anyhow::Error;

use crate::{
    domain::users::User,
    infrastructure::repository::{FakeUserRepository, UserRepository},
};

pub async fn create_user(user: User) -> Result<User, Error> {
    // Simulate user creation logic
    FakeUserRepository::default().create_user(user).await
}
