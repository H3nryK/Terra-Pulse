use candid::Principal;
use ic_cdk_macros::*;
mod types;
mod error;
mod state;

use crate::types::*;
use crate::error::*;

// Authentication
#[update]
async fn register_user(username: String, email: Option<String>) -> Result<Principal> {
    let caller = ic_cdk::caller();
    if caller == Principal::anonymous() {
        return Err(TerraPulseError::NotAuthorized);
    }

    state::USERS.with(|users| {
        if users.borrow().contains_key(&caller) {
            return Err(TerraPulseError::InvalidOperation("User already exists".into()));
        }

        let new_user = User {
            principal_id: caller,
            username,
            email,
            adopted_nfts: Vec::new(),
            rewards_points: 0,
            created_at: ic_cdk::api::time(),
            last_login: ic_cdk::api::time(),
            profile_image: None,
            conservation_contributions: Vec::new(),
        };

        users.borrow_mut().insert(caller, new_user);
        Ok(caller)
    })
}

// NFT Management
#[update]
async fn mint_nft(metadata: NFTMetadata, entity_type: EntityType) -> Result<String> {
    let caller = ic_cdk::caller();
    ensure_authorized(caller)?;

    let nft_id = generate_unique_nft_id();
    let timestamp = ic_cdk::api::time();

    let nft = NFT {
        id: nft_id.clone(),
        entity_type,
        metadata,
        owner: Some(caller),
        price: None,
        creation_date: timestamp,
        transaction_history: vec![Transaction {
            transaction_type: TransactionType::Mint,
            from: Principal::anonymous(),
            to: caller,
            price: 0,
            timestamp,
            transaction_hash: generate_transaction_hash(timestamp, &caller),
        }],
        conservation_data: ConservationData {
            status: ConservationStatus::LeastConcern,
            population_trend: PopulationTrend::Unknown,
            threats: Vec::new(),
            conservation_actions: Vec::new(),
            last_updated: timestamp,
        },
    };

    state::NFTS.with(|nfts| {
        nfts.borrow_mut().insert(nft_id.clone(), nft);
        Ok(nft_id)
    })
}

// Marketplace
#[update]
async fn list_nft_for_sale(nft_id: String, price: u64) -> Result<()> {
    let caller = ic_cdk::caller();
    ensure_authorized(caller)?;

    state::NFTS.with(|nfts| {
        let mut nfts = nfts.borrow_mut();
        let nft = nfts.get_mut(&nft_id)
            .ok_or(TerraPulseError::NFTNotFound)?;

        if nft.owner != Some(caller) {
            return Err(TerraPulseError::NotAuthorized);
        }

        nft.price = Some(price);
        state::MARKETPLACE_LISTINGS.with(|listings| {
            listings.borrow_mut().insert(nft_id, price);
        });
        Ok(())
    })
}

#[update]
async fn purchase_nft(nft_id: String) -> Result<()> {
    let buyer = ic_cdk::caller();
    ensure_authorized(buyer)?;

    state::NFTS.with(|nfts| {
        let mut nfts = nfts.borrow_mut();
        let nft = nfts.get_mut(&nft_id)
            .ok_or(TerraPulseError::NFTNotFound)?;

        let price = nft.price
            .ok_or(TerraPulseError::InvalidOperation("NFT not for sale".into()))?;

        // Here you would implement actual payment logic using the ICP ledger
        // For now, we'll simulate the transfer

        let timestamp = ic_cdk::api::time();
        let seller = nft.owner
            .ok_or(TerraPulseError::SystemError("No owner found".into()))?;

        nft.owner = Some(buyer);
        nft.price = None;
        nft.transaction_history.push(Transaction {
            transaction_type: TransactionType::Sale,
            from: seller,
            to: buyer,
            price,
            timestamp,
            transaction_hash: generate_transaction_hash(timestamp, &buyer),
        });

        state::MARKETPLACE_LISTINGS.with(|listings| {
            listings.borrow_mut().remove(&nft_id);
        });

        // Update user records
        update_user_records(buyer, seller, &nft_id, price)?;

        Ok(())
    })
}

// Helper Functions
fn ensure_authorized(principal: Principal) -> Result<()> {
    if principal == Principal::anonymous() {
        Err(TerraPulseError::NotAuthorized)
    } else {
        Ok(())
    }
}

fn generate_unique_nft_id() -> String {
    use sha2::{Sha256, Digest};
    let timestamp = ic_cdk::api::time();
    let random = ic_cdk::api::call::arg_data_raw();
    let mut hasher = Sha256::new();
    hasher.update(timestamp.to_be_bytes());
    hasher.update(&random);
    let result = hasher.finalize();
    hex::encode(&result[..8])
}

fn generate_transaction_hash(timestamp: u64, principal: &Principal) -> String {
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(timestamp.to_be_bytes());
    hasher.update(principal.as_slice());
    let result = hasher.finalize();
    hex::encode(&result[..16])
}

fn update_user_records(buyer: Principal, seller: Principal, nft_id: &str, price: u64) -> Result<()> {
    state::USERS.with(|users| {
        let mut users = users.borrow_mut();
        
        // Update buyer's records
        if let Some(buyer_profile) = users.get_mut(&buyer) {
            buyer_profile.adopted_nfts.push(nft_id.to_string());
            buyer_profile.rewards_points += calculate_rewards(price);
        }

        // Update seller's records
        if let Some(seller_profile) = users.get_mut(&seller) {
            seller_profile.adopted_nfts.retain(|id| id != nft_id);
        }

        Ok(())
    })
}

fn calculate_rewards(price: u64) -> u64 {
    // Implement your rewards calculation logic
    price / 100 // Simple 1% rewards
}

// Query Methods
#[query]
fn get_nft(nft_id: String) -> Result<NFT> {
    state::NFTS.with(|nfts| {
        nfts.borrow()
            .get(&nft_id)
            .cloned()
            .ok_or(TerraPulseError::NFTNotFound)
    })
}

#[query]
fn get_user_profile(principal_id: Principal) -> Result<User> {
    state::USERS.with(|users| {
        users.borrow()
            .get(&principal_id)
            .cloned()
            .ok_or(TerraPulseError::UserNotFound)
    })
}

#[query]
fn get_marketplace_listings() -> Vec<(String, u64)> {
    state::MARKETPLACE_LISTINGS.with(|listings| {
        listings.borrow()
            .iter()
            .map(|(id, price)| (id.clone(), *price))
            .collect()
    })
}

// Export Candid interface
#[query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    candid::export_service!();
    __export_service()
}

// System initialization
#[init]
fn init() {
    // Initialize any necessary state
    ic_cdk::setup();
}

#[post_upgrade]
fn post_upgrade() {
    // Handle any necessary state migrations after upgrades
    ic_cdk::setup();
}