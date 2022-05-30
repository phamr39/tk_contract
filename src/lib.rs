pub use crate::actions_of_cluster::*;
pub use crate::cluster::*;
pub use crate::constants::*;
pub use crate::utils::*;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, near_bindgen, setup_alloc, AccountId, Balance, BorshStorageKey, Gas, PanicOnDefault,
};

mod actions_of_cluster;
mod cluster;
mod constants;
mod utils;

setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    // pub owner_id: AccountId,
    pub player_per_game: LookupMap<GameId, UnorderedSet<AccountId>>,
    pub lotery_game: LookupMap<GameId, Cluster>,
    pub cluster_metadata: UnorderedMap<ClusterId, ClusterMetaData>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        Self {
            // owner_id: env::predecessor_account_id(),
            cluster_per_owner: LookupMap::new(StorageKey::ClusterPerOwner),
            cluster: LookupMap::new(StorageKey::Cluster),
            cluster_metadata: UnorderedMap::new(StorageKey::ClusterMetadata),
        }
    }
}
