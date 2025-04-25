#[derive(Debug)]
pub enum Gender {
    Male,
    Female,
}

#[derive(Debug)]
pub struct User {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub age: u8,
    pub weight: f32,
    pub height: f32,
    pub gender: Gender,
}
