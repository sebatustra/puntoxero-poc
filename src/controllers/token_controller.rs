use axum::http::StatusCode;
use spl_token::state::Mint;
use crate::clients::solana_rpc_client::SolanaRpcClient;
use super::ApiError;

#[derive(Clone)]
pub struct TokenController {
    solana_rpc_client: SolanaRpcClient
}

impl TokenController {
    pub fn new(solana_rpc_client: SolanaRpcClient) -> Self {
        Self { solana_rpc_client }
    }

    pub async fn get_token_account(
        &self, 
        mint_pubkey: &str
    ) -> Result<Mint, ApiError> {
        let mint = self.solana_rpc_client
            .fetch_token_account(mint_pubkey)
            .await
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Error fetching mint".to_string()))?;

        Ok(mint)
    }
}