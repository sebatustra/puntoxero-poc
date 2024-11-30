use serde::Serialize;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig, 
    instruction::Instruction, 
    message::Message, 
    program_pack::Pack, 
    pubkey::Pubkey, 
    signature::{
        Keypair, 
        Signature
    }, 
    signer::Signer, 
    system_instruction::create_account, 
    transaction::Transaction
};
use spl_associated_token_account::{
    get_associated_token_address, 
    instruction::create_associated_token_account
};
use spl_token::{state::Mint, instruction::{
    initialize_mint, 
    mint_to
}};
use thiserror::Error;
use tokio::task;
use std::sync::Arc;

use crate::helpers::solana_helper::SolanaHelper;

#[derive(Clone)]
pub struct SolanaRpcClient {
    client: Arc<RpcClient>,
    keypair: Arc<Keypair>,
}

impl SolanaRpcClient {
    pub fn new(
        rpc_url: &str, 
        commitment: CommitmentConfig,
        keypair_base58_string: &str,
    ) -> Self {
        let client = RpcClient::new_with_commitment(
            rpc_url, 
            commitment
        );

        let keypair = Keypair::from_base58_string(&keypair_base58_string);
        
        Self { 
            client: Arc::new(client),
            keypair: Arc::new(keypair)
        }
    }

    pub async fn fetch_token_account(
        &self, 
        mint_pubkey_str: &str
    ) -> Result<MintResponse, SolanaError> {
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
        }).await;

        match mint_result {
            Ok(Ok(mint)) => {
                let mint_response = MintResponse {
                    pubkey: mint_pubkey_str.to_string(),
                    supply: mint.supply,
                    decimals: mint.decimals
                };

                Ok(mint_response)
            },
            Ok(Err(e)) => Err(e),
            Err(_) => Err(SolanaError::UnkownError)
        }
    
    }

    pub async fn create_token_mint(
        &self
    ) -> Result<CreateMintResponse, SolanaError> {
        let payer = self.keypair.clone();
        let client = Arc::clone(&self.client);
        let mint_keypair = Keypair::new();

        let mint_pubkey = mint_keypair.pubkey();
        
        let task_result = task::spawn_blocking(move || -> Result<Signature, SolanaError> {
            let lamports = client
                .get_minimum_balance_for_rent_exemption(Mint::LEN)
                .map_err(|e| {
                    println!("Error getting minimum balance: {}", e);
                    SolanaError::GetMinimumBalanceError
                })?;
    
            let create_account_instruction = create_account(
                &payer.pubkey(), 
                &mint_pubkey, 
                lamports, 
                Mint::LEN as u64, 
                &spl_token::ID
            );
    
            let initialize_mint_instruction = initialize_mint(
                &spl_token::ID,
                &mint_pubkey, 
                &payer.pubkey(), 
                Some(&payer.pubkey()), 
                6
            ).map_err(|e| {
                println!("Error creating initialize_mint instruction: {}", e);
                SolanaError::CreateInstructionError
            })?;
    
            let message = Message::new(
                &[create_account_instruction, initialize_mint_instruction], 
                Some(&payer.pubkey())
            );
    
            let recent_blockhash = client.get_latest_blockhash().map_err(|e| {
                println!("Error getting latest blockhash: {}", e);
                SolanaError::GetBlockhashError
            })?;
    
            let transaction = Transaction::new(
                &[&payer, &mint_keypair], 
                message, 
                recent_blockhash
            );

            client
                .send_and_confirm_transaction(&transaction)
                .map_err(|e| {
                    println!("Error sending transaction: {}", e);
                    SolanaError::SendTransactionError
                })
        }).await;

        match task_result {
            Ok(Ok(signature)) => {
                let create_mint_response = CreateMintResponse {
                    pubkey: mint_pubkey.to_string(),
                    signature: signature.to_string()
                };
        
                Ok(create_mint_response)
            },
            Ok(Err(e)) => Err(e),
            Err(_) => Err(SolanaError::UnkownError)
        }
    }

    pub async fn mint_token_to(
        &self,
        mint_pubkey_str: &str,
        receiver_pubkey_str: &str,
        amount: u64
    ) -> Result<MintToResponse, SolanaError> {
        let mint_pubkey = SolanaHelper::try_to_convert_str_to_pubkey(mint_pubkey_str)?;
        let receiver_pubkey = SolanaHelper::try_to_convert_str_to_pubkey(receiver_pubkey_str)?;
        let payer = self.keypair.clone();

        let client = Arc::clone(&self.client);

        let task_result = task::spawn_blocking(move || -> Result<Signature, SolanaError> {
            let ata = Self::get_and_verify_ata(
                &client, 
                &receiver_pubkey, 
                &mint_pubkey
            )?;
    
            let mut instructions: Vec<Instruction> = Vec::new();
    
            if !ata.is_created {
                let create_ata_instruction = create_associated_token_account(
                    &payer.pubkey(), 
                    &receiver_pubkey, 
                    &mint_pubkey, 
                    &spl_token::ID
                );
    
                instructions.push(create_ata_instruction);
            }
    
            let mint_to_instruction = mint_to(
                &spl_token::ID, 
                &mint_pubkey, 
                &ata.ata_pubkey, 
                &payer.pubkey(), 
                &[], 
                amount
            ).map_err(|e| {
                println!("Error creating mint_to instruction: {}", e);
                SolanaError::CreateInstructionError
            })?;
    
            instructions.push(mint_to_instruction);
    
            let recent_blockhash = client.get_latest_blockhash().map_err(|e| {
                println!("Error getting latest blockhash: {}", e);
                SolanaError::GetBlockhashError
            })?;
    
    
            let message = Message::new(
                &instructions, 
                Some(&payer.pubkey())
            );
    
            let transaction = Transaction::new(
                &[payer], 
                message, 
                recent_blockhash
            );
    
            client
                .send_and_confirm_transaction(&transaction)
                .map_err(|e| {
                    println!("Error sending transaction: {}", e);
                    SolanaError::SendTransactionError
                })

        }).await;

        match task_result {
            Ok(Ok(signature)) => {
                let mint_to_response = MintToResponse {
                    signature: signature.to_string()
                };
        
                Ok(mint_to_response)
            },
            Ok(Err(e)) => Err(e),
            Err(_) => Err(SolanaError::UnkownError)
        }
      
    }

    fn get_and_verify_ata(
        rpc_client: &RpcClient,
        wallet_pubkey: &Pubkey,
        mint_pubkey: &Pubkey
    ) -> Result<VerifyAndGetAtaResponse, SolanaError> {
        let ata_address = get_associated_token_address(
            wallet_pubkey, 
            mint_pubkey
        );

        let result = rpc_client.get_account_with_commitment(&ata_address, rpc_client.commitment())
            .map_err(|e| {
                println!("Error in get_account_with_commitment: {}", e);
                SolanaError::AccountFetchError
            })?;
        
        match result.value {
            Some(account) => {
                if account.owner == spl_token::ID {
                    Ok(VerifyAndGetAtaResponse {
                        ata_pubkey: ata_address,
                        is_created: true
                    })
                } else {
                    Err(SolanaError::AtaOwnerError)
                }
            },
            None => {
                Ok(VerifyAndGetAtaResponse {
                    ata_pubkey: ata_address,
                    is_created: false
                })
            }
        }
    }
}

#[derive(Debug)]
pub struct VerifyAndGetAtaResponse {
    pub ata_pubkey: Pubkey,
    pub is_created: bool
}

#[derive(Serialize, Debug)]
pub struct CreateMintResponse {
    pub pubkey: String,
    pub signature: String
}

#[derive(Serialize, Debug)]
pub struct MintToResponse {
    pub signature: String
}

#[derive(Serialize, Debug)]
pub struct MintResponse {
    pub pubkey: String,
    pub supply: u64,
    pub decimals: u8
}

#[derive(Error, Debug)]
pub enum SolanaError {
    #[error("Pubkey could not be parsed")]
    PubkeyParsingError,
    #[error("Mint account could not be parsed")]
    MintParseError,
    #[error("Unknown error occurred")]
    UnkownError,
    #[error("Error fetching account")]
    AccountFetchError,
    #[error("Error creating instruction")]
    CreateInstructionError,
    #[error("Error getting latest blockhash")]
    GetBlockhashError,
    #[error("Error sending transaction")]
    SendTransactionError,
    #[error("Error getting minimum balance")]
    GetMinimumBalanceError,
    #[error("ATA is not owned by spl program")]
    AtaOwnerError
}