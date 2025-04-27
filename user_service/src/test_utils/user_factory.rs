use rust_decimal::dec;

use crate::domain::{Gender, User};

#[allow(unused)]
pub fn create_domain_user() -> User {
    User {
        username: "testuser".to_string(),
        first_name: "Test".to_string(),
        last_name: "User".to_string(),
        age: 30,
        weight: dec!(70.0),
        height: dec!(175.0),
        gender: Gender::Male,
    }
}
