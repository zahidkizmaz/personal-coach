use rust_decimal::Decimal;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Gender {
    Male,
    Female,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub age: u8,
    pub weight: Decimal,
    pub height: Decimal,
    pub gender: Gender,
}
