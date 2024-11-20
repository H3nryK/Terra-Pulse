use candid::{CandidType, Deserialize, Principal};
use ic-cdk::export::{
    candid::export_service,
    Principal as ExportPrincipal,
};
use ic-cdk_macros::*;
use std::collections::HashMap;
use std::cell::RefCell;

// Types
#[derive(CandidType, Deserialize, Clone)]
struct User {
    principal_id: Principal,
    username: String,
    adopted_nfts: Vec<String>,
    rewards_points: u64,
}

#[derive(CandidType, Deserialize, Clone)]
struct NFT {
    id: String,
    entity_type: EntityType,
    metadata: NFTMetadata,
    owner: Option<Principal>,
    price: u64,
}

#[derive(CandidType, Deserialize, Clone)]
struct NFTMetadata {
    name: String,
    description: String,
    image_url: String,
    conservation_status: String,
    location: String,
}

#[derive(CandidType, Deserialize, Clone)]
enum EntityType {
    Wildlife,
    Hotel,
    Reserve,
}

// State management
thread_local! {
    static USERS: RefCell<HashMap<Principal, User>> = RefCell::new(HashMap::new());
    static NFTS: RefCell<HashMap<String, NFT>> = RefCell::new(HashMap::new());
    static MARKETPLACE_LISTINGS: RefCell<HashMap<String, u64>> = RefCell::new(HashMap::new());
}

// Authentication
#[update]
fn register_user(username: String) -> Result<(), String> {
    let caller = ic_cdk::caller();
    USERS.with(|users| {
        if users.borrow().contains_key(&caller) {
            return Err("User already registered".to_string());
        }
        
        let new_user = User {
            principal_id: caller,
            username,
            adopted_nfts: Vec::new(),
            rewards_points: 0,
        };
        
        users.borrow_mut().insert(caller, new_user);
        Ok(())
    })
}

// NFT Management
#[update]
fn mint_nft(metadata: NFTMetadata, entity_type: EntityType, price: u64) -> Result<String, String> {
    let caller = ic_cdk::caller();
    let nft_id = generate_nft_id();
    
    let nft = NFT {
        id: nft_id.clone(),
        entity_type,
        metadata,
        owner: Some(caller),
        price,
    };
    
    NFTS.with(|nfts| {
        nfts.borrow_mut().insert(nft_id.clone(), nft);
    });
    
    Ok(nft_id)
}

#[query]
fn get_nft(nft_id: String) -> Option<NFT> {
    NFTS.with(|nfts| {
        nfts.borrow().get(&nft_id).cloned()
    })
}

// Marketplace
#[update]
fn list_nft_for_sale(nft_id: String, price: u64) -> Result<(), String> {
    let caller = ic_cdk::caller();
    
    NFTS.with(|nfts| {
        let mut nfts = nfts.borrow_mut();
        if let Some(nft) = nfts.get_mut(&nft_id) {
            if nft.owner != Some(caller) {
                return Err("Not the owner".to_string());
            }
            nft.price = price;
            MARKETPLACE_LISTINGS.with(|listings| {
                listings.borrow_mut().insert(nft_id, price);
            });
            Ok(())
        } else {
            Err("NFT not found".to_string())
        }
    })
}

#[update]
async fn purchase_nft(nft_id: String) -> Result<(), String> {
    let buyer = ic_cdk::caller();
    
    NFTS.with(|nfts| {
        let mut nfts = nfts.borrow_mut();
        if let Some(nft) = nfts.get_mut(&nft_id) {
            // Handle payment transfer (simplified)
            nft.owner = Some(buyer);
            MARKETPLACE_LISTINGS.with(|listings| {
                listings.borrow_mut().remove(&nft_id);
            });
            
            // Update user's adopted NFTs
            USERS.with(|users| {
                if let Some(user) = users.borrow_mut().get_mut(&buyer) {
                    user.adopted_nfts.push(nft_id.clone());
                    user.rewards_points += 100; // Reward points for adoption
                }
            });
            
            Ok(())
        } else {
            Err("NFT not found".to_string())
        }
    })
}

// Helper Functions
fn generate_nft_id() -> String {
    let timestamp = ic_cdk::api::time();
    format!("NFT-{}", timestamp)
}

// Query Methods
#[query]
fn get_user_profile(principal_id: Principal) -> Option<User> {
    USERS.with(|users| {
        users.borrow().get(&principal_id).cloned()
    })
}

#[query]
fn get_marketplace_listings() -> Vec<(String, u64)> {
    MARKETPLACE_LISTINGS.with(|listings| {
        listings.borrow().iter()
            .map(|(id, price)| (id.clone(), *price))
            .collect()
    })
}

// System Candid interface
#[query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    export_service!();
    __export_service()
}

// Initialize canister
#[init]
fn init() {
    // Initialize state if needed
}