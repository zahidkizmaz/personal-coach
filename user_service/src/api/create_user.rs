use axum::{Json, extract, http::StatusCode, response::IntoResponse};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::{
    domain::{Gender, User},
    services::users::create_user,
};

use super::app::AppError;

#[derive(Serialize, Deserialize, Debug)]
pub enum GenderPayload {
    Male,
    Female,
}
impl From<Gender> for GenderPayload {
    fn from(value: Gender) -> Self {
        match value {
            Gender::Male => GenderPayload::Male,
            Gender::Female => GenderPayload::Female,
        }
    }
}
impl From<GenderPayload> for Gender {
    fn from(value: GenderPayload) -> Self {
        match value {
            GenderPayload::Male => Gender::Male,
            GenderPayload::Female => Gender::Female,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserPayload {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub age: u8,
    pub weight: Decimal,
    pub height: Decimal,
    pub gender: GenderPayload,
}
impl From<User> for UserPayload {
    fn from(user: User) -> Self {
        Self {
            username: user.username,
            first_name: user.first_name,
            last_name: user.last_name,
            age: user.age,
            weight: user.weight,
            height: user.height,
            gender: user.gender.into(),
        }
    }
}
impl From<UserPayload> for User {
    fn from(val: UserPayload) -> Self {
        User {
            username: val.username,
            first_name: val.first_name,
            last_name: val.last_name,
            age: val.age,
            weight: val.weight,
            height: val.height,
            gender: val.gender.into(),
        }
    }
}

#[tracing::instrument]
pub async fn create(
    extract::Json(payload): Json<UserPayload>,
) -> Result<impl IntoResponse, AppError> {
    let payload = create_user(payload.into()).await?;
    info!("Created user: {:?}", payload);
    Ok((StatusCode::CREATED, Json(UserPayload::from(payload))))
}

#[cfg(test)]
mod tests {
    use crate::api::app;

    use super::*;
    use axum::{http::Request, http::StatusCode};
    use http_body_util::BodyExt;
    use rust_decimal::dec;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_create_user() {
        let user_payload = UserPayload {
            username: "testuser".to_string(),
            first_name: "Test".to_string(),
            last_name: "User".to_string(),
            age: 30,
            weight: dec!(70.0),
            height: dec!(175.0),
            gender: GenderPayload::Male,
        };
        let json_payload = serde_json::to_string(&user_payload).unwrap();
        let response = app()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .header("Content-Type", "application/json")
                    .uri("/api/users/create")
                    .body(json_payload.clone())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);
        let body = response.into_body();
        let bytes = body.collect().await.unwrap().to_bytes();
        let response_payload = String::from_utf8(bytes.to_vec()).unwrap();

        assert_eq!(response_payload, json_payload);
    }
}
