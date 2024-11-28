use axum::http::StatusCode;
use chrono::Utc;
use uuid::Uuid;

use crate::{
    helpers::solana_helper::SolanaHelper, 
    models::user_model::{
        CreateUserRequest, 
        UserForResponse
    }, 
    repositories::user_repository::UserRepository
};

use super::ApiError;

#[derive(Clone)]
pub struct UserController {
    user_repository: UserRepository
}

impl UserController {
    pub fn new(user_repository: UserRepository) -> Self {
        Self { user_repository }
    }

    pub async fn create_user(
        &self, 
        body: CreateUserRequest
    ) -> Result<UserForResponse, ApiError> {

        let email = body.email;
        let id = Uuid::new_v4();
        let now = Utc::now();
        let (public_key, secret_key) = SolanaHelper::get_keypair();

        match self.user_repository
            .create_user(
                &id,
                &now,
                &email,
                &public_key,
                &secret_key
            )
            .await 
        {
            Ok(user) => Ok(user.into()),
            Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "Error creating user!".to_string()))
        }
    }

    pub async fn fetch_user(
        &self,
        id: Uuid
    ) -> Result<UserForResponse, ApiError> {

        match self.user_repository
            .fetch_user(&id)
            .await 
        {
            Ok(user) => Ok(user.into()),
            Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "Error fetching user!".to_string()))
        }
    }

    pub async fn fetch_all(&self) -> Result<Vec<UserForResponse>, ApiError> {
        match self.user_repository
            .fetch_all_users()
            .await 
        {
            Ok(users) => Ok(users.into_iter().map(|user| user.into()).collect()),
            Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "Error fetching users!".to_string()))
        }
    }
}
