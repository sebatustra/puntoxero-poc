use axum::{extract::{Path, State}, routing::get, Json, Router};
use crate::controllers::{token_controller::TokenController, ApiError};
use serde_json::{json, Value};

pub fn token_routes(token_controller: TokenController) -> Router {
    Router::new()
        .route("/mint/:pubkey", get(get_mint_account))
        .with_state(token_controller)
}

async fn get_mint_account(
    State(token_controller): State<TokenController>,
    Path(pubkey): Path<String>
) -> Result<Json<Value>, ApiError> {
    let mint = token_controller.get_token_account(&pubkey).await?;

    let mint_json = json!({
        "supply": mint.supply,
        "decimals": mint.decimals
    });

    Ok(Json(mint_json))
}