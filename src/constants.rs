use crate::*;

pub const NEAR_DECIMAL: Balance = 1_000_000_000_000_000_000_000_000;
pub const JOIN_DEFAULT_FEE: Balance = 1_000_000_000_000_000_000_000_000;
pub const EXECUTION_CASH_BACK_FEE: Balance = 500_000_000_000_000_000_000_000;

pub type GameId = String;
pub type PlayerId = String;
pub static CURRENT_GAME_ID: &str = "current_game";
pub static PREVIOUS_GAME_ID: &str = "previous_game";
pub static REWARD_PERCENT: &str = "90";

#[derive(BorshStorageKey, BorshSerialize)]
pub(crate) enum StorageKey {
    LotteryGame,
    // LotteryPlayer,
    GameMetadata,
}
