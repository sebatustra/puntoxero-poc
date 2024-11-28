
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use chrono::{DateTime, Utc};

#[derive(Serialize, FromRow, Deserialize, Debug)]
pub struct User {
    pub id: uuid::Uuid,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub public_key: String,
    pub secret_key: String,
}

#[derive(Serialize, FromRow, Deserialize, Debug)]
pub struct UserForResponse {
    pub id: uuid::Uuid,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub public_key: String
}

impl From<User> for UserForResponse {
    fn from(value: User) -> Self {
        Self {
            id: value.id,
            email: value.email,
            created_at: value.created_at,
            public_key: value.public_key,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct CreateUserRequest {
    pub email: String,
}

