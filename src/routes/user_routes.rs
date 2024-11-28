use axum::{
    extract::{Path, State}, 
    routing::{get, post}, 
    Json, 
    Router
};
use uuid::Uuid;

use crate::{
    controllers::{user_controller::UserController, ApiError}, 
    models::user_model::{
        CreateUserRequest, 
        UserForResponse
    }
};


pub fn user_routes(user_controller: UserController) -> Router {
    Router::new()
        .route("/users", post(create_user).get(fetch_all_users))
        .route("/users/:id", get(fetch_user))
        .with_state(user_controller)
}

async fn create_user(
    State(user_controller): State<UserController>,
    Json(body): Json<CreateUserRequest>,
) -> Result<Json<UserForResponse>, ApiError> {
    println!("creating user");
    let user = user_controller.create_user(body).await?;

    Ok(Json(user))
}

async fn fetch_user(
    State(user_controller): State<UserController>,
    Path(id): Path<Uuid>
) -> Result<Json<UserForResponse>, ApiError> {
    let user = user_controller.fetch_user(id).await?;

    Ok(Json(user))
}

async fn fetch_all_users(
    State(user_controller): State<UserController>,
) -> Result<Json<Vec<UserForResponse>>, ApiError> {
    let users = user_controller.fetch_all().await?;

    Ok(Json(users))
}