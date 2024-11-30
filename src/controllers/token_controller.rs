use axum::http::StatusCode;
use crate::clients::solana_rpc_client::{CreateMintResponse, MintResponse, MintToResponse, SolanaRpcClient};
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
    ) -> Result<MintResponse, ApiError> {
        let mint = self.solana_rpc_client
            .fetch_token_account(mint_pubkey)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        Ok(mint)
    }

    pub async fn create_mint(
        &self,
    ) -> Result<CreateMintResponse, ApiError> {
        let mint = self.solana_rpc_client
            .create_token_mint()
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        Ok(mint)
    }

    pub async fn mint_to(
        &self,
        mint_pubkey_str: &str,
        receiver_pubkey_str: &str,
        amount: u64
    ) -> Result<MintToResponse, ApiError> {
        let signature = self.solana_rpc_client
            .mint_token_to(
                mint_pubkey_str, 
                receiver_pubkey_str, 
                amount
            )
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        Ok(signature)
    }
}