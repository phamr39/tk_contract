// pub use crate::actions_of_cluster::*;
pub use crate::game::*;
pub use crate::constants::*;
pub use crate::utils::*;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedMap, LookupMap};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    // env, near_bindgen, setup_alloc, AccountId, Balance, BorshStorageKey, Gas, PanicOnDefault,
    env, near_bindgen, AccountId, Balance, PanicOnDefault, BorshStorageKey, Promise, 
};

// mod actions_of_cluster;
mod game;
mod constants;
mod utils;

// setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub owner_id: AccountId,
    // pub players_in_game: LookupMap<PlayerId, UnorderedSet<AccountId>>,
    pub lottery_games: LookupMap<String, String>,
    pub game_metadata: UnorderedMap<GameId, GameMetaData>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        Self {
            owner_id: env::current_account_id(),
            // players_in_game: LookupMap::new(StorageKey::PlayerInGame),
            lottery_games: LookupMap::new(StorageKey::LotteryGame),
            game_metadata: UnorderedMap::new(StorageKey::GameMetadata),
        }
    }
}
