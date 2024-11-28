use axum::http::StatusCode;

pub mod user_controller;
pub mod token_controller;

pub type ApiError = (StatusCode, String);