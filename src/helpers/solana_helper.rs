use solana_sdk::{self, pubkey::Pubkey, signature::Keypair, signer::Signer};

use crate::clients::solana_rpc_client::SolanaError;

pub struct SolanaHelper;

impl SolanaHelper {
    pub fn get_keypair() -> (String, String) {
        let keypair = Keypair::new();

        (
            keypair.pubkey().to_string(),
            keypair.to_base58_string()
        )
    }

    pub fn try_to_convert_str_to_pubkey(pubkey_str: &str) -> Result<Pubkey, SolanaError> {
        match pubkey_str.parse::<Pubkey>() {
            Ok(pubkey) => Ok(pubkey),
            Err(e) => {
                println!("Error parsing into Pubkey: {}", e);
                Err(SolanaError::PubkeyParsingError)
            }
        }
    }
}