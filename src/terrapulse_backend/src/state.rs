pub(crate) use std::cell::RefCell;
use std::collections::HashMap;

thread_local! {
    static USERS: RefCell<HashMap<Principal, User>> = RefCell::new(HashMap::new());
    static NFTS: RefCell<HashMap<String, NFT>> = RefCell::new(HashMap::new());
    static MARKETPLACE_LISTINGS: RefCell<HashMap<String, u64>> = RefCell::new(HashMap::new());
    static CONSERVATION_PROJECTS: RefCell<HashMap<String, ConservationProject>> = RefCell::new(HashMap::new());
}