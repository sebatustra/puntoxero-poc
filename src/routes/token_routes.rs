use axum::{extract::{Path, State}, routing::{get, post}, Json, Router};
use serde::Deserialize;
use crate::{clients::solana_rpc_client::{CreateMintResponse, MintResponse, MintToResponse}, controllers::{token_controller::TokenController, ApiError}};

pub fn token_routes(token_controller: TokenController) -> Router {
    Router::new()
        .route("/mint", post(create_mint))
        .route("/mint/mint_to", post(mint_to))
        .route("/mint/:pubkey", get(get_mint_account))
        .with_state(token_controller)
}

async fn get_mint_account(
    State(token_controller): State<TokenController>,
    Path(pubkey): Path<String>
) -> Result<Json<MintResponse>, ApiError> {
    let mint = token_controller.get_token_account(&pubkey).await?;

    Ok(Json(mint))
}

async fn create_mint(
    State(token_controller): State<TokenController>
) -> Result<Json<CreateMintResponse>, ApiError> {
    let mint = token_controller.create_mint().await?;

    Ok(Json(mint))
}

#[derive(Deserialize)]
struct MintToRequest {
    mint_pubkey: String,
    receiver_pubkey: String,
    amount: u64
}

async fn mint_to(
    State(token_controller): State<TokenController>,
    Json(payload): Json<MintToRequest>
) -> Result<Json<MintToResponse>, ApiError> {
    let mint_pubkey_str = payload.mint_pubkey;
    let receiver_pubkey_str = payload.receiver_pubkey;
    let amount = payload.amount;

    let signature = token_controller.mint_to(
        &mint_pubkey_str, 
        &receiver_pubkey_str, 
        amount
    ).await?;

    Ok(Json(signature))
}