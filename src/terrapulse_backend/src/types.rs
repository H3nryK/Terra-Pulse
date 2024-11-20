use candid::{CandidType, Deserialize, Principal};
use serde::Serialize;
use std::collections::HashMap;

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct User {
    pub principal_id: Principal,
    pub username: String,
    pub email: Option<String>,
    pub adopted_nfts: Vec<String>,
    pub rewards_points: u64,
    pub created_at: u64,
    pub last_login: u64,
    pub profile_image: Option<String>,
    pub conservation_contributions: Vec<Contribution>,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct Contribution {
    pub amount: u64,
    pub project_id: String,
    pub timestamp: u64,
    pub transaction_hash: String,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct NFT {
    pub id: String,
    pub entity_type: EntityType,
    pub metadata: NFTMetadata,
    pub owner: Option<Principal>,
    pub price: Option<u64>,
    pub creation_date: u64,
    pub transaction_history: Vec<Transaction>,
    pub conservation_data: ConservationData,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct NFTMetadata {
    pub name: String,
    pub description: String,
    pub image_url: String,
    pub conservation_status: ConservationStatus,
    pub location: Location,
    pub attributes: HashMap<String, String>,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
    pub region: String,
    pub country: String,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct Transaction {
    pub transaction_type: TransactionType,
    pub from: Principal,
    pub to: Principal,
    pub price: u64,
    pub timestamp: u64,
    pub transaction_hash: String,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct ConservationData {
    pub status: ConservationStatus,
    pub population_trend: PopulationTrend,
    pub threats: Vec<String>,
    pub conservation_actions: Vec<String>,
    pub last_updated: u64,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub enum EntityType {
    Wildlife { species: String, category: String },
    Hotel { star_rating: u8, eco_rating: u8 },
    Reserve { area_size: u64, habitat_type: String },
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub enum ConservationStatus {
    LeastConcern,
    NearThreatened,
    Vulnerable,
    Endangered,
    CriticallyEndangered,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub enum PopulationTrend {
    Increasing,
    Stable,
    Decreasing,
    Unknown,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub enum TransactionType {
    Mint,
    Transfer,
    Sale,
    Adoption,
}