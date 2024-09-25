#![no_std]
use soroban_sdk::{contract, contracttype, contractimpl, log, Env, Symbol, String, symbol_short};

const NFT_OWNER: Symbol = symbol_short!("NFT_O");
const NFT_METADATA: Symbol = symbol_short!("NFT_M");

// Define the NFT structure
#[contracttype]
#[derive(Clone)]
pub struct NFT {
    pub owner: String,
    pub metadata: String,
}

// Define the smart contract
#[contract]
pub struct NFTContract;

#[contractimpl]
impl NFTContract {
    /// This function allows an artist to mint a new NFT.
    /// The NFT includes metadata and the owner's information.
    pub fn mint_nft(env: Env, nft_id: u64, owner: String, metadata: String) {
        let nft = NFT {
            owner: owner.clone(),
            metadata: metadata.clone(),
        };

        // Store NFT metadata and owner in the contract's storage
        env.storage().instance().set(&(NFT_OWNER, nft_id), &owner);
        env.storage().instance().set(&(NFT_METADATA, nft_id), &metadata);

        log!(&env, "NFT minted with ID: {}, Owner: {}, Metadata: {}", nft_id, owner, metadata);
    }

    /// This function allows transferring ownership of an NFT.
    pub fn transfer_nft(env: Env, nft_id: u64, new_owner: String) {
        // Fetch current NFT owner
        let current_owner_opt: Option<String> = env.storage().instance().get(&(NFT_OWNER, nft_id));

        match current_owner_opt {
            Some(current_owner) => {
                // Update NFT owner information
                env.storage().instance().set(&(NFT_OWNER, nft_id), &new_owner);
                log!(&env, "NFT ID: {} ownership transferred from: {} to: {}", nft_id, current_owner, new_owner);
            }
            None => {
                log!(&env, "NFT ID: {} does not exist", nft_id);
                panic!("NFT does not exist");
            }
        }
    }

    /// This function retrieves the metadata and owner of a specific NFT.
    pub fn view_nft(env: Env, nft_id: u64) -> NFT {
        let owner = env.storage().instance().get(&(NFT_OWNER, nft_id)).unwrap_or(String::from_str(&env, ""));
        let metadata = env.storage().instance().get(&(NFT_METADATA, nft_id)).unwrap_or(String::from_str(&env, ""));

        NFT {
            owner,
            metadata,
        }
    }
}

// Working fine
