use solana_client::rpc_client::RpcClient;
use solana_sdk::{commitment_config::CommitmentConfig, program_pack::Pack};
use spl_token::state::Mint;
use thiserror::Error;
use tokio::task;
use std::sync::Arc;

use crate::helpers::solana_helper::SolanaHelper;

#[derive(Clone)]
pub struct SolanaRpcClient {
    client: Arc<RpcClient>
}

impl SolanaRpcClient {
    pub fn new(rpc_url: &str, commitment: CommitmentConfig) -> Self {
        let client = RpcClient::new_with_commitment(
            rpc_url, 
            commitment
        );
        
        Self { 
            client: Arc::new(client)
        }
    }

    pub async fn fetch_token_account(
        &self, 
        mint_pubkey_str: &str
    ) -> Result<Mint, SolanaError> {
        let mint_pubkey = SolanaHelper::try_to_convert_str_to_pubkey(mint_pubkey_str)?;

        let client = Arc::clone(&self.client);

        let mint_result = task::spawn_blocking(move || -> Result<Mint, SolanaError> {
            let account = client.get_account(&mint_pubkey).map_err(|e| {
                println!("Error getting account: {}", e);
                SolanaError::AccountFetchError
            })?;

            Mint::unpack(&account.data).map_err(|e| {
                println!("Error parsing mint account: {}", e);
                SolanaError::MintParseError
            })
        })
        .await;

        match mint_result {
            Ok(Ok(mint)) => Ok(mint),
            Ok(Err(e)) => Err(e),
            Err(_) => Err(SolanaError::UnkownError)
        }
    
    }
}

#[derive(Error, Debug)]
pub enum SolanaError {
    #[error("Pubkey could not be parsed")]
    PubkeyParsingError,
    #[error("Mint account could not be parsed")]
    MintParseError,
    #[error("Unknown error occurred")]
    UnkownError,
    #[error("Error Fetching account")]
    AccountFetchError
}