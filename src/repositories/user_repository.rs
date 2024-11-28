use sqlx::PgPool;
use thiserror::Error;
use sqlx::Error as SqlxError;
use uuid::Uuid;
use crate::models::user_model::User;
use chrono::{DateTime, Utc};

#[derive(Clone)]
pub struct UserRepository {
    pool: PgPool
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_user(&self, 
        id: &Uuid,
        date: &DateTime<Utc>,
        user_email: &str,
        public_key: &str,
        secret_key: &str,
    ) -> Result<User, UserRepositoryError> {

        match sqlx::query_as::<_, User>("INSERT INTO users (id, email, created_at, public_key, secret_key) VALUES ($1, $2, $3, $4, $5) RETURNING id, email, created_at, public_key, secret_key")
            .bind(id)
            .bind(user_email.to_string())
            .bind(date)
            .bind(public_key)
            .bind(secret_key)
            .fetch_one(&self.pool)
            .await
        {
            Ok(user) => Ok(user),
            Err(e) => {
                println!("error: {}", e);
                Err(UserRepositoryError::DatabaseError(e))
            }
        }
    }

    pub async fn fetch_user(&self, id: &Uuid) -> Result<User, UserRepositoryError> {
        match sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_one(&self.pool)
            .await
        {
            Ok(user) => Ok(user),
            Err(e) => match e {
                SqlxError::RowNotFound => Err(UserRepositoryError::RowNotFound),
                e => Err(UserRepositoryError::DatabaseError(e))
            }
        }
    }

    pub async fn fetch_all_users(&self) -> Result<Vec<User>, UserRepositoryError> {
        match sqlx::query_as::<_, User>("SELECT * FROM users")
            .fetch_all(&self.pool)
            .await
        {
            Ok(users) => Ok(users),
            Err(e) => match e {
                SqlxError::RowNotFound => Err(UserRepositoryError::RowNotFound),
                e => Err(UserRepositoryError::DatabaseError(e))
            }
        }
    }
}

#[derive(Error, Debug)]
pub enum UserRepositoryError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] SqlxError),
    #[error("User was not found")]
    RowNotFound
}
