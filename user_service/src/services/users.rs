use anyhow::Error;

use crate::domain::users::User;

pub fn create_user(user: User) -> Result<User, Error> {
    // Simulate user creation logic
    Ok(user)
}
